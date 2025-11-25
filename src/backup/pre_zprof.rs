//! Pre-zprof backup creation and validation
//!
//! This module handles the automatic backup of shell configurations
//! before zprof modifies them during initialization.

use anyhow::{Context, Result};
use log::info;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::backup_manifest::{BackupManifest, BackedUpFile, DetectedFramework};
use crate::frameworks::{detect_existing_framework, FrameworkInfo};

/// List of shell config files to back up from HOME directory
const SHELL_CONFIG_FILES: &[&str] = &[
    ".zshrc",
    ".zshenv",
    ".zprofile",
    ".zlogin",
    ".zlogout",
    ".zsh_history",
];

/// Check if a pre-zprof backup already exists
///
/// # Arguments
///
/// * `backup_dir` - Path to the pre-zprof backup directory
pub fn backup_exists(backup_dir: &Path) -> bool {
    backup_dir.exists() && backup_dir.join("backup-manifest.toml").exists()
}

/// Create a pre-zprof backup of the user's shell configuration
///
/// This function is idempotent - if a backup already exists, it will not be overwritten.
///
/// # Arguments
///
/// * `home_dir` - User's HOME directory
/// * `backup_dir` - Directory where backup should be created (e.g., ~/.zsh-profiles/backups/pre-zprof)
///
/// # Returns
///
/// The backup manifest on success, or an error if backup creation fails.
///
/// # Errors
///
/// Returns error if:
/// - Cannot create backup directory
/// - Cannot read or copy source files
/// - Cannot write manifest file
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use zprof::backup::create_backup;
///
/// let home = dirs::home_dir().unwrap();
/// let backup_dir = home.join(".zsh-profiles/backups/pre-zprof");
/// let manifest = create_backup(&home, &backup_dir).unwrap();
/// ```
pub fn create_backup(home_dir: &Path, backup_dir: &Path) -> Result<BackupManifest> {
    // Check if backup already exists (idempotent)
    if backup_exists(backup_dir) {
        info!("Pre-zprof backup already exists at {}, skipping creation", backup_dir.display());
        return validate_backup(backup_dir);
    }

    info!("Creating pre-zprof backup at {}", backup_dir.display());

    // Create backup directory with permissions 700
    fs::create_dir_all(backup_dir)
        .with_context(|| format!("Failed to create backup directory at {}", backup_dir.display()))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = fs::Permissions::from_mode(0o700);
        fs::set_permissions(backup_dir, permissions)
            .with_context(|| format!("Failed to set permissions on backup directory at {}", backup_dir.display()))?;
    }

    // Get system information for manifest
    let zsh_version = get_zsh_version().unwrap_or_else(|_| "unknown".to_string());
    let os = std::env::consts::OS.to_string();
    let zprof_version = env!("CARGO_PKG_VERSION").to_string();

    // Create manifest
    let mut manifest = BackupManifest::new(zsh_version, os, zprof_version);

    // Detect existing framework
    if let Some(framework_info) = detect_existing_framework() {
        info!("Detected {} framework", framework_info.framework_type.name());
        manifest.set_framework(framework_info_to_detected(&framework_info));
    } else {
        info!("No existing framework detected");
    }

    // Backup each shell config file
    let mut backed_up_count = 0;
    for config_file in SHELL_CONFIG_FILES {
        let source_path = home_dir.join(config_file);

        if !source_path.exists() {
            info!("Skipping {} (does not exist)", config_file);
            continue;
        }

        // Copy file to backup directory
        let dest_path = backup_dir.join(config_file);
        fs::copy(&source_path, &dest_path)
            .with_context(|| format!("Failed to copy {} to backup", config_file))?;

        info!("Backed up {}", config_file);

        // Create BackedUpFile entry with checksum and metadata
        let backed_up_file = BackedUpFile::from_path(
            PathBuf::from(config_file),
            &source_path,
        )?;

        manifest.add_file(backed_up_file);
        backed_up_count += 1;
    }

    // Save manifest to backup directory
    let manifest_path = backup_dir.join("backup-manifest.toml");
    manifest.save_to_file(&manifest_path)
        .context("Failed to save backup manifest")?;

    info!("Pre-zprof backup complete: {} files backed up", backed_up_count);

    Ok(manifest)
}

