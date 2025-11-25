//! Uninstall command implementation
//!
//! Handles complete removal of zprof with flexible restoration options:
//! - Restore original pre-zprof backup
//! - Promote a profile to root configuration
//! - Clean removal without restoration

use anyhow::{bail, Context, Result};
use chrono::Local;
use clap::{Args, ValueEnum};
use log::info;
use std::path::Path;

use crate::backup::{pre_zprof, snapshot, SafetySummary};
use crate::cleanup::{self, CleanupConfig};
use crate::core::{config, profile};
use crate::tui::uninstall_select;

/// Uninstall zprof and optionally restore shell configuration
#[derive(Debug, Args)]
pub struct UninstallArgs {
    /// Skip confirmation prompts (non-interactive)
    #[arg(long, short = 'y')]
    pub yes: bool,

    /// Specify restoration option directly (skip TUI menu)
    #[arg(long, value_enum)]
    pub restore: Option<RestoreOptionCli>,

    /// Skip creating safety backup before uninstall
    #[arg(long)]
    pub no_backup: bool,

    /// Keep backups directory when removing profiles
    #[arg(long)]
    pub keep_backups: bool,
}

/// CLI representation of restoration options
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum RestoreOptionCli {
    /// Restore original pre-zprof backup
    Original,
    /// Promote a profile to root configuration
    Promote,
    /// Clean removal without restoration
    Clean,
}

/// Internal restoration option with profile name for promote
#[derive(Debug, Clone)]
pub enum RestoreOption {
    Original,
    Promote(String),
    Clean,
}

/// Execute the uninstall command
pub fn execute(args: UninstallArgs) -> Result<()> {
    info!("Starting uninstall process");

    // Step 1: Validate zprof is installed
    validate_zprof_installed()?;

    // Step 2: Load configuration
    let home_dir = dirs::home_dir()
        .context("Failed to determine home directory")?;
    let profiles_dir = profile::get_profiles_dir()?;
    let backup_dir = home_dir.join(".zsh-profiles/backups/pre-zprof");

    let config = config::load_config()?;

    // Step 3: Get restoration option
    let restore_option = if let Some(cli_option) = args.restore {
        // User specified option via CLI flag
        match cli_option {
            RestoreOptionCli::Original => {
                // Validate backup exists
                if !pre_zprof::backup_exists(&backup_dir) {
                    bail!(
                        "Cannot restore original: Pre-zprof backup not found at {}\n\n\
                         The backup may not have been created during initialization.\n\
                         Please choose a different restoration option.",
                        backup_dir.display()
                    );
                }
                RestoreOption::Original
            }
            RestoreOptionCli::Promote => {
                // Need to get profile name - show TUI selector even with --restore=promote
                let profiles = profile::scan_profiles(&profiles_dir, config.active_profile.as_deref())?;
                if profiles.is_empty() {
                    bail!("Cannot promote profile: No profiles found");
                }

                let profile_name = uninstall_select::select_profile_to_promote(&profiles)?;
                RestoreOption::Promote(profile_name)
            }
            RestoreOptionCli::Clean => RestoreOption::Clean,
        }
    } else {
        // Interactive mode - show TUI menu
        let backup_available = pre_zprof::backup_exists(&backup_dir);
        let profiles = profile::scan_profiles(&profiles_dir, config.active_profile.as_deref())?;

        uninstall_select::select_restoration_option(backup_available, !profiles.is_empty())?
    };

    // Step 4: Show confirmation (unless --yes)
    if !args.yes
        && !confirm_uninstall(&restore_option, &profiles_dir)? {
            println!("\nUninstall cancelled. No changes were made.");
            return Ok(());
        }

    // Step 4.5: Create safety backup (unless --no-backup)
    let safety_backup_summary = if !args.no_backup {
        create_safety_backup(&home_dir)?
    } else {
        println!("\n⚠️  Skipping safety backup (--no-backup flag set)");
        None
    };

    // Step 5: Execute restoration
    info!("Executing restoration: {:?}", restore_option);
    match restore_option {
        RestoreOption::Original => {
            restore_original(&home_dir, &backup_dir)?;
        }
        RestoreOption::Promote(ref profile_name) => {
            promote_profile(&home_dir, &profiles_dir, profile_name)?;
        }
        RestoreOption::Clean => {
            // No restoration needed
            info!("Clean removal - no restoration");
        }
    }

    // Step 6: Execute cleanup
    let cleanup_config = CleanupConfig {
        profiles_dir: profiles_dir.clone(),
        home_dir: home_dir.clone(),
        keep_backups: args.keep_backups,
    };

    println!();
    println!("🗑️  Removing zprof files...");
    let cleanup_report = cleanup::cleanup_all(&cleanup_config)?;

    if !cleanup_report.is_successful() {
        eprintln!("\n⚠ Some files could not be removed. You may need to remove them manually.");
    }

    // Step 7: Display success message
    display_success_message(&restore_option, safety_backup_summary.as_ref())?;

    Ok(())
}

