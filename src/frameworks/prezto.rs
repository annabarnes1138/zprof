//! prezto framework detection
//!
//! Detects prezto installations by looking for ~/.zprezto directory
//! and parsing ~/.zpreztorc for module and theme configuration.

use super::{get_home_dir, Framework, FrameworkInfo, FrameworkType, Plugin, Theme};
use anyhow::Result;
use std::fs;
use std::path::Path;

pub struct Prezto;


impl Framework for Prezto {
    fn name(&self) -> &str {
        "prezto"
    }

    fn detect() -> Option<FrameworkInfo> {
        let home = get_home_dir()?;
        let install_path = home.join(".zprezto");
        let config_path = home.join(".zpreztorc");

        // Check if prezto directory exists
        if !install_path.exists() {
            return None;
        }

        // Check if .zpreztorc exists
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
                log::warn!("Could not read metadata for {config_path:?}: {e:#}");
                return None;
            }
        }

        // Read .zpreztorc content
        let content = match fs::read_to_string(&config_path) {
            Ok(c) => c,
            Err(e) => {
                log::warn!("Could not read .zpreztorc at {config_path:?}: {e:#}");
                return None;
            }
        };

        // Extract modules (plugins)
        let plugins = extract_modules(&content);

        // Extract theme
        let theme = extract_theme(&content);

        Some(FrameworkInfo {
            framework_type: FrameworkType::Prezto,
            plugins,
            theme,
            config_path,
            install_path,
        })
    }

    fn install(profile_path: &Path) -> Result<()> {
        // Forward to the installer module for actual implementation
        crate::frameworks::installer::install_framework(&FrameworkType::Prezto, profile_path)
    }

    fn get_plugins() -> Vec<Plugin> {
        crate::frameworks::plugin::get_plugins_for_framework(&FrameworkType::Prezto)
    }

    fn get_themes() -> Vec<Theme> {
        crate::frameworks::theme::get_themes_for_framework(&FrameworkType::Prezto)
    }
}

/// Extracts modules from prezto .zpreztorc content
///
/// Looks for patterns like: zstyle ':prezto:load' pmodule 'git' 'docker'
/// Processes line-by-line to prevent ReDoS attacks
fn extract_modules(content: &str) -> Vec<String> {
    const MAX_LINES: usize = 10000; // Limit lines processed
    let mut modules = Vec::new();
    let mut in_pmodule_section = false;

    for line in content.lines().take(MAX_LINES) {
        let trimmed = line.trim();

        // Check if this line starts a pmodule declaration
        if trimmed.contains("':prezto:load'") && trimmed.contains("pmodule") {
            in_pmodule_section = true;
        }

        // Extract quoted module names
        if in_pmodule_section || (trimmed.contains("':prezto:load'") && trimmed.contains("pmodule")) {
            // Simple quote extraction without regex
            let chars = trimmed.chars().peekable();
            let mut in_quote = false;
            let mut current_module = String::new();

            for ch in chars {
                if ch == '\'' {
                    if in_quote {
                        // End of quoted string
                        if !current_module.is_empty()
                            && !current_module.contains(':')
                            && current_module != "pmodule"
                        {
                            modules.push(current_module.clone());
                        }
                        current_module.clear();
                        in_quote = false;
                    } else {
                        // Start of quoted string
                        in_quote = true;
                    }
                } else if in_quote {
                    current_module.push(ch);
                }
            }
        }

        // Check if line ends the multiline declaration
        if trimmed.ends_with(')') || (!trimmed.contains('\\') && in_pmodule_section) {
            in_pmodule_section = false;
        }
    }

    modules
}

/// Extracts theme from prezto .zpreztorc content
///
/// Looks for patterns like: zstyle ':prezto:module:prompt' theme 'powerlevel10k'
/// Processes line-by-line to prevent ReDoS attacks
fn extract_theme(content: &str) -> String {
    const MAX_LINES: usize = 10000; // Limit lines processed

    for line in content.lines().take(MAX_LINES) {
        let trimmed = line.trim_start();

        if trimmed.starts_with("zstyle")
            && trimmed.contains("':prezto:module:prompt'")
            && trimmed.contains("theme")
        {
            // Extract theme from quotes
            if let Some(last_quote_start) = trimmed.rfind('\'') {
                if let Some(prev_quote) = trimmed[..last_quote_start].rfind('\'') {
                    let theme = &trimmed[prev_quote + 1..last_quote_start];
                    if !theme.is_empty() {
                        return theme.to_string();
                    }
                }
            }
        }
    }

    // Default theme
    "sorin".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_modules_single_line() {
        let content = "zstyle ':prezto:load' pmodule 'git' 'docker' 'kubectl'";
        let modules = extract_modules(content);
        assert_eq!(modules, vec!["git", "docker", "kubectl"]);
    }

    #[test]
    fn test_extract_modules_multiline() {
        let content = r#"
zstyle ':prezto:load' pmodule \
  'environment' \
  'terminal' \
  'editor' \
  'git' \
  'docker'
"#;
        let modules = extract_modules(content);
        assert!(modules.contains(&"environment".to_string()));
        assert!(modules.contains(&"git".to_string()));
        assert!(modules.contains(&"docker".to_string()));
    }

    #[test]
    fn test_extract_modules_with_spaces() {
        let content = "  zstyle ':prezto:load' pmodule  'git'  'docker'  ";
        let modules = extract_modules(content);
        assert!(modules.contains(&"git".to_string()));
        assert!(modules.contains(&"docker".to_string()));
    }

    #[test]
    fn test_extract_modules_empty() {
        let content = "# No modules";
        let modules = extract_modules(content);
        assert!(modules.is_empty());
    }

    #[test]
    fn test_extract_theme_powerlevel10k() {
        let content = "zstyle ':prezto:module:prompt' theme 'powerlevel10k'";
        let theme = extract_theme(content);
        assert_eq!(theme, "powerlevel10k");
    }

    #[test]
    fn test_extract_theme_pure() {
        let content = "zstyle ':prezto:module:prompt' theme 'pure'";
        let theme = extract_theme(content);
        assert_eq!(theme, "pure");
    }

    #[test]
    fn test_extract_theme_with_spaces() {
        let content = "  zstyle ':prezto:module:prompt' theme 'agnoster'  ";
        let theme = extract_theme(content);
        assert_eq!(theme, "agnoster");
    }

    #[test]
    fn test_extract_theme_not_found() {
        let content = "# No theme";
        let theme = extract_theme(content);
        assert_eq!(theme, "sorin"); // Default
    }
}
