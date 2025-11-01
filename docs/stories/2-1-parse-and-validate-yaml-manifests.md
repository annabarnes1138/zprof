# Story 2.1: Parse and Validate TOML Manifests

Status: ready-for-dev

## Story

As a developer,
I want zprof to parse and validate my profile TOML manifests,
so that I can ensure my profile configuration is correct before applying it.

## Acceptance Criteria

1. System reads profile.toml files and validates schema (name, framework, plugins, theme, environment variables)
2. Validation checks for required fields and correct data types
3. Clear error messages identify specific validation failures with line numbers
4. Successfully validated manifests are marked as ready for use
5. Invalid manifests prevent profile activation with helpful guidance

## Tasks / Subtasks

- [ ] Define TOML manifest schema (AC: #1)
  - [ ] Create `core/manifest.rs` module
  - [ ] Define ProfileManifest struct with serde Deserialize
  - [ ] Define nested structs: ProfileMeta, PluginsConfig, EnvVars
  - [ ] Follow Pattern 4 (TOML Manifest Schema) from architecture
  - [ ] Use serde attributes for validation (#[serde(rename, default)])
  - [ ] Add required field markers
  - [ ] Include chrono DateTime fields for created/modified timestamps
  - [ ] Support all 5 frameworks: oh-my-zsh, zimfw, prezto, zinit, zap
- [ ] Implement TOML parsing (AC: #1, #2)
  - [ ] Add toml 0.9 and serde 1.0 dependencies to Cargo.toml
  - [ ] Implement parse_manifest(path) function
  - [ ] Use toml::from_str() with error context
  - [ ] Catch deserialization errors with anyhow::Context
  - [ ] Map TOML syntax errors to user-friendly messages
  - [ ] Extract line numbers from parse errors when available
  - [ ] Return parsed ProfileManifest struct on success
- [ ] Implement schema validation (AC: #2, #3)
  - [ ] Create validate_manifest(manifest) function
  - [ ] Check required fields: profile.name, profile.framework
  - [ ] Validate framework is one of 5 supported values
  - [ ] Validate plugins.enabled is array of strings
  - [ ] Validate env variables are string key-value pairs
  - [ ] Check theme field is non-empty string if present
  - [ ] Verify timestamps are valid ISO 8601 format
  - [ ] Return Vec<ValidationError> with field path + message
  - [ ] Include line number hints in error messages (if available from TOML parser)
- [ ] Implement user-friendly error reporting (AC: #3, #5)
  - [ ] Create custom error types for validation failures
  - [ ] Format errors with field path, expected type, actual value
  - [ ] Include suggestions for fixing common mistakes
  - [ ] Show example of correct TOML syntax
  - [ ] Use anyhow for error context chaining
  - [ ] Log detailed errors with env_logger (debug level)
  - [ ] Display simplified errors to users (info level)
- [ ] Implement load and validate workflow (AC: #4, #5)
  - [ ] Create load_and_validate(profile_name) convenience function
  - [ ] Construct path to profile.toml: `~/.zsh-profiles/profiles/<name>/profile.toml`
  - [ ] Check file exists with helpful error if missing
  - [ ] Parse TOML file
  - [ ] Run validation on parsed manifest
  - [ ] Return Ok(ProfileManifest) if all validations pass
  - [ ] Return detailed error if parsing or validation fails
  - [ ] Mark manifest as "validated" in logs
- [ ] Add helper functions for common operations (AC: All)
  - [ ] get_manifest_path(profile_name) -> PathBuf
  - [ ] manifest_exists(profile_name) -> bool
  - [ ] get_supported_frameworks() -> Vec<&str>
  - [ ] validate_framework(framework: &str) -> Result<()>
  - [ ] save_manifest(manifest, profile_name) -> Result<()> (for future use)
- [ ] Write comprehensive tests (AC: All)
  - [ ] Unit test parsing valid TOML manifest
  - [ ] Unit test parsing invalid TOML syntax
  - [ ] Unit test validation with missing required fields
  - [ ] Unit test validation with invalid framework name
  - [ ] Unit test validation with invalid plugin format
  - [ ] Unit test validation with malformed env vars
  - [ ] Unit test error messages include line numbers
  - [ ] Integration test load_and_validate() with sample manifests
  - [ ] Create fixtures/test manifests (valid + various invalid states)
  - [ ] Test all 5 supported frameworks validate correctly
  - [ ] Test helpful error messages for common mistakes

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `core/manifest.rs`
- Dependencies: `serde 1.0`, `toml 0.9`, `anyhow 2.0`, `chrono`
- Follow Pattern 2 (Error Handling) with anyhow::Result
- Follow Pattern 4 (TOML Manifest Schema)
- Implements ADR-002: Use TOML instead of YAML

**ProfileManifest Schema (Pattern 4):**
```rust
// core/manifest.rs
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use anyhow::{bail, Context, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfileManifest {
    pub profile: ProfileMeta,
    #[serde(default)]
    pub plugins: PluginsConfig,
    #[serde(default)]
    pub env: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProfileMeta {
    pub name: String,
    pub framework: String,  // oh-my-zsh | zimfw | prezto | zinit | zap
    #[serde(default)]
    pub theme: String,
    #[serde(default = "default_timestamp")]
    pub created: DateTime<Utc>,
    #[serde(default = "default_timestamp")]
    pub modified: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PluginsConfig {
    #[serde(default)]
    pub enabled: Vec<String>,
}

fn default_timestamp() -> DateTime<Utc> {
    Utc::now()
}

impl ProfileManifest {
    /// Validate the manifest schema and values
    pub fn validate(&self) -> Result<()> {
        // Validate profile name
        if self.profile.name.trim().is_empty() {
            bail!("Validation error: profile.name is required and cannot be empty");
        }

        // Validate framework
        const SUPPORTED_FRAMEWORKS: &[&str] = &[
            "oh-my-zsh", "zimfw", "prezto", "zinit", "zap"
        ];

        if !SUPPORTED_FRAMEWORKS.contains(&self.profile.framework.as_str()) {
            bail!(
                "Validation error: profile.framework must be one of: {}\n  Found: '{}'\n  → Check your profile.toml framework field",
                SUPPORTED_FRAMEWORKS.join(", "),
                self.profile.framework
            );
        }

        // Validate plugins are non-empty strings
        for (idx, plugin) in self.plugins.enabled.iter().enumerate() {
            if plugin.trim().is_empty() {
                bail!(
                    "Validation error: plugins.enabled[{}] cannot be empty string",
                    idx
                );
            }
        }

        // Validate environment variable keys are valid shell identifiers
        for (key, value) in &self.env {
            if key.trim().is_empty() {
                bail!("Validation error: env variable key cannot be empty");
            }
            if !key.chars().all(|c| c.is_alphanumeric() || c == '_') {
                bail!(
                    "Validation error: env variable key '{}' contains invalid characters\n  → Keys must be alphanumeric with underscores only",
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
```

**Parsing and Loading Pattern:**
```rust
// core/manifest.rs (continued)

/// Parse TOML manifest from string content
pub fn parse_manifest(toml_content: &str) -> Result<ProfileManifest> {
    toml::from_str(toml_content)
        .context("Failed to parse profile.toml - check TOML syntax")
        .map_err(|e| {
            // Extract line number from error if available
            if let Some(span) = e.downcast_ref::<toml::de::Error>() {
                if let Some(line) = span.line_col() {
                    anyhow::anyhow!(
                        "TOML parse error at line {}, column {}\n  {}\n\n  → Check TOML syntax at that location",
                        line.0 + 1, line.1 + 1, e
                    )
                } else {
                    e
                }
            } else {
                e
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
pub fn load_and_validate(profile_name: &str) -> Result<ProfileManifest> {
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
        .context(format!("Failed to read profile.toml at {:?}", manifest_path))?;

    let manifest = parse_manifest(&toml_content)
        .context(format!("Invalid TOML in {:?}", manifest_path))?;

    manifest.validate()
        .context("Manifest validation failed")?;

    log::info!("✓ Manifest validated successfully: {}", profile_name);
    Ok(manifest)
}

/// Save manifest to profile directory (for future use in Story 2.2)
pub fn save_manifest(manifest: &ProfileManifest, profile_name: &str) -> Result<()> {
    let manifest_path = get_manifest_path(profile_name);

    let toml_string = toml::to_string_pretty(manifest)
        .context("Failed to serialize manifest to TOML")?;

    std::fs::write(&manifest_path, toml_string)
        .context(format!("Failed to write profile.toml to {:?}", manifest_path))?;

    log::debug!("Saved manifest to {:?}", manifest_path);
    Ok(())
}

/// Get list of supported frameworks
pub fn get_supported_frameworks() -> Vec<&'static str> {
    vec!["oh-my-zsh", "zimfw", "prezto", "zinit", "zap"]
}

/// Validate framework name is supported
pub fn validate_framework(framework: &str) -> Result<()> {
    if !get_supported_frameworks().contains(&framework) {
        bail!(
            "Unsupported framework: '{}'\n  Supported frameworks: {}",
            framework,
            get_supported_frameworks().join(", ")
        );
    }
    Ok(())
}
```

**Example Usage in Other Modules:**
```rust
// Example: Story 1.9 (use command) could validate manifest before switching
use crate::core::manifest;

pub fn execute(args: UseCmdArgs) -> Result<()> {
    // Validate profile manifest before switching
    let manifest = manifest::load_and_validate(&args.profile_name)
        .context("Cannot switch to profile with invalid manifest")?;

    log::debug!("Profile framework: {}", manifest.profile.framework);
    log::debug!("Plugins: {:?}", manifest.plugins.enabled);

    // Proceed with profile switching...
    Ok(())
}
```

**Example Valid TOML Manifest:**
```toml
[profile]
name = "work"
framework = "oh-my-zsh"
theme = "robbyrussell"
created = "2025-10-31T14:30:00Z"
modified = "2025-10-31T14:30:00Z"

[plugins]
enabled = [
    "git",
    "docker",
    "kubectl",
    "fzf"
]

[env]
EDITOR = "vim"
GOPATH = "$HOME/go"
```

**Error Message Examples:**
```bash
# Missing required field
✗ Error: Validation error: profile.framework is required
  → Check your profile.toml file

# Invalid framework
✗ Error: Validation error: profile.framework must be one of: oh-my-zsh, zimfw, prezto, zinit, zap
  Found: 'bash-it'
  → Check your profile.toml framework field

# TOML syntax error
✗ Error: TOML parse error at line 5, column 12
  expected newline, found period at line 5 column 12

  → Check TOML syntax at that location

# Missing manifest file
✗ Error: Profile manifest not found
  Path: "/Users/anna/.zsh-profiles/profiles/experimental/profile.toml"
  → Run 'zprof create experimental' to create this profile

# Invalid environment variable key
✗ Error: Validation error: env variable key 'MY-VAR' contains invalid characters
  → Keys must be alphanumeric with underscores only
```

**Testing Pattern:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

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
    }

    #[test]
    fn test_validate_invalid_framework() {
        let manifest = ProfileManifest {
            profile: ProfileMeta {
                name: "test".to_string(),
                framework: "invalid-framework".to_string(),
                theme: "default".to_string(),
                created: Utc::now(),
                modified: Utc::now(),
            },
            plugins: Default::default(),
            env: Default::default(),
        };

        let result = manifest.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must be one of"));
    }

    #[test]
    fn test_all_supported_frameworks_validate() {
        for framework in get_supported_frameworks() {
            let manifest = ProfileManifest {
                profile: ProfileMeta {
                    name: "test".to_string(),
                    framework: framework.to_string(),
                    theme: "default".to_string(),
                    created: Utc::now(),
                    modified: Utc::now(),
                },
                plugins: Default::default(),
                env: Default::default(),
            };

            manifest.validate().expect(&format!(
                "Framework '{}' should validate successfully",
                framework
            ));
        }
    }

    #[test]
    fn test_parse_invalid_toml() {
        let toml = r#"
[profile
name = "broken"
        "#;

        let result = parse_manifest(toml);
        assert!(result.is_err());
        // Should contain line number hint
    }

    #[test]
    fn test_empty_plugin_validation() {
        let manifest = ProfileManifest {
            profile: ProfileMeta {
                name: "test".to_string(),
                framework: "oh-my-zsh".to_string(),
                theme: "default".to_string(),
                created: Utc::now(),
                modified: Utc::now(),
            },
            plugins: PluginsConfig {
                enabled: vec!["git".to_string(), "".to_string()],
            },
            env: Default::default(),
        };

        let result = manifest.validate();
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));
    }
}
```

### Project Structure Notes

**New Files Created:**
- `src/core/manifest.rs` - TOML manifest parsing and validation

**Modified Files:**
- `Cargo.toml` - Add dependencies: toml = "0.9", serde = { version = "1.0", features = ["derive"] }, chrono = { version = "latest", features = ["serde"] }
- `src/core/mod.rs` - Export manifest module

**Dependencies to Add:**
```toml
[dependencies]
serde = { version = "1.0", features = ["derive"] }
toml = "0.9"
chrono = { version = "0.4", features = ["serde"] }
anyhow = "2.0"
log = "0.4"
env_logger = "0.11"
dirs = "5.0"
```

**Learnings from Previous Stories:**

**From Story 1.10: Delete Profile (Status: drafted)**

Story 1.10 establishes patterns for working with profile directories. Story 2.1 builds on this to locate and validate profile.toml files:

- **Profile Path Pattern**: Profiles at `~/.zsh-profiles/profiles/<profile-name>/`
- **Manifest Location**: Each profile has `profile.toml` in its directory
- **Error Handling**: Use anyhow::Context for rich error messages
- **Path Utilities**: Build on existing path resolution patterns
- **Validation Pattern**: Similar defensive checks as profile validation

**From Story 1.9: Switch Active Profile (Status: drafted)**

Story 1.9 will eventually need to validate manifests before switching. Story 2.1 provides the validation infrastructure:

- **Pre-Switch Validation**: Story 1.9 can call `load_and_validate()` before exec
- **Config Reading**: Both stories work with TOML files
- **Error Messages**: Consistent error formatting across stories
- **Framework Field**: Manifest framework must match one of 5 supported values

**Integration Requirements:**
- Story 2.1 provides foundation for all manifest operations
- Story 2.2 will use this module to generate .zshrc/.zshenv
- Story 2.3 will validate manifests after manual editing
- Story 2.4-2.6 will use manifest parsing for import/export
- Stories 1.5-1.8 will generate manifests (reverse operation)

**ADR-002 Implementation Notes:**

This story implements ADR-002's decision to use TOML instead of YAML:

- **No indentation sensitivity**: TOML parse errors are clearer than YAML
- **Explicit typing**: [profile] sections are unambiguous
- **serde integration**: Well-maintained toml crate with serde
- **Rust ecosystem fit**: Follows Cargo.toml precedent

**Benefits Realized:**
- Better error messages with line numbers
- Easier to validate schema with strong typing
- Prevents common YAML indentation mistakes
- Familiar to Rust/Go developers

### References

- [Source: docs/epics.md#Story-2.1]
- [Source: docs/PRD.md#FR012-parse-validate-manifests]
- [Source: docs/architecture.md#ADR-002-TOML-not-YAML]
- [Source: docs/architecture.md#Pattern-4-TOML-Manifest-Schema]
- [Source: docs/architecture.md#Epic-2-Story-2.1-Mapping]
- [Source: docs/stories/1-9-switch-active-profile.md#config-pattern]
- [Source: docs/stories/1-10-delete-profile.md#profile-path-pattern]

## Dev Agent Record

### Context Reference

- docs/stories/2-1-parse-and-validate-yaml-manifests.context.xml

### Agent Model Used

<!-- Will be populated by dev agent during implementation -->

### Debug Log References

<!-- Will be populated by dev agent during implementation -->

### Completion Notes List

<!-- Will be populated by dev agent during implementation -->

### File List

<!-- Will be populated by dev agent during implementation -->

## Change Log

- 2025-10-31: Story drafted by SM agent (Bob)
