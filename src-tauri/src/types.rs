//! Type definitions for Tauri IPC commands
//!
//! These types are serialized to JSON and sent between the Rust backend
//! and the Svelte frontend. They must implement Serialize + Deserialize.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Profile information for display in profile list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileInfo {
    /// Profile name (unique identifier)
    pub name: String,
    /// Framework name (oh-my-zsh, zimfw, etc.)
    pub framework: String,
    /// Prompt mode: "prompt_engine" or "framework_theme"
    pub prompt_mode: String,
    /// Prompt engine name (if using prompt_engine mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_engine: Option<String>,
    /// Framework theme name (if using framework_theme mode)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_theme: Option<String>,
    /// Whether this profile is currently active
    pub active: bool,
    /// ISO 8601 timestamp when profile was created
    pub created_at: String,
    /// Number of enabled plugins
    pub plugin_count: usize,
}

/// Full profile details including configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileDetails {
    /// Profile name
    pub name: String,
    /// Framework name
    pub framework: String,
    /// Prompt mode discriminator
    pub prompt_mode: PromptModeInfo,
    /// List of enabled plugins
    pub plugins: Vec<String>,
    /// Environment variables
    pub env_vars: HashMap<String, String>,
    /// ISO 8601 timestamp when created
    pub created_at: String,
    /// ISO 8601 timestamp when last modified
    pub modified_at: String,
}

/// Prompt mode information for profile details
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum PromptModeInfo {
    /// Using a standalone prompt engine
    PromptEngine {
        /// Engine name (starship, powerlevel10k, etc.)
        engine: String,
    },
    /// Using framework's built-in theme system
    FrameworkTheme {
        /// Theme name (robbyrussell, agnoster, etc.)
        theme: String,
    },
}

/// Profile creation/update configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileConfig {
    /// Profile name (must be unique, lowercase, alphanumeric + hyphens)
    pub name: String,
    /// Framework to use
    pub framework: String,
    /// Prompt mode: "prompt_engine" or "framework_theme"
    pub prompt_mode: String,
    /// Prompt engine name (if prompt_mode = "prompt_engine")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_engine: Option<String>,
    /// Framework theme (if prompt_mode = "framework_theme")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework_theme: Option<String>,
    /// List of plugin names to enable
    #[serde(default)]
    pub plugins: Vec<String>,
    /// Environment variables to set
    #[serde(default)]
    pub env_vars: HashMap<String, String>,
}

impl ProfileConfig {
    /// Validate the profile configuration
    pub fn validate(&self) -> Result<(), String> {
        // Validate name
        if self.name.trim().is_empty() {
            return Err("Profile name cannot be empty".to_string());
        }

        if !self.name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err("Profile name must contain only alphanumeric characters, hyphens, and underscores".to_string());
        }

        // Validate framework
        const SUPPORTED_FRAMEWORKS: &[&str] = &["oh-my-zsh", "zimfw", "prezto", "zinit", "zap"];
        if !SUPPORTED_FRAMEWORKS.contains(&self.framework.as_str()) {
            return Err(format!(
                "Framework must be one of: {}",
                SUPPORTED_FRAMEWORKS.join(", ")
            ));
        }

        // Validate prompt mode
        match self.prompt_mode.as_str() {
            "prompt_engine" => {
                if self.prompt_engine.is_none() || self.prompt_engine.as_ref().unwrap().is_empty() {
                    return Err("prompt_engine is required when prompt_mode is 'prompt_engine'".to_string());
                }
            }
            "framework_theme" => {
                // framework_theme can be empty (no theme)
            }
            _ => {
                return Err("prompt_mode must be either 'prompt_engine' or 'framework_theme'".to_string());
            }
        }

        // Validate plugins (no empty strings)
        for plugin in &self.plugins {
            if plugin.trim().is_empty() {
                return Err("Plugin names cannot be empty".to_string());
            }
        }

        // Validate env var keys
        for key in self.env_vars.keys() {
            if key.trim().is_empty() {
                return Err("Environment variable names cannot be empty".to_string());
            }
            if !key.chars().all(|c| c.is_alphanumeric() || c == '_') {
                return Err(format!(
                    "Environment variable name '{}' contains invalid characters (use alphanumeric and underscores only)",
                    key
                ));
            }
        }

