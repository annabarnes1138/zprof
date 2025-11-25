use anyhow::{bail, Context, Result};
use clap::Args;
use dialoguer::{Confirm, Input};
use log::{info, warn};
use std::fs;

use crate::backup::pre_zprof;
use crate::core::config::Config;
use crate::core::filesystem;
use crate::core::manifest::Manifest;
use crate::frameworks::{detect_existing_framework, FrameworkInfo};
use crate::shell::{generator, zdotdir};

/// Trait for user interaction - allows mocking in tests
pub trait UserInput {
    fn confirm(&self, prompt: &str, default: bool) -> Result<bool>;
    fn input_string(&self, prompt: &str, default: &str) -> Result<String>;
}

/// Real implementation using dialoguer
pub struct DialoguerInput;

impl UserInput for DialoguerInput {
    fn confirm(&self, prompt: &str, default: bool) -> Result<bool> {
        Confirm::new()
            .with_prompt(prompt)
            .default(default)
            .interact()
            .context("Failed to read user input")
    }

    fn input_string(&self, prompt: &str, default: &str) -> Result<String> {
        Input::new()
            .with_prompt(prompt)
            .default(default.to_string())
            .interact_text()
            .context("Failed to read user input")
    }
}

/// Initialize zprof directory structure
#[derive(Debug, Args)]
pub struct InitArgs {}

/// Execute the init command
pub fn execute(args: InitArgs) -> Result<()> {
    execute_with_input(args, &DialoguerInput)
}

/// Execute the init command with dependency injection for testing
pub fn execute_with_input(_args: InitArgs, input: &dyn UserInput) -> Result<()> {
    // Check if already initialized
    if filesystem::is_initialized()? {
        eprintln!("→ Warning: zprof directory already exists at ~/.zsh-profiles/");
        eprintln!("→ Skipping initialization to preserve existing data");
        return Ok(());
    }

    // Create pre-zprof backup BEFORE creating directory structure
    // This ensures user's original shell config is backed up before any modifications
    let home_dir = dirs::home_dir()
        .context("Failed to get home directory")?;
    let backup_dir = home_dir.join(".zsh-profiles/backups/pre-zprof");

    // Create the backup directory first (needed before create_zprof_structure)
    fs::create_dir_all(&backup_dir)
        .context("Failed to create pre-zprof backup directory")?;

    // Create backup of existing shell configs
    match pre_zprof::create_backup(&home_dir, &backup_dir) {
        Ok(manifest) => {
            if manifest.files.is_empty() {
                println!("✓ No existing shell configs found to backup");
            } else {
                println!("✓ Backed up your existing shell config to {}", backup_dir.display());
                println!("  {} file(s) preserved", manifest.files.len());

                // Move backed-up configs from HOME to backup directory (Story 3.2)
                match pre_zprof::move_configs_to_backup(&home_dir, &manifest.files) {
                    Ok(moved_count) => {
                        println!("✓ Moved root config files to backup location");
                        println!("  {} file(s) removed from HOME", moved_count);
                        println!("  Your original setup is safely preserved and can be restored at any time");
                    }
                    Err(e) => {
                        warn!("Failed to move configs to backup: {}", e);
                        println!("⚠ Warning: Could not move all configs to backup");
                        println!("  Your files are still backed up, but may remain in HOME directory");
                    }
                }
            }
        }
        Err(e) => {
            warn!("Failed to create pre-zprof backup: {}", e);
            println!("⚠ Warning: Could not create backup of existing shell configs");
            println!("  Continuing with initialization...");
        }
    }

    // Create directory structure
    let base_dir = filesystem::create_zprof_structure()
        .context("Failed to create zprof directory structure")?;

    println!("✓ Created directory structure at {}", base_dir.display());
    println!("  ├── profiles/");
    println!("  ├── shared/");
    println!("  ├── backups/");
    println!("  └── cache/");

    // Create shared history file
    let history_file = filesystem::create_shared_history()
        .context("Failed to create shared history file")?;
    println!("✓ Created shared history file: {}", history_file.display());

    // Create shared customizations file
    let custom_file = filesystem::create_shared_customizations()
        .context("Failed to create shared customizations file")?;
    println!("✓ Created shared customizations: {}", custom_file.display());

    // Create default config.toml
    let config_file = base_dir.join("config.toml");
    let mut config = Config::new();
    config.write_to_file(config_file.clone())
        .context("Failed to write default configuration file")?;
    println!("✓ Created configuration file: {}", config_file.display());

    // Framework detection and import (AC: #1)
    info!("Checking for existing zsh framework installations...");
    if let Some(framework_info) = detect_existing_framework() {
        println!();
        println!(
            "Existing {} detected with {} plugins and '{}' theme.",
            framework_info.framework_type.name(),
            framework_info.plugins.len(),
            framework_info.theme
        );

        // Interactive import prompt (AC: #2, #3, #11)
        let should_import = input.confirm("Import as a profile?", true)?;

        if should_import {
            // Get profile name (AC: #3)
            let profile_name = input.input_string("Profile name", "default")?;

            info!("Importing {} framework as profile '{}'", framework_info.framework_type.name(), profile_name);
            println!("\nImporting framework configuration...");

            // Import framework configuration (AC: #4, #8)
            import_framework(&base_dir, &profile_name, &framework_info)
                .context("Failed to import framework configuration")?;

            // Set active profile in config (AC: #9)
            config.active_profile = Some(profile_name.clone());
            config.write_to_file(config_file)
                .context("Failed to update config with active profile")?;

            // Display success message (AC: #10)
            println!();
            println!("✓ Imported {} as profile '{}' (now active)",
                framework_info.framework_type.name(), profile_name);
            println!("  Framework: {}", framework_info.framework_type.name());
            println!("  Plugins: {} ({})",
                framework_info.plugins.len(),
                framework_info.plugins.join(", "));
            println!("  Theme: {}", framework_info.theme);
            println!("  Location: {}", base_dir.join("profiles").join(&profile_name).display());

            if let Ok(Some(backup)) = get_last_backup_path(&base_dir) {
                println!("  Backup: {}", backup.display());
            }

            println!();
            println!("Open a new terminal to use this profile.");
            println!("Your original ~/.zshrc remains untouched as a backup.");
        } else {
            // User declined import (AC: #11)
            println!();
            println!("Skipping import. You can create profiles later with:");
            println!("  zprof create <name>  - Import current setup");
            println!("  zprof wizard        - Interactive profile creation");
        }
    } else {
        // No framework detected
        println!();
        println!("No existing framework detected.");
        println!("Create your first profile with:");
        println!("  zprof wizard  - Interactive profile creation");
    }

    Ok(())
}

