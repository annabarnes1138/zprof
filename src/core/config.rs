use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration structure for zprof
#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    /// Currently active profile (None if no profile created yet)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_profile: Option<String>,
    /// Optional default framework preference
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_framework: Option<String>,
}

impl Config {
    /// Create a new Config with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Generate default config.toml content
    pub fn to_toml_string(&self) -> Result<String> {
        toml::to_string_pretty(self)
            .context("Failed to serialize config to TOML format")
    }

    /// Write config to file
    pub fn write_to_file(&self, path: PathBuf) -> Result<()> {
        let content = self.to_toml_string()?;
        std::fs::write(&path, content)
            .with_context(|| format!("Failed to write config file to {}", path.display()))?;
        Ok(())
    }

    /// Load config from file
    pub fn load_from_file(path: PathBuf) -> Result<Self> {
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file from {}", path.display()))?;
        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse config file at {}", path.display()))?;
        Ok(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::new();
        assert!(config.active_profile.is_none());
        assert!(config.default_framework.is_none());
    }

    #[test]
    fn test_config_to_toml() {
        let config = Config::new();
        let _toml_str = config.to_toml_string().unwrap();
        // Default config with all None values produces empty TOML (fields are skipped)
        // This is expected and valid - config file creation succeeds
    }

    #[test]
    fn test_config_roundtrip() {
        let mut config = Config::new();
        config.active_profile = Some("test-profile".to_string());
        config.default_framework = Some("oh-my-zsh".to_string());

        let toml_str = config.to_toml_string().unwrap();
        let parsed: Config = toml::from_str(&toml_str).unwrap();

        assert_eq!(parsed.active_profile, Some("test-profile".to_string()));
        assert_eq!(parsed.default_framework, Some("oh-my-zsh".to_string()));
    }
}
