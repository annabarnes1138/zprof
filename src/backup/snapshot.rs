//! Safety snapshot creation before uninstall
//!
//! Creates a final tarball backup of the entire .zsh-profiles directory
//! before any destructive operations during uninstall.

use anyhow::{Context, Result};
use flate2::write::GzEncoder;
use flate2::Compression;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs::{self, File};
use std::path::Path;
use tar::Builder;

/// Summary of the created safety backup
#[derive(Debug)]
pub struct SafetySummary {
    pub backup_path: std::path::PathBuf,
    pub backup_size: u64,
}

/// Create a final safety snapshot of the entire .zsh-profiles directory
///
/// This creates a tarball archive of all profiles, history, backups, and configuration
/// before uninstall proceeds. The backup is stored in .zsh-profiles/backups/ with a
/// timestamp-based filename.
///
/// # Arguments
/// * `profiles_dir` - Path to the .zsh-profiles directory to archive
/// * `output_path` - Path where the tarball should be created
///
/// # Returns
/// Size of the created tarball in bytes
///
/// # Errors
/// Returns error if:
/// - Insufficient disk space
/// - Permission denied
/// - Unable to read source files
/// - Unable to write tarball
pub fn create_final_snapshot(profiles_dir: &Path, output_path: &Path) -> Result<u64> {
    // Validate source directory exists
    if !profiles_dir.exists() {
        anyhow::bail!(
            "Profiles directory does not exist: {}",
            profiles_dir.display()
        );
    }

    // Create output directory if it doesn't exist
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)
            .with_context(|| format!("Failed to create output directory: {}", parent.display()))?;
    }

    // Calculate total size for progress tracking
    let total_size = calculate_archive_size(profiles_dir)
        .context("Failed to calculate archive size")?;

    // Create progress bar
    let progress = ProgressBar::new(total_size);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{bar:40.cyan/blue} {bytes}/{total_bytes} ({percent}%)")
            .expect("Valid template")
            .progress_chars("=>-"),
    );
    progress.set_message("Creating safety backup...");

    // Create the tarball
    create_tar_gz(profiles_dir, output_path, &progress)
        .with_context(|| format!("Failed to create tarball at {}", output_path.display()))?;

    progress.finish_with_message("✓ Safety backup created");

    // Get the final size of the created tarball
    let metadata = fs::metadata(output_path)
        .with_context(|| format!("Failed to read tarball metadata: {}", output_path.display()))?;

    Ok(metadata.len())
}

/// Calculate the total size of files to be archived
///
/// Recursively walks the directory tree and sums file sizes.
/// This is used to display accurate progress during backup creation.
fn calculate_archive_size(dir: &Path) -> Result<u64> {
    let mut total_size = 0u64;

    if dir.is_file() {
        return Ok(fs::metadata(dir)?.len());
    }

    // Recursively walk directory
    for entry in fs::read_dir(dir)
        .with_context(|| format!("Failed to read directory: {}", dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            total_size += calculate_archive_size(&path)?;
        } else {
            total_size += fs::metadata(&path)?.len();
        }
    }

    Ok(total_size)
}

/// Create a gzip-compressed tarball of a directory
///
/// # Arguments
/// * `source` - Directory to archive
/// * `output` - Path to write the .tar.gz file
/// * `progress` - Progress bar to update during archiving
fn create_tar_gz(source: &Path, output: &Path, progress: &ProgressBar) -> Result<()> {
    // Create the output file
    let tar_file = File::create(output)
        .with_context(|| format!("Failed to create output file: {}", output.display()))?;

    // Create gzip encoder
    let encoder = GzEncoder::new(tar_file, Compression::default());

    // Create tar builder
    let mut tar = Builder::new(encoder);

    // Get the directory name to use as the archive root
    let dir_name = source
        .file_name()
        .context("Source directory has no name")?;

    // Add all files to the archive
    add_directory_to_tar(&mut tar, source, dir_name.as_ref(), progress)?;

    // Finish writing the tar archive
    tar.into_inner()
        .context("Failed to finalize tar archive")?
        .finish()
        .context("Failed to finalize gzip compression")?;

    // Set permissions on the tarball (owner read/write only)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(output, permissions)
            .with_context(|| format!("Failed to set permissions on {}", output.display()))?;
    }

    Ok(())
}

