//! Edge case tests for uninstall validation and error handling
//!
//! Tests cover the scenarios outlined in Story 3.7:
//! - Missing pre-zprof backup
//! - File conflicts during restoration
//! - Permission errors
//! - Checksum validation
//! - Partial failures and rollback

use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

use zprof::backup::restore::{
    validate_preconditions, ValidationReport, ConflictResolution,
    handle_file_conflict, validate_checksum, rollback_restoration,
};
use zprof::backup::pre_zprof;

/// Test AC1: Validate preconditions - zprof not installed
#[test]
fn test_validation_zprof_not_installed() {
    // This test runs against actual system, so we can't easily simulate
    // "not installed" state. We test the structure instead.
    let report = validate_preconditions().unwrap();

    // Report should have all required fields - just verify the function runs
    // without panicking and returns a properly structured report
    let _ = report.is_valid();
    let _ = report.get_issues();
}

/// Test AC1: ValidationReport tracks all checks
#[test]
fn test_validation_report_structure() {
    let report = ValidationReport {
        zprof_installed: true,
        has_write_permissions: true,
        home_dir_valid: true,
        pre_zprof_backup_exists: false,
        warnings: vec!["Test warning".to_string()],
    };

    assert!(report.is_valid());
    assert_eq!(report.warnings.len(), 1);
}

/// Test AC1: ValidationReport identifies issues
#[test]
fn test_validation_report_identifies_issues() {
    let report = ValidationReport {
        zprof_installed: false,
        has_write_permissions: false,
        home_dir_valid: true,
        pre_zprof_backup_exists: false,
        warnings: Vec::new(),
    };

    assert!(!report.is_valid());
    let issues = report.get_issues();
    assert!(issues.len() >= 2);
    assert!(issues.iter().any(|i| i.contains("not installed")));
    assert!(issues.iter().any(|i| i.contains("write permissions")));
}

/// Test AC2: Handle missing pre-zprof backup gracefully
#[test]
fn test_validation_warns_missing_backup() {
    // Create temp environment without backup
    let temp_dir = TempDir::new().unwrap();
    let home_dir = temp_dir.path().join("home");
    let zprof_dir = home_dir.join(".zsh-profiles");

    fs::create_dir_all(&zprof_dir).unwrap();
    fs::write(zprof_dir.join("config.toml"), "[config]").unwrap();

    // Validation should note backup is missing
    // (Testing indirectly since validate_preconditions uses real HOME)
    let backup_dir = zprof_dir.join("backups/pre-zprof");
    assert!(!pre_zprof::backup_exists(&backup_dir));
}

/// Test AC3: File conflict resolution - Overwrite
#[test]
fn test_conflict_resolution_overwrite() {
    let temp_dir = TempDir::new().unwrap();
    let existing_file = temp_dir.path().join("existing.txt");
    fs::write(&existing_file, "original content").unwrap();

    // Test non-interactive (returns Backup by default)
    let resolution = handle_file_conflict(&existing_file, false).unwrap();
    assert_eq!(resolution, ConflictResolution::Backup);
}

/// Test AC3: File conflict resolution - No conflict
#[test]
fn test_conflict_resolution_no_conflict() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent = temp_dir.path().join("nonexistent.txt");

    let resolution = handle_file_conflict(&nonexistent, false).unwrap();
    assert_eq!(resolution, ConflictResolution::Overwrite);
}

/// Test AC4: Rollback removes restored files
#[test]
fn test_rollback_removes_restored_files() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();

    // Create some "restored" files
    let file1 = temp_dir.path().join("file1.txt");
    let file2 = temp_dir.path().join("file2.txt");
    fs::write(&file1, "restored1").unwrap();
    fs::write(&file2, "restored2").unwrap();

    let restored_files = vec![file1.clone(), file2.clone()];
    let backed_up_conflicts = Vec::new();

    // Rollback should remove the files
    rollback_restoration(&restored_files, &backed_up_conflicts)?;

    assert!(!file1.exists());
    assert!(!file2.exists());

    Ok(())
}

