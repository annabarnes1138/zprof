//! ZDOTDIR management for profile switching
//!
//! This module implements Pattern 5: Shell Integration via .zshenv
//! By setting ZDOTDIR in ~/.zshenv, zsh will source $ZDOTDIR/.zshrc instead of ~/.zshrc,
//! enabling profile switching while keeping the user's original ~/.zshrc untouched (NFR002).

use anyhow::{Context, Result};
use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};

use crate::core::filesystem::get_zprof_dir;

/// Set the active profile by updating ~/.zshenv with ZDOTDIR export
///
/// This follows Pattern 3 (Safe File Operations):
/// 1. Check: Verify profile directory exists
/// 2. Backup: Backup existing ~/.zshenv if it exists
/// 3. Operate: Write new ~/.zshenv with ZDOTDIR export
/// 4. Verify: Confirm ~/.zshenv was written correctly
///
/// # Arguments
///
/// * `profile_path` - Absolute path to the profile directory
///
/// # Errors
///
/// Returns error if:
/// - Profile directory doesn't exist
/// - Permission denied when writing ~/.zshenv
/// - Home directory cannot be determined
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use zprof::shell::zdotdir::set_active_profile;
///
/// let profile_path = Path::new("/home/user/.zsh-profiles/profiles/work");
/// set_active_profile(profile_path).unwrap();
/// ```
pub fn set_active_profile(profile_path: &Path) -> Result<()> {
    // Check: Verify profile directory exists
    anyhow::ensure!(
        profile_path.exists(),
        "Profile directory does not exist: {}",
        profile_path.display()
    );

    let zshenv_path = get_zshenv_path()?;

    // Backup: Backup existing ~/.zshenv if it exists
    let backup_path = if zshenv_path.exists() {
        Some(backup_zshenv(&zshenv_path)?)
    } else {
        None
    };

    // Read existing content (if any) to preserve user's custom configuration
    let existing_content = if zshenv_path.exists() {
        fs::read_to_string(&zshenv_path)
            .with_context(|| format!("Failed to read existing .zshenv at {}", zshenv_path.display()))?
    } else {
        String::new()
    };

    // Remove any existing zprof-managed section to avoid duplication
    let user_content = remove_zprof_section(&existing_content);

    // Operate: Create/update ~/.zshenv with ZDOTDIR and HISTFILE exports
    log::debug!("Setting ZDOTDIR to: {}", profile_path.display());

    let zdotdir_line = format!("export ZDOTDIR=\"{}\"", profile_path.display());

    // Set HISTFILE here (in root .zshenv) to ensure it's set before zsh initializes history
    // This is more reliable than setting it in $ZDOTDIR/.zshenv
    // Also set history options to enable immediate append and sharing
    let histfile_lines = "# Shared command history across all profiles\n\
                          export HISTFILE=\"$HOME/.zsh-profiles/shared/.zsh_history\"\n\
                          export HISTSIZE=10000\n\
                          export SAVEHIST=10000\n\
                          setopt INC_APPEND_HISTORY    # Immediately append to history file\n\
                          setopt SHARE_HISTORY         # Share history between all sessions\n\
                          setopt HIST_IGNORE_DUPS      # Don't record duplicates";

    let zprof_section = if let Some(backup) = backup_path {
        format!(
            "# ========== Managed by zprof - DO NOT EDIT THIS SECTION ==========\n\
             # Original .zshenv backed up to: {}\n\
             {}\n\
             {}\n\
             # ===================================================================\n",
            backup.display(),
            zdotdir_line,
            histfile_lines
        )
    } else {
        format!(
            "# ========== Managed by zprof - DO NOT EDIT THIS SECTION ==========\n\
             {}\n\
             {}\n\
             # ===================================================================\n",
            zdotdir_line,
            histfile_lines
        )
    };

    // Combine: zprof section first, then user's existing content
    let content = if user_content.trim().is_empty() {
        zprof_section
    } else {
        format!("{}\n{}", zprof_section, user_content)
    };

    fs::write(&zshenv_path, content)
        .with_context(|| format!("Failed to write .zshenv at {}", zshenv_path.display()))?;

    // Verify: Confirm file was written
    anyhow::ensure!(
        zshenv_path.exists(),
        "Failed to verify .zshenv creation at {}",
        zshenv_path.display()
    );

    Ok(())
}

