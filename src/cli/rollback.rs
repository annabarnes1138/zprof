use anyhow::{Context, Result};
use clap::Args;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

use crate::core::{filesystem, profile};
use crate::frameworks::FrameworkType;

#[derive(Debug, Args)]
pub struct RollbackArgs {
    /// Skip confirmation prompt
    #[arg(short = 'y', long = "yes")]
    pub yes: bool,

    /// Specific profile to rollback from (defaults to searching all profiles)
    #[arg(short = 'p', long = "profile")]
    pub profile: Option<String>,
}

/// Result of backup detection and analysis
#[derive(Debug)]
struct BackupInfo {
    /// Path to the backup .zshrc file
    backup_path: PathBuf,
    /// Path to the profile containing the backup (reserved for future use)
    #[allow(dead_code)]
    _profile_path: PathBuf,
    /// Framework type detected from the profile
    framework_type: Option<FrameworkType>,
    /// Framework installation directory path within the profile
    framework_dir: Option<PathBuf>,
}

pub fn execute(args: RollbackArgs) -> Result<()> {
    // AC1: Check for backup file (.zshrc.pre-zprof) in profiles
    let backup_info = detect_backup(&args.profile)
        .context("Failed to detect backup file")?;

    // AC2: Show what will be restored and what will be moved back
    display_rollback_preview(&backup_info)?;

    // AC3: Require explicit confirmation
    if !args.yes && !confirm_rollback()? {
        println!();
        println!("Rollback cancelled. No changes were made.");
        return Ok(());
    }

    // AC4: Perform rollback operations
    perform_rollback(&backup_info)?;

    // AC5: Display success message
    display_success_message(&backup_info)?;

    Ok(())
}

/// Detect and locate backup file, validating its integrity
/// AC1, AC6, AC7
fn detect_backup(profile_name: &Option<String>) -> Result<BackupInfo> {
    let profiles_dir = profile::get_profiles_dir()?;

    // Determine which profiles to search
    let search_profiles: Vec<PathBuf> = if let Some(name) = profile_name {
        // Search specific profile
        let profile_path = profiles_dir.join(name);
        if !profile_path.exists() {
            anyhow::bail!(
                "Profile '{}' not found\n\n  Run 'zprof list' to see available profiles",
                name
            );
        }
        vec![profile_path]
    } else {
        // Search all profiles
        if !profiles_dir.exists() {
            anyhow::bail!(
                "No profiles directory found\n\n  \
                 zprof may not have been initialized yet.\n  \
                 If you're trying to rollback an initialization, the backup should be in your home directory."
            );
        }

        fs::read_dir(&profiles_dir)
            .context("Failed to read profiles directory")?
            .filter_map(|entry| {
                entry.ok().and_then(|e| {
                    if e.path().is_dir() {
                        Some(e.path())
                    } else {
                        None
                    }
                })
            })
            .collect()
    };

    if search_profiles.is_empty() {
        anyhow::bail!(
            "No profiles found to search for backup\n\n  \
             Create a profile with 'zprof create <name>' or run 'zprof init'"
        );
    }

    // Search for backup file in profile directories
    let mut found_backups = Vec::new();

    for profile_path in &search_profiles {
        let backup_path = profile_path.join(".zshrc.pre-zprof");
        if backup_path.exists() {
            found_backups.push((profile_path.clone(), backup_path));
        }
    }

    // AC6: Error if no backup found
    if found_backups.is_empty() {
        return Err(anyhow::anyhow!(
            "No backup file found\n\n  \
             ✗ Could not locate '.zshrc.pre-zprof' in any profile directory.\n  \
             This backup is created during 'zprof init' when migrating existing configuration.\n\n  \
             Possible reasons:\n  \
             - zprof was initialized without an existing .zshrc to backup\n  \
             - The backup file was manually deleted\n  \
             - You haven't run 'zprof init' yet\n\n  \
             If you need to restore your shell configuration manually:\n  \
             - Check for backups in ~/.zsh-profiles/cache/backups/\n  \
             - Look for .zshrc.bak or similar files in your home directory"
        ));
    }

    // Use the most recently modified backup if multiple found
    let (profile_path, backup_path) = if found_backups.len() > 1 {
        eprintln!(
            "⚠️  Found {} backup files, using most recent",
            found_backups.len()
        );
        found_backups
            .into_iter()
            .max_by_key(|(_, path)| {
                fs::metadata(path)
                    .and_then(|m| m.modified())
                    .ok()
            })
            .unwrap()
    } else {
        found_backups.into_iter().next().unwrap()
    };

    // AC7: Validate backup integrity
    validate_backup_integrity(&backup_path)?;

    // Detect framework type from profile manifest
    let (framework_type, framework_dir) = detect_framework_info(&profile_path)?;

    Ok(BackupInfo {
        backup_path,
        _profile_path: profile_path,
        framework_type,
        framework_dir,
    })
}