/// Create a safety backup of the entire .zsh-profiles directory
fn create_safety_backup(home_dir: &Path) -> Result<Option<SafetySummary>> {
    println!();
    println!("📦 Creating safety backup...");

    let profiles_dir = home_dir.join(".zsh-profiles");

    if !profiles_dir.exists() {
        // Nothing to backup
        return Ok(None);
    }

    // Create backup directory if it doesn't exist
    let backups_dir = profiles_dir.join("backups");
    std::fs::create_dir_all(&backups_dir)
        .context("Failed to create backups directory")?;

    // Generate timestamped filename
    let timestamp = Local::now().format("%Y%m%d-%H%M%S");
    let backup_filename = format!("final-snapshot-{}.tar.gz", timestamp);
    let backup_path = backups_dir.join(&backup_filename);

    // Create the snapshot
    let size = snapshot::create_final_snapshot(&profiles_dir, &backup_path)
        .context("Failed to create safety backup")?;

    // Display result
    let size_mb = size as f64 / 1_048_576.0;
    println!("✓ Safety backup created: {} ({:.2} MB)", backup_filename, size_mb);
    println!("  Location: {}", backup_path.display());

    Ok(Some(SafetySummary {
        backup_path,
        backup_size: size,
    }))
}

/// Validate that zprof is installed
fn validate_zprof_installed() -> Result<()> {
    let home = dirs::home_dir()
        .context("Failed to determine home directory")?;
    let zprof_dir = home.join(".zsh-profiles");

    if !zprof_dir.exists() {
        bail!(
            "zprof is not installed\n\n\
             The ~/.zsh-profiles directory does not exist.\n\
             There is nothing to uninstall."
        );
    }

    let config_path = zprof_dir.join("config.toml");
    if !config_path.exists() {
        bail!(
            "zprof installation appears incomplete\n\n\
             The configuration file (config.toml) is missing.\n\
             You may need to remove ~/.zsh-profiles manually."
        );
    }

    Ok(())
}

/// Restore original pre-zprof backup
fn restore_original(home_dir: &Path, backup_dir: &Path) -> Result<()> {
    println!("\n🔄 Restoring original shell configuration...");

    // Validate backup
    let manifest = pre_zprof::validate_backup(backup_dir)
        .context("Failed to validate pre-zprof backup")?;

    println!("  Backup created: {}", manifest.metadata.created_at.format("%Y-%m-%d %H:%M"));
    println!("  Files to restore: {}", manifest.files.len());

    // Restore each file from backup
    for backed_up_file in &manifest.files {
        let source = backup_dir.join(&backed_up_file.path);
        let dest = home_dir.join(&backed_up_file.path);

        if !source.exists() {
            eprintln!("  ⚠ Warning: Backup file missing: {}", backed_up_file.path.display());
            continue;
        }

        // Copy file from backup to HOME
        std::fs::copy(&source, &dest)
            .with_context(|| format!("Failed to restore {}", backed_up_file.path.display()))?;

        // Restore permissions on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = std::fs::Permissions::from_mode(backed_up_file.permissions);
            std::fs::set_permissions(&dest, permissions)
                .with_context(|| format!("Failed to set permissions on {}", backed_up_file.path.display()))?;
        }

        println!("  ✓ Restored {}", backed_up_file.path.display());
    }

    println!("✓ Original configuration restored");
    Ok(())
}

