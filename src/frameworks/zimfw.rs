//! zimfw framework detection
//!
//! Detects zimfw installations by looking for ~/.zim or ~/.zimfw directory
//! and parsing ~/.zimrc for module configuration.

use super::{get_home_dir, Framework, FrameworkInfo, FrameworkType, Plugin, PluginCategory, Theme};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct Zimfw;


impl Framework for Zimfw {
    fn name(&self) -> &str {
        "zimfw"
    }

    fn detect() -> Option<FrameworkInfo> {
        let home = get_home_dir()?;

        // Check for either ~/.zim or ~/.zimfw
        let install_path = if home.join(".zim").exists() {
            home.join(".zim")
        } else if home.join(".zimfw").exists() {
            home.join(".zimfw")
        } else {
            return None;
        };

        let config_path = home.join(".zimrc");

        // Check if .zimrc exists
        if !config_path.exists() {
            return None;
        }

        // Check file size before reading to prevent memory exhaustion
        const MAX_CONFIG_SIZE: u64 = 1_048_576; // 1MB limit
        match fs::metadata(&config_path) {
            Ok(metadata) => {
                if metadata.len() > MAX_CONFIG_SIZE {
                    log::warn!(
                        "Config file too large ({} bytes): {:?}",
                        metadata.len(),
                        config_path
                    );
                    return None;
                }
            }
            Err(e) => {
                log::warn!("Could not read metadata for {:?}: {:#}", config_path, e);
                return None;
            }
        }

        // Read .zimrc content
        let content = match fs::read_to_string(&config_path) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("Could not read .zimrc at {:?}: {:#}", config_path, e);
                return None;
            }
        };

        // Extract modules (plugins) and theme
        let (plugins, theme) = extract_modules(&content);

        Some(FrameworkInfo {
            framework_type: FrameworkType::Zimfw,
            plugins,
            theme,
            config_path,
            install_path,
        })
    }

    fn install(_profile_path: &Path) -> Result<()> {
        unimplemented!("zimfw installation not yet implemented")
    }

    fn get_plugins() -> Vec<Plugin> {
        vec![
            Plugin {
                name: "git".to_string(),
                description: "Git aliases and functions".to_string(),
                category: PluginCategory::Git,
            },
            Plugin {
                name: "docker".to_string(),
                description: "Docker completion and aliases".to_string(),
                category: PluginCategory::Docker,
            },
            Plugin {
                name: "kubectl".to_string(),
                description: "Kubectl completion and aliases".to_string(),
                category: PluginCategory::Kubernetes,
            },
            Plugin {
                name: "rust".to_string(),
                description: "Rust and Cargo completions".to_string(),
                category: PluginCategory::Language,
            },
            Plugin {
                name: "fzf".to_string(),
                description: "Fuzzy finder integration".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "zsh-autosuggestions".to_string(),
                description: "Fish-like autosuggestions".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "zsh-syntax-highlighting".to_string(),
                description: "Syntax highlighting for zsh".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "zoxide".to_string(),
                description: "Smarter cd command that learns your habits".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "node".to_string(),
                description: "Node.js completion and utilities".to_string(),
                category: PluginCategory::Language,
            },
            Plugin {
                name: "python".to_string(),
                description: "Python virtual environment support".to_string(),
                category: PluginCategory::Language,
            },
            Plugin {
                name: "exa".to_string(),
                description: "Modern replacement for ls".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "archive".to_string(),
                description: "Archive extraction utilities".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "ssh".to_string(),
                description: "SSH completion and utilities".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "terraform".to_string(),
                description: "Terraform completion".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "completion".to_string(),
                description: "Enhanced tab completion".to_string(),
                category: PluginCategory::Utility,
            },
        ]
    }

    fn get_themes() -> Vec<Theme> {
        vec![]
    }
}

/// Extracts modules and theme from zimfw .zimrc content
///
/// Looks for patterns like:
/// - zmodule ohmyzsh/ohmyzsh --root plugins/git
/// - zmodule romkatv/powerlevel10k
/// Processes line-by-line to prevent ReDoS attacks
fn extract_modules(content: &str) -> (Vec<String>, String) {
    const MAX_LINES: usize = 10000; // Limit lines processed

    let mut plugins = Vec::new();
    let mut theme = String::from("default");

    for line in content.lines().take(MAX_LINES) {
        let trimmed = line.trim_start();

        // Skip comments
        if trimmed.starts_with('#') {
            continue;
        }

        if trimmed.starts_with("zmodule") {
            // Extract module name (first token after zmodule)
            let parts: Vec<&str> = trimmed.split_whitespace().collect();
            if parts.len() >= 2 {
                let module_str = parts[1];

                // Check if this is a theme (common theme modules)
                if module_str.contains("powerlevel10k")
                    || module_str.contains("pure")
                    || module_str.contains("starship")
                    || module_str.contains("agnoster")
                {
                    theme = module_str
                        .split('/')
                        .last()
                        .unwrap_or("default")
                        .to_string();
                } else {
                    // Extract plugin name from module path
                    let plugin_name = if line.contains("--root plugins/") {
                        // Handle ohmyzsh/ohmyzsh --root plugins/git case
                        line.split("plugins/")
                            .nth(1)
                            .and_then(|s| s.split_whitespace().next())
                            .unwrap_or(module_str)
                    } else {
                        // Use last component of module path
                        module_str.split('/').last().unwrap_or(module_str)
                    };
                    plugins.push(plugin_name.to_string());
                }
            }
        }
    }

    (plugins, theme)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_modules_ohmyzsh_style() {
        let content = r#"
zmodule ohmyzsh/ohmyzsh --root plugins/git
zmodule ohmyzsh/ohmyzsh --root plugins/docker
"#;
        let (plugins, _) = extract_modules(content);
        assert!(plugins.contains(&"git".to_string()));
        assert!(plugins.contains(&"docker".to_string()));
    }

    #[test]
    fn test_extract_modules_with_theme() {
        let content = r#"
zmodule romkatv/powerlevel10k
zmodule zsh-users/zsh-autosuggestions
"#;
        let (plugins, theme) = extract_modules(content);
        assert_eq!(theme, "powerlevel10k");
        assert!(plugins.contains(&"zsh-autosuggestions".to_string()));
    }

    #[test]
    fn test_extract_modules_simple() {
        let content = r#"
zmodule zsh-users/zsh-syntax-highlighting
zmodule zsh-users/zsh-autosuggestions
"#;
        let (plugins, _) = extract_modules(content);
        assert_eq!(plugins.len(), 2);
        assert!(plugins.contains(&"zsh-syntax-highlighting".to_string()));
        assert!(plugins.contains(&"zsh-autosuggestions".to_string()));
    }

    #[test]
    fn test_extract_modules_empty() {
        let content = "# No modules";
        let (plugins, theme) = extract_modules(content);
        assert!(plugins.is_empty());
        assert_eq!(theme, "default");
    }

    #[test]
    fn test_extract_modules_with_comments() {
        let content = r#"
# This is a comment
zmodule zsh-users/zsh-syntax-highlighting
# zmodule commented-out-plugin
zmodule zsh-users/zsh-autosuggestions
"#;
        let (plugins, _) = extract_modules(content);
        assert_eq!(plugins.len(), 2);
    }
}
