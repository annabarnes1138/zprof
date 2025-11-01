use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents profile metadata for display (used by list command)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileInfo {
    pub name: String,
    pub framework: String,
    pub is_active: bool,
}

/// Full profile metadata including timestamps (used by current command)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProfileMetadataFull {
    pub name: String,
    pub framework: String,
    pub theme: Option<String>,
    pub created: Option<String>,
    pub modified: Option<String>,
}

/// Profile manifest structure (profile.toml)
#[derive(Debug, Serialize, Deserialize)]
struct ProfileManifest {
    profile: ProfileMetadata,
}

#[derive(Debug, Serialize, Deserialize)]
struct ProfileMetadata {
    name: String,
    framework: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    theme: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    created: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    modified: Option<String>,
}

/// Scans the profiles directory and returns a list of profile information
pub fn scan_profiles(profiles_dir: &Path, active_profile: Option<&str>) -> Result<Vec<ProfileInfo>> {
    // Check if profiles directory exists
    if !profiles_dir.exists() {
        return Ok(Vec::new());
    }

    let mut profiles = Vec::new();

    // Read all entries in profiles directory
    let entries = fs::read_dir(profiles_dir)
        .with_context(|| format!("Failed to read profiles directory at {}", profiles_dir.display()))?;

    for entry in entries {
        let entry = entry.context("Failed to read directory entry")?;
        let path = entry.path();

        // Only process directories
        if !path.is_dir() {
            continue;
        }

        // Get profile name from directory name
        let profile_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name.to_string(),
            None => continue,
        };

        // Look for profile.toml in the profile directory
        let manifest_path = path.join("profile.toml");

        if !manifest_path.exists() {
            eprintln!("⚠ Warning: Profile '{}' is missing profile.toml, skipping", profile_name);
            continue;
        }

        // Read and parse profile.toml
        match read_profile_manifest(&manifest_path) {
            Ok(manifest) => {
                let is_active = active_profile.map(|ap| ap == manifest.profile.name).unwrap_or(false);
                profiles.push(ProfileInfo {
                    name: manifest.profile.name,
                    framework: manifest.profile.framework,
                    is_active,
                });
            }
            Err(e) => {
                eprintln!("⚠ Warning: Failed to read profile.toml for '{}': {}", profile_name, e);
                continue;
            }
        }
    }

    // Sort profiles alphabetically by name
    profiles.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(profiles)
}

/// Read and parse a profile.toml manifest file
fn read_profile_manifest(path: &Path) -> Result<ProfileManifest> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read profile manifest from {}", path.display()))?;

    let manifest: ProfileManifest = toml::from_str(&content)
        .with_context(|| format!("Failed to parse profile manifest at {}", path.display()))?;

    Ok(manifest)
}

/// Get the profiles directory path
pub fn get_profiles_dir() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .context("Failed to determine home directory")?;
    Ok(home.join(".zsh-profiles").join("profiles"))
}

/// Get the config file path
pub fn get_config_path() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .context("Failed to determine home directory")?;
    Ok(home.join(".zsh-profiles").join("config.toml"))
}

/// Load full profile metadata for a specific profile
pub fn load_profile_metadata(profile_name: &str) -> Result<ProfileMetadataFull> {
    let profiles_dir = get_profiles_dir()?;
    let profile_dir = profiles_dir.join(profile_name);
    let manifest_path = profile_dir.join("profile.toml");

    if !manifest_path.exists() {
        anyhow::bail!(
            "Active profile '{}' not found (may have been deleted)\n\n\
             Suggestion: Run 'zprof list' to see available profiles, then 'zprof use <name>' to activate one",
            profile_name
        );
    }

    let manifest = read_profile_manifest(&manifest_path)?;

    Ok(ProfileMetadataFull {
        name: manifest.profile.name,
        framework: manifest.profile.framework,
        theme: manifest.profile.theme,
        created: manifest.profile.created,
        modified: manifest.profile.modified,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_profile(dir: &Path, name: &str, framework: &str) -> Result<()> {
        let profile_dir = dir.join(name);
        fs::create_dir(&profile_dir)?;

        let manifest = format!(
            r#"[profile]
name = "{}"
framework = "{}"
theme = "robbyrussell"
created = "2025-10-31T14:30:00Z"
modified = "2025-10-31T14:30:00Z"
"#,
            name, framework
        );

        fs::write(profile_dir.join("profile.toml"), manifest)?;
        Ok(())
    }

    #[test]
    fn test_scan_profiles_empty_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let profiles = scan_profiles(temp_dir.path(), None)?;
        assert_eq!(profiles.len(), 0);
        Ok(())
    }

    #[test]
    fn test_scan_profiles_with_profiles() -> Result<()> {
        let temp_dir = TempDir::new()?;
        create_test_profile(temp_dir.path(), "work", "oh-my-zsh")?;
        create_test_profile(temp_dir.path(), "experimental", "zimfw")?;
        create_test_profile(temp_dir.path(), "minimal", "zinit")?;

        let profiles = scan_profiles(temp_dir.path(), Some("work"))?;

        assert_eq!(profiles.len(), 3);

        // Should be sorted alphabetically
        assert_eq!(profiles[0].name, "experimental");
        assert_eq!(profiles[0].framework, "zimfw");
        assert!(!profiles[0].is_active);

        assert_eq!(profiles[1].name, "minimal");
        assert_eq!(profiles[1].framework, "zinit");
        assert!(!profiles[1].is_active);

        assert_eq!(profiles[2].name, "work");
        assert_eq!(profiles[2].framework, "oh-my-zsh");
        assert!(profiles[2].is_active);

        Ok(())
    }

    #[test]
    fn test_scan_profiles_nonexistent_directory() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let nonexistent = temp_dir.path().join("nonexistent");
        let profiles = scan_profiles(&nonexistent, None)?;
        assert_eq!(profiles.len(), 0);
        Ok(())
    }

    #[test]
    fn test_scan_profiles_skips_invalid_entries() -> Result<()> {
        let temp_dir = TempDir::new()?;

        // Create valid profile
        create_test_profile(temp_dir.path(), "valid", "oh-my-zsh")?;

        // Create profile directory without profile.toml
        fs::create_dir(temp_dir.path().join("invalid"))?;

        // Create a file (not a directory) in profiles dir
        fs::write(temp_dir.path().join("somefile.txt"), "content")?;

        let profiles = scan_profiles(temp_dir.path(), None)?;

        // Should only find the valid profile
        assert_eq!(profiles.len(), 1);
        assert_eq!(profiles[0].name, "valid");

        Ok(())
    }
}
