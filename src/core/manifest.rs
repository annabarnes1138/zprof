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

/// Prompt mode discriminates between standalone prompt engines and framework themes
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "prompt_mode", rename_all = "snake_case")]
pub enum PromptMode {
    /// Use a standalone prompt engine (Starship, Powerlevel10k, etc.)
    PromptEngine {
        #[serde(rename = "prompt_engine")]
        engine: String,
    },
    /// Use framework's built-in theme system
    FrameworkTheme {
        #[serde(rename = "framework_theme")]
        theme: String,
    },
}

impl Default for PromptMode {
    fn default() -> Self {
        PromptMode::FrameworkTheme {
            theme: String::new(),
        }
    }
}

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
#[derive(Debug, Serialize, Clone, PartialEq)]
pub struct ProfileSection {
    pub name: String,
    pub framework: String,
    #[serde(flatten)]
    pub prompt_mode: PromptMode,
    #[serde(default = "default_timestamp")]
    pub created: DateTime<Utc>,
    #[serde(default = "default_timestamp")]
    pub modified: DateTime<Utc>,
}

impl ProfileSection {
    /// Get the theme if using framework_theme mode, otherwise empty string
    pub fn theme(&self) -> &str {
        match &self.prompt_mode {
            PromptMode::FrameworkTheme { theme } => theme.as_str(),
            PromptMode::PromptEngine { .. } => "",
        }
    }
}

// Custom deserializer for backward compatibility with old manifests
impl<'de> Deserialize<'de> for ProfileSection {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::{self, MapAccess, Visitor};
        use std::fmt;

        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field {
            Name,
            Framework,
            Theme,
            PromptMode,
            PromptEngine,
            FrameworkTheme,
            Created,
            Modified,
        }

        struct ProfileSectionVisitor;

        impl<'de> Visitor<'de> for ProfileSectionVisitor {
            type Value = ProfileSection;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ProfileSection")
            }