/// Promote a profile to root configuration
fn promote_profile(home_dir: &Path, profiles_dir: &Path, profile_name: &str) -> Result<()> {
    println!("\n🔄 Promoting profile '{}' to root configuration...", profile_name);

    let profile_dir = profiles_dir.join(profile_name);

    if !profile_dir.exists() {
        bail!("Profile '{}' not found at {}", profile_name, profile_dir.display());
    }

    // Copy profile's config files to HOME
    let shell_files = [".zshrc", ".zshenv", ".zprofile", ".zlogin", ".zlogout"];

    let mut copied_count = 0;
    for file_name in &shell_files {
        let source = profile_dir.join(file_name);
        if source.exists() {
            let dest = home_dir.join(file_name);
            std::fs::copy(&source, &dest)
                .with_context(|| format!("Failed to copy {} from profile", file_name))?;
            println!("  ✓ Copied {}", file_name);
            copied_count += 1;
        }
    }

    // Copy history file if it exists
    let history_source = profile_dir.join(".zsh_history");
    if history_source.exists() {
        let history_dest = home_dir.join(".zsh_history");
        std::fs::copy(&history_source, &history_dest)
            .context("Failed to copy .zsh_history from profile")?;
        println!("  ✓ Copied .zsh_history");
        copied_count += 1;
    }

    if copied_count == 0 {
        println!("  ⚠ Warning: No configuration files found in profile");
    }

    println!("✓ Profile '{}' promoted to root configuration", profile_name);
    Ok(())
}


/// Display success message
fn display_success_message(restore_option: &RestoreOption, safety_backup: Option<&SafetySummary>) -> Result<()> {
    println!("\n✅ zprof uninstalled successfully");
    println!();

    match restore_option {
        RestoreOption::Original => {
            println!("  Your shell configuration has been restored to its pre-zprof state.");
        }
        RestoreOption::Promote(profile_name) => {
            println!("  Profile '{}' has been promoted to your root shell configuration.", profile_name);
        }
        RestoreOption::Clean => {
            println!("  All zprof files have been removed.");
            println!("  You can now configure your shell manually or install a different tool.");
        }
    }

    // Show safety backup information if created
    if let Some(summary) = safety_backup {
        let size_mb = summary.backup_size as f64 / 1_048_576.0;
        println!();
        println!("  Safety backup available ({:.2} MB):", size_mb);
        println!("  {}", summary.backup_path.display());
        println!("  You can extract this backup if you need to recover any data.");
    }

    println!();
    println!("  Restart your shell to complete the uninstall:");
    println!("  exec zsh");
    println!();

    Ok(())
}

/// Confirm uninstall operation
fn confirm_uninstall(restore_option: &RestoreOption, profiles_dir: &Path) -> Result<bool> {
    println!();
    println!("╭─────────────────────────────────────────────────────────────╮");
    println!("│                    Uninstall Summary                        │");
    println!("╰─────────────────────────────────────────────────────────────╯");
    println!();

    // Show restoration details
    match restore_option {
        RestoreOption::Original => {
            println!("  Restoration:");
            println!("    • Restore pre-zprof backup to HOME");
            println!("    • Shell config will be returned to original state");
        }
        RestoreOption::Promote(profile_name) => {
            println!("  Restoration:");
            println!("    • Promote profile '{}' to HOME", profile_name);
            println!("    • Profile configs will become your root shell config");
        }
        RestoreOption::Clean => {
            println!("  Restoration:");
            println!("    • No restoration (clean removal)");
            println!("    • HOME directory will be left without shell configs");
        }
    }

    println!();

    // Show cleanup details
    println!("  Cleanup:");

    // Count profiles
    if profiles_dir.exists() {
        if let Ok(entries) = std::fs::read_dir(profiles_dir) {
            let profile_count = entries.filter(|e| {
                e.as_ref().map(|e| e.path().is_dir()).unwrap_or(false)
            }).count();
            if profile_count > 0 {
                println!("    • Remove {} profile(s)", profile_count);
            }
        }
    }

    println!("    • Remove ~/.zsh-profiles/ directory");
    println!("    • Remove zprof shell integration");

    println!();
    println!("╭─────────────────────────────────────────────────────────────╮");
    println!("│  ⚠️  This action cannot be undone!                          │");
    println!("╰─────────────────────────────────────────────────────────────╯");
    println!();

    print!("Continue with uninstall? [y/N]: ");
    std::io::Write::flush(&mut std::io::stdout())?;

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    let response = input.trim().to_lowercase();
    Ok(response == "y" || response == "yes")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_restore_option_cli_values() {
        // Test that CLI enum variants are defined correctly
        use clap::ValueEnum;

        let values = RestoreOptionCli::value_variants();
        assert_eq!(values.len(), 3);
    }

    #[test]
    fn test_restore_option_debug() {
        let opt = RestoreOption::Original;
        let debug_str = format!("{:?}", opt);
        assert!(debug_str.contains("Original"));
    }
}
