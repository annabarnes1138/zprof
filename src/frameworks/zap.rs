//! zap framework detection
//!
//! Detects zap installations by looking for ~/.local/share/zap directory
//! and parsing ~/.zshrc for zap plugin declarations.

use super::{get_home_dir, Framework, FrameworkInfo, FrameworkType, Plugin, Theme};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct Zap;


impl Framework for Zap {
    fn name(&self) -> &str {
        "zap"
    }

    fn detect() -> Option<FrameworkInfo> {
        let home = get_home_dir()?;
        let install_path = home.join(".local/share/zap");
        let config_path = home.join(".zshrc");

        // Check if zap directory exists
        if !install_path.exists() {
            return None;
        }

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

        // Verify zap is actually configured (should have zap source line)
        if !content.contains("zap") && !content.contains("plug ") {
            return None;
        }

        // Extract plugins and theme
        let (plugins, theme) = extract_zap_plugins(&content);

        Some(FrameworkInfo {
            framework_type: FrameworkType::Zap,
            plugins,
            theme,
            config_path,
            install_path,
        })
    }

    fn install(_profile_path: &Path) -> Result<()> {
        unimplemented!("zap installation not yet implemented")
    }

    fn get_plugins() -> Vec<Plugin> {
        vec![]
    }

    fn get_themes() -> Vec<Theme> {
        vec![]
    }
}

/// Extracts plugins and theme from zap .zshrc content
///
/// Looks for patterns like:
/// - plug "zsh-users/zsh-autosuggestions"
/// - plug "zsh-users/zsh-syntax-highlighting"
/// Processes line-by-line to prevent ReDoS attacks
fn extract_zap_plugins(content: &str) -> (Vec<String>, String) {
    const MAX_LINES: usize = 10000; // Limit lines processed

    let mut plugins = Vec::new();
    let mut theme = String::from("default");

    for line in content.lines().take(MAX_LINES) {
        let trimmed = line.trim_start();

        // Skip comments
        if trimmed.starts_with('#') {
            continue;
        }

        if trimmed.starts_with("plug") {
            // Extract plugin name from quotes (both " and ')
            for quote_char in &['"', '\''] {
                if let Some(start) = trimmed.find(*quote_char) {
                    if let Some(end) = trimmed[start + 1..].find(*quote_char) {
                        let plugin_str = &trimmed[start + 1..start + 1 + end];

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
                        break;
                    }
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
    fn test_extract_zap_plugins_double_quotes() {
        let content = r#"
plug "zsh-users/zsh-autosuggestions"
plug "zsh-users/zsh-syntax-highlighting"
"#;
        let (plugins, _) = extract_zap_plugins(content);
        assert_eq!(plugins.len(), 2);
        assert!(plugins.contains(&"zsh-autosuggestions".to_string()));
        assert!(plugins.contains(&"zsh-syntax-highlighting".to_string()));
    }

    #[test]
    fn test_extract_zap_plugins_single_quotes() {
        let content = r#"
plug 'zsh-users/zsh-autosuggestions'
plug 'zsh-users/zsh-syntax-highlighting'
"#;
        let (plugins, _) = extract_zap_plugins(content);
        assert_eq!(plugins.len(), 2);
    }

    #[test]
    fn test_extract_zap_with_theme() {
        let content = r#"
plug "romkatv/powerlevel10k"
plug "zsh-users/zsh-autosuggestions"
"#;
        let (plugins, theme) = extract_zap_plugins(content);
        assert_eq!(theme, "powerlevel10k");
        assert_eq!(plugins.len(), 1);
        assert!(plugins.contains(&"zsh-autosuggestions".to_string()));
    }

    #[test]
    fn test_extract_zap_simple_names() {
        let content = r#"
plug "zap-zsh/supercharge"
plug "zap-zsh/completions"
"#;
        let (plugins, _) = extract_zap_plugins(content);
        assert!(plugins.contains(&"supercharge".to_string()));
        assert!(plugins.contains(&"completions".to_string()));
    }

    #[test]
    fn test_extract_zap_empty() {
        let content = "# No zap plugins";
        let (plugins, theme) = extract_zap_plugins(content);
        assert!(plugins.is_empty());
        assert_eq!(theme, "default");
    }

    #[test]
    fn test_extract_zap_with_comments() {
        let content = r#"
# plug "commented-out-plugin"
plug "zsh-users/zsh-autosuggestions"
"#;
        let (plugins, _) = extract_zap_plugins(content);
        assert_eq!(plugins.len(), 1);
    }

    #[test]
    fn test_extract_zap_with_spaces() {
        let content = r#"  plug  "zsh-users/zsh-autosuggestions"  "#;
        let (plugins, _) = extract_zap_plugins(content);
        assert_eq!(plugins.len(), 1);
    }
}