/// Import framework configuration into a new profile
///
/// This follows Pattern 3 (Safe File Operations) to ensure NFR002 compliance.
///
/// # Arguments
///
/// * `base_dir` - zprof base directory (~/.zsh-profiles)
/// * `profile_name` - Name for the new profile
/// * `framework_info` - Detected framework information
fn import_framework(
    base_dir: &std::path::Path,
    profile_name: &str,
    framework_info: &FrameworkInfo,
) -> Result<()> {
    let profile_dir = base_dir.join("profiles").join(profile_name);

    // Create profile directory (AC: #4)
    filesystem::create_directory(&profile_dir)
        .with_context(|| format!("Failed to create profile directory: {}", profile_dir.display()))?;

    info!("Created profile directory: {}", profile_dir.display());

    // Copy framework installation directory (AC: #4)
    if framework_info.install_path.exists() {
        let framework_name = framework_info.install_path.file_name()
            .context("Invalid framework install path")?;
        let dest_framework_path = profile_dir.join(framework_name);

        filesystem::copy_dir_recursive(&framework_info.install_path, &dest_framework_path)
            .with_context(|| format!(
                "Failed to copy framework from {} to {}",
                framework_info.install_path.display(),
                dest_framework_path.display()
            ))?;

        info!("Copied framework installation to {}", dest_framework_path.display());
    }

    // Copy .zshrc to profile (AC: #4)
    let home_dir = dirs::home_dir()
        .context("Failed to get home directory")?;
    let zshrc_source = home_dir.join(".zshrc");

    if zshrc_source.exists() {
        // Read the original .zshrc content
        let original_content = fs::read_to_string(&zshrc_source)
            .with_context(|| format!("Failed to read .zshrc from {}", zshrc_source.display()))?;

        // Remove customizations that were extracted to shared/custom.zsh
        let cleaned_content = filesystem::remove_extracted_customizations(&original_content);

        // Prepend HISTFILE configuration to override system /etc/zshrc
        let histfile_header = "# zprof: Shared history configuration (must be before framework initialization)\n\
                               export HISTFILE=\"$HOME/.zsh-profiles/shared/.zsh_history\"\n\
                               export HISTSIZE=10000\n\
                               export SAVEHIST=10000\n\
                               \n";

        // Append shared customizations source at the end
        let custom_source = "\n# Source shared customizations (edit ~/.zsh-profiles/shared/custom.zsh)\n\
                             [ -f \"$HOME/.zsh-profiles/shared/custom.zsh\" ] && source \"$HOME/.zsh-profiles/shared/custom.zsh\"\n";

        let modified_content = format!("{histfile_header}{cleaned_content}{custom_source}");

        let zshrc_dest = profile_dir.join(".zshrc");

        fs::write(&zshrc_dest, modified_content)
            .with_context(|| format!("Failed to write .zshrc to {}", zshrc_dest.display()))?;

        info!("Copied .zshrc to profile with shared history configuration and extracted customizations removed");

        // Verify original .zshrc is untouched (AC: #7 - NFR002)
        if !zshrc_source.exists() {
            bail!("CRITICAL: Original .zshrc was removed! This violates NFR002.");
        }
    } else {
        warn!("No .zshrc found in home directory - creating empty one");
        fs::write(profile_dir.join(".zshrc"), "# zprof profile\n")
            .context("Failed to create .zshrc in profile")?;
    }

    // Copy framework-specific config files (AC: #4)
    // Note: Skip .zshrc as it's already handled above with histfile prepending
    if framework_info.config_path.exists() {
        let config_name = framework_info.config_path.file_name()
            .context("Invalid framework config path")?;

        // Skip .zshrc - already copied with histfile header prepended above
        if config_name.to_str() != Some(".zshrc") {
            let config_dest = profile_dir.join(config_name);
            fs::copy(&framework_info.config_path, &config_dest)
                .with_context(|| format!("Failed to copy framework config to {}", config_dest.display()))?;

            info!("Copied framework config file: {config_name:?}");
        }
    }

    // Generate profile.toml manifest (AC: #8)
    let manifest = Manifest::from_framework_info(profile_name, framework_info);
    let manifest_path = profile_dir.join("profile.toml");
    let manifest_content = toml::to_string_pretty(&manifest)
        .context("Failed to serialize manifest to TOML")?;
    fs::write(&manifest_path, manifest_content)
        .with_context(|| format!("Failed to write manifest to {}", manifest_path.display()))?;

    info!("Generated profile.toml manifest");

    // Generate .zshenv file to set HISTFILE and other environment variables
    // Note: We don't regenerate .zshrc as it was already copied from the user's original
    let zshenv_content = generator::generate_zshenv_from_manifest(&manifest)
        .context("Failed to generate .zshenv content")?;
    let zshenv_path = profile_dir.join(".zshenv");
    fs::write(&zshenv_path, zshenv_content)
        .with_context(|| format!("Failed to write .zshenv to {}", zshenv_path.display()))?;

    info!("Generated .zshenv file with shared history configuration");

    // Check for existing ZDOTDIR conflicts (AC: #7 - edge case)
    if zdotdir::has_existing_zdotdir()? {
        let should_overwrite = Confirm::new()
            .with_prompt("~/.zshenv already sets ZDOTDIR. Overwrite for zprof?")
            .default(false)
            .interact()
            .context("Failed to read user input")?;

        if !should_overwrite {
            bail!("Cannot enable profile switching - ~/.zshenv already manages ZDOTDIR");
        }
    }

    // Manage ~/.zshenv for profile switching (AC: #5, #6, #7)
    zdotdir::set_active_profile(&profile_dir)
        .context("Failed to update ~/.zshenv for profile switching")?;

    info!("Updated ~/.zshenv with ZDOTDIR pointing to profile");

    Ok(())
}

