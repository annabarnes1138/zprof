//! oh-my-zsh framework detection
//!
//! Detects oh-my-zsh installations by looking for ~/.oh-my-zsh directory
//! and parsing ~/.zshrc for plugin and theme configuration.

use super::{get_home_dir, Framework, FrameworkInfo, FrameworkType, Plugin, PluginCategory, Theme};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct OhMyZsh;

impl Framework for OhMyZsh {
    fn name(&self) -> &str {
        "oh-my-zsh"
    }

    fn detect() -> Option<FrameworkInfo> {
        let home = get_home_dir()?;
        let install_path = home.join(".oh-my-zsh");
        let config_path = home.join(".zshrc");

        // Check if oh-my-zsh directory exists
        if !install_path.exists() {
            return None;
        }

        // Check if .zshrc exists and contains oh-my-zsh configuration
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

        // Verify this is actually an oh-my-zsh installation
        if !content.contains("oh-my-zsh.sh") && !content.contains("$ZSH/oh-my-zsh.sh") {
            return None;
        }

        // Extract plugins
        let plugins = extract_plugins(&content);

        // Extract theme
        let theme = extract_theme(&content);

        Some(FrameworkInfo {
            framework_type: FrameworkType::OhMyZsh,
            plugins,
            theme,
            config_path,
            install_path,
        })
    }

    fn install(profile_path: &Path) -> Result<()> {
        // Forward to the installer module for actual implementation
        crate::frameworks::installer::install_framework(&FrameworkType::OhMyZsh, profile_path)
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
                description: "Docker aliases and completion".to_string(),
                category: PluginCategory::Docker,
            },
            Plugin {
                name: "kubectl".to_string(),
                description: "Kubernetes kubectl aliases and completion".to_string(),
                category: PluginCategory::Kubernetes,
            },
            Plugin {
                name: "rust".to_string(),
                description: "Rust language support and cargo aliases".to_string(),
                category: PluginCategory::Language,
            },
            Plugin {
                name: "fzf".to_string(),
                description: "Fuzzy file finder integration".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "zsh-autosuggestions".to_string(),
                description: "Fish-like autosuggestions for zsh".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "zsh-syntax-highlighting".to_string(),
                description: "Syntax highlighting for commands".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "z".to_string(),
                description: "Jump to frequently used directories".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "nvm".to_string(),
                description: "Node Version Manager integration".to_string(),
                category: PluginCategory::Language,
            },
            Plugin {
                name: "brew".to_string(),
                description: "Homebrew completion and aliases".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "sudo".to_string(),
                description: "Prefix last command with sudo using ESC-ESC".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "history".to_string(),
                description: "History search and aliases".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "aws".to_string(),
                description: "AWS CLI completion".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "terraform".to_string(),
                description: "Terraform completion and aliases".to_string(),
                category: PluginCategory::Utility,
            },
            Plugin {
                name: "node".to_string(),
                description: "Node.js aliases and utilities".to_string(),
                category: PluginCategory::Language,
            },
        ]
    }

    fn get_themes() -> Vec<Theme> {
        vec![
            Theme {
                name: "robbyrussell".to_string(),
                description: "Default oh-my-zsh theme, minimal and fast".to_string(),
                preview: "➜ user@host:~/dir (git:main)".to_string(),
            },
            Theme {
                name: "agnoster".to_string(),
                description: "Powerline-style theme with git info".to_string(),
                preview: "Powerline arrows, git status, dark background".to_string(),
            },
            Theme {
                name: "powerlevel10k/powerlevel10k".to_string(),
                description: "Fast, customizable, feature-rich theme".to_string(),
                preview: "Highly customizable, extensive git integration".to_string(),
            },
            Theme {
                name: "pure".to_string(),
                description: "Minimal, fast, asynchronous prompt".to_string(),
                preview: "Clean single-line prompt with git status".to_string(),
            },
            Theme {
                name: "spaceship".to_string(),
                description: "Minimalist, powerful prompt for astronauts".to_string(),
                preview: "Modern icons, git info, execution time".to_string(),
            },
            Theme {
                name: "bullet-train".to_string(),
                description: "Powerline-inspired with customizable cars".to_string(),
                preview: "Modular sections, colorful, git integration".to_string(),
            },
            Theme {
                name: "af-magic".to_string(),
                description: "Two-line theme with timestamp and git".to_string(),
                preview: "Timestamp, path, git status on separate lines".to_string(),
            },
            Theme {
                name: "bira".to_string(),
                description: "Two-line theme with user and time".to_string(),
                preview: "User@host, time, path, git on two lines".to_string(),
            },
            Theme {
                name: "cloud".to_string(),
                description: "Minimalist cloud-inspired theme".to_string(),
                preview: "Simple, clean, hostname and path".to_string(),
            },
            Theme {
                name: "avit".to_string(),
                description: "Clean theme with git status colors".to_string(),
                preview: "Colored git indicators, clean layout".to_string(),
            },
        ]
    }
}