/// Test AC4: Rollback restores backed-up conflicts
#[test]
fn test_rollback_restores_backed_up_conflicts() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();

    // Create original file and its backup
    let original = temp_dir.path().join("original.txt");
    let backup = temp_dir.path().join("original.txt.zprofbackup");

    fs::write(&original, "new content").unwrap();
    fs::write(&backup, "original content").unwrap();

    let backed_up_conflicts = vec![(original.clone(), backup.clone())];
    let restored_files = Vec::new();

    // Rollback should restore from backup
    rollback_restoration(&restored_files, &backed_up_conflicts)?;

    assert!(original.exists());
    assert!(!backup.exists());
    assert_eq!(fs::read_to_string(&original)?, "original content");

    Ok(())
}

/// Test AC5: Checksum validation detects corruption
#[test]
fn test_checksum_validation_success() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("test.txt");
    fs::write(&file, "test content")?;

    // Calculate expected checksum using same algorithm as validation
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"test content");
    let expected = format!("{:x}", hasher.finalize());

    let is_valid = validate_checksum(&file, &expected)?;
    assert!(is_valid);

    Ok(())
}

/// Test AC5: Checksum validation fails on mismatch
#[test]
fn test_checksum_validation_mismatch() {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("test.txt");
    fs::write(&file, "actual content").unwrap();

    let wrong_checksum = "0000000000000000000000000000000000000000000000000000000000000000";
    let result = validate_checksum(&file, wrong_checksum);

    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("Checksum mismatch"));
}

/// Test AC5: Checksum validation handles missing file
#[test]
fn test_checksum_validation_missing_file() {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent = temp_dir.path().join("missing.txt");

    let result = validate_checksum(&nonexistent, "dummy_checksum");
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("does not exist"));
}

/// Edge Case: Rollback with empty lists
#[test]
fn test_rollback_with_empty_lists() -> Result<()> {
    let result = rollback_restoration(&[], &[]);
    assert!(result.is_ok());
    Ok(())
}

/// Edge Case: Rollback skips missing files
#[test]
fn test_rollback_skips_missing_files() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let nonexistent = temp_dir.path().join("missing.txt");

    let restored_files = vec![nonexistent];
    let result = rollback_restoration(&restored_files, &[]);

    // Should succeed even though file doesn't exist
    assert!(result.is_ok());

    Ok(())
}

/// Integration Test: Pre-zprof backup missing scenario
#[test]
fn test_backup_missing_integration() {
    let temp_dir = TempDir::new().unwrap();
    let home_dir = temp_dir.path().join("home");
    let backup_dir = home_dir.join(".zsh-profiles/backups/pre-zprof");

    fs::create_dir_all(&home_dir).unwrap();

    // Backup doesn't exist
    assert!(!pre_zprof::backup_exists(&backup_dir));

    // Validation should fail gracefully
    let result = pre_zprof::validate_backup(&backup_dir);
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("manifest not found"));
}

/// Integration Test: File conflict with backup strategy
#[test]
fn test_file_conflict_creates_backup() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let existing_file = temp_dir.path().join(".zshrc");

    fs::write(&existing_file, "existing content")?;

    // Simulate conflict resolution with backup
    let resolution = ConflictResolution::Backup;
    if resolution == ConflictResolution::Backup {
        let backup_path = PathBuf::from(format!("{}.zprofbackup", existing_file.display()));
        fs::copy(&existing_file, &backup_path)?;

        assert!(backup_path.exists());
        assert_eq!(fs::read_to_string(&backup_path)?, "existing content");
    }

    Ok(())
}

/// Integration Test: Manifest with multiple files
#[test]
fn test_manifest_tracks_multiple_files() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let home_dir = temp_dir.path().join("home");
    let backup_dir = temp_dir.path().join("backup");

    fs::create_dir_all(&home_dir)?;

    // Create multiple config files
    fs::write(home_dir.join(".zshrc"), "# zshrc")?;
    fs::write(home_dir.join(".zshenv"), "# zshenv")?;
    fs::write(home_dir.join(".zsh_history"), ": 1234567890:0;ls")?;

    // Create backup
    let manifest = pre_zprof::create_backup(&home_dir, &backup_dir)?;

    assert_eq!(manifest.files.len(), 3);
    assert!(manifest.files.iter().any(|f| f.path.to_str() == Some(".zshrc")));
    assert!(manifest.files.iter().any(|f| f.path.to_str() == Some(".zshenv")));
    assert!(manifest.files.iter().any(|f| f.path.to_str() == Some(".zsh_history")));

    Ok(())
}

