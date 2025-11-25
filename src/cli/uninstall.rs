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

use crate::backup::{pre_zprof, restore, snapshot, SafetySummary};
use crate::cleanup::{self, CleanupConfig, CleanupSummary};
use crate::core::{config, profile};
use crate::tui::{uninstall_confirm, uninstall_select};

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

    // Step 1: Validate preconditions
    let validation_report = restore::validate_preconditions()
        .context("Failed to validate preconditions")?;

    // Check for critical issues
    if !validation_report.is_valid() {
        let issues = validation_report.get_issues();
        bail!(
            "Cannot proceed with uninstall due to validation failures:\n  - {}\n\n\
             Please resolve these issues before attempting to uninstall.",
            issues.join("\n  - ")
        );
    }

    // Display warnings if any
    if !validation_report.warnings.is_empty() {
        println!("\n⚠️  Warnings:");
        for warning in &validation_report.warnings {
            println!("  • {}", warning);
        }
        println!();
    }

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

    // Step 4: Create safety backup (unless --no-backup)
    let safety_backup_summary = if !args.no_backup {
        create_safety_backup(&home_dir)?
    } else {
        println!("\n⚠️  Skipping safety backup (--no-backup flag set)");
        // For non-backup mode, create a dummy summary for display purposes
        Some(SafetySummary {
            backup_path: home_dir.join(".zsh-profiles/backups/skipped.tar.gz"),
            backup_size: 0,
        })
    };

    // Unwrap is safe here because we always create Some variant above
    let safety_summary = safety_backup_summary.as_ref().unwrap();

    // Step 5: Build uninstall summary and show confirmation (unless --yes)
    if !args.yes {
        let uninstall_summary = build_uninstall_summary(
            &restore_option,
            &profiles_dir,
            &backup_dir,
            safety_summary,
        )?;

        if !uninstall_confirm::show_confirmation(&uninstall_summary)? {
            println!("\nUninstall cancelled. No changes were made.");
            return Ok(());
        }
    }

    // Step 6: Execute restoration
    info!("Executing restoration: {:?}", restore_option);
    match restore_option {
        RestoreOption::Original => {
            // Use new validation and rollback-enabled restoration
            let interactive = !args.yes;
            restore::restore_pre_zprof_with_validation(&home_dir, &backup_dir, interactive)?;
        }
        RestoreOption::Promote(ref profile_name) => {
            promote_profile(&home_dir, &profiles_dir, profile_name)?;
        }
        RestoreOption::Clean => {
            // No restoration needed
            info!("Clean removal - no restoration");
        }
    }

    // Step 7: Execute cleanup
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

    // Step 8: Display success message
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

/// Build comprehensive uninstall summary for confirmation
fn build_uninstall_summary(
    restore_option: &RestoreOption,
    profiles_dir: &Path,
    backup_dir: &Path,
    safety_summary: &SafetySummary,
) -> Result<uninstall_confirm::UninstallSummary> {
    // Build restoration summary
    let restoration = match restore_option {
        RestoreOption::Original => {
            // Load backup manifest to get details
            let manifest = pre_zprof::validate_backup(backup_dir)?;
            let file_count = manifest.files.len();
            let source_date = Some(manifest.metadata.created_at);

            // Count history entries if history file exists
            let history_entries = manifest.files.iter()
                .find(|f| f.path.to_str() == Some(".zsh_history"))
                .and_then(|_| {
                    let history_path = backup_dir.join(".zsh_history");
                    count_history_lines(&history_path).ok()
                });

            uninstall_confirm::RestorationSummary {
                option: uninstall_confirm::RestoreOption::PreZprof,
                file_count,
                history_entries,
                source_date,
            }
        }
        RestoreOption::Promote(profile_name) => {
            // Count config files in the profile
            let profile_dir = profiles_dir.join(profile_name);
            let config_files = [".zshrc", ".zshenv", ".zprofile", ".zlogin", ".zlogout"];
            let file_count = config_files.iter()
                .filter(|f| profile_dir.join(f).exists())
                .count();

            // Count history entries if profile has history
            let history_path = profile_dir.join(".zsh_history");
            let history_entries = count_history_lines(&history_path).ok();

            uninstall_confirm::RestorationSummary {
                option: uninstall_confirm::RestoreOption::PromoteProfile(profile_name.clone()),
                file_count,
                history_entries,
                source_date: None,
            }
        }
        RestoreOption::Clean => {
            uninstall_confirm::RestorationSummary {
                option: uninstall_confirm::RestoreOption::NoRestore,
                file_count: 0,
                history_entries: None,
                source_date: None,
            }
        }
    };

    // Build cleanup summary
    let cleanup = build_cleanup_summary(profiles_dir)?;

    // Create complete summary
    Ok(uninstall_confirm::UninstallSummary {
        restoration,
        cleanup,
        safety: SafetySummary {
            backup_path: safety_summary.backup_path.clone(),
            backup_size: safety_summary.backup_size,
        },
    })
}

/// Build cleanup summary by scanning profiles directory
fn build_cleanup_summary(profiles_dir: &Path) -> Result<CleanupSummary> {
    let mut profile_count = 0;
    let mut total_size = 0u64;
    let mut directories = Vec::new();

    if profiles_dir.exists() {
        // Calculate total size recursively
        total_size = calculate_dir_size(profiles_dir)?;

        // Count profiles
        let profiles_subdir = profiles_dir.join("profiles");
        if profiles_subdir.exists() {
            if let Ok(entries) = std::fs::read_dir(&profiles_subdir) {
                profile_count = entries
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                    .count();
            }
        }

        // Track main directory
        directories.push(profiles_dir.to_path_buf());
    }

    Ok(CleanupSummary {
        profile_count,
        total_size,
        directories,
    })
}

/// Calculate total size of a directory recursively
fn calculate_dir_size(dir: &Path) -> Result<u64> {
    let mut total = 0u64;

    if dir.is_file() {
        return Ok(std::fs::metadata(dir)?.len());
    }

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            total += calculate_dir_size(&path)?;
        } else {
            total += std::fs::metadata(&path)?.len();
        }
    }

    Ok(total)
}

/// Count number of lines in history file
fn count_history_lines(path: &Path) -> Result<usize> {
    if !path.exists() {
        return Ok(0);
    }

    let content = std::fs::read_to_string(path)?;
    Ok(content.lines().count())
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