/// Extracts plugins from oh-my-zsh .zshrc content
///
/// Looks for patterns like: plugins=(git docker kubectl)
/// Processes line-by-line to prevent ReDoS attacks
fn extract_plugins(content: &str) -> Vec<String> {
    const MAX_LINES: usize = 10000; // Limit lines processed
    const MAX_PLUGINS: usize = 200; // Reasonable limit for plugin count
    const MAX_NAME_LEN: usize = 128; // Max length for a plugin name

    let mut plugins = Vec::new();
    let mut in_plugins_block = false;

    for line in content.lines().take(MAX_LINES) {
        let trimmed = line.trim();

        if trimmed.starts_with("plugins") && trimmed.contains('(') {
            in_plugins_block = true;

            // Check if single-line declaration: plugins=(git docker kubectl)
            if let Some(start) = trimmed.find('(') {
                if let Some(end) = trimmed.find(')') {
                    if start < end {
                        let plugins_str = &trimmed[start + 1..end];
                        return plugins_str
                            .split_whitespace()
                            .filter(|s| !s.is_empty() && s.len() <= MAX_NAME_LEN)
                            .take(MAX_PLUGINS)
                            .map(|s| s.to_string())
                            .collect();
                    }
                } else {
                    // Opening paren found but no closing - multiline
                    let start_str = &trimmed[start + 1..];
                    for plugin in start_str
                        .split_whitespace()
                        .filter(|s| !s.is_empty() && s.len() <= MAX_NAME_LEN)
                    {
                        if plugins.len() >= MAX_PLUGINS {
                            break;
                        }
                        plugins.push(plugin.to_string());
                    }
                }
            }
        } else if in_plugins_block {
            // We're in a multiline plugins block
            if trimmed.contains(')') {
                // End of plugins block
                let end = trimmed
                    .find(')')
                    .expect("')' must exist as we just checked with contains");

                for plugin in trimmed[..end]
                    .split_whitespace()
                    .filter(|s| !s.is_empty() && s.len() <= MAX_NAME_LEN)
                {
                    if plugins.len() >= MAX_PLUGINS {
                        break;
                    }
                    plugins.push(plugin.to_string());
                }
                return plugins;
            } else {
                // Still in the block, collect plugins
                for plugin in trimmed
                    .split_whitespace()
                    .filter(|s| !s.is_empty() && s.len() <= MAX_NAME_LEN)
                {
                    if plugins.len() >= MAX_PLUGINS {
                        break;
                    }
                    plugins.push(plugin.to_string());
                }
            }
        }
    }

    plugins
}

/// Extracts theme from oh-my-zsh .zshrc content
///
/// Looks for patterns like: ZSH_THEME="robbyrussell"
/// Processes line-by-line to prevent ReDoS attacks
fn extract_theme(content: &str) -> String {
    const MAX_LINES: usize = 10000; // Limit lines processed

    for line in content.lines().take(MAX_LINES) {
        let trimmed = line.trim_start();
        if trimmed.starts_with("ZSH_THEME") && trimmed.contains('=') {
            // Extract theme value between quotes
            if let Some(eq_pos) = trimmed.find('=') {
                let value_part = trimmed[eq_pos + 1..].trim();
                // Handle both double and single quotes
                for quote_char in &['"', '\''] {
                    if value_part.starts_with(*quote_char) {
                        if let Some(end_quote) = value_part[1..].find(*quote_char) {
                            return value_part[1..end_quote + 1].to_string();
                        }
                    }
                }
            }
        }
    }

    // Default theme if not found
    "robbyrussell".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_plugins_simple() {
        let content = "plugins=(git docker kubectl)";
        let plugins = extract_plugins(content);
        assert_eq!(plugins, vec!["git", "docker", "kubectl"]);
    }

    #[test]
    fn test_extract_plugins_multiline() {
        let content = r#"
plugins=(
    git
    docker
    kubectl
)
"#;
        let plugins = extract_plugins(content);
        assert_eq!(plugins, vec!["git", "docker", "kubectl"]);
    }

    #[test]
    fn test_extract_plugins_with_spaces() {
        let content = "plugins=( git  docker   kubectl )";
        let plugins = extract_plugins(content);
        assert_eq!(plugins, vec!["git", "docker", "kubectl"]);
    }

    #[test]
    fn test_extract_plugins_empty() {
        let content = "plugins=()";
        let plugins = extract_plugins(content);
        assert!(plugins.is_empty());
    }

    #[test]
    fn test_extract_plugins_not_found() {
        let content = "# No plugins here";
        let plugins = extract_plugins(content);
        assert!(plugins.is_empty());
    }

    #[test]
    fn test_extract_theme_double_quotes() {
        let content = r#"ZSH_THEME="robbyrussell""#;
        let theme = extract_theme(content);
        assert_eq!(theme, "robbyrussell");
    }

    #[test]
    fn test_extract_theme_single_quotes() {
        let content = "ZSH_THEME='agnoster'";
        let theme = extract_theme(content);
        assert_eq!(theme, "agnoster");
    }

    #[test]
    fn test_extract_theme_with_spaces() {
        let content = r#"  ZSH_THEME = "powerlevel10k/powerlevel10k"  "#;
        let theme = extract_theme(content);
        assert_eq!(theme, "powerlevel10k/powerlevel10k");
    }

    #[test]
    fn test_extract_theme_not_found() {
        let content = "# No theme here";
        let theme = extract_theme(content);
        assert_eq!(theme, "robbyrussell"); // Default
    }
}
