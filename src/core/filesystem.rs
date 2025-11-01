use anyhow::{Context, Result};
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
}
