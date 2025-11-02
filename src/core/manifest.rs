//! Profile manifest management
//!
//! This module handles the generation, parsing, and management of profile.toml
//! manifests that define profile configurations following Pattern 4.

use anyhow::{bail, Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::frameworks::FrameworkInfo;

/// Supported zsh frameworks
const SUPPORTED_FRAMEWORKS: &[&str] = &["oh-my-zsh", "zimfw", "prezto", "zinit", "zap"];

/// Profile manifest structure following Pattern 4: TOML Manifest Schema
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Manifest {
    pub profile: ProfileSection,
    #[serde(default)]
    pub plugins: PluginsSection,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

/// Profile metadata section
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProfileSection {
    pub name: String,
    pub framework: String,
    #[serde(default)]
    pub theme: String,
    #[serde(default = "default_timestamp")]
    pub created: DateTime<Utc>,
    #[serde(default = "default_timestamp")]
    pub modified: DateTime<Utc>,
}

/// Plugins section
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct PluginsSection {
    #[serde(default)]
    pub enabled: Vec<String>,
}

/// Default timestamp for serde
fn default_timestamp() -> DateTime<Utc> {
    Utc::now()
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

    /// Validate the manifest schema and values
    ///
    /// Checks:
    /// - Profile name is not empty
    /// - Framework is one of the 5 supported values
    /// - Plugins are non-empty strings
    /// - Environment variable keys are valid shell identifiers
    pub fn validate(&self) -> Result<()> {
        // Validate profile name
        if self.profile.name.trim().is_empty() {
            bail!("Validation error: profile.name is required and cannot be empty");
        }

        // Validate framework
        if !SUPPORTED_FRAMEWORKS.contains(&self.profile.framework.as_str()) {
            bail!(
                "Validation error: profile.framework must be one of: {}\n  Found: '{}'\n  → Check your profile.toml framework field\n\nExample:\n  [profile]\n  framework = \"oh-my-zsh\"",
                SUPPORTED_FRAMEWORKS.join(", "),
                self.profile.framework
            );
        }

        // Validate plugins are non-empty strings
        for (idx, plugin) in self.plugins.enabled.iter().enumerate() {
            if plugin.trim().is_empty() {
                bail!(
                    "Validation error: plugins.enabled[{}] cannot be empty string\n\nExample:\n  [plugins]\n  enabled = [\"git\", \"docker\"]",
                    idx
                );
            }
        }

        // Validate theme field is not whitespace-only if present
        if !self.profile.theme.is_empty() && self.profile.theme.trim().is_empty() {
            bail!("Validation error: profile.theme cannot be whitespace-only");
        }

        // Validate environment variable keys are valid shell identifiers
        for (key, value) in &self.env {
            if key.trim().is_empty() {
                bail!("Validation error: env variable key cannot be empty");
            }
            if !key.chars().all(|c| c.is_alphanumeric() || c == '_') {
                bail!(
                    "Validation error: env variable key '{}' contains invalid characters\n  → Keys must be alphanumeric with underscores only\n\nExample:\n  [env]\n  EDITOR = \"vim\"\n  MY_VAR = \"value\"",
                    key
                );
            }
            if value.trim().is_empty() {
                log::warn!("env variable '{}' has empty value", key);
            }
        }

        Ok(())
    }
}

/// Parse TOML manifest from string content with enhanced error reporting
pub fn parse_manifest(toml_content: &str) -> Result<Manifest> {
    toml::from_str(toml_content).map_err(|e| {
        // Try to extract line/column information from the error
        let error_msg = e.to_string();
        if error_msg.contains("line") || error_msg.contains("column") {
            anyhow::anyhow!(
                "TOML parse error: {}\n\n  → Check TOML syntax at the indicated location",
                error_msg
            )
        } else {
            anyhow::anyhow!("Failed to parse profile.toml - check TOML syntax\n  {}", error_msg)
        }
    })
}

/// Get path to profile's manifest file
pub fn get_manifest_path(profile_name: &str) -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".zsh-profiles")
        .join("profiles")
        .join(profile_name)
        .join("profile.toml")
}

/// Check if profile has a manifest file
pub fn manifest_exists(profile_name: &str) -> bool {
    get_manifest_path(profile_name).exists()
}

/// Load and validate a profile manifest
///
/// This is the main entry point for loading and validating manifests.
/// It combines file reading, parsing, and validation in one step.
pub fn load_and_validate(profile_name: &str) -> Result<Manifest> {
    let manifest_path = get_manifest_path(profile_name);

    if !manifest_path.exists() {
        bail!(
            "✗ Error: Profile manifest not found\n  Path: {:?}\n  → Run 'zprof create {}' to create this profile",
            manifest_path,
            profile_name
        );
    }

    log::debug!("Loading manifest from {:?}", manifest_path);

    let toml_content = std::fs::read_to_string(&manifest_path)
        .with_context(|| format!("Failed to read profile.toml at {:?}", manifest_path))?;

    let manifest = parse_manifest(&toml_content)
        .with_context(|| format!("Invalid TOML in {:?}", manifest_path))?;

    manifest
        .validate()
        .context("Manifest validation failed")?;

    log::info!("✓ Manifest validated successfully: {}", profile_name);
    Ok(manifest)
}

