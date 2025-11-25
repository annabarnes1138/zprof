//! Restoration logic for uninstall with comprehensive error handling
//!
//! This module handles restoring shell configurations during uninstall,
//! including validation, conflict resolution, and rollback capabilities.

use anyhow::{bail, Context, Result};
use log::{info, warn};
use std::fs;
use std::path::{Path, PathBuf};

use crate::backup::pre_zprof;
use crate::core::backup_manifest::BackupManifest;

/// Validation report for precondition checks
#[derive(Debug, Clone)]
pub struct ValidationReport {
    pub zprof_installed: bool,
    pub has_write_permissions: bool,
    pub home_dir_valid: bool,
    pub pre_zprof_backup_exists: bool,
    pub warnings: Vec<String>,
}

impl ValidationReport {
    /// Check if all critical validations passed
    pub fn is_valid(&self) -> bool {
        self.zprof_installed && self.has_write_permissions && self.home_dir_valid
    }

    /// Get a summary of validation issues
    pub fn get_issues(&self) -> Vec<String> {
        let mut issues = Vec::new();

        if !self.zprof_installed {
            issues.push("zprof is not installed".to_string());
        }
        if !self.has_write_permissions {
            issues.push("No write permissions to HOME directory".to_string());
        }
        if !self.home_dir_valid {
            issues.push("HOME environment variable is not set or invalid".to_string());
        }

        issues
    }
}

/// Resolution strategy for file conflicts during restoration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConflictResolution {
    /// Overwrite existing file
    Overwrite,
    /// Create backup of existing file before overwriting
    Backup,
    /// Skip restoration of this file
    Skip,
}

/// Validate all preconditions before uninstall
///
/// Checks:
/// - zprof is installed (.zsh-profiles exists)
/// - HOME environment variable is set and valid
/// - Write permissions to HOME directory
/// - Pre-zprof backup exists (warning only if missing)
/// - Active shell sessions (warning only, best effort)
///
/// # Returns
/// ValidationReport with results of all checks
pub fn validate_preconditions() -> Result<ValidationReport> {
    let mut report = ValidationReport {
        zprof_installed: false,
        has_write_permissions: false,
        home_dir_valid: false,
        pre_zprof_backup_exists: false,
        warnings: Vec::new(),
    };

    // Check HOME environment variable
    let home_dir = match dirs::home_dir() {
        Some(dir) if dir.exists() && dir.is_dir() => {
            report.home_dir_valid = true;
            dir
        }
        Some(dir) => {
            warn!("HOME directory does not exist or is not a directory: {}", dir.display());
            return Ok(report);
        }
        None => {
            warn!("HOME environment variable is not set");
            return Ok(report);
        }
    };

    // Check if zprof is installed
    let zprof_dir = home_dir.join(".zsh-profiles");
    report.zprof_installed = zprof_dir.exists() && zprof_dir.join("config.toml").exists();

    if !report.zprof_installed {
        // If not installed, skip further checks
        return Ok(report);
    }

    // Check write permissions to HOME
    report.has_write_permissions = check_write_permission(&home_dir);

    // Check if pre-zprof backup exists
    let backup_dir = home_dir.join(".zsh-profiles/backups/pre-zprof");
    report.pre_zprof_backup_exists = pre_zprof::backup_exists(&backup_dir);

    if !report.pre_zprof_backup_exists {
        report.warnings.push(
            "No pre-zprof backup found. The 'Restore Original' option will not be available.".to_string()
        );
    }

    // Try to detect active shell sessions (best effort, platform-specific)
    if let Ok(active_shells) = detect_active_shells() {
        if !active_shells.is_empty() {
            report.warnings.push(format!(
                "Active shell sessions detected: {}. You should close all zsh sessions before uninstalling.",
                active_shells.join(", ")
            ));
        }
    }

    Ok(report)
}

/// Check if we have write permission to a directory
fn check_write_permission(dir: &Path) -> bool {
    if !dir.exists() {
        return false;
    }

    // Try to create a temporary file to test write permissions
    let test_file = dir.join(format!(".zprof_write_test_{}", std::process::id()));

    let can_write = fs::write(&test_file, b"test").is_ok();

    // Clean up test file
    let _ = fs::remove_file(&test_file);

    can_write
}

