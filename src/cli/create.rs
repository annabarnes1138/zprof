//! Profile creation command implementation
//!
//! This module implements the `zprof create` command following Pattern 1: CLI Command Structure.
//! It handles profile creation with optional import from existing framework configuration.

use anyhow::{bail, Context, Result};
use clap::Args;
use dialoguer::Confirm;
use regex::Regex;
use std::fs;
use std::path::PathBuf;

use crate::core::config::Config;
use crate::core::filesystem::{copy_dir_recursive, get_zprof_dir};
use crate::core::manifest::Manifest;
use crate::frameworks::detect_existing_framework;

/// Arguments for the create command
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Name of the profile to create
    #[arg(value_name = "NAME")]
    pub name: String,
}

/// Execute the create command
///
/// Follows Pattern 1: CLI Command Structure
/// 1. Validate inputs
/// 2. Load config
/// 3. Perform operation
/// 4. Display output
pub fn execute(args: CreateArgs) -> Result<()> {
    // 1. Validate profile name
    validate_profile_name(&args.name)?;

    // Check if profile already exists
    let profile_dir = get_profile_dir(&args.name)?;
    if profile_dir.exists() {
        bail!(
            "✗ Error: Profile '{}' already exists\n  → Use 'zprof delete {}' first or choose a different name",
            args.name,
            args.name
        );
    }

    // 2. Detect existing framework
    let framework_info = match detect_existing_framework() {
        Some(info) => info,
        None => {
            println!("No existing zsh framework detected.");
            println!("  → Run the TUI wizard to create a profile from scratch (coming in Story 1.6)");
            return Ok(());
        }
    };

    // 3. Prompt for import
    println!(
        "Detected {} with {} plugins ({}) and theme '{}'.",
        framework_info.framework_type.name(),
        framework_info.plugins.len(),
        framework_info.plugins.join(", "),
        framework_info.theme
    );

    let should_import = Confirm::new()
        .with_prompt("Import current setup?")
        .default(true)
        .interact()
        .context("Failed to read user input for import confirmation")?;

    if !should_import {
        println!("Import skipped.");
        println!("  → Run the TUI wizard to create a profile from scratch (coming in Story 1.6)");
        return Ok(());
    }

    // 4. Create profile directory
    fs::create_dir_all(&profile_dir).with_context(|| {
        format!(
            "Failed to create profile directory at {}",
            profile_dir.display()
        )
    })?;

    // 5. Copy framework files
    copy_framework_files(&framework_info, &profile_dir)?;

    // 6. Generate TOML manifest
    let manifest = Manifest::from_framework_info(&args.name, &framework_info);
    let manifest_path = profile_dir.join("profile.toml");
    manifest
        .write_to_file(&manifest_path)
        .context("Failed to write profile manifest")?;

    // 7. Update global config (create if doesn't exist)
    update_global_config(&args.name)?;

    // 8. Display success message
    display_success(&args.name, &framework_info, &profile_dir)?;

    Ok(())
}

/// Validate profile name against allowed pattern
///
/// Profile names must:
/// - Be non-empty
/// - Contain only alphanumeric characters and hyphens
/// - Not contain path traversal attempts
fn validate_profile_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("✗ Error: Profile name cannot be empty");
    }

    // Check for valid characters (alphanumeric and hyphens)
    let valid_pattern = Regex::new(r"^[a-zA-Z0-9\-]+$").unwrap();
    if !valid_pattern.is_match(name) {
        bail!(
            "✗ Error: Invalid profile name '{}'\n  → Use alphanumeric characters and hyphens only",
            name
        );
    }

    // Check for path traversal attempts
    if name.contains("..") || name.contains('/') || name.contains('\\') {
        bail!(
            "✗ Error: Invalid profile name '{}'\n  → Profile names cannot contain path separators",
            name
        );
    }

    Ok(())
}

/// Get the profile directory path
fn get_profile_dir(name: &str) -> Result<PathBuf> {
    let base_dir = get_zprof_dir()?;
    Ok(base_dir.join("profiles").join(name))
}