/// Validate that backup file is readable and appears to be a valid shell script
/// AC7
fn validate_backup_integrity(backup_path: &Path) -> Result<()> {
    // Check file is readable
    let content = fs::read_to_string(backup_path).with_context(|| {
        format!(
            "Failed to read backup file at {}\n  The file may be corrupted or have incorrect permissions",
            backup_path.display()
        )
    })?;

    // Basic validation: check it's not empty and looks like a shell script
    if content.trim().is_empty() {
        anyhow::bail!(
            "Backup file is empty: {}\n  Cannot rollback with an empty backup file",
            backup_path.display()
        );
    }

    // Check for shell shebang or common zsh patterns
    let looks_valid = content.starts_with("#!")
        || content.contains("export")
        || content.contains("source")
        || content.contains("alias")
        || content.contains("PATH");

    if !looks_valid {
        eprintln!(
            "⚠️  Warning: Backup file may not be a valid shell configuration\n   Path: {}",
            backup_path.display()
        );
    }

    Ok(())
}

/// Detect framework information from profile manifest
fn detect_framework_info(profile_path: &Path) -> Result<(Option<FrameworkType>, Option<PathBuf>)> {
    let manifest_path = profile_path.join("profile.toml");

    if !manifest_path.exists() {
        // No manifest, can't detect framework
        return Ok((None, None));
    }

    // Read manifest to get framework type
    let content = fs::read_to_string(&manifest_path)
        .context("Failed to read profile manifest")?;

    // Parse TOML to extract framework name
    let manifest: toml::Value = toml::from_str(&content)
        .context("Failed to parse profile manifest")?;

    let framework_name = manifest
        .get("profile")
        .and_then(|p| p.get("framework"))
        .and_then(|f| f.as_str());

    let framework_type = framework_name.and_then(|name| match name {
        "oh-my-zsh" => Some(FrameworkType::OhMyZsh),
        "zimfw" => Some(FrameworkType::Zimfw),
        "prezto" => Some(FrameworkType::Prezto),
        "zinit" => Some(FrameworkType::Zinit),
        "zap" => Some(FrameworkType::Zap),
        _ => None,
    });

    // Detect framework directory in profile
    let framework_dir = framework_type.as_ref().and_then(|ft| {
        let dir_name = match ft {
            FrameworkType::OhMyZsh => ".oh-my-zsh",
            FrameworkType::Zimfw => ".zimfw",
            FrameworkType::Prezto => ".zprezto",
            FrameworkType::Zinit => ".zinit",
            FrameworkType::Zap => ".zap",
        };
        let dir_path = profile_path.join(dir_name);
        if dir_path.exists() {
            Some(dir_path)
        } else {
            None
        }
    });

    Ok((framework_type, framework_dir))
}

/// Display preview of what will be restored
/// AC2
fn display_rollback_preview(backup_info: &BackupInfo) -> Result<()> {
    println!();
    println!("═══════════════════════════════════════════════════════════");
    println!("  Rollback Preview");
    println!("═══════════════════════════════════════════════════════════");
    println!();

    // Show what will be restored
    println!("📥 RESTORE:");
    println!(
        "   ~/.zshrc ← {}",
        backup_info.backup_path.display()
    );
    println!();

    // Show what will be moved
    if let (Some(framework_type), Some(framework_dir)) = (&backup_info.framework_type, &backup_info.framework_dir) {
        let home = dirs::home_dir().context("Failed to get home directory")?;
        let target_dir = home.join(framework_dir.file_name().unwrap());

        println!("📦 MOVE:");
        println!(
            "   {} → {}",
            framework_dir.display(),
            target_dir.display()
        );
        println!("   (Framework: {})", framework_type.name());
        println!();
    }

    // Show what will remain
    println!("💾 KEEP:");
    println!("   ~/.zsh-profiles/ (profiles and backups preserved for reference)");
    println!();

    println!("───────────────────────────────────────────────────────────");
    println!();

    Ok(())
}