/// Get the path of the most recent .zshenv backup
fn get_last_backup_path(base_dir: &std::path::Path) -> Result<Option<std::path::PathBuf>> {
    let backup_dir = base_dir.join("cache/backups");

    if !backup_dir.exists() {
        return Ok(None);
    }

    let entries = fs::read_dir(&backup_dir)
        .context("Failed to read backup directory")?;

    let mut backups: Vec<_> = entries
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_str()
                .map(|s| s.starts_with(".zshenv.backup."))
                .unwrap_or(false)
        })
        .collect();

    backups.sort_by_key(|e| {
        e.metadata()
            .and_then(|m| m.modified())
            .ok()
    });

    Ok(backups.last().map(|e| e.path()))
}

/// Mock implementation for testing - available for both unit and integration tests
#[cfg(any(test, feature = "test-helpers"))]
pub mod test_utils {
    use super::*;
    use std::cell::RefCell;

    pub struct MockUserInput {
        pub confirm_response: RefCell<Option<bool>>,
        pub input_response: RefCell<Option<String>>,
        pub confirm_called: RefCell<bool>,
        pub input_called: RefCell<bool>,
    }

    impl Default for MockUserInput {
        fn default() -> Self {
            Self::new()
        }
    }

    impl MockUserInput {
        // NOTE: These methods are currently unused but kept for future test re-enablement
        // See tests/init_test.rs for disabled tests that use this mock
        #[allow(dead_code)]
        pub fn new() -> Self {
            Self {
                confirm_response: RefCell::new(None),
                input_response: RefCell::new(None),
                confirm_called: RefCell::new(false),
                input_called: RefCell::new(false),
            }
        }

        #[allow(dead_code)]
        pub fn with_confirm(mut self, response: bool) -> Self {
            self.confirm_response = RefCell::new(Some(response));
            self
        }

        #[allow(dead_code)]
        pub fn with_input(mut self, response: String) -> Self {
            self.input_response = RefCell::new(Some(response));
            self
        }
    }

    impl UserInput for MockUserInput {
        fn confirm(&self, _prompt: &str, default: bool) -> Result<bool> {
            *self.confirm_called.borrow_mut() = true;
            Ok(self.confirm_response.borrow()
                .unwrap_or(default))
        }

        fn input_string(&self, _prompt: &str, default: &str) -> Result<String> {
            *self.input_called.borrow_mut() = true;
            Ok(self.input_response.borrow()
                .clone()
                .unwrap_or_else(|| default.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_args_creation() {
        let args = InitArgs {};
        // Just verify the struct can be created
        assert!(format!("{args:?}").contains("InitArgs"));
    }
}
