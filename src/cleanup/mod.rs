//! Cleanup module for uninstall operations
//!
//! Handles removal of zprof files and directories during uninstall.
//! Provides detailed error reporting and progress feedback.

use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::{Path, PathBuf};

/// Configuration for cleanup operations
#[derive(Debug, Clone)]
pub struct CleanupConfig {
    /// Base .zsh-profiles directory to clean
    pub profiles_dir: PathBuf,
    /// User's HOME directory
    pub home_dir: PathBuf,
    /// Whether to keep the backups/ subdirectory
    pub keep_backups: bool,
}

/// Report of cleanup operations with successes and failures
#[derive(Debug)]
pub struct CleanupReport {
    /// Files that were successfully removed
    pub removed_files: Vec<PathBuf>,
    /// Directories that were successfully removed
    pub removed_dirs: Vec<PathBuf>,
    /// Errors encountered during cleanup
    pub errors: Vec<CleanupError>,
}

/// Error encountered during cleanup
#[derive(Debug, Clone)]
pub struct CleanupError {
    /// Path that failed to be removed
    pub path: PathBuf,
    /// Error message describing what went wrong
    pub error: String,
}

/// Summary of cleanup operations for display
#[derive(Debug)]
pub struct CleanupSummary {
    /// Number of profiles removed
    pub profile_count: usize,
    /// Total size of removed data in bytes
    pub total_size: u64,
    /// Directories that were removed
    #[allow(dead_code)]
    pub directories: Vec<PathBuf>,
}

impl CleanupReport {
    /// Create a new empty cleanup report
    fn new() -> Self {
        Self {
            removed_files: Vec::new(),
            removed_dirs: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Check if cleanup was fully successful (no errors)
    pub fn is_successful(&self) -> bool {
        self.errors.is_empty()
    }

    /// Count total items removed
    pub fn total_removed(&self) -> usize {
        self.removed_files.len() + self.removed_dirs.len()
    }
}

/// Execute complete cleanup of zprof files and directories
pub fn cleanup_all(config: &CleanupConfig) -> Result<CleanupReport> {
    let mut report = CleanupReport::new();

    // Create progress spinner
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.cyan} {msg}")
            .expect("Failed to set progress style"),
    );

    // Step 1: Remove zprof-generated .zshenv
    spinner.set_message("Removing zprof-generated .zshenv...");
    if let Err(e) = remove_zprof_zshenv(&config.home_dir, &mut report) {
        report.errors.push(CleanupError {
            path: config.home_dir.join(".zshenv"),
            error: format!("Failed to remove .zshenv: {}", e),
        });
    }
    spinner.tick();

    // Step 2: Remove profiles directory (or individual subdirectories if keep_backups)
    spinner.set_message("Removing profiles and configurations...");
    if let Err(e) = remove_profiles_dir(&config.profiles_dir, config.keep_backups, &mut report) {
        report.errors.push(CleanupError {
            path: config.profiles_dir.clone(),
            error: format!("Failed to remove profiles directory: {}", e),
        });
    }
    spinner.tick();

    spinner.finish_and_clear();

    // Display results
    display_cleanup_results(&report);

    Ok(report)
}

/// Remove zprof-generated .zshenv from HOME
pub fn remove_zprof_zshenv(home_dir: &Path, report: &mut CleanupReport) -> Result<()> {
    let zshenv = home_dir.join(".zshenv");

    if !zshenv.exists() {
        // Not an error - file doesn't exist
        return Ok(());
    }

    // Check if this is zprof's .zshenv by looking for ZDOTDIR marker
    match std::fs::read_to_string(&zshenv) {
        Ok(content) => {
            if content.contains("ZDOTDIR") && content.contains(".zsh-profiles") {
                // This is zprof's .zshenv - safe to remove
                std::fs::remove_file(&zshenv)
                    .context("Failed to remove zprof-generated .zshenv")?;
                report.removed_files.push(zshenv.clone());
                println!("  ✓ Removed zprof-generated .zshenv");
            } else {
                // Not zprof's .zshenv - preserve it
                println!("  ⓘ Preserved .zshenv (not created by zprof)");
            }
        }
        Err(e) => {
            // Can't read file - don't remove it to be safe
            return Err(anyhow::anyhow!("Cannot read .zshenv: {}", e));
        }
    }

    Ok(())
}

