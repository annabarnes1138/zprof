//! Create profile from preset implementation
//!
//! This module handles the creation of profiles using pre-defined presets.
//! It reuses core logic from the main create command but skips the wizard steps.

use anyhow::{bail, Context, Result};
use std::fs;

use crate::cli::create::{get_profile_dir, update_global_config, validate_profile_name};
use crate::core::filesystem::{self, create_shared_history};
use crate::core::manifest::Manifest;
use crate::frameworks::installer::{self, WizardState};
use crate::presets::Preset;
use crate::shell::generator;

/// Create a new profile from a preset configuration
///
/// # Arguments
///
/// * `profile_name` - Name of the new profile
/// * `preset` - The preset configuration to use
/// * `interactive` - Whether to allow interactive prompts (e.g. for switching profile)
pub fn create_from_preset(profile_name: &str, preset: &Preset, interactive: bool) -> Result<()> {
    println!("Creating profile '{}' using '{}' preset...", profile_name, preset.name);

    // 1. Validate profile name
    validate_profile_name(profile_name)?;

    // Check if profile already exists
    let profile_dir = get_profile_dir(profile_name)?;
    if profile_dir.exists() {
        bail!(
            "✗ Error: Profile '{}' already exists\n  → Use 'zprof delete {}' first or choose a different name",
            profile_name,
            profile_name
        );
    }

    // 2. Create profile directory and ensure shared history exists
    fs::create_dir_all(&profile_dir).with_context(|| {
        format!(
            "Failed to create profile directory at {}",
            profile_dir.display()
        )
    })?;

    // Ensure shared history file exists for cross-profile history sharing
    create_shared_history()
        .context("Failed to create shared history file")?;

    // Ensure shared customizations file exists
    filesystem::create_shared_customizations()
        .context("Failed to create shared customizations file")?;

    // 3. Install framework and plugins
    // Create a WizardState from the preset configuration
    let wizard_state = WizardState {
        profile_name: profile_name.to_string(),
        framework: preset.config.framework.clone(),
        plugins: preset.config.plugins.iter().map(|s| s.to_string()).collect(),
        theme: preset.config.framework_theme.unwrap_or_default().to_string(),
        prompt_engine: preset.config.prompt_engine.map(|s| s.to_string()),
    };

    println!(); // Blank line before progress indicator
    installer::install_profile(&wizard_state, &profile_dir)
        .context("Failed to install framework and plugins")?;

    // 4. Generate TOML manifest
    let manifest = Manifest::from_preset(profile_name, preset);
    let manifest_path = profile_dir.join("profile.toml");
    manifest
        .write_to_file(&manifest_path)
        .context("Failed to write profile manifest")?;

    // 5. Generate shell configuration files
    generator::write_generated_files(profile_name, &manifest)
        .context("Failed to generate shell configuration files")?;

    // 6. Update global config
    update_global_config(profile_name)?;

    // 7. Display success message
    // Create a temporary FrameworkInfo for display_success
    // Note: This is a bit of a hack since display_success expects FrameworkInfo,
    // but we can construct a minimal one that satisfies the display needs.
    let framework_info = crate::frameworks::FrameworkInfo {
        framework_type: preset.config.framework.clone(),
        plugins: preset.config.plugins.iter().map(|s| s.to_string()).collect(),
        theme: preset.config.framework_theme.unwrap_or_default().to_string(),
        config_path: std::path::PathBuf::new(), // Not used for display
        install_path: std::path::PathBuf::new(), // Not used for display
    };

    crate::cli::create::display_success(profile_name, &framework_info, &profile_dir, interactive)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_profile_name_reused() {
        // Verify we can access the reused validation logic
        assert!(validate_profile_name("valid-name").is_ok());
        assert!(validate_profile_name("invalid/name").is_err());
    }

    // Note: Full integration testing of create_from_preset requires mocking
    // the filesystem and installer, which is better handled in integration tests
    // or by refactoring the installer to be more testable.
    // For now, we rely on the shared components being tested in their respective modules.
}