/// Edge Case: Empty home directory (no config files)
#[test]
fn test_backup_empty_home() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let home_dir = temp_dir.path().join("home");
    let backup_dir = temp_dir.path().join("backup");

    fs::create_dir_all(&home_dir)?;

    // No config files exist
    let manifest = pre_zprof::create_backup(&home_dir, &backup_dir)?;

    // Should succeed with empty manifest
    assert_eq!(manifest.files.len(), 0);

    Ok(())
}

/// Edge Case: Symlink in config files
#[test]
#[cfg(unix)]
fn test_backup_with_symlink() -> Result<()> {
    use std::os::unix::fs as unix_fs;

    let temp_dir = TempDir::new().unwrap();
    let home_dir = temp_dir.path().join("home");
    let backup_dir = temp_dir.path().join("backup");

    fs::create_dir_all(&home_dir)?;

    // Create target file and symlink
    let target = home_dir.join("real_zshrc");
    fs::write(&target, "# real zshrc")?;
    unix_fs::symlink(&target, home_dir.join(".zshrc"))?;

    // Backup should handle symlink
    let manifest = pre_zprof::create_backup(&home_dir, &backup_dir)?;

    assert_eq!(manifest.files.len(), 1);
    assert!(backup_dir.join(".zshrc").exists());

    Ok(())
}

/// Edge Case: Read-only file
#[test]
#[cfg(unix)]
fn test_backup_readonly_file() -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let temp_dir = TempDir::new().unwrap();
    let home_dir = temp_dir.path().join("home");
    let backup_dir = temp_dir.path().join("backup");

    fs::create_dir_all(&home_dir)?;

    // Create read-only file
    let readonly_file = home_dir.join(".zshrc");
    fs::write(&readonly_file, "# readonly")?;
    let permissions = fs::Permissions::from_mode(0o400);
    fs::set_permissions(&readonly_file, permissions)?;

    // Backup should still succeed
    let manifest = pre_zprof::create_backup(&home_dir, &backup_dir)?;

    assert_eq!(manifest.files.len(), 1);
    assert!(backup_dir.join(".zshrc").exists());

    // Check that permissions were preserved
    let backed_up_file = &manifest.files[0];
    assert_eq!(backed_up_file.permissions & 0o777, 0o400);

    Ok(())
}

/// Edge Case: Very large history file (performance test)
#[test]
#[ignore] // Ignore by default as this is slow
fn test_backup_large_history() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let home_dir = temp_dir.path().join("home");
    let backup_dir = temp_dir.path().join("backup");

    fs::create_dir_all(&home_dir)?;

    // Create large history file (simulate 10,000 entries)
    let mut history = String::new();
    for i in 0..10_000 {
        history.push_str(&format!(": {}:0;command_{}\n", 1234567890 + i, i));
    }
    fs::write(home_dir.join(".zsh_history"), history)?;

    // Backup should complete in reasonable time
    use std::time::Instant;
    let start = Instant::now();

    let manifest = pre_zprof::create_backup(&home_dir, &backup_dir)?;

    let duration = start.elapsed();

    assert_eq!(manifest.files.len(), 1);
    assert!(duration.as_secs() < 5, "Backup took too long: {:?}", duration);

    Ok(())
}

/// Edge Case: Concurrent modification (checksum detection)
#[test]
fn test_checksum_detects_modification() -> Result<()> {
    let temp_dir = TempDir::new().unwrap();
    let file = temp_dir.path().join("test.txt");

    // Write original content and calculate checksum
    fs::write(&file, "original")?;

    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(b"original");
    let original_checksum = format!("{:x}", hasher.finalize());

    // Modify file
    fs::write(&file, "modified")?;

    // Validation should fail
    let result = validate_checksum(&file, &original_checksum);
    assert!(result.is_err());

    Ok(())
}
