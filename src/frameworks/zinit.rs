//! zinit framework detection
//!
//! Detects zinit installations by looking for ~/.zinit or ~/.local/share/zinit directory
//! and parsing ~/.zshrc for zinit plugin declarations.

use super::{get_home_dir, Framework, FrameworkInfo, FrameworkType, Plugin, PluginCategory, Theme};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct Zinit;


impl Framework for Zinit {
    fn name(&self) -> &str {
        "zinit"
    }

    fn detect() -> Option<FrameworkInfo> {
        let home = get_home_dir()?;

        // Check for either ~/.zinit or ~/.local/share/zinit
        let install_path = if home.join(".zinit").exists() {
            home.join(".zinit")
        } else if home.join(".local/share/zinit").exists() {
            home.join(".local/share/zinit")
        } else {
            return None;
        };

        let config_path = home.join(".zshrc");

        // Check if .zshrc exists
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

        // Read .zshrc content
        let content = match fs::read_to_string(&config_path) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("Could not read .zshrc at {:?}: {:#}", config_path, e);
                return None;
            }
        };

        // Verify zinit is actually configured
        if !content.contains("zinit") {
            return None;
        }

        // Extract plugins and theme
        let (plugins, theme) = extract_zinit_plugins(&content);

        Some(FrameworkInfo {
            framework_type: FrameworkType::Zinit,
            plugins,
            theme,
            config_path,
            install_path,
        })
    }

    fn install(_profile_path: &Path) -> Result<()> {
        unimplemented!("zinit installation not yet implemented")
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
                description: "Docker completion and utilities".to_string(),
                category: PluginCategory::Docker,
            },
            Plugin {
                name: "kubectl".to_string(),
                description: "Kubernetes completion".to_string(),
                category: PluginCategory::Kubernetes,
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
                name: "fast-syntax-highlighting".to_string(),
                description: "Fast syntax highlighting for zinit".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "zsh-completions".to_string(),
                description: "Additional completion definitions".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "history-search-multi-word".to_string(),
                description: "Multi-word history search".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "rust".to_string(),
                description: "Rust toolchain support".to_string(),
                category: PluginCategory::Language,
            },
            Plugin {
                name: "node".to_string(),
                description: "Node.js utilities".to_string(),
                category: PluginCategory::Language,
            },
            Plugin {
                name: "python".to_string(),
                description: "Python virtual environment support".to_string(),
                category: PluginCategory::Language,
            },
            Plugin {
                name: "direnv".to_string(),
                description: "Environment switcher for shell".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "aws".to_string(),
                description: "AWS CLI completion".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "terraform".to_string(),
                description: "Terraform completion".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "colored-man-pages".to_string(),
                description: "Colorized man pages".to_string(),
                category: PluginCategory::Utility,
            },
        ]
    }

    fn get_themes() -> Vec<Theme> {
        vec![]
    }
}

/// Extracts plugins and theme from zinit .zshrc content
///
/// Looks for patterns like:
/// - zinit load zdharma-continuum/fast-syntax-highlighting
/// - zinit light zsh-users/zsh-autosuggestions
/// - zinit ice lucid; zinit light romkatv/powerlevel10k
/// Processes line-by-line to prevent ReDoS attacks
fn extract_zinit_plugins(content: &str) -> (Vec<String>, String) {
    const MAX_LINES: usize = 10000; // Limit lines processed

    let mut plugins = Vec::new();
    let mut theme = String::from("default");

    for line in content.lines().take(MAX_LINES) {
        let trimmed = line.trim_start();

        // Skip comments
        if trimmed.starts_with('#') {
            continue;
        }

        if trimmed.starts_with("zinit") && (trimmed.contains("load") || trimmed.contains("light")) {
            // Extract plugin name (last token on line before comment)
            let parts: Vec<&str> = trimmed
                .split('#')
                .next()
                .unwrap_or("")
                .split_whitespace()
                .collect();

            if parts.len() >= 3 {
                let plugin_str = parts[parts.len() - 1];

                // Check if this is a theme (common theme plugins)
                if plugin_str.contains("powerlevel10k")
                    || plugin_str.contains("pure")
                    || plugin_str.contains("starship")
                    || plugin_str.contains("agnoster")
                {
                    theme = plugin_str
                        .split('/')
                        .last()
                        .unwrap_or("default")
                        .to_string();
                } else {
                    // Extract plugin name from path
                    let plugin_name = plugin_str.split('/').last().unwrap_or(plugin_str);
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
    fn test_extract_zinit_plugins_load() {
        let content = r#"
zinit load zdharma-continuum/fast-syntax-highlighting
zinit load zsh-users/zsh-autosuggestions
"#;
        let (plugins, _) = extract_zinit_plugins(content);
        assert_eq!(plugins.len(), 2);
        assert!(plugins.contains(&"fast-syntax-highlighting".to_string()));
        assert!(plugins.contains(&"zsh-autosuggestions".to_string()));
    }

    #[test]
    fn test_extract_zinit_plugins_light() {
        let content = r#"
zinit light zsh-users/zsh-syntax-highlighting
zinit light zsh-users/zsh-autosuggestions
"#;
        let (plugins, _) = extract_zinit_plugins(content);
        assert_eq!(plugins.len(), 2);
        assert!(plugins.contains(&"zsh-syntax-highlighting".to_string()));
        assert!(plugins.contains(&"zsh-autosuggestions".to_string()));
    }

    #[test]
    fn test_extract_zinit_with_theme() {
        let content = r#"
zinit light romkatv/powerlevel10k
zinit light zsh-users/zsh-autosuggestions
"#;
        let (plugins, theme) = extract_zinit_plugins(content);
        assert_eq!(theme, "powerlevel10k");
        assert_eq!(plugins.len(), 1);
        assert!(plugins.contains(&"zsh-autosuggestions".to_string()));
    }

    #[test]
    fn test_extract_zinit_mixed() {
        let content = r#"
zinit load zdharma-continuum/fast-syntax-highlighting
zinit light zsh-users/zsh-autosuggestions
"#;
        let (plugins, _) = extract_zinit_plugins(content);
        assert_eq!(plugins.len(), 2);
    }

    #[test]
    fn test_extract_zinit_empty() {
        let content = "# No zinit plugins";
        let (plugins, theme) = extract_zinit_plugins(content);
        assert!(plugins.is_empty());
        assert_eq!(theme, "default");
    }

    #[test]
    fn test_extract_zinit_with_comments() {
        let content = r#"
# zinit light commented-out-plugin
zinit light zsh-users/zsh-autosuggestions
"#;
        let (plugins, _) = extract_zinit_plugins(content);
        assert_eq!(plugins.len(), 1);
    }
}