/// Recursively add a directory to a tar archive
fn add_directory_to_tar<W: std::io::Write>(
    tar: &mut Builder<W>,
    source_dir: &Path,
    archive_path: &Path,
    progress: &ProgressBar,
) -> Result<()> {
    if source_dir.is_file() {
        // Add single file
        let mut file = File::open(source_dir)?;
        let metadata = fs::metadata(source_dir)?;
        tar.append_file(archive_path, &mut file)?;
        progress.inc(metadata.len());
        return Ok(());
    }

    // Add directory entry
    tar.append_dir(archive_path, source_dir)?;

    // Add all directory contents
    for entry in fs::read_dir(source_dir)
        .with_context(|| format!("Failed to read directory: {}", source_dir.display()))?
    {
        let entry = entry?;
        let path = entry.path();
        let file_name = entry.file_name();
        let archive_entry_path = archive_path.join(&file_name);

        if path.is_dir() {
            // Recursively add subdirectory
            add_directory_to_tar(tar, &path, &archive_entry_path, progress)?;
        } else if path.is_file() {
            // Add file
            let mut file = File::open(&path)
                .with_context(|| format!("Failed to open file: {}", path.display()))?;
            let metadata = fs::metadata(&path)?;

            tar.append_file(&archive_entry_path, &mut file)
                .with_context(|| format!("Failed to add file to archive: {}", path.display()))?;

            progress.inc(metadata.len());
        } else if path.is_symlink() {
            // Handle symlinks - we'll follow them and add the target file/dir instead
            // This is safer for backup/restore purposes than preserving the link
            let metadata = fs::metadata(&path)?;

            if metadata.is_file() {
                // Symlink points to a file - add the file content
                let mut file = File::open(&path)
                    .with_context(|| format!("Failed to open symlink target: {}", path.display()))?;

                tar.append_file(&archive_entry_path, &mut file)
                    .with_context(|| format!("Failed to add symlink file to archive: {}", path.display()))?;

                progress.inc(metadata.len());
            } else if metadata.is_dir() {
                // Symlink points to a directory - recursively add it
                add_directory_to_tar(tar, &path, &archive_entry_path, progress)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_calculate_archive_size_single_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        fs::write(&file_path, "hello world").unwrap();

        let size = calculate_archive_size(&file_path).unwrap();
        assert_eq!(size, 11); // "hello world" is 11 bytes
    }

    #[test]
    fn test_calculate_archive_size_directory() {
        let temp_dir = TempDir::new().unwrap();

        // Create some test files
        fs::write(temp_dir.path().join("file1.txt"), "12345").unwrap();
        fs::write(temp_dir.path().join("file2.txt"), "67890").unwrap();

        let subdir = temp_dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        fs::write(subdir.join("file3.txt"), "abc").unwrap();

        let size = calculate_archive_size(temp_dir.path()).unwrap();
        assert_eq!(size, 13); // 5 + 5 + 3 = 13 bytes
    }

    #[test]
    fn test_create_final_snapshot_creates_tarball() {
        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("profiles");
        let output_path = temp_dir.path().join("backup.tar.gz");

        // Create source directory with some content
        fs::create_dir(&source_dir).unwrap();
        fs::write(source_dir.join("config.toml"), "[test]").unwrap();

        let profiles_subdir = source_dir.join("profiles");
        fs::create_dir(&profiles_subdir).unwrap();
        fs::write(profiles_subdir.join("profile1.txt"), "profile data").unwrap();

        // Create snapshot
        let size = create_final_snapshot(&source_dir, &output_path).unwrap();

        // Verify tarball was created
        assert!(output_path.exists());
        assert!(size > 0);

        // Verify tarball is valid by checking metadata
        let metadata = fs::metadata(&output_path).unwrap();
        assert_eq!(metadata.len(), size);
    }

    #[test]
    fn test_create_final_snapshot_validates_source_exists() {
        let temp_dir = TempDir::new().unwrap();
        let nonexistent = temp_dir.path().join("nonexistent");
        let output_path = temp_dir.path().join("backup.tar.gz");

        let result = create_final_snapshot(&nonexistent, &output_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    #[cfg(unix)]
    fn test_tarball_permissions_are_600() {
        use std::os::unix::fs::PermissionsExt;

        let temp_dir = TempDir::new().unwrap();
        let source_dir = temp_dir.path().join("profiles");
        let output_path = temp_dir.path().join("backup.tar.gz");

        fs::create_dir(&source_dir).unwrap();
        fs::write(source_dir.join("test.txt"), "test").unwrap();

        create_final_snapshot(&source_dir, &output_path).unwrap();

        let metadata = fs::metadata(&output_path).unwrap();
        let permissions = metadata.permissions();
        assert_eq!(permissions.mode() & 0o777, 0o600);
    }

    #[test]
    fn test_safety_summary_fields() {
        let summary = SafetySummary {
            backup_path: PathBuf::from("/test/backup.tar.gz"),
            backup_size: 12345,
        };

        assert_eq!(summary.backup_path.to_str().unwrap(), "/test/backup.tar.gz");
        assert_eq!(summary.backup_size, 12345);
    }
}