            fn visit_map<V>(self, mut map: V) -> Result<ProfileSection, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut name: Option<String> = None;
                let mut framework: Option<String> = None;
                let mut legacy_theme: Option<String> = None;
                let mut prompt_mode_tag: Option<String> = None;
                let mut prompt_engine: Option<String> = None;
                let mut framework_theme: Option<String> = None;
                let mut created: Option<DateTime<Utc>> = None;
                let mut modified: Option<DateTime<Utc>> = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Name => {
                            name = Some(map.next_value()?);
                        }
                        Field::Framework => {
                            framework = Some(map.next_value()?);
                        }
                        Field::Theme => {
                            legacy_theme = Some(map.next_value()?);
                        }
                        Field::PromptMode => {
                            prompt_mode_tag = Some(map.next_value()?);
                        }
                        Field::PromptEngine => {
                            prompt_engine = Some(map.next_value()?);
                        }
                        Field::FrameworkTheme => {
                            framework_theme = Some(map.next_value()?);
                        }
                        Field::Created => {
                            created = Some(map.next_value()?);
                        }
                        Field::Modified => {
                            modified = Some(map.next_value()?);
                        }
                    }
                }

                let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
                let framework = framework.ok_or_else(|| de::Error::missing_field("framework"))?;
                let created = created.unwrap_or_else(default_timestamp);
                let modified = modified.unwrap_or_else(default_timestamp);

                // Backward compatibility: if no prompt_mode is specified, use legacy theme field
                let prompt_mode = if let Some(mode_tag) = prompt_mode_tag {
                    match mode_tag.as_str() {
                        "prompt_engine" => {
                            let engine = prompt_engine
                                .ok_or_else(|| de::Error::missing_field("prompt_engine"))?;
                            PromptMode::PromptEngine { engine }
                        }
                        "framework_theme" => {
                            let theme = framework_theme
                                .ok_or_else(|| de::Error::missing_field("framework_theme"))?;
                            PromptMode::FrameworkTheme { theme }
                        }
                        _ => {
                            return Err(de::Error::unknown_variant(
                                &mode_tag,
                                &["prompt_engine", "framework_theme"],
                            ))
                        }
                    }
                } else {
                    // Legacy manifest: use theme field, default to empty string
                    PromptMode::FrameworkTheme {
                        theme: legacy_theme.unwrap_or_default(),
                    }
                };

                Ok(ProfileSection {
                    name,
                    framework,
                    prompt_mode,
                    created,
                    modified,
                })
            }
        }

        deserializer.deserialize_struct(
            "ProfileSection",
            &[
                "name",
                "framework",
                "theme",
                "prompt_mode",
                "prompt_engine",
                "framework_theme",
                "created",
                "modified",
            ],
            ProfileSectionVisitor,
        )
    }
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
                // Backward compatibility: default to framework_theme mode
                prompt_mode: PromptMode::FrameworkTheme {
                    theme: framework_info.theme.clone(),
                },
                created: now,
                modified: now,
            },
            plugins: PluginsSection {
                enabled: framework_info.plugins.clone(),
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

        // Validate prompt_mode fields based on the variant
        match &self.profile.prompt_mode {
            PromptMode::PromptEngine { engine } => {
                if engine.trim().is_empty() {
                    bail!(
                        "Validation error: prompt_engine cannot be empty when prompt_mode is 'prompt_engine'\n\nExample:\n  [profile]\n  prompt_mode = \"prompt_engine\"\n  prompt_engine = \"starship\""
                    );
                }
            }
            PromptMode::FrameworkTheme { theme } => {
                // Theme can be empty (no theme), but cannot be whitespace-only
                if !theme.is_empty() && theme.trim().is_empty() {
                    bail!("Validation error: framework_theme cannot be whitespace-only");
                }
            }
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

// Test-only helper functions
#[cfg(test)]
pub fn manifest_exists(profile_name: &str) -> bool {
    get_manifest_path(profile_name).exists()
}

#[cfg(test)]
pub fn get_supported_frameworks() -> Vec<&'static str> {
    SUPPORTED_FRAMEWORKS.to_vec()
}

#[cfg(test)]
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
        assert_eq!(
            manifest.profile.prompt_mode,
            PromptMode::FrameworkTheme {
                theme: "robbyrussell".to_string()
            }
        );
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
        assert!(toml_str.contains("prompt_mode = \"framework_theme\""));
        assert!(toml_str.contains("framework_theme = \"robbyrussell\""));
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
        assert_eq!(
            parsed.profile.prompt_mode,
            PromptMode::FrameworkTheme {
                theme: "robbyrussell".to_string()
            }
        );
        assert_eq!(parsed.plugins.enabled.len(), 3);
    }

    // Story 2.1 validation tests

    #[test]
    fn test_parse_valid_manifest() {
        let toml = r#"
[profile]
name = "test"
framework = "oh-my-zsh"
prompt_mode = "framework_theme"
framework_theme = "robbyrussell"
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
        // Backward compatibility: no prompt_mode defaults to framework_theme with empty theme
        assert_eq!(
            manifest.profile.prompt_mode,
            PromptMode::FrameworkTheme {
                theme: String::new()
            }
        );
        assert!(manifest.plugins.enabled.is_empty()); // defaults to empty vec
        assert!(manifest.env.is_empty()); // defaults to empty map
    }

    #[test]
    fn test_validate_valid_manifest() {
        let manifest = Manifest {
            profile: ProfileSection {
                name: "test".to_string(),
                framework: "oh-my-zsh".to_string(),
                prompt_mode: PromptMode::FrameworkTheme {
                    theme: "robbyrussell".to_string(),
                },
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
                prompt_mode: PromptMode::FrameworkTheme {
                    theme: "default".to_string(),
                },
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
                prompt_mode: PromptMode::FrameworkTheme {
                    theme: "default".to_string(),
                },
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
                    prompt_mode: PromptMode::FrameworkTheme {
                        theme: "default".to_string(),
                    },
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
                prompt_mode: PromptMode::FrameworkTheme {
                    theme: "default".to_string(),
                },
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
                prompt_mode: PromptMode::FrameworkTheme {
                    theme: "default".to_string(),
                },
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
                prompt_mode: PromptMode::FrameworkTheme {
                    theme: "   ".to_string(), // whitespace-only
                },
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
prompt_mode = "framework_theme"
framework_theme = "robbyrussell"
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
        assert_eq!(
            parsed.profile.prompt_mode,
            PromptMode::FrameworkTheme {
                theme: "robbyrussell".to_string()
            }
        );
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

    // Story 1.1 tests: Prompt Mode functionality

    #[test]
    fn test_parse_manifest_with_prompt_engine() {
        let toml = r#"
[profile]
name = "test"
framework = "oh-my-zsh"
prompt_mode = "prompt_engine"
prompt_engine = "starship"
created = "2025-11-01T10:00:00Z"
modified = "2025-11-01T10:00:00Z"
        "#;

        let manifest = parse_manifest(toml).expect("Should parse prompt_engine mode");
        assert_eq!(
            manifest.profile.prompt_mode,
            PromptMode::PromptEngine {
                engine: "starship".to_string()
            }
        );
    }

    #[test]
    fn test_parse_manifest_with_framework_theme() {
        let toml = r#"
[profile]
name = "test"
framework = "oh-my-zsh"
prompt_mode = "framework_theme"
framework_theme = "robbyrussell"
created = "2025-11-01T10:00:00Z"
modified = "2025-11-01T10:00:00Z"
        "#;

        let manifest = parse_manifest(toml).expect("Should parse framework_theme mode");
        assert_eq!(
            manifest.profile.prompt_mode,
            PromptMode::FrameworkTheme {
                theme: "robbyrussell".to_string()
            }
        );
    }

    #[test]
    fn test_backward_compatibility_with_legacy_theme_field() {
        let toml = r#"
[profile]
name = "test"
framework = "oh-my-zsh"
theme = "robbyrussell"
created = "2025-11-01T10:00:00Z"
modified = "2025-11-01T10:00:00Z"
        "#;

        let manifest = parse_manifest(toml).expect("Should parse legacy manifest");
        assert_eq!(
            manifest.profile.prompt_mode,
            PromptMode::FrameworkTheme {
                theme: "robbyrussell".to_string()
            }
        );
    }

    #[test]
    fn test_backward_compatibility_empty_theme() {
        let toml = r#"
[profile]
name = "test"
framework = "zimfw"
created = "2025-11-01T10:00:00Z"
modified = "2025-11-01T10:00:00Z"
        "#;

        let manifest = parse_manifest(toml).expect("Should parse manifest without theme");
        assert_eq!(
            manifest.profile.prompt_mode,
            PromptMode::FrameworkTheme {
                theme: String::new()
            }
        );
    }

    #[test]
    fn test_validate_prompt_engine_with_empty_engine() {
        let manifest = Manifest {
            profile: ProfileSection {
                name: "test".to_string(),
                framework: "oh-my-zsh".to_string(),
                prompt_mode: PromptMode::PromptEngine {
                    engine: "".to_string(),
                },
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
            .contains("prompt_engine cannot be empty"));
    }

    #[test]
    fn test_validate_prompt_engine_success() {
        let manifest = Manifest {
            profile: ProfileSection {
                name: "test".to_string(),
                framework: "oh-my-zsh".to_string(),
                prompt_mode: PromptMode::PromptEngine {
                    engine: "starship".to_string(),
                },
                created: Utc::now(),
                modified: Utc::now(),
            },
            plugins: Default::default(),
            env: Default::default(),
        };

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn test_validate_framework_theme_allows_empty_theme() {
        let manifest = Manifest {
            profile: ProfileSection {
                name: "test".to_string(),
                framework: "zimfw".to_string(),
                prompt_mode: PromptMode::FrameworkTheme {
                    theme: String::new(),
                },
                created: Utc::now(),
                modified: Utc::now(),
            },
            plugins: Default::default(),
            env: Default::default(),
        };

        assert!(manifest.validate().is_ok());
    }

    #[test]
    fn test_roundtrip_with_prompt_engine() {
        let manifest = Manifest {
            profile: ProfileSection {
                name: "test".to_string(),
                framework: "oh-my-zsh".to_string(),
                prompt_mode: PromptMode::PromptEngine {
                    engine: "starship".to_string(),
                },
                created: Utc::now(),
                modified: Utc::now(),
            },
            plugins: Default::default(),
            env: Default::default(),
        };

        let toml_str = manifest.to_toml_string().unwrap();
        let parsed: Manifest = toml::from_str(&toml_str).unwrap();

        assert_eq!(
            parsed.profile.prompt_mode,
            PromptMode::PromptEngine {
                engine: "starship".to_string()
            }
        );
    }

    #[test]
    fn test_roundtrip_with_framework_theme() {
        let manifest = Manifest {
            profile: ProfileSection {
                name: "test".to_string(),
                framework: "zimfw".to_string(),
                prompt_mode: PromptMode::FrameworkTheme {
                    theme: "powerlevel10k".to_string(),
                },
                created: Utc::now(),
                modified: Utc::now(),
            },
            plugins: Default::default(),
            env: Default::default(),
        };

        let toml_str = manifest.to_toml_string().unwrap();
        let parsed: Manifest = toml::from_str(&toml_str).unwrap();

        assert_eq!(
            parsed.profile.prompt_mode,
            PromptMode::FrameworkTheme {
                theme: "powerlevel10k".to_string()
            }
        );
    }

    #[test]
    fn test_toml_serialization_includes_prompt_mode() {
        let manifest = Manifest {
            profile: ProfileSection {
                name: "test".to_string(),
                framework: "oh-my-zsh".to_string(),
                prompt_mode: PromptMode::PromptEngine {
                    engine: "starship".to_string(),
                },
                created: Utc::now(),
                modified: Utc::now(),
            },
            plugins: Default::default(),
            env: Default::default(),
        };

        let toml_str = manifest.to_toml_string().unwrap();
        assert!(toml_str.contains("prompt_mode = \"prompt_engine\""));
        assert!(toml_str.contains("prompt_engine = \"starship\""));
        assert!(!toml_str.contains("framework_theme"));
    }

    #[test]
    fn test_from_framework_info_defaults_to_framework_theme() {
        let info = create_test_framework_info();
        let manifest = Manifest::from_framework_info("test", &info);

        assert_eq!(
            manifest.profile.prompt_mode,
            PromptMode::FrameworkTheme {
                theme: "robbyrussell".to_string()
            }
        );
    }
}