/// Copy framework files to profile directory
///
/// Follows Pattern 3: Safe File Operations
/// - Uses copy NOT move (NFR002 compliance)
/// - Preserves original dotfiles
fn copy_framework_files(
    framework_info: &crate::frameworks::FrameworkInfo,
    profile_dir: &PathBuf,
) -> Result<()> {
    let home_dir = dirs::home_dir().context("Failed to get home directory")?;

    // Copy framework installation directory (e.g., ~/.oh-my-zsh)
    if framework_info.install_path.exists() {
        let framework_dest = profile_dir.join(
            framework_info
                .install_path
                .file_name()
                .context("Failed to get framework directory name")?,
        );
        copy_dir_recursive(&framework_info.install_path, &framework_dest).with_context(|| {
            format!(
                "Failed to copy framework directory from {}",
                framework_info.install_path.display()
            )
        })?;
    }

    // Copy .zshrc
    let zshrc_source = home_dir.join(".zshrc");
    if zshrc_source.exists() {
        let zshrc_dest = profile_dir.join(".zshrc");
        fs::copy(&zshrc_source, &zshrc_dest).with_context(|| {
            format!(
                "Failed to copy .zshrc from {} to {}",
                zshrc_source.display(),
                zshrc_dest.display()
            )
        })?;

        // Verify original .zshrc still exists (NFR002)
        if !zshrc_source.exists() {
            bail!("Original .zshrc missing after copy! This should never happen.");
        }
    }

    // Copy .zshenv if exists
    let zshenv_source = home_dir.join(".zshenv");
    if zshenv_source.exists() {
        let zshenv_dest = profile_dir.join(".zshenv");
        fs::copy(&zshenv_source, &zshenv_dest).with_context(|| {
            format!(
                "Failed to copy .zshenv from {} to {}",
                zshenv_source.display(),
                zshenv_dest.display()
            )
        })?;
    }

    // Copy framework-specific config files if they exist
    let framework_configs = vec![".zimrc", ".zpreztorc"];
    for config_file in framework_configs {
        let config_source = home_dir.join(config_file);
        if config_source.exists() {
            let config_dest = profile_dir.join(config_file);
            fs::copy(&config_source, &config_dest).with_context(|| {
                format!(
                    "Failed to copy {} from {} to {}",
                    config_file,
                    config_source.display(),
                    config_dest.display()
                )
            })?;
        }
    }

    Ok(())
}

/// Update global config to track new profile
fn update_global_config(profile_name: &str) -> Result<()> {
    let base_dir = get_zprof_dir()?;
    let config_path = base_dir.join("config.toml");

    // Load or create config
    let mut config = if config_path.exists() {
        Config::load_from_file(config_path.clone())?
    } else {
        Config::new()
    };

    // Set as active profile if no active profile exists
    if config.active_profile.is_none() {
        config.active_profile = Some(profile_name.to_string());
    }

    // Write config
    config.write_to_file(config_path)?;

    Ok(())
}

/// Display success message with profile details
fn display_success(
    name: &str,
    framework_info: &crate::frameworks::FrameworkInfo,
    profile_dir: &PathBuf,
) -> Result<()> {
    println!("\n✓ Profile '{}' created successfully", name);
    println!("  Framework: {}", framework_info.framework_type.name());
    println!("  Plugins: {} ({})", framework_info.plugins.len(), framework_info.plugins.join(", "));
    println!("  Theme: {}", framework_info.theme);
    println!("  Location: {}", profile_dir.display());
    println!("\n  → Use 'zprof use {}' to switch to this profile", name);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_profile_name_valid() {
        assert!(validate_profile_name("work").is_ok());
        assert!(validate_profile_name("personal-2024").is_ok());
        assert!(validate_profile_name("MyProfile").is_ok());
        assert!(validate_profile_name("dev-env-123").is_ok());
    }

    #[test]
    fn test_validate_profile_name_invalid() {
        assert!(validate_profile_name("").is_err());
        assert!(validate_profile_name("profile/name").is_err());
        assert!(validate_profile_name("profile\\name").is_err());
        assert!(validate_profile_name("../etc").is_err());
        assert!(validate_profile_name("profile name").is_err());
        assert!(validate_profile_name("profile@name").is_err());
    }

    #[test]
    fn test_validate_profile_name_error_messages() {
        let result = validate_profile_name("");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("cannot be empty"));

        let result = validate_profile_name("profile@name");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("alphanumeric characters and hyphens"));
    }
}
