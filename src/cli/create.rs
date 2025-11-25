//! Profile creation command implementation
//!
//! This module implements the `zprof create` command following Pattern 1: CLI Command Structure.
//! It handles profile creation with optional import from existing framework configuration.

use anyhow::{bail, Context, Result};
use clap::Args;
use dialoguer::Confirm;
use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::config::Config;
use crate::core::filesystem::{self, copy_dir_recursive, create_shared_history, get_zprof_dir};
use crate::core::manifest::{Manifest, PromptMode};
use crate::frameworks::detect_existing_framework;
use crate::frameworks::installer::{self, WizardState};
use crate::tui::{framework_select, plugin_browser, preset_select, prompt_mode_select, setup_mode_select, theme_select};
use crate::shell::generator;

/// Arguments for the create command
#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Name of the profile to create
    #[arg(value_name = "NAME")]
    pub name: String,

    /// Create profile from a preset (minimal, performance, fancy, developer)
    ///
    /// Skip the interactive wizard and use a pre-configured preset.
    /// Example: zprof create work --preset performance
    #[arg(long, value_name = "PRESET_NAME")]
    pub preset: Option<String>,
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

    // 2. If --preset flag provided, skip TUI and create from preset
    if let Some(preset_name) = &args.preset {
        use crate::presets::find_preset_by_id;

        // Look up preset (case-insensitive)
        let preset = find_preset_by_id(preset_name).ok_or_else(|| {
            // Build helpful error message with available presets
            let available: Vec<&str> = crate::presets::PRESET_REGISTRY
                .iter()
                .map(|p| p.id)
                .collect();

            anyhow::anyhow!(
                "✗ Error: Preset '{}' not found\n  → Available presets: {}\n  → Use 'zprof create {}' for interactive wizard",
                preset_name,
                available.join(", "),
                args.name
            )
        })?;

        // Create profile from preset (non-interactive)
        crate::cli::create_from_preset::create_from_preset(&args.name, preset, false)?;

        // Show summary message
        println!("\n✓ Profile '{}' created using {} preset", args.name, preset.name);
        println!("  → Framework: {}", preset.config.framework.name());
        if let Some(engine) = preset.config.prompt_engine {
            println!("  → Prompt Engine: {}", engine);
        } else if let Some(theme) = preset.config.framework_theme {
            println!("  → Theme: {}", theme);
        }
        println!("  → Plugins: {} configured", preset.config.plugins.len());
        println!("\n  Use 'zprof use {}' to activate this profile", args.name);

        return Ok(());
    }

    // 3. Detect existing framework
    let detected_framework = detect_existing_framework();

    // 3. Determine framework (from detection or TUI wizard)
    let (selected_framework, selected_theme, should_import_files, wizard_plugins) = if let Some(info) = detected_framework.as_ref() {
        // Framework detected - prompt for import
        println!(
            "Detected {} with {} plugins ({}) and theme '{}'.",
            info.framework_type.name(),
            info.plugins.len(),
            info.plugins.join(", "),
            info.theme
        );

        let should_import = Confirm::new()
            .with_prompt("Import current setup?")
            .default(false)
            .interact()
            .context("Failed to read user input for import confirmation")?;

        if should_import {
            (info.framework_type.clone(), info.theme.clone(), true, vec![])
        } else {
            // User declined import - launch TUI wizard
            println!("Import skipped. Launching TUI wizard...\n");

            // Step 1: Setup mode selection (Quick vs Custom)
            let setup_mode = setup_mode_select::select_setup_mode()
                .context("Setup mode selection cancelled. Profile creation aborted.")?;

            // Branch based on setup mode
            match setup_mode {
                setup_mode_select::SetupMode::Quick => {
                    println!("Quick setup selected.\n");

                    // Launch preset selection (Story 2.4)
                    let preset_choice = preset_select::select_preset()
                        .context("Preset selection cancelled. Profile creation aborted.")?;

                    match preset_choice {
                        preset_select::PresetChoice::Preset(preset) => {
                            // Story 2.5: Create profile from preset
                            crate::cli::create_from_preset::create_from_preset(&args.name, preset, true)
                                .context("Failed to create profile from preset")?;
                            
                            // Return early as create_from_preset handles everything including success message
                            return Ok(());
                        }
                        preset_select::PresetChoice::Custom => {
                            // User chose "Customize (advanced)" - fall through to custom wizard
                            println!("Custom setup selected.\n");
                        }
                    }
                }
                setup_mode_select::SetupMode::Custom => {
                    println!("Custom setup selected.\n");
                }
            }

            let selected = framework_select::run_framework_selection(&args.name)
                .context("Framework selection cancelled. Profile creation aborted.")?;

            // Launch prompt mode selection (Story 1.2)
            let prompt_mode_type = prompt_mode_select::run_prompt_mode_selection()
                .context("Prompt mode selection cancelled. Profile creation aborted.")?;

            // Launch plugin browser (Story 1.7)
            let plugins = plugin_browser::run_plugin_selection(selected.clone())
                .context("Plugin selection cancelled. Profile creation aborted.")?;

            // Launch theme selection (Story 1.5) - conditional based on prompt mode
            let prompt_mode = match prompt_mode_type {
                prompt_mode_select::PromptModeType::PromptEngine => {
                    // TODO: For now, we'll use a placeholder engine. Story 1.4 will implement engine selection.
                    PromptMode::PromptEngine {
                        engine: "starship".to_string(),
                    }
                }
                prompt_mode_select::PromptModeType::FrameworkTheme => {
                    let theme = theme_select::run_theme_selection(selected.clone(), &plugins, PromptMode::FrameworkTheme { theme: String::new() })
                        .context("Theme selection cancelled. Profile creation aborted.")?;
                    PromptMode::FrameworkTheme { theme }
                }
            };

            let theme = match &prompt_mode {
                PromptMode::FrameworkTheme { theme } => theme.clone(),
                PromptMode::PromptEngine { .. } => String::new(),
            };

            let prompt_engine = match &prompt_mode {
                PromptMode::PromptEngine { engine } => Some(engine.clone()),
                PromptMode::FrameworkTheme { .. } => None,
            };

            // Show confirmation screen (Story 1.8)
            let wizard_state = WizardState {
                profile_name: args.name.clone(),
                framework: selected.clone(),
                plugins: plugins.clone(),
                theme: theme.clone(),
                prompt_engine: prompt_engine.clone(),
            };

            let confirmed = theme_select::show_confirmation_screen(&wizard_state)
                .context("Failed to show confirmation screen")?;

            if !confirmed {
                bail!("Profile creation cancelled by user.");
            }

            log::info!("User selected {} plugins, theme '{}' for {:?}", plugins.len(), theme, selected);

            (selected, theme, false, plugins)
        }
    } else {
        // No framework detected - launch TUI wizard
        println!("No existing zsh framework detected.");
        println!("Launching TUI wizard to create profile from scratch...\n");

        // Step 1: Setup mode selection (Quick vs Custom)
        let setup_mode = setup_mode_select::select_setup_mode()
            .context("Setup mode selection cancelled. Profile creation aborted.")?;

        // Branch based on setup mode
        match setup_mode {
            setup_mode_select::SetupMode::Quick => {
                // TODO: Quick setup flow will be implemented in subsequent stories
                // For now, fall back to custom setup
                println!("Quick setup selected. (Note: Quick setup with presets will be implemented in upcoming stories)");
                println!("Falling back to custom setup for now...\n");
            }
            setup_mode_select::SetupMode::Custom => {
                println!("Custom setup selected.\n");
            }
        }

        let selected = framework_select::run_framework_selection(&args.name)
            .context("Framework selection cancelled. Profile creation aborted.")?;

        // Launch prompt mode selection (Story 1.2)
        let prompt_mode_type = prompt_mode_select::run_prompt_mode_selection()
            .context("Prompt mode selection cancelled. Profile creation aborted.")?;

        // Launch plugin browser (Story 1.7)
        let plugins = plugin_browser::run_plugin_selection(selected.clone())
            .context("Plugin selection cancelled. Profile creation aborted.")?;

        // Launch theme selection (Story 1.5) - conditional based on prompt mode
        let prompt_mode = match prompt_mode_type {
            prompt_mode_select::PromptModeType::PromptEngine => {
                // TODO: For now, we'll use a placeholder engine. Story 1.4 will implement engine selection.
                PromptMode::PromptEngine {
                    engine: "starship".to_string(),
                }
            }
            prompt_mode_select::PromptModeType::FrameworkTheme => {
                let theme = theme_select::run_theme_selection(selected.clone(), &plugins, PromptMode::FrameworkTheme { theme: String::new() })
                    .context("Theme selection cancelled. Profile creation aborted.")?;
                PromptMode::FrameworkTheme { theme }
            }
        };

        let theme = match &prompt_mode {
            PromptMode::FrameworkTheme { theme } => theme.clone(),
            PromptMode::PromptEngine { .. } => String::new(),
        };

            let prompt_engine = match &prompt_mode {
                PromptMode::PromptEngine { engine } => Some(engine.clone()),
                PromptMode::FrameworkTheme { .. } => None,
            };

            // Show confirmation screen (Story 1.8)
            let wizard_state = WizardState {
                profile_name: args.name.clone(),
                framework: selected.clone(),
                plugins: plugins.clone(),
                theme: theme.clone(),
                prompt_engine: prompt_engine.clone(),
            };

        let confirmed = theme_select::show_confirmation_screen(&wizard_state)
            .context("Failed to show confirmation screen")?;

        if !confirmed {
            bail!("Profile creation cancelled by user.");
        }

        log::info!("User selected {} plugins, theme '{}' for {:?}", plugins.len(), theme, selected);

        (selected, theme, false, plugins)
    };

    // Build framework info for profile creation
    let framework_info = if should_import_files {
        // Use detected framework info
        detected_framework.unwrap()
    } else {
        // TUI was used - create framework info with wizard-selected plugins and theme
        crate::frameworks::FrameworkInfo {
            framework_type: selected_framework.clone(),
            plugins: wizard_plugins,
            theme: selected_theme.clone(),
            config_path: std::path::PathBuf::new(),
            install_path: std::path::PathBuf::new(),
        }
    };

    // 4. Create profile directory and ensure shared history exists
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

    // 5. Install framework and plugins (Story 1.8) or copy existing files
    if should_import_files {
        // Import path: copy existing framework files
        copy_framework_files(&framework_info, &profile_dir)?;
    } else {
        // Wizard path: install framework and plugins (AC #4, #7)
        let wizard_state = WizardState {
            profile_name: args.name.clone(),
            framework: selected_framework.clone(),
            plugins: framework_info.plugins.clone(),
            theme: selected_theme.clone(),
            // For now, if we came from import, we don't have prompt engine info easily available
            // If we came from wizard, we lost it because FrameworkInfo doesn't store it
            // TODO: Update FrameworkInfo to store prompt_engine
            prompt_engine: None, 
        };

        println!(); // Blank line before progress indicator
        installer::install_profile(&wizard_state, &profile_dir)
            .context("Failed to install framework and plugins")?;
    }

    // 6. Generate TOML manifest
    let manifest = Manifest::from_framework_info(&args.name, &framework_info);
    let manifest_path = profile_dir.join("profile.toml");
    manifest
        .write_to_file(&manifest_path)
        .context("Failed to write profile manifest")?;

    // 6.5. Generate shell configuration files (Story 1.8)
    generator::write_generated_files(&args.name, &manifest)
        .context("Failed to generate shell configuration files")?;

    // 7. Update global config (create if doesn't exist)
    update_global_config(&args.name)?;

    // 8. Display success message
    display_success(&args.name, &framework_info, &profile_dir, true)?;

    Ok(())
}