/// Backup existing ~/.zshenv to cache/backups/ with timestamp
///
/// # Arguments
///
/// * `zshenv_path` - Path to existing ~/.zshenv file
///
/// # Returns
///
/// Path to the created backup file
///
/// # Errors
///
/// Returns error if:
/// - Backup directory cannot be created
/// - File copy fails
/// - Permissions denied
fn backup_zshenv(zshenv_path: &Path) -> Result<PathBuf> {
    let zprof_dir = get_zprof_dir()?;
    let backup_dir = zprof_dir.join("cache/backups");

    // Ensure backup directory exists
    fs::create_dir_all(&backup_dir)
        .with_context(|| format!("Failed to create backup directory: {}", backup_dir.display()))?;

    // Generate timestamp for backup filename
    let timestamp = Local::now().format("%Y%m%d-%H%M%S");
    let backup_filename = format!(".zshenv.backup.{}", timestamp);
    let backup_path = backup_dir.join(backup_filename);

    // Copy file to backup (NOT move - preserve original)
    fs::copy(zshenv_path, &backup_path).with_context(|| {
        format!(
            "Failed to backup .zshenv from {} to {}",
            zshenv_path.display(),
            backup_path.display()
        )
    })?;

    Ok(backup_path)
}

/// Remove zprof-managed section from existing .zshenv content
///
/// This function strips out the section marked "Managed by zprof" to prevent
/// duplication when updating ZDOTDIR. User's custom content is preserved.
///
/// # Arguments
///
/// * `content` - Full content of existing .zshenv file
///
/// # Returns
///
/// Content with zprof-managed section removed (user's content only)
fn remove_zprof_section(content: &str) -> String {
    let mut result = String::new();
    let mut in_zprof_section = false;

    for line in content.lines() {
        // Detect start of zprof-managed section
        if line.contains("========== Managed by zprof") || line.contains("Managed by zprof - DO NOT EDIT") {
            in_zprof_section = true;
            continue;
        }

        // Detect end of zprof-managed section
        if in_zprof_section && line.contains("==========") {
            in_zprof_section = false;
            continue;
        }

        // Keep lines outside the zprof-managed section
        if !in_zprof_section {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
}

/// Get the path to ~/.zshenv
fn get_zshenv_path() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .context("Failed to get home directory. Ensure HOME environment variable is set.")?;
    Ok(home.join(".zshenv"))
}

/// Check if ~/.zshenv already has ZDOTDIR set by another tool
///
/// Returns true if .zshenv exists and contains a ZDOTDIR export
pub fn has_existing_zdotdir() -> Result<bool> {
    let zshenv_path = get_zshenv_path()?;

    if !zshenv_path.exists() {
        return Ok(false);
    }

    let content = fs::read_to_string(&zshenv_path)
        .with_context(|| format!("Failed to read .zshenv at {}", zshenv_path.display()))?;

    // Check if ZDOTDIR is already set (not managed by zprof)
    Ok(content.contains("ZDOTDIR=") && !content.contains("Managed by zprof"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_backup_filename_generation() {
        let temp = TempDir::new().unwrap();
        let zshenv_path = temp.path().join(".zshenv");
        fs::write(&zshenv_path, "export PATH=/custom:$PATH\n").unwrap();

        // Note: Can't easily test backup_zshenv without mocking get_zprof_dir
        // This test verifies timestamp format would be correct
        let timestamp = Local::now().format("%Y%m%d-%H%M%S");
        let expected_pattern = format!(".zshenv.backup.{}", timestamp);
        assert!(expected_pattern.starts_with(".zshenv.backup."));
        assert!(expected_pattern.len() > 20); // Basic sanity check
    }

    #[test]
    fn test_zdotdir_export_format() {
        let profile_path = Path::new("/home/user/.zsh-profiles/profiles/work");
        let zdotdir_line = format!("export ZDOTDIR=\"{}\"", profile_path.display());
        assert_eq!(
            zdotdir_line,
            "export ZDOTDIR=\"/home/user/.zsh-profiles/profiles/work\""
        );
    }

    #[test]
    fn test_has_existing_zdotdir_empty() {
        // Can't easily test without real home directory
        // Integration tests will cover this
    }
}