/// Validate an existing pre-zprof backup
///
/// Loads the manifest and verifies it can be read correctly.
/// Does not verify checksums (that's expensive and unnecessary for most cases).
///
/// # Arguments
///
/// * `backup_dir` - Path to the pre-zprof backup directory
pub fn validate_backup(backup_dir: &Path) -> Result<BackupManifest> {
    let manifest_path = backup_dir.join("backup-manifest.toml");

    if !manifest_path.exists() {
        anyhow::bail!("Backup manifest not found at {}", manifest_path.display());
    }

    let manifest = BackupManifest::load_from_file(&manifest_path)
        .context("Failed to load backup manifest")?;

    info!("Validated backup created at {}", manifest.metadata.created_at);
    info!("  {} files in manifest", manifest.files.len());

    if let Some(ref framework) = manifest.detected_framework {
        info!("  Framework: {}", framework.name);
    }

    Ok(manifest)
}

/// Get the current zsh version
fn get_zsh_version() -> Result<String> {
    use std::process::Command;

    let output = Command::new("zsh")
        .arg("--version")
        .output()
        .context("Failed to execute zsh --version")?;

    if !output.status.success() {
        anyhow::bail!("zsh --version returned non-zero exit code");
    }

    let version_str = String::from_utf8(output.stdout)
        .context("zsh version output is not valid UTF-8")?;

    // Parse version from output like "zsh 5.9 (x86_64-apple-darwin23.0)"
    Ok(version_str.trim().to_string())
}

/// Move backed-up config files from HOME to backup directory
///
/// This function moves (not copies) files that were previously backed up
/// from the user's HOME directory to the backup location. It should be called
/// after `create_backup` successfully completes.
///
/// # Arguments
///
/// * `home_dir` - User's HOME directory
/// * `files` - List of files that were backed up
///
/// # Returns
///
/// Number of files successfully moved
///
/// # Errors
///
/// Returns error if:
/// - File move operations fail
///
/// # Example
///
/// ```no_run
/// use std::path::Path;
/// use zprof::backup::pre_zprof::{create_backup, move_configs_to_backup};
///
/// let home = dirs::home_dir().unwrap();
/// let backup_dir = home.join(".zsh-profiles/backups/pre-zprof");
/// let manifest = create_backup(&home, &backup_dir).unwrap();
/// let moved = move_configs_to_backup(&home, &manifest.files).unwrap();
/// ```
pub fn move_configs_to_backup(home_dir: &Path, files: &[BackedUpFile]) -> Result<usize> {

    let mut moved_count = 0;

    for file_entry in files {
        let source_path = home_dir.join(&file_entry.path);

        // Skip if file doesn't exist (may have been removed already)
        if !source_path.exists() {
            info!("Skipping {} (no longer exists in HOME)", file_entry.path.display());
            continue;
        }

        // Check if file is a symlink - resolve and backup the target
        let is_symlink = source_path.is_symlink();
        if is_symlink {
            info!("Resolving symlink: {}", file_entry.path.display());
            // The symlink was already resolved during backup, so we can safely remove it
        }

        // Remove the file from HOME (it's already in backup)
        // Handle read-only files by temporarily changing permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let metadata = fs::metadata(&source_path)
                .with_context(|| format!("Failed to get metadata for {}", source_path.display()))?;
            let permissions = metadata.permissions();

            // If file is read-only, make it writable temporarily
            if permissions.readonly() {
                let mut new_perms = permissions.clone();
                new_perms.set_mode(new_perms.mode() | 0o200); // Add owner write permission
                fs::set_permissions(&source_path, new_perms)
                    .with_context(|| format!("Failed to make {} writable", source_path.display()))?;
                info!("Made {} writable for removal", file_entry.path.display());
            }
        }

        // Use fs::remove_file for both regular files and symlinks
        fs::remove_file(&source_path)
            .with_context(|| format!(
                "Failed to remove {} from HOME after backup",
                file_entry.path.display()
            ))?;

        info!("Moved {} to backup (removed from HOME)", file_entry.path.display());
        moved_count += 1;
    }

    Ok(moved_count)
}

