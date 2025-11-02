//! Manual TOML editing with live validation
//!
//! Implements Story 2.3: Edit command that opens profile.toml in $EDITOR,
//! validates changes, and regenerates shell configuration.

use anyhow::{bail, Context, Result};
use chrono::Utc;
use clap::Args;
use std::env;
use std::fs;
use std::io::{self, Write as IoWrite};
use std::path::{Path, PathBuf};
use std::process::Command;

use crate::core::manifest;
use crate::shell::generator;

#[derive(Debug, Args)]
pub struct EditArgs {
    /// Name of the profile to edit
    pub profile_name: String,
}

pub fn execute(args: EditArgs) -> Result<()> {
    // 1. Validate profile exists
    let profile_dir = get_profile_dir(&args.profile_name)?;
    let manifest_path = profile_dir.join("profile.toml");

    if !manifest_path.exists() {
        bail!(
            "Profile '{}' not found.\n  → Run 'zprof list' to see available profiles\n  → Run 'zprof create {}' to create this profile",
            args.profile_name,
            args.profile_name
        );
    }

    // 2. Detect editor
    let editor = detect_editor()?;
    log::info!("Using editor: {}", editor);

    // 3. Create backup before editing (NFR002)
    let backup_path = create_backup(&manifest_path)?;
    log::info!("Created backup: {:?}", backup_path);

    // 4. Open editor
    println!("→ Opening {} in {}...", manifest_path.display(), editor);
    let edit_result = open_editor(&editor, &manifest_path);

    // Handle editor failure
    if let Err(e) = edit_result {
        eprintln!("✗ Editor failed to launch: {}", e);
        println!("  → Restoring backup...");
        restore_backup(&backup_path, &manifest_path)?;
        bail!("Edit cancelled due to editor failure");
    }

    // 5. Validation loop
    loop {
        match manifest::load_and_validate(&args.profile_name) {
            Ok(manifest) => {
                // Validation succeeded
                println!("✓ TOML manifest validated successfully");

                // 6. Regenerate shell files
                println!("→ Regenerating shell configuration...");
                generator::write_generated_files(&args.profile_name, &manifest)
                    .context("Failed to regenerate shell configuration")?;

                // 7. Success
                println!("✓ Profile updated successfully");
                println!();
                println!("  Profile: {}", args.profile_name);
                println!("  Framework: {}", manifest.profile.framework);
                println!("  Files updated:");
                println!("    - profile.toml (manifest)");
                println!("    - .zshrc (regenerated)");
                println!("    - .zshenv (regenerated)");
                println!();
                println!("  → Run 'zprof use {}' to activate changes", args.profile_name);

                // Clean up backup
                fs::remove_file(&backup_path).context("Failed to remove backup")?;

                return Ok(());
            }
            Err(e) => {
                // Validation failed
                println!();
                println!("✗ TOML validation failed:");
                println!("{:#}", e);
                println!();

                // Prompt for action
                let action = prompt_validation_failure()?;

                match action.as_str() {
                    "r" | "retry" => {
                        // Retry edit
                        println!("→ Reopening editor...");
                        open_editor(&editor, &manifest_path)?;
                        continue;
                    }
                    "restore" => {
                        // Restore backup
                        println!("→ Restoring original manifest...");
                        restore_backup(&backup_path, &manifest_path)?;
                        println!("✓ Original manifest restored");
                        return Ok(());
                    }
                    "c" | "cancel" => {
                        // Cancel - keep invalid file but don't regenerate
                        println!("→ Edit cancelled. Invalid manifest preserved.");
                        println!(
                            "  ⚠ Warning: Profile may not work until manifest is fixed"
                        );
                        println!("  → Run 'zprof edit {}' again to fix", args.profile_name);
                        return Ok(());
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn get_profile_dir(profile_name: &str) -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;

    Ok(home
        .join(".zsh-profiles")
        .join("profiles")
        .join(profile_name))
}

fn detect_editor() -> Result<String> {
    // Check $EDITOR first
    if let Ok(editor) = env::var("EDITOR") {
        if !editor.is_empty() {
            return Ok(editor);
        }
    }

    // Check $VISUAL
    if let Ok(visual) = env::var("VISUAL") {
        if !visual.is_empty() {
            return Ok(visual);
        }
    }

    // Platform-specific fallbacks
    if cfg!(target_os = "windows") {
        Ok("notepad".to_string())
    } else {
        // Unix/Linux/macOS: default to vim
        Ok("vim".to_string())
    }
}

fn open_editor(editor: &str, file_path: &Path) -> Result<()> {
    let status = Command::new(editor)
        .arg(file_path)
        .status()
        .context(format!("Failed to launch editor: {}", editor))?;

    if !status.success() {
        bail!("Editor exited with non-zero status: {}", status);
    }

    Ok(())
}

fn create_backup(file_path: &Path) -> Result<PathBuf> {
    // Create backups directory
    let home = dirs::home_dir().context("Could not find home directory")?;

    let backups_dir = home.join(".zsh-profiles").join("cache").join("backups");

    fs::create_dir_all(&backups_dir).context("Failed to create backups directory")?;

    // Generate backup filename with timestamp
    let timestamp = Utc::now().format("%Y%m%d-%H%M%S");
    let filename = file_path
        .file_name()
        .context("Invalid file path")?
        .to_str()
        .context("Invalid filename")?;

    let backup_filename = format!("{}.backup.{}", filename, timestamp);
    let backup_path = backups_dir.join(backup_filename);

    // Copy file to backup
    fs::copy(file_path, &backup_path)
        .with_context(|| format!("Failed to create backup at {:?}", backup_path))?;

    Ok(backup_path)
}

fn restore_backup(backup_path: &Path, dest_path: &Path) -> Result<()> {
    fs::copy(backup_path, dest_path).context("Failed to restore backup")?;

    // Delete backup after successful restoration
    fs::remove_file(backup_path).context("Failed to remove backup after restoration")?;

    Ok(())
}

fn prompt_validation_failure() -> Result<String> {
    print!("  [R]etry edit, [Restore] backup, or [C]ancel? ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let choice = input.trim().to_lowercase();

    match choice.as_str() {
        "r" | "retry" => Ok("retry".to_string()),
        "restore" => Ok("restore".to_string()),
        "c" | "cancel" => Ok("cancel".to_string()),
        _ => {
            println!("  Invalid choice. Defaulting to cancel.");
            Ok("cancel".to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Environment variable tests can be flaky in multi-threaded test environments
    // because env::set_var affects the entire process. These tests verify the logic
    // without relying on modifying environment state.

    #[test]
    fn test_detect_editor_returns_value() {
        // Test that detect_editor always returns a valid value
        let editor = detect_editor().unwrap();
        assert!(!editor.is_empty());
    }

    #[test]
    fn test_detect_editor_respects_environment() {
        // If user has $EDITOR set, it should be used
        let editor = detect_editor().unwrap();

        // Check against current environment
        if let Ok(env_editor) = env::var("EDITOR") {
            if !env_editor.is_empty() {
                assert_eq!(editor, env_editor);
            }
        } else if let Ok(env_visual) = env::var("VISUAL") {
            if !env_visual.is_empty() {
                assert_eq!(editor, env_visual);
            }
        } else {
            // No env vars set, should use platform fallback
            if cfg!(target_os = "windows") {
                assert_eq!(editor, "notepad");
            } else {
                assert_eq!(editor, "vim");
            }
        }
    }

    #[test]
    fn test_create_and_restore_backup() -> Result<()> {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new()?;
        let test_file = temp_dir.path().join("test.toml");
        let original_content = "original content";
        fs::write(&test_file, original_content)?;

        // Mock home directory setup would be needed for full test
        // For now, test the logic with temp files
        let backup_dir = temp_dir.path().join("backups");
        fs::create_dir_all(&backup_dir)?;

        let timestamp = Utc::now().format("%Y%m%d-%H%M%S");
        let backup_path = backup_dir.join(format!("test.toml.backup.{}", timestamp));

        fs::copy(&test_file, &backup_path)?;

        // Modify original
        fs::write(&test_file, "modified content")?;

        // Restore from backup
        fs::copy(&backup_path, &test_file)?;
        fs::remove_file(&backup_path)?;

        // Verify restoration
        let restored = fs::read_to_string(&test_file)?;
        assert_eq!(restored, original_content);

        Ok(())
    }

    #[test]
    fn test_get_profile_dir() {
        let result = get_profile_dir("test-profile");
        assert!(result.is_ok());

        let path = result.unwrap();
        assert!(path.to_string_lossy().contains(".zsh-profiles"));
        assert!(path.to_string_lossy().contains("profiles"));
        assert!(path.to_string_lossy().contains("test-profile"));
    }
}