/// Save manifest to profile directory
pub fn save_manifest(manifest: &Manifest, profile_name: &str) -> Result<()> {
    let manifest_path = get_manifest_path(profile_name);

    let toml_string = toml::to_string_pretty(manifest)
        .context("Failed to serialize manifest to TOML")?;

    std::fs::write(&manifest_path, toml_string)
        .with_context(|| format!("Failed to write profile.toml to {:?}", manifest_path))?;

    log::debug!("Saved manifest to {:?}", manifest_path);
    Ok(())
}

/// Get list of supported frameworks
pub fn get_supported_frameworks() -> Vec<&'static str> {
    SUPPORTED_FRAMEWORKS.to_vec()
}

/// Validate framework name is supported
pub fn validate_framework(framework: &str) -> Result<()> {
    if !SUPPORTED_FRAMEWORKS.contains(&framework) {
        bail!(
            "Unsupported framework: '{}'\n  Supported frameworks: {}",
            framework,
            SUPPORTED_FRAMEWORKS.join(", ")
        );
    }
    Ok(())
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

    // Story 2.1 validation tests

    #[test]
    fn test_parse_valid_manifest() {
        let toml = r#"
[profile]
name = "test"
framework = "oh-my-zsh"
theme = "robbyrussell"
created = "2025-10-31T14:30:00Z"
modified = "2025-10-31T14:30:00Z"

[plugins]
enabled = ["git", "docker"]

[env]
EDITOR = "vim"
        "#;

        let manifest = parse_manifest(toml).expect("Should parse valid TOML");
        assert_eq!(manifest.profile.name, "test");
        assert_eq!(manifest.profile.framework, "oh-my-zsh");
        assert_eq!(manifest.plugins.enabled.len(), 2);
        assert_eq!(manifest.env.get("EDITOR"), Some(&"vim".to_string()));
    }

    #[test]
    fn test_parse_minimal_manifest() {
        let toml = r#"
[profile]
name = "minimal"
framework = "zimfw"
        "#;

        let manifest = parse_manifest(toml).expect("Should parse minimal TOML");
        assert_eq!(manifest.profile.name, "minimal");
        assert_eq!(manifest.profile.framework, "zimfw");
        assert_eq!(manifest.profile.theme, ""); // defaults to empty
        assert!(manifest.plugins.enabled.is_empty()); // defaults to empty vec
        assert!(manifest.env.is_empty()); // defaults to empty map
    }

    #[test]
    fn test_validate_valid_manifest() {
        let manifest = Manifest {
            profile: ProfileSection {
                name: "test".to_string(),
                framework: "oh-my-zsh".to_string(),
                theme: "robbyrussell".to_string(),
                created: Utc::now(),
                modified: Utc::now(),
            },
            plugins: PluginsSection {
                enabled: vec!["git".to_string(), "docker".to_string()],
            },
            env: {
                let mut map = HashMap::new();
                map.insert("EDITOR".to_string(), "vim".to_string());
                map
            },
        };

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn test_validate_empty_profile_name() {
        let manifest = Manifest {
            profile: ProfileSection {
                name: "".to_string(),
                framework: "oh-my-zsh".to_string(),
                theme: "default".to_string(),
                created: Utc::now(),
                modified: Utc::now(),
            },
            plugins: Default::default(),
            env: Default::default(),
        };

        let result = manifest.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_invalid_framework() {
        let manifest = Manifest {
            profile: ProfileSection {
                name: "test".to_string(),
                framework: "bash-it".to_string(),
                theme: "default".to_string(),
                created: Utc::now(),
                modified: Utc::now(),
            },
            plugins: Default::default(),
            env: Default::default(),
        };

        let result = manifest.validate();
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("must be one of"));
        assert!(err_msg.contains("bash-it"));
    }

    #[test]
    fn test_all_supported_frameworks_validate() {
        for framework in get_supported_frameworks() {
            let manifest = Manifest {
                profile: ProfileSection {
                    name: "test".to_string(),
                    framework: framework.to_string(),
                    theme: "default".to_string(),
                    created: Utc::now(),
                    modified: Utc::now(),
                },
                plugins: Default::default(),
                env: Default::default(),
            };

            manifest
                .validate()
                .unwrap_or_else(|_| panic!("Framework '{}' should validate successfully", framework));
        }
    }

    #[test]
    fn test_validate_empty_plugin_string() {
        let manifest = Manifest {
            profile: ProfileSection {
                name: "test".to_string(),
                framework: "oh-my-zsh".to_string(),
                theme: "default".to_string(),
                created: Utc::now(),
                modified: Utc::now(),
            },
            plugins: PluginsSection {
                enabled: vec!["git".to_string(), "".to_string()],
            },
            env: Default::default(),
        };

        let result = manifest.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }

    #[test]
    fn test_validate_invalid_env_key() {
        let manifest = Manifest {
            profile: ProfileSection {
                name: "test".to_string(),
                framework: "oh-my-zsh".to_string(),
                theme: "default".to_string(),
                created: Utc::now(),
                modified: Utc::now(),
            },
            plugins: Default::default(),
            env: {
                let mut map = HashMap::new();
                map.insert("MY-VAR".to_string(), "value".to_string());
                map
            },
        };

        let result = manifest.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("invalid characters"));
    }

    #[test]
    fn test_parse_invalid_toml() {
        let toml = r#"
[profile
name = "broken"
        "#;

        let result = parse_manifest(toml);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("parse error") || err_msg.contains("TOML"));
        // Verify line number extraction works
        assert!(
            err_msg.contains("line") || err_msg.contains("column"),
            "Error should include line/column information: {}",
            err_msg
        );
    }

    #[test]
    fn test_get_supported_frameworks() {
        let frameworks = get_supported_frameworks();
        assert_eq!(frameworks.len(), 5);
        assert!(frameworks.contains(&"oh-my-zsh"));
        assert!(frameworks.contains(&"zimfw"));
        assert!(frameworks.contains(&"prezto"));
        assert!(frameworks.contains(&"zinit"));
        assert!(frameworks.contains(&"zap"));
    }

    #[test]
    fn test_validate_framework_success() {
        assert!(validate_framework("oh-my-zsh").is_ok());
        assert!(validate_framework("zimfw").is_ok());
        assert!(validate_framework("prezto").is_ok());
        assert!(validate_framework("zinit").is_ok());
        assert!(validate_framework("zap").is_ok());
    }

    #[test]
    fn test_validate_framework_failure() {
        let result = validate_framework("bash-it");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unsupported framework"));
    }

    #[test]
    fn test_validate_whitespace_only_theme() {
        let manifest = Manifest {
            profile: ProfileSection {
                name: "test".to_string(),
                framework: "oh-my-zsh".to_string(),
                theme: "   ".to_string(), // whitespace-only
                created: Utc::now(),
                modified: Utc::now(),
            },
            plugins: Default::default(),
            env: Default::default(),
        };

        let result = manifest.validate();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("whitespace-only"));
    }

    #[test]
    fn test_manifest_exists() {
        // This will be false unless user has a profile named "nonexistent"
        assert!(!manifest_exists("nonexistent-profile-12345"));
    }

    // Integration test for load_and_validate() full workflow
    #[test]
    fn test_load_and_validate_integration() {
        use std::fs;
        use tempfile::TempDir;

        // Create temp directory structure
        let temp_dir = TempDir::new().expect("Failed to create temp dir");
        let profiles_dir = temp_dir.path().join(".zsh-profiles").join("profiles");
        let profile_dir = profiles_dir.join("test-profile");
        fs::create_dir_all(&profile_dir).expect("Failed to create profile dir");

        // Write valid TOML manifest
        let manifest_path = profile_dir.join("profile.toml");
        let valid_toml = r#"
[profile]
name = "test-profile"
framework = "oh-my-zsh"
theme = "robbyrussell"
created = "2025-11-01T10:00:00Z"
modified = "2025-11-01T10:00:00Z"

[plugins]
enabled = ["git", "docker"]

[env]
EDITOR = "vim"
"#;
        fs::write(&manifest_path, valid_toml).expect("Failed to write manifest");

        // Mock home_dir to point to temp directory (can't actually override dirs::home_dir)
        // Instead, we'll test that get_manifest_path constructs the expected path
        let expected_path = dirs::home_dir()
            .unwrap()
            .join(".zsh-profiles/profiles/test-profile/profile.toml");

        // Verify get_manifest_path constructs correct path
        let actual_path = get_manifest_path("test-profile");
        assert_eq!(actual_path, expected_path);

        // Note: Full integration test with load_and_validate() requires either:
        // 1. Creating actual profile in user's home (not safe for tests)
        // 2. Mocking dirs::home_dir (requires test feature flag)
        // 3. Refactoring to accept base_path parameter

        // For now, test the parse_manifest -> validate workflow
        let parsed = parse_manifest(valid_toml).expect("Should parse valid TOML");
        parsed.validate().expect("Should validate successfully");

        // Verify parsed values
        assert_eq!(parsed.profile.name, "test-profile");
        assert_eq!(parsed.profile.framework, "oh-my-zsh");
        assert_eq!(parsed.profile.theme, "robbyrussell");
        assert_eq!(parsed.plugins.enabled.len(), 2);
        assert_eq!(parsed.plugins.enabled[0], "git");
        assert_eq!(parsed.env.get("EDITOR"), Some(&"vim".to_string()));

        // Test error case: invalid TOML
        let invalid_toml = r#"
[profile
name = "broken"
"#;
        let result = parse_manifest(invalid_toml);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(
            err_msg.contains("parse") || err_msg.contains("TOML"),
            "Error should mention parsing: {}",
            err_msg
        );
    }
}