/// Convert FrameworkInfo to DetectedFramework for manifest
fn framework_info_to_detected(info: &FrameworkInfo) -> DetectedFramework {
    DetectedFramework {
        name: info.framework_type.name().to_string(),
        path: info.install_path.clone(),
        config_files: vec![info.config_path.clone()],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_backup_exists_when_no_backup() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups/pre-zprof");

        assert!(!backup_exists(&backup_dir));
    }

    #[test]
    fn test_backup_exists_with_manifest() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups/pre-zprof");
        fs::create_dir_all(&backup_dir).unwrap();

        // Create manifest
        let manifest_path = backup_dir.join("backup-manifest.toml");
        fs::write(&manifest_path, "# test manifest\n").unwrap();

        assert!(backup_exists(&backup_dir));
    }

    #[test]
    fn test_create_backup_with_zshrc() {
        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().join("home");
        let backup_dir = temp_dir.path().join("backups/pre-zprof");

        // Create fake home directory with .zshrc
        fs::create_dir_all(&home_dir).unwrap();
        fs::write(home_dir.join(".zshrc"), "# my zshrc\n").unwrap();

        // Create backup
        let manifest = create_backup(&home_dir, &backup_dir).unwrap();

        // Verify backup directory created
        assert!(backup_dir.exists());
        assert!(backup_dir.join("backup-manifest.toml").exists());
        assert!(backup_dir.join(".zshrc").exists());

        // Verify manifest contents
        assert_eq!(manifest.files.len(), 1);
        assert_eq!(manifest.files[0].path, PathBuf::from(".zshrc"));

        // Verify file was copied (not moved)
        assert!(home_dir.join(".zshrc").exists());
    }

    #[test]
    fn test_create_backup_idempotent() {
        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().join("home");
        let backup_dir = temp_dir.path().join("backups/pre-zprof");

        fs::create_dir_all(&home_dir).unwrap();
        fs::write(home_dir.join(".zshrc"), "# my zshrc\n").unwrap();

        // Create backup twice
        let manifest1 = create_backup(&home_dir, &backup_dir).unwrap();
        let manifest2 = create_backup(&home_dir, &backup_dir).unwrap();

        // Second call should load existing backup, not create new one
        assert_eq!(manifest1.files.len(), manifest2.files.len());
        assert_eq!(manifest1.metadata.created_at, manifest2.metadata.created_at);
    }

    #[test]
    fn test_create_backup_with_multiple_files() {
        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().join("home");
        let backup_dir = temp_dir.path().join("backups/pre-zprof");

        fs::create_dir_all(&home_dir).unwrap();
        fs::write(home_dir.join(".zshrc"), "# zshrc\n").unwrap();
        fs::write(home_dir.join(".zshenv"), "# zshenv\n").unwrap();
        fs::write(home_dir.join(".zsh_history"), ": 1234567890:0;ls\n").unwrap();

        let manifest = create_backup(&home_dir, &backup_dir).unwrap();

        // Should have backed up 3 files
        assert_eq!(manifest.files.len(), 3);

        // Verify all files exist in backup
        assert!(backup_dir.join(".zshrc").exists());
        assert!(backup_dir.join(".zshenv").exists());
        assert!(backup_dir.join(".zsh_history").exists());
    }

    #[test]
    fn test_create_backup_skips_missing_files() {
        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().join("home");
        let backup_dir = temp_dir.path().join("backups/pre-zprof");

        fs::create_dir_all(&home_dir).unwrap();
        // Only create .zshrc, leave others missing
        fs::write(home_dir.join(".zshrc"), "# zshrc\n").unwrap();

        let manifest = create_backup(&home_dir, &backup_dir).unwrap();

        // Should only have 1 file
        assert_eq!(manifest.files.len(), 1);
        assert_eq!(manifest.files[0].path, PathBuf::from(".zshrc"));
    }

    #[test]
    fn test_validate_backup() {
        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().join("home");
        let backup_dir = temp_dir.path().join("backups/pre-zprof");

        fs::create_dir_all(&home_dir).unwrap();
        fs::write(home_dir.join(".zshrc"), "# test\n").unwrap();

        // Create backup
        let original_manifest = create_backup(&home_dir, &backup_dir).unwrap();

        // Validate backup
        let validated_manifest = validate_backup(&backup_dir).unwrap();

        assert_eq!(original_manifest.files.len(), validated_manifest.files.len());
        assert_eq!(original_manifest.metadata.created_at, validated_manifest.metadata.created_at);
    }

    #[test]
    fn test_validate_backup_missing_manifest() {
        let temp_dir = TempDir::new().unwrap();
        let backup_dir = temp_dir.path().join("backups/pre-zprof");
        fs::create_dir_all(&backup_dir).unwrap();

        let result = validate_backup(&backup_dir);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("manifest not found"));
    }

    #[test]
    #[cfg(unix)]
    fn test_backup_directory_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().join("home");
        let backup_dir = temp_dir.path().join("backups/pre-zprof");

        fs::create_dir_all(&home_dir).unwrap();
        fs::write(home_dir.join(".zshrc"), "# test\n").unwrap();

        create_backup(&home_dir, &backup_dir).unwrap();

        // Check directory permissions are 700
        let metadata = fs::metadata(&backup_dir).unwrap();
        let mode = metadata.permissions().mode();
        assert_eq!(mode & 0o777, 0o700);
    }

    // Story 3.2: Move configs to backup tests
    #[test]
    fn test_move_configs_to_backup() {
        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().join("home");
        let backup_dir = temp_dir.path().join("backups/pre-zprof");

        fs::create_dir_all(&home_dir).unwrap();
        fs::write(home_dir.join(".zshrc"), "# zshrc\n").unwrap();
        fs::write(home_dir.join(".zshenv"), "# zshenv\n").unwrap();

        let manifest = create_backup(&home_dir, &backup_dir).unwrap();
        assert_eq!(manifest.files.len(), 2);

        // Verify files exist in both HOME and backup before move
        assert!(home_dir.join(".zshrc").exists());
        assert!(home_dir.join(".zshenv").exists());
        assert!(backup_dir.join(".zshrc").exists());
        assert!(backup_dir.join(".zshenv").exists());

        // Move configs
        let moved_count = move_configs_to_backup(&home_dir, &manifest.files).unwrap();
        assert_eq!(moved_count, 2);

        // Verify files removed from HOME
        assert!(!home_dir.join(".zshrc").exists());
        assert!(!home_dir.join(".zshenv").exists());

        // Verify files still in backup
        assert!(backup_dir.join(".zshrc").exists());
        assert!(backup_dir.join(".zshenv").exists());
    }

    #[test]
    fn test_move_configs_skips_missing_files() {
        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().join("home");
        let backup_dir = temp_dir.path().join("backups/pre-zprof");

        fs::create_dir_all(&home_dir).unwrap();
        fs::write(home_dir.join(".zshrc"), "# zshrc\n").unwrap();

        let manifest = create_backup(&home_dir, &backup_dir).unwrap();
        assert_eq!(manifest.files.len(), 1);

        // Manually remove file from HOME before move
        fs::remove_file(home_dir.join(".zshrc")).unwrap();

        // Move should succeed but skip the missing file
        let moved_count = move_configs_to_backup(&home_dir, &manifest.files).unwrap();
        assert_eq!(moved_count, 0);

        // Backup file should still exist
        assert!(backup_dir.join(".zshrc").exists());
    }

    #[test]
    #[cfg(unix)]
    fn test_move_configs_handles_readonly_files() {
        use std::os::unix::fs::PermissionsExt;

        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().join("home");
        let backup_dir = temp_dir.path().join("backups/pre-zprof");

        fs::create_dir_all(&home_dir).unwrap();
        fs::write(home_dir.join(".zshrc"), "# zshrc\n").unwrap();

        // Make file read-only
        let permissions = fs::Permissions::from_mode(0o400);
        fs::set_permissions(home_dir.join(".zshrc"), permissions).unwrap();

        let manifest = create_backup(&home_dir, &backup_dir).unwrap();

        // Move should succeed even with read-only file
        let moved_count = move_configs_to_backup(&home_dir, &manifest.files).unwrap();
        assert_eq!(moved_count, 1);

        // Verify file removed from HOME
        assert!(!home_dir.join(".zshrc").exists());

        // Verify file still in backup
        assert!(backup_dir.join(".zshrc").exists());
    }

    #[test]
    #[cfg(unix)]
    fn test_move_configs_handles_symlinks() {
        use std::os::unix::fs;

        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().join("home");
        let backup_dir = temp_dir.path().join("backups/pre-zprof");

        std::fs::create_dir_all(&home_dir).unwrap();

        // Create target file and symlink
        let target_file = home_dir.join("real_zshrc");
        std::fs::write(&target_file, "# real zshrc\n").unwrap();
        fs::symlink(&target_file, home_dir.join(".zshrc")).unwrap();

        let manifest = create_backup(&home_dir, &backup_dir).unwrap();

        // Move should handle symlink
        let moved_count = move_configs_to_backup(&home_dir, &manifest.files).unwrap();
        assert_eq!(moved_count, 1);

        // Verify symlink removed from HOME
        assert!(!home_dir.join(".zshrc").exists());

        // Verify file still in backup
        assert!(backup_dir.join(".zshrc").exists());

        // Verify original target still exists
        assert!(target_file.exists());
    }

    #[test]
    fn test_move_configs_with_no_files() {
        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().join("home");
        let backup_dir = temp_dir.path().join("backups/pre-zprof");

        std::fs::create_dir_all(&home_dir).unwrap();

        let manifest = create_backup(&home_dir, &backup_dir).unwrap();
        assert_eq!(manifest.files.len(), 0);

        // Move with empty file list should succeed
        let moved_count = move_configs_to_backup(&home_dir, &manifest.files).unwrap();
        assert_eq!(moved_count, 0);
    }
}
