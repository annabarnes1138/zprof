use anyhow::{Context, Result};
use clap::Args;

use crate::core::{config::Config, profile};

/// List all available zsh profiles
#[derive(Debug, Args)]
pub struct ListArgs {}

pub fn execute(_args: ListArgs) -> Result<()> {
    // Get the profiles directory path
    let profiles_dir = profile::get_profiles_dir()?;
    let config_path = profile::get_config_path();

    // Check if zsh-profiles directory exists
    if !profiles_dir.parent().unwrap().exists() {
        anyhow::bail!(
            "✗ zprof directory not found\n\n\
             Run 'zprof init' to initialize the directory structure first."
        );
    }

    // Load active profile from config (if exists)
    let active_profile = match config_path {
        Ok(path) if path.exists() => {
            Config::load_from_file(path)
                .ok()
                .and_then(|c| c.active_profile)
        }
        _ => None,
    };

    // Scan profiles directory
    let profiles = profile::scan_profiles(&profiles_dir, active_profile.as_deref())
        .context("Failed to scan profiles directory")?;

    // Handle empty profiles directory
    if profiles.is_empty() {
        println!("No profiles found. Create your first profile with 'zprof create <name>'");
        return Ok(());
    }

    // Display profiles
    println!("Available profiles:\n");

    for profile_info in profiles {
        let indicator = if profile_info.is_active { "→" } else { " " };
        println!(
            "{} {:<15} ({})",
            indicator, profile_info.name, profile_info.framework
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;
    use std::fs;
    use tempfile::TempDir;

    fn setup_test_environment() -> Result<TempDir> {
        let temp_dir = TempDir::new()?;
        let zprof_dir = temp_dir.path().join(".zsh-profiles");
        let profiles_dir = zprof_dir.join("profiles");
        fs::create_dir_all(&profiles_dir)?;

        Ok(temp_dir)
    }

    fn create_test_profile(base_dir: &std::path::Path, name: &str, framework: &str) -> Result<()> {
        let profile_dir = base_dir
            .join(".zsh-profiles")
            .join("profiles")
            .join(name);
        fs::create_dir(&profile_dir)?;

        let manifest = format!(
            r#"[profile]
name = "{name}"
framework = "{framework}"
theme = "robbyrussell"
created = "2025-10-31T14:30:00Z"
modified = "2025-10-31T14:30:00Z"
"#
        );

        fs::write(profile_dir.join("profile.toml"), manifest)?;
        Ok(())
    }

    fn create_test_config(base_dir: &std::path::Path, active_profile: Option<&str>) -> Result<()> {
        let mut config = Config::new();
        if let Some(active) = active_profile {
            config.active_profile = Some(active.to_string());
        }
        config.write_to_file(base_dir.join(".zsh-profiles").join("config.toml"))?;
        Ok(())
    }

    #[test]
    fn test_list_profiles_basic() -> Result<()> {
        let temp_dir = setup_test_environment()?;
        create_test_profile(temp_dir.path(), "work", "oh-my-zsh")?;
        create_test_profile(temp_dir.path(), "experimental", "zimfw")?;
        create_test_config(temp_dir.path(), Some("work"))?;

        // Note: This is a unit test for the module structure
        // Full integration tests with output capture are in tests/list_test.rs
        Ok(())
    }
}