/// Remove profiles directory or selective subdirectories
pub fn remove_profiles_dir(
    profiles_dir: &Path,
    keep_backups: bool,
    report: &mut CleanupReport,
) -> Result<()> {
    if !profiles_dir.exists() {
        // Nothing to remove
        return Ok(());
    }

    if keep_backups {
        // Remove subdirectories individually, preserving backups/
        remove_profiles_subdirs(profiles_dir, report)?;
    } else {
        // Remove entire directory
        remove_directory_recursive(profiles_dir, report)?;
    }

    Ok(())
}

/// Remove individual subdirectories within profiles dir, keeping backups/
fn remove_profiles_subdirs(profiles_dir: &Path, report: &mut CleanupReport) -> Result<()> {
    let subdirs_to_remove = ["profiles", "shared", "cache"];

    for subdir_name in &subdirs_to_remove {
        let subdir = profiles_dir.join(subdir_name);
        if subdir.exists() {
            match std::fs::remove_dir_all(&subdir) {
                Ok(()) => {
                    report.removed_dirs.push(subdir.clone());
                    println!("  ✓ Removed {}/", subdir_name);
                }
                Err(e) => {
                    report.errors.push(CleanupError {
                        path: subdir.clone(),
                        error: format!("Failed to remove {}: {}", subdir_name, e),
                    });
                    eprintln!("  ✗ Failed to remove {}/: {}", subdir_name, e);
                }
            }
        }
    }

    // Remove config.toml
    let config_file = profiles_dir.join("config.toml");
    if config_file.exists() {
        match std::fs::remove_file(&config_file) {
            Ok(()) => {
                report.removed_files.push(config_file.clone());
                println!("  ✓ Removed config.toml");
            }
            Err(e) => {
                report.errors.push(CleanupError {
                    path: config_file.clone(),
                    error: format!("Failed to remove config.toml: {}", e),
                });
                eprintln!("  ✗ Failed to remove config.toml: {}", e);
            }
        }
    }

    println!("  ⓘ Preserved backups/ directory (--keep-backups flag set)");

    Ok(())
}

/// Remove a directory recursively, tracking all removed items
fn remove_directory_recursive(dir: &Path, report: &mut CleanupReport) -> Result<()> {
    std::fs::remove_dir_all(dir)
        .with_context(|| format!("Failed to remove directory: {}", dir.display()))?;

    report.removed_dirs.push(dir.to_path_buf());
    println!("  ✓ Removed {}", dir.display());

    Ok(())
}