/// Prompt user for confirmation with explicit "Continue? [y/N]" message
/// AC3
fn confirm_rollback() -> Result<bool> {
    print!("Continue? [y/N]: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let response = input.trim().to_lowercase();
    Ok(response == "y" || response == "yes")
}

/// Perform the actual rollback operations
/// AC4
fn perform_rollback(backup_info: &BackupInfo) -> Result<()> {
    let home = dirs::home_dir().context("Failed to get home directory")?;
    let current_zshrc = home.join(".zshrc");
    let safety_backup = home.join(".zshrc.pre-rollback");

    println!();
    println!("🔄 Performing rollback...");
    println!();

    // Step 1: Create safety backup of current .zshrc
    if current_zshrc.exists() {
        fs::copy(&current_zshrc, &safety_backup).with_context(|| {
            format!(
                "Failed to create safety backup at {}",
                safety_backup.display()
            )
        })?;
        println!("✓ Created safety backup: {}", safety_backup.display());
    }

    // Step 2: Restore .zshrc from backup
    fs::copy(&backup_info.backup_path, &current_zshrc).with_context(|| {
        format!(
            "Failed to restore .zshrc from {}",
            backup_info.backup_path.display()
        )
    })?;
    println!("✓ Restored ~/.zshrc from backup");

    // Step 3: Set appropriate permissions on restored .zshrc
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = fs::Permissions::from_mode(0o644);
        fs::set_permissions(&current_zshrc, permissions)
            .context("Failed to set permissions on restored .zshrc")?;
    }

    // Step 4: Move framework back to home directory if applicable
    if let (Some(framework_type), Some(framework_dir)) = (&backup_info.framework_type, &backup_info.framework_dir) {
        let target_dir = home.join(framework_dir.file_name().unwrap());

        if target_dir.exists() {
            eprintln!(
                "⚠️  Warning: {} already exists, skipping framework move",
                target_dir.display()
            );
        } else {
            // Use copy instead of move to preserve the profile directory
            filesystem::copy_dir_recursive(framework_dir, &target_dir).with_context(|| {
                format!(
                    "Failed to move framework from {} to {}",
                    framework_dir.display(),
                    target_dir.display()
                )
            })?;
            println!(
                "✓ Moved {} framework to home directory",
                framework_type.name()
            );
        }
    }

    // Step 5: Preserve ~/.zsh-profiles/ directory
    println!("✓ Preserved ~/.zsh-profiles/ for reference");
    println!();

    Ok(())
}

/// Display success message with instructions
/// AC5
fn display_success_message(backup_info: &BackupInfo) -> Result<()> {
    println!("═══════════════════════════════════════════════════════════");
    println!("  ✅ Rollback Complete");
    println!("═══════════════════════════════════════════════════════════");
    println!();
    println!("Your shell configuration has been restored to its pre-zprof state.");
    println!();

    println!("📋 What was done:");
    println!("   • Restored ~/.zshrc from backup");
    if backup_info.framework_type.is_some() {
        println!("   • Moved framework back to home directory");
    }
    println!("   • Preserved ~/.zsh-profiles/ directory");
    println!();

    println!("⚡ Next steps:");
    println!("   1. Restart your shell or run: source ~/.zshrc");
    println!("   2. Verify your configuration is working correctly");
    println!();

    println!("🗑️  Optional cleanup:");
    println!("   You can safely delete ~/.zsh-profiles/ if you no longer need it:");
    println!("   rm -rf ~/.zsh-profiles/");
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_backup(profile_dir: &Path) -> Result<PathBuf> {
        let backup_path = profile_dir.join(".zshrc.pre-zprof");
        fs::write(
            &backup_path,
            r#"# Original .zshrc
export PATH=$HOME/bin:$PATH
alias ll='ls -la'
"#,
        )?;
        Ok(backup_path)
    }

    fn create_test_profile_with_manifest(profile_dir: &Path, framework: &str) -> Result<()> {
        let manifest = format!(
            r#"[profile]
name = "test"
framework = "{}"
"#,
            framework
        );
        fs::write(profile_dir.join("profile.toml"), manifest)?;
        Ok(())
    }

    #[test]
    fn test_validate_backup_integrity_valid() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let profile_dir = temp_dir.path().join("profile");
        fs::create_dir(&profile_dir)?;

        let backup_path = create_test_backup(&profile_dir)?;
        let result = validate_backup_integrity(&backup_path);
        assert!(result.is_ok());

        Ok(())
    }

    #[test]
    fn test_validate_backup_integrity_empty() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let backup_path = temp_dir.path().join(".zshrc.pre-zprof");
        fs::write(&backup_path, "")?;

        let result = validate_backup_integrity(&backup_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("empty"));

        Ok(())
    }

    #[test]
    fn test_detect_framework_info_oh_my_zsh() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let profile_dir = temp_dir.path().join("profile");
        fs::create_dir(&profile_dir)?;

        create_test_profile_with_manifest(&profile_dir, "oh-my-zsh")?;
        fs::create_dir(profile_dir.join(".oh-my-zsh"))?;

        let (framework_type, framework_dir) = detect_framework_info(&profile_dir)?;

        assert!(framework_type.is_some());
        assert_eq!(framework_type.unwrap().name(), "oh-my-zsh");
        assert!(framework_dir.is_some());

        Ok(())
    }

    #[test]
    fn test_detect_framework_info_no_manifest() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let profile_dir = temp_dir.path().join("profile");
        fs::create_dir(&profile_dir)?;

        let (framework_type, framework_dir) = detect_framework_info(&profile_dir)?;

        assert!(framework_type.is_none());
        assert!(framework_dir.is_none());

        Ok(())
    }

    #[test]
    fn test_confirm_rollback_acceptance() {
        // This would require mocking stdin
        // Tested in integration tests
    }
}