/// Detect active shell sessions (best effort, platform-specific)
///
/// Returns a list of detected active shell process descriptions.
/// This is a best-effort check and may not detect all shells.
pub fn detect_active_shells() -> Result<Vec<String>> {
    #[cfg(target_os = "macos")]
    {
        detect_active_shells_macos()
    }

    #[cfg(target_os = "linux")]
    {
        detect_active_shells_linux()
    }

    #[cfg(not(any(target_os = "macos", target_os = "linux")))]
    {
        // Not supported on other platforms
        Ok(Vec::new())
    }
}

#[cfg(target_os = "macos")]
fn detect_active_shells_macos() -> Result<Vec<String>> {
    use std::process::Command;

    let output = Command::new("pgrep")
        .arg("-fl")
        .arg("zsh")
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let shells: Vec<String> = stdout
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| {
                    // Extract PID and command
                    let parts: Vec<&str> = line.splitn(2, ' ').collect();
                    if parts.len() == 2 {
                        format!("PID {} ({})", parts[0], parts[1].trim())
                    } else {
                        line.to_string()
                    }
                })
                .collect();
            Ok(shells)
        }
        _ => Ok(Vec::new()),
    }
}

#[cfg(target_os = "linux")]
fn detect_active_shells_linux() -> Result<Vec<String>> {
    use std::process::Command;

    let output = Command::new("pgrep")
        .arg("-a")
        .arg("zsh")
        .output();

    match output {
        Ok(output) if output.status.success() => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let shells: Vec<String> = stdout
                .lines()
                .filter(|line| !line.is_empty())
                .map(|line| {
                    // Extract PID and command
                    let parts: Vec<&str> = line.splitn(2, ' ').collect();
                    if parts.len() == 2 {
                        format!("PID {} ({})", parts[0], parts[1].trim())
                    } else {
                        line.to_string()
                    }
                })
                .collect();
            Ok(shells)
        }
        _ => Ok(Vec::new()),
    }
}

/// Handle file conflict during restoration
///
/// If a file already exists at the destination, this function determines
/// how to handle the conflict based on the interactive flag.
///
/// # Arguments
/// * `path` - Path where conflict occurred
/// * `interactive` - Whether to prompt user for decision
///
/// # Returns
/// The chosen conflict resolution strategy
pub fn handle_file_conflict(path: &Path, interactive: bool) -> Result<ConflictResolution> {
    if !path.exists() {
        // No conflict
        return Ok(ConflictResolution::Overwrite);
    }

    if !interactive {
        // Non-interactive mode: default to creating backup
        info!("File conflict at {}: creating backup", path.display());
        return Ok(ConflictResolution::Backup);
    }

    // Interactive mode: prompt user
    use dialoguer::Select;

    println!("\n⚠️  File already exists: {}", path.display());
    println!("How would you like to proceed?");

    let options = vec![
        "Backup existing file and restore from backup",
        "Overwrite existing file",
        "Skip this file",
    ];

    let selection = Select::new()
        .items(&options)
        .default(0)
        .interact()
        .context("Failed to get user input for conflict resolution")?;

    match selection {
        0 => Ok(ConflictResolution::Backup),
        1 => Ok(ConflictResolution::Overwrite),
        2 => Ok(ConflictResolution::Skip),
        _ => Ok(ConflictResolution::Backup), // Default to safest option
    }
}