/// Validate profile name against allowed pattern
///
/// Profile names must:
/// - Be non-empty
/// - Contain only alphanumeric characters and hyphens
/// - Not contain path traversal attempts
pub(crate) fn validate_profile_name(name: &str) -> Result<()> {
    if name.is_empty() {
        bail!("✗ Error: Profile name cannot be empty");
    }

    // Check for valid characters (alphanumeric and hyphens)
    let valid_pattern = Regex::new(r"^[a-zA-Z0-9\-]+$").unwrap();
    if !valid_pattern.is_match(name) {
        bail!(
            "✗ Error: Invalid profile name '{name}'\n  → Use alphanumeric characters and hyphens only"
        );
    }

    // Check for path traversal attempts
    if name.contains("..") || name.contains('/') || name.contains('\\') {
        bail!(
            "✗ Error: Invalid profile name '{name}'\n  → Profile names cannot contain path separators"
        );
    }

    Ok(())
}

/// Get the profile directory path
pub(crate) fn get_profile_dir(name: &str) -> Result<PathBuf> {
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
    profile_dir: &Path,
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

    // Copy .zshrc with prepended histfile configuration
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
pub(crate) fn update_global_config(profile_name: &str) -> Result<()> {
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

/// Display success message with profile details and optionally switch to the profile
pub(crate) fn display_success(
    name: &str,
    _framework_info: &crate::frameworks::FrameworkInfo,
    profile_dir: &Path,
    interactive: bool,
) -> Result<()> {
    println!("\n✓ Profile '{name}' created successfully");
    println!("  Location: {}", profile_dir.display());

    // Display profile details using shared function
    crate::cli::show::display_profile_details(name)?;

    if !interactive {
        return Ok(());
    }

    // Ask if user wants to switch to the new profile
    let should_switch = Confirm::new()
        .with_prompt(format!("Switch to '{name}' now?"))
        .default(true)
        .interact()
        .context("Failed to read user input for profile switch confirmation")?;

    if should_switch {
        // Use the use_cmd module to switch profiles
        crate::cli::use_cmd::execute(crate::cli::use_cmd::UseArgs {
            profile_name: name.to_string(),
        })?;
    } else {
        println!("  → Use 'zprof use {name}' to switch to this profile later");
    }

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