/// Display cleanup results to the user
fn display_cleanup_results(report: &CleanupReport) {
    if report.is_successful() {
        println!("\n✓ Cleanup completed successfully");
        println!("  Removed {} items ({} files, {} directories)",
            report.total_removed(),
            report.removed_files.len(),
            report.removed_dirs.len()
        );
    } else {
        println!("\n⚠ Cleanup completed with errors");
        println!("  Removed {} items ({} files, {} directories)",
            report.total_removed(),
            report.removed_files.len(),
            report.removed_dirs.len()
        );
        println!("  {} error(s) encountered", report.errors.len());
        println!("\nErrors:");
        for error in &report.errors {
            println!("  ✗ {}: {}", error.path.display(), error.error);
        }
        println!("\nSome files could not be removed. You may need to remove them manually.");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_cleanup_report_new() {
        let report = CleanupReport::new();
        assert!(report.removed_files.is_empty());
        assert!(report.removed_dirs.is_empty());
        assert!(report.errors.is_empty());
        assert!(report.is_successful());
    }

    #[test]
    fn test_cleanup_report_with_errors() {
        let mut report = CleanupReport::new();
        report.errors.push(CleanupError {
            path: PathBuf::from("/test/path"),
            error: "test error".to_string(),
        });
        assert!(!report.is_successful());
        assert_eq!(report.total_removed(), 0);
    }

    #[test]
    fn test_cleanup_report_total_removed() {
        let mut report = CleanupReport::new();
        report.removed_files.push(PathBuf::from("/test/file1"));
        report.removed_files.push(PathBuf::from("/test/file2"));
        report.removed_dirs.push(PathBuf::from("/test/dir1"));
        assert_eq!(report.total_removed(), 3);
    }

    #[test]
    fn test_remove_zprof_zshenv_preserves_non_zprof_file() {
        let temp_dir = TempDir::new().unwrap();
        let home = temp_dir.path();
        let zshenv = home.join(".zshenv");

        // Create a non-zprof .zshenv
        fs::write(&zshenv, "export PATH=/usr/local/bin:$PATH\n").unwrap();

        let mut report = CleanupReport::new();
        let result = remove_zprof_zshenv(home, &mut report);

        // Should succeed without removing the file
        assert!(result.is_ok());
        assert!(zshenv.exists(), ".zshenv should still exist");
        assert!(report.removed_files.is_empty(), "Should not have removed any files");
    }

    #[test]
    fn test_remove_zprof_zshenv_removes_zprof_file() {
        let temp_dir = TempDir::new().unwrap();
        let home = temp_dir.path();
        let zshenv = home.join(".zshenv");

        // Create a zprof .zshenv with ZDOTDIR marker
        fs::write(&zshenv, "export ZDOTDIR=$HOME/.zsh-profiles/profiles/default\n").unwrap();

        let mut report = CleanupReport::new();
        let result = remove_zprof_zshenv(home, &mut report);

        // Should remove the file
        assert!(result.is_ok());
        assert!(!zshenv.exists(), ".zshenv should be removed");
        assert_eq!(report.removed_files.len(), 1);
        assert_eq!(report.removed_files[0], zshenv);
    }

    #[test]
    fn test_remove_zprof_zshenv_when_file_doesnt_exist() {
        let temp_dir = TempDir::new().unwrap();
        let home = temp_dir.path();

        let mut report = CleanupReport::new();
        let result = remove_zprof_zshenv(home, &mut report);

        // Should succeed silently
        assert!(result.is_ok());
        assert!(report.removed_files.is_empty());
        assert!(report.errors.is_empty());
    }

    #[test]
    fn test_remove_profiles_dir_completely() {
        let temp_dir = TempDir::new().unwrap();
        let profiles_dir = temp_dir.path().join(".zsh-profiles");

        // Create directory structure
        fs::create_dir_all(profiles_dir.join("profiles")).unwrap();
        fs::create_dir_all(profiles_dir.join("shared")).unwrap();
        fs::create_dir_all(profiles_dir.join("backups")).unwrap();
        fs::write(profiles_dir.join("config.toml"), "test").unwrap();

        let mut report = CleanupReport::new();
        let result = remove_profiles_dir(&profiles_dir, false, &mut report);

        // Should remove entire directory
        assert!(result.is_ok());
        assert!(!profiles_dir.exists(), "profiles_dir should be removed");
        assert_eq!(report.removed_dirs.len(), 1);
    }

    #[test]
    fn test_remove_profiles_dir_keep_backups() {
        let temp_dir = TempDir::new().unwrap();
        let profiles_dir = temp_dir.path().join(".zsh-profiles");

        // Create directory structure
        fs::create_dir_all(profiles_dir.join("profiles/work")).unwrap();
        fs::create_dir_all(profiles_dir.join("shared")).unwrap();
        fs::create_dir_all(profiles_dir.join("backups/pre-zprof")).unwrap();
        fs::write(profiles_dir.join("config.toml"), "test").unwrap();

        let mut report = CleanupReport::new();
        let result = remove_profiles_dir(&profiles_dir, true, &mut report);

        // Should keep backups/ but remove others
        assert!(result.is_ok());
        assert!(profiles_dir.exists(), "profiles_dir parent should still exist");
        assert!(profiles_dir.join("backups").exists(), "backups/ should be preserved");
        assert!(!profiles_dir.join("profiles").exists(), "profiles/ should be removed");
        assert!(!profiles_dir.join("shared").exists(), "shared/ should be removed");
        assert!(!profiles_dir.join("config.toml").exists(), "config.toml should be removed");
    }
}