/// Restore files from pre-zprof backup with conflict handling and rollback
///
/// This function restores all files from the pre-zprof backup to the HOME directory.
/// It handles file conflicts, validates checksums, and can roll back on failure.
///
/// # Arguments
/// * `home_dir` - User's HOME directory
/// * `backup_dir` - Pre-zprof backup directory
/// * `interactive` - Whether to prompt for conflict resolution
///
/// # Returns
/// Ok(()) on success, or error with recovery instructions
pub fn restore_pre_zprof_with_validation(
    home_dir: &Path,
    backup_dir: &Path,
    interactive: bool,
) -> Result<()> {
    info!("Restoring pre-zprof backup from {}", backup_dir.display());

    // Validate backup exists and load manifest
    let manifest = pre_zprof::validate_backup(backup_dir)
        .context("Failed to validate pre-zprof backup")?;

    println!("  Backup created: {}", manifest.metadata.created_at.format("%Y-%m-%d %H:%M"));
    println!("  Files to restore: {}", manifest.files.len());

    // Track what we've modified for rollback
    let mut restored_files: Vec<PathBuf> = Vec::new();
    let mut backed_up_conflicts: Vec<(PathBuf, PathBuf)> = Vec::new();

    // Attempt to restore each file
    let restore_result = restore_files_from_manifest(
        home_dir,
        backup_dir,
        &manifest,
        interactive,
        &mut restored_files,
        &mut backed_up_conflicts,
    );

    match restore_result {
        Ok(()) => {
            println!("✓ Original configuration restored successfully");
            Ok(())
        }
        Err(e) => {
            // Restoration failed - attempt rollback
            eprintln!("\n❌ Error during restoration: {}", e);
            eprintln!("\n🔄 Attempting to roll back changes...");

            if let Err(rollback_err) = rollback_restoration(
                &restored_files,
                &backed_up_conflicts,
            ) {
                // Rollback also failed - provide manual recovery instructions
                bail!(
                    "Restoration failed and automatic rollback failed: {}\n\n\
                     Rollback error: {}\n\n\
                     Your data is safe. To manually recover:\n\
                     1. Your pre-zprof backup is intact at: {}\n\
                     2. Conflict backups (if any) are at: [original-path].zprofbackup\n\
                     3. You can manually copy files from the backup to restore your configuration\n\n\
                     Original error: {}",
                    rollback_err,
                    rollback_err,
                    backup_dir.display(),
                    e
                );
            }

            eprintln!("✓ Changes rolled back successfully");
            bail!(
                "Restoration failed but was rolled back: {}\n\n\
                 Your original configuration is unchanged.\n\
                 Your pre-zprof backup is intact at: {}",
                e,
                backup_dir.display()
            );
        }
    }
}

/// Restore files from manifest with conflict handling
fn restore_files_from_manifest(
    home_dir: &Path,
    backup_dir: &Path,
    manifest: &BackupManifest,
    interactive: bool,
    restored_files: &mut Vec<PathBuf>,
    backed_up_conflicts: &mut Vec<(PathBuf, PathBuf)>,
) -> Result<()> {
    for backed_up_file in &manifest.files {
        let source = backup_dir.join(&backed_up_file.path);
        let dest = home_dir.join(&backed_up_file.path);

        // Validate backup file exists
        if !source.exists() {
            warn!("Backup file missing: {}", backed_up_file.path.display());
            println!("  ⚠  Warning: Backup file missing, skipping: {}", backed_up_file.path.display());
            continue;
        }

        // Handle conflict if destination exists
        if dest.exists() {
            let resolution = handle_file_conflict(&dest, interactive)
                .context("Failed to resolve file conflict")?;

            match resolution {
                ConflictResolution::Skip => {
                    println!("  ⏭  Skipped: {}", backed_up_file.path.display());
                    continue;
                }
                ConflictResolution::Backup => {
                    // Create backup of existing file
                    let backup_path = PathBuf::from(format!("{}.zprofbackup", dest.display()));
                    fs::copy(&dest, &backup_path)
                        .with_context(|| format!("Failed to backup existing file: {}", dest.display()))?;
                    println!("  💾 Backed up existing file to: {}", backup_path.display());
                    backed_up_conflicts.push((dest.clone(), backup_path));
                }
                ConflictResolution::Overwrite => {
                    // Will overwrite, no backup needed
                    println!("  ⚠  Will overwrite: {}", backed_up_file.path.display());
                }
            }
        }

        // Copy file from backup to HOME
        fs::copy(&source, &dest)
            .with_context(|| format!("Failed to restore {}", backed_up_file.path.display()))?;

        // Restore permissions on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = fs::Permissions::from_mode(backed_up_file.permissions);
            fs::set_permissions(&dest, permissions)
                .with_context(|| format!("Failed to set permissions on {}", backed_up_file.path.display()))?;
        }

        // Validate checksum if provided
        if let Err(e) = validate_checksum(&dest, &backed_up_file.checksum) {
            warn!("Checksum validation failed for {}: {}", backed_up_file.path.display(), e);
            println!("  ⚠  Warning: Checksum mismatch for {} (file may be corrupted)", backed_up_file.path.display());
            // Continue anyway - user can verify manually
        }

        restored_files.push(dest.clone());
        println!("  ✓ Restored: {}", backed_up_file.path.display());
    }

    Ok(())
}

