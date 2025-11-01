use anyhow::{ensure, Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Get the zprof base directory path (~/.zsh-profiles/)
pub fn get_zprof_dir() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .context("Failed to get home directory. Ensure HOME environment variable is set.")?;
    Ok(home.join(".zsh-profiles"))
}

/// Check if zprof directory structure already exists
pub fn is_initialized() -> Result<bool> {
    let base_dir = get_zprof_dir()?;
    Ok(base_dir.exists())
}

/// Create a directory if it doesn't exist
pub fn create_directory<P: AsRef<Path>>(path: P) -> Result<()> {
    let path = path.as_ref();
    fs::create_dir_all(path)
        .with_context(|| format!("Failed to create directory: {}", path.display()))?;
    Ok(())
}

/// Create the complete zprof directory structure
pub fn create_zprof_structure() -> Result<PathBuf> {
    let base_dir = get_zprof_dir()?;

    // Create base directory
    create_directory(&base_dir)
        .context("Failed to create base zprof directory")?;

    // Create subdirectories
    let subdirs = ["profiles", "shared", "cache"];
    for subdir in &subdirs {
        let dir_path = base_dir.join(subdir);
        create_directory(&dir_path)
            .with_context(|| format!("Failed to create {} subdirectory", subdir))?;
    }

    // Create cache subdirectories
    create_directory(base_dir.join("cache/backups"))
        .context("Failed to create cache/backups subdirectory")?;
    create_directory(base_dir.join("cache/downloads"))
        .context("Failed to create cache/downloads subdirectory")?;

    Ok(base_dir)
}

/// Create the shared history file with appropriate permissions
pub fn create_shared_history() -> Result<PathBuf> {
    let base_dir = get_zprof_dir()?;
    let history_file = base_dir.join("shared/.zsh_history");

    // Create empty file
    fs::write(&history_file, "")
        .with_context(|| format!("Failed to create history file at {}", history_file.display()))?;

    // Set permissions to 0600 (user read/write only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(&history_file, permissions)
            .with_context(|| format!("Failed to set permissions on history file at {}", history_file.display()))?;
    }

    Ok(history_file)
}

/// Recursively copy a directory and all its contents
///
/// This follows Pattern 3: Safe File Operations with the Check -> Backup -> Operate -> Verify flow.
/// Critically important for NFR002 compliance - uses copy NOT move to preserve originals.
///
/// # Arguments
///
/// * `source` - Source directory path to copy from
/// * `dest` - Destination directory path to copy to
///
/// # Errors
///
/// Returns error if:
/// - Source does not exist
/// - Permission denied during copy
/// - Disk space exhausted
/// - Original files are missing after copy (safety check)
///
/// # Examples
///
/// ```no_run
/// use std::path::Path;
/// use zprof::core::filesystem::copy_dir_recursive;
///
/// let source = Path::new("/home/user/.oh-my-zsh");
/// let dest = Path::new("/home/user/.zsh-profiles/profiles/work/.oh-my-zsh");
/// copy_dir_recursive(source, dest).unwrap();
/// ```
pub fn copy_dir_recursive(source: &Path, dest: &Path) -> Result<()> {
    // Check: Verify source exists
    ensure!(
        source.exists(),
        "Source directory does not exist: {}",
        source.display()
    );

    // Create destination directory
    fs::create_dir_all(dest)
        .with_context(|| format!("Failed to create destination directory: {}", dest.display()))?;

    // Operate: Copy directory contents recursively
    for entry in fs::read_dir(source)
        .with_context(|| format!("Failed to read source directory: {}", source.display()))?
    {
        let entry = entry.with_context(|| {
            format!("Failed to read directory entry in: {}", source.display())
        })?;
        let file_type = entry
            .file_type()
            .with_context(|| format!("Failed to get file type for: {}", entry.path().display()))?;

        let source_path = entry.path();
        let dest_path = dest.join(entry.file_name());

        if file_type.is_dir() {
            // Recursively copy subdirectory
            copy_dir_recursive(&source_path, &dest_path).with_context(|| {
                format!(
                    "Failed to copy subdirectory from {} to {}",
                    source_path.display(),
                    dest_path.display()
                )
            })?;
        } else if file_type.is_file() {
            // Copy file (NOT move - preserving originals per NFR002)
            fs::copy(&source_path, &dest_path).with_context(|| {
                format!(
                    "Failed to copy file from {} to {}",
                    source_path.display(),
                    dest_path.display()
                )
            })?;
        } else if file_type.is_symlink() {
            // Copy symlink target
            let target = fs::read_link(&source_path).with_context(|| {
                format!("Failed to read symlink target: {}", source_path.display())
            })?;

            #[cfg(unix)]
            {
                std::os::unix::fs::symlink(&target, &dest_path).with_context(|| {
                    format!(
                        "Failed to create symlink from {} to {}",
                        dest_path.display(),
                        target.display()
                    )
                })?;
            }

            #[cfg(not(unix))]
            {
                // On non-Unix systems, copy the symlink target as a regular file
                if target.is_file() {
                    fs::copy(&target, &dest_path).with_context(|| {
                        format!(
                            "Failed to copy symlink target from {} to {}",
                            target.display(),
                            dest_path.display()
                        )
                    })?;
                }
            }
        }
    }

    // Verify: Ensure source still exists (sanity check for NFR002)
    ensure!(
        source.exists(),
        "Original source directory missing after copy! This should never happen. Source: {}",
        source.display()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_create_directory() {
        let temp = TempDir::new().unwrap();
        let test_dir = temp.path().join("test_dir");

        create_directory(&test_dir).unwrap();
        assert!(test_dir.exists());
        assert!(test_dir.is_dir());
    }

    #[test]
    fn test_create_nested_directory() {
        let temp = TempDir::new().unwrap();
        let nested_dir = temp.path().join("parent/child/grandchild");

        create_directory(&nested_dir).unwrap();
        assert!(nested_dir.exists());
        assert!(nested_dir.is_dir());
    }

    #[test]
    fn test_copy_dir_recursive() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("source");
        let dest = temp.path().join("dest");

        // Create source directory structure
        fs::create_dir_all(&source).unwrap();
        fs::write(source.join("file1.txt"), "content1").unwrap();
        fs::create_dir_all(source.join("subdir")).unwrap();
        fs::write(source.join("subdir/file2.txt"), "content2").unwrap();

        // Copy directory
        copy_dir_recursive(&source, &dest).unwrap();

        // Verify destination exists
        assert!(dest.exists());
        assert!(dest.join("file1.txt").exists());
        assert!(dest.join("subdir").exists());
        assert!(dest.join("subdir/file2.txt").exists());

        // Verify contents
        let content1 = fs::read_to_string(dest.join("file1.txt")).unwrap();
        assert_eq!(content1, "content1");
        let content2 = fs::read_to_string(dest.join("subdir/file2.txt")).unwrap();
        assert_eq!(content2, "content2");

        // CRITICAL: Verify source still exists (NFR002)
        assert!(source.exists());
        assert!(source.join("file1.txt").exists());
        assert!(source.join("subdir/file2.txt").exists());
    }

    #[test]
    fn test_copy_dir_recursive_source_not_exists() {
        let temp = TempDir::new().unwrap();
        let source = temp.path().join("nonexistent");
        let dest = temp.path().join("dest");

        let result = copy_dir_recursive(&source, &dest);
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Source directory does not exist"));
    }
}
