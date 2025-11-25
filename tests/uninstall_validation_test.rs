//! Precondition validation tests for uninstall
//!
//! Tests the validation system that checks all preconditions before
//! allowing uninstall to proceed.

use zprof::backup::restore::{validate_preconditions, ValidationReport};

/// Test validation report structure
#[test]
fn test_validation_report_all_valid() {
    let report = ValidationReport {
        zprof_installed: true,
        has_write_permissions: true,
        home_dir_valid: true,
        pre_zprof_backup_exists: true,
        warnings: Vec::new(),
    };

    assert!(report.is_valid());
    assert!(report.get_issues().is_empty());
    assert!(report.warnings.is_empty());
}

/// Test validation fails when zprof not installed
#[test]
fn test_validation_fails_not_installed() {
    let report = ValidationReport {
        zprof_installed: false,
        has_write_permissions: true,
        home_dir_valid: true,
        pre_zprof_backup_exists: true,
        warnings: Vec::new(),
    };

    assert!(!report.is_valid());
    let issues = report.get_issues();
    assert!(!issues.is_empty());
    assert!(issues[0].contains("not installed"));
}

/// Test validation fails without write permissions
#[test]
fn test_validation_fails_no_write_permissions() {
    let report = ValidationReport {
        zprof_installed: true,
        has_write_permissions: false,
        home_dir_valid: true,
        pre_zprof_backup_exists: true,
        warnings: Vec::new(),
    };

    assert!(!report.is_valid());
    let issues = report.get_issues();
    assert!(issues.iter().any(|i| i.contains("write permissions")));
}

/// Test validation fails with invalid HOME
#[test]
fn test_validation_fails_invalid_home() {
    let report = ValidationReport {
        zprof_installed: true,
        has_write_permissions: true,
        home_dir_valid: false,
        pre_zprof_backup_exists: true,
        warnings: Vec::new(),
    };

    assert!(!report.is_valid());
    let issues = report.get_issues();
    assert!(issues.iter().any(|i| i.contains("HOME")));
}

/// Test validation warns about missing backup
#[test]
fn test_validation_warns_missing_backup() {
    let report = ValidationReport {
        zprof_installed: true,
        has_write_permissions: true,
        home_dir_valid: true,
        pre_zprof_backup_exists: false,
        warnings: vec!["No pre-zprof backup found".to_string()],
    };

    assert!(report.is_valid()); // Still valid, just has warning
    assert!(!report.warnings.is_empty());
}

/// Test validation collects multiple issues
#[test]
fn test_validation_multiple_issues() {
    let report = ValidationReport {
        zprof_installed: false,
        has_write_permissions: false,
        home_dir_valid: false,
        pre_zprof_backup_exists: false,
        warnings: vec!["Warning 1".to_string(), "Warning 2".to_string()],
    };

    assert!(!report.is_valid());
    let issues = report.get_issues();
    assert_eq!(issues.len(), 3); // zprof, permissions, HOME
    assert_eq!(report.warnings.len(), 2);
}

/// Test actual validation function runs without panicking
#[test]
fn test_validate_preconditions_runs() {
    // This test runs against actual system state
    // We just verify it completes without panicking
    let result = validate_preconditions();

    assert!(result.is_ok());
    let report = result.unwrap();

    // Report should have all required fields
    // We can't assert specific values since they depend on system state
    let _ = report.is_valid();
    let _ = report.get_issues();
}

/// Test validation report debug format
#[test]
fn test_validation_report_debug() {
    let report = ValidationReport {
        zprof_installed: true,
        has_write_permissions: true,
        home_dir_valid: true,
        pre_zprof_backup_exists: false,
        warnings: Vec::new(),
    };

    let debug_str = format!("{:?}", report);
    assert!(debug_str.contains("ValidationReport"));
    assert!(debug_str.contains("zprof_installed"));
}

/// Test validation report clone
#[test]
fn test_validation_report_clone() {
    let report = ValidationReport {
        zprof_installed: true,
        has_write_permissions: true,
        home_dir_valid: true,
        pre_zprof_backup_exists: true,
        warnings: vec!["test".to_string()],
    };

    let cloned = report.clone();
    assert_eq!(report.zprof_installed, cloned.zprof_installed);
    assert_eq!(report.warnings.len(), cloned.warnings.len());
}