        Ok(())
    }
}

/// Framework information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FrameworkInfo {
    /// Framework name
    pub name: String,
    /// Human-readable description
    pub description: String,
    /// Whether this framework supports themes
    pub supports_themes: bool,
    /// Whether this framework supports plugins
    pub supports_plugins: bool,
}

/// Plugin information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    /// Plugin name
    pub name: String,
    /// Description of what the plugin does
    pub description: String,
    /// Category (git, docker, utility, etc.)
    pub category: String,
    /// Framework this plugin is for
    pub framework: String,
}

/// Theme information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeInfo {
    /// Theme name
    pub name: String,
    /// Description of the theme
    pub description: String,
    /// Framework this theme is for
    pub framework: String,
    /// Optional URL to preview image
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preview_url: Option<String>,
}

/// Prompt engine information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptEngineInfo {
    /// Engine name
    pub name: String,
    /// Description of the engine
    pub description: String,
    /// Whether this engine requires Nerd Fonts
    pub nerd_font_required: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_profile_config_validation_success() {
        let config = ProfileConfig {
            name: "work".to_string(),
            framework: "oh-my-zsh".to_string(),
            prompt_mode: "prompt_engine".to_string(),
            prompt_engine: Some("starship".to_string()),
            framework_theme: None,
            plugins: vec!["git".to_string(), "docker".to_string()],
            env_vars: HashMap::new(),
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_profile_config_validation_empty_name() {
        let config = ProfileConfig {
            name: "".to_string(),
            framework: "oh-my-zsh".to_string(),
            prompt_mode: "prompt_engine".to_string(),
            prompt_engine: Some("starship".to_string()),
            framework_theme: None,
            plugins: vec![],
            env_vars: HashMap::new(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_profile_config_validation_invalid_name() {
        let config = ProfileConfig {
            name: "my profile!".to_string(),
            framework: "oh-my-zsh".to_string(),
            prompt_mode: "prompt_engine".to_string(),
            prompt_engine: Some("starship".to_string()),
            framework_theme: None,
            plugins: vec![],
            env_vars: HashMap::new(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_profile_config_validation_invalid_framework() {
        let config = ProfileConfig {
            name: "work".to_string(),
            framework: "bash-it".to_string(),
            prompt_mode: "prompt_engine".to_string(),
            prompt_engine: Some("starship".to_string()),
            framework_theme: None,
            plugins: vec![],
            env_vars: HashMap::new(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_profile_config_validation_missing_prompt_engine() {
        let config = ProfileConfig {
            name: "work".to_string(),
            framework: "oh-my-zsh".to_string(),
            prompt_mode: "prompt_engine".to_string(),
            prompt_engine: None,
            framework_theme: None,
            plugins: vec![],
            env_vars: HashMap::new(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_profile_config_validation_framework_theme_mode() {
        let config = ProfileConfig {
            name: "work".to_string(),
            framework: "oh-my-zsh".to_string(),
            prompt_mode: "framework_theme".to_string(),
            prompt_engine: None,
            framework_theme: Some("robbyrussell".to_string()),
            plugins: vec![],
            env_vars: HashMap::new(),
        };

        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_profile_config_validation_empty_plugin() {
        let config = ProfileConfig {
            name: "work".to_string(),
            framework: "oh-my-zsh".to_string(),
            prompt_mode: "framework_theme".to_string(),
            prompt_engine: None,
            framework_theme: Some("robbyrussell".to_string()),
            plugins: vec!["git".to_string(), "".to_string()],
            env_vars: HashMap::new(),
        };

        assert!(config.validate().is_err());
    }

    #[test]
    fn test_profile_config_validation_invalid_env_var() {
        let mut env_vars = HashMap::new();
        env_vars.insert("MY-VAR".to_string(), "value".to_string());

        let config = ProfileConfig {
            name: "work".to_string(),
            framework: "oh-my-zsh".to_string(),
            prompt_mode: "framework_theme".to_string(),
            prompt_engine: None,
            framework_theme: Some("robbyrussell".to_string()),
            plugins: vec![],
            env_vars,
        };

        assert!(config.validate().is_err());
    }
}
