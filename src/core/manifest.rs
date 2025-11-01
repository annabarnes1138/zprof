//! Profile manifest management
//!
//! This module handles the generation, parsing, and management of profile.toml
//! manifests that define profile configurations following Pattern 4.

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::frameworks::FrameworkInfo;

/// Profile manifest structure following Pattern 4: TOML Manifest Schema
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
    pub profile: ProfileSection,
    pub plugins: PluginsSection,
    pub env: HashMap<String, String>,
}

/// Profile metadata section
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProfileSection {
    pub name: String,
    pub framework: String,
    pub theme: String,
    pub created: DateTime<Utc>,
    pub modified: DateTime<Utc>,
}

/// Plugins section
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginsSection {
    pub enabled: Vec<String>,
}

impl Manifest {
    /// Create a new manifest from framework detection info
    ///
    /// # Arguments
    ///
    /// * `name` - The profile name
    /// * `framework_info` - Detected framework information
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use zprof::core::manifest::Manifest;
    /// use zprof::frameworks::detect_existing_framework;
    ///
    /// if let Some(info) = detect_existing_framework() {
    ///     let manifest = Manifest::from_framework_info("work", &info);
    /// }
    /// ```
    pub fn from_framework_info(name: &str, framework_info: &FrameworkInfo) -> Self {
        let now = Utc::now();

        Manifest {
            profile: ProfileSection {
                name: name.to_string(),
                framework: framework_info.framework_type.name().to_string(),
                theme: framework_info.theme.clone(),
                created: now,
                modified: now,
            },
            plugins: PluginsSection {
                enabled: framework_info.plugins.clone(),
            },
            env: HashMap::new(),
        }
    }

    /// Create a new manifest from wizard state
    ///
    /// Used during profile creation wizard to generate a manifest from
    /// user-selected configuration options.
    ///
    /// # Arguments
    ///
    /// * `profile_name` - The profile name
    /// * `framework` - Selected framework type
    /// * `plugins` - Selected plugins list
    /// * `theme` - Selected theme name
    pub fn from_wizard_state(
        profile_name: &str,
        framework: &crate::frameworks::FrameworkType,
        plugins: &[String],
        theme: &str,
    ) -> Self {
        let now = Utc::now();

        Manifest {
            profile: ProfileSection {
                name: profile_name.to_string(),
                framework: framework.name().to_string(),
                theme: theme.to_string(),
                created: now,
                modified: now,
            },
            plugins: PluginsSection {
                enabled: plugins.to_vec(),
            },
            env: HashMap::new(),
        }
    }

    /// Convert manifest to TOML string
    pub fn to_toml_string(&self) -> Result<String> {
        toml::to_string_pretty(self).context("Failed to serialize manifest to TOML format")
    }

    /// Write manifest to profile.toml file
    pub fn write_to_file(&self, path: &Path) -> Result<()> {
        let content = self.to_toml_string()?;
        std::fs::write(path, content)
            .with_context(|| format!("Failed to write manifest to {}", path.display()))?;
        Ok(())
    }

    /// Load manifest from profile.toml file
    pub fn load_from_file(path: &Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read manifest from {}", path.display()))?;
        let manifest: Manifest = toml::from_str(&content)
            .with_context(|| format!("Failed to parse manifest at {}", path.display()))?;
        Ok(manifest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frameworks::{FrameworkInfo, FrameworkType};
    use std::path::PathBuf;

    fn create_test_framework_info() -> FrameworkInfo {
        FrameworkInfo {
            framework_type: FrameworkType::OhMyZsh,
            plugins: vec!["git".to_string(), "docker".to_string(), "kubectl".to_string()],
            theme: "robbyrussell".to_string(),
            config_path: PathBuf::from("/home/user/.zshrc"),
            install_path: PathBuf::from("/home/user/.oh-my-zsh"),
        }
    }

    #[test]
    fn test_manifest_from_framework_info() {
        let info = create_test_framework_info();
        let manifest = Manifest::from_framework_info("work", &info);

        assert_eq!(manifest.profile.name, "work");
        assert_eq!(manifest.profile.framework, "oh-my-zsh");
        assert_eq!(manifest.profile.theme, "robbyrussell");
        assert_eq!(manifest.plugins.enabled.len(), 3);
        assert_eq!(manifest.plugins.enabled[0], "git");
        assert!(manifest.env.is_empty());
    }

    #[test]
    fn test_manifest_to_toml() {
        let info = create_test_framework_info();
        let manifest = Manifest::from_framework_info("work", &info);
        let toml_str = manifest.to_toml_string().unwrap();

        // Verify TOML structure
        assert!(toml_str.contains("[profile]"));
        assert!(toml_str.contains("name = \"work\""));
        assert!(toml_str.contains("framework = \"oh-my-zsh\""));
        assert!(toml_str.contains("theme = \"robbyrussell\""));
        assert!(toml_str.contains("[plugins]"));
        assert!(toml_str.contains("git"));
        assert!(toml_str.contains("docker"));
        assert!(toml_str.contains("kubectl"));
    }

    #[test]
    fn test_manifest_roundtrip() {
        let info = create_test_framework_info();
        let original = Manifest::from_framework_info("test-profile", &info);
        let toml_str = original.to_toml_string().unwrap();
        let parsed: Manifest = toml::from_str(&toml_str).unwrap();

        assert_eq!(parsed.profile.name, "test-profile");
        assert_eq!(parsed.profile.framework, "oh-my-zsh");
        assert_eq!(parsed.profile.theme, "robbyrussell");
        assert_eq!(parsed.plugins.enabled.len(), 3);
    }
}