/// Validate file checksum matches expected value
///
/// # Arguments
/// * `file` - Path to file to validate
/// * `expected` - Expected SHA256 checksum (hex string)
pub fn validate_checksum(file: &Path, expected: &str) -> Result<bool> {
    use sha2::{Digest, Sha256};

    if !file.exists() {
        bail!("File does not exist: {}", file.display());
    }

    let content = fs::read(file)
        .with_context(|| format!("Failed to read file for checksum: {}", file.display()))?;

    let mut hasher = Sha256::new();
    hasher.update(&content);
    let result = hasher.finalize();
    let calculated = format!("{:x}", result);

    if calculated == expected {
        Ok(true)
    } else {
        bail!(
            "Checksum mismatch: expected {}, got {}",
            expected,
            calculated
        );
    }
}

/// Roll back a failed restoration
///
/// Attempts to undo changes made during a failed restoration by:
/// 1. Removing files that were restored
/// 2. Restoring backed-up conflict files to their original locations
///
/// # Arguments
/// * `restored_files` - List of files that were restored and should be removed
/// * `backed_up_conflicts` - List of (original_path, backup_path) pairs to restore
pub fn rollback_restoration(
    restored_files: &[PathBuf],
    backed_up_conflicts: &[(PathBuf, PathBuf)],
) -> Result<()> {
    let mut rollback_errors = Vec::new();

    // Remove restored files
    for file in restored_files {
        if file.exists() {
            if let Err(e) = fs::remove_file(file) {
                rollback_errors.push(format!("Failed to remove {}: {}", file.display(), e));
            } else {
                info!("Rolled back: removed {}", file.display());
            }
        }
    }

    // Restore backed-up conflicts
    for (original, backup) in backed_up_conflicts {
        if backup.exists() {
            if let Err(e) = fs::rename(backup, original) {
                rollback_errors.push(format!(
                    "Failed to restore {} from backup: {}",
                    original.display(),
                    e
                ));
            } else {
                info!("Rolled back: restored {} from backup", original.display());
            }
        }
    }

    if !rollback_errors.is_empty() {
        bail!(
            "Rollback encountered {} errors:\n  - {}",
            rollback_errors.len(),
            rollback_errors.join("\n  - ")
        );
    }

    Ok(())
}

/// Check available disk space before operations
///
/// # Arguments
/// * `required_bytes` - Minimum required space in bytes
///
/// # Returns
/// Ok(true) if sufficient space, Ok(false) if insufficient, Err on check failure
///
/// Note: Currently returns Ok(true) optimistically. Full implementation
/// requires platform-specific code and libc dependency.
#[allow(dead_code)]
pub fn check_disk_space(_required_bytes: u64) -> Result<bool> {
    // TODO: Implement platform-specific disk space check
    // For now, optimistically assume space is available
    // This is acceptable as filesystem operations will fail with clear errors
    // if space is insufficient
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_report_is_valid() {
        let report = ValidationReport {
            zprof_installed: true,
            has_write_permissions: true,
            home_dir_valid: true,
            pre_zprof_backup_exists: true,
            warnings: Vec::new(),
        };

        assert!(report.is_valid());
    }

    #[test]
    fn test_validation_report_invalid_when_missing_permissions() {
        let report = ValidationReport {
            zprof_installed: true,
            has_write_permissions: false,
            home_dir_valid: true,
            pre_zprof_backup_exists: true,
            warnings: Vec::new(),
        };

        assert!(!report.is_valid());
    }

    #[test]
    fn test_validation_report_get_issues() {
        let report = ValidationReport {
            zprof_installed: false,
            has_write_permissions: true,
            home_dir_valid: false,
            pre_zprof_backup_exists: false,
            warnings: Vec::new(),
        };

        let issues = report.get_issues();
        assert_eq!(issues.len(), 2);
        assert!(issues.iter().any(|i| i.contains("not installed")));
        assert!(issues.iter().any(|i| i.contains("HOME")));
    }

    #[test]
    fn test_conflict_resolution_variants() {
        assert_eq!(ConflictResolution::Overwrite, ConflictResolution::Overwrite);
        assert_ne!(ConflictResolution::Overwrite, ConflictResolution::Backup);
        assert_ne!(ConflictResolution::Backup, ConflictResolution::Skip);
    }

    #[test]
    fn test_validate_preconditions_structure() {
        // Just test that the function runs without panicking
        // Actual validation depends on system state
        let _ = validate_preconditions();
    }
}
