//! Backup manifest data structures for zprof
//!
//! This module defines the data structures used to track backups created
//! by zprof, particularly the pre-zprof backup created during initialization.
//!
//! The manifest provides:
//! - Metadata about when and how the backup was created
//! - Framework detection information
//! - File-level details including checksums for verification

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Complete backup manifest containing all backup metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupManifest {
    /// Metadata about the backup creation
    pub metadata: BackupMetadata,
    /// Detected zsh framework (if any)
    pub detected_framework: Option<DetectedFramework>,
    /// List of backed up files with their metadata
    pub files: Vec<BackedUpFile>,
}

impl BackupManifest {
    /// Create a new backup manifest
    pub fn new(
        zsh_version: String,
        os: String,
        zprof_version: String,
    ) -> Self {
        Self {
            metadata: BackupMetadata {
                created_at: Utc::now(),
                zsh_version,
                os,
                zprof_version,
            },
            detected_framework: None,
            files: Vec::new(),
        }
    }

    /// Add a file to the backup manifest
    pub fn add_file(&mut self, file: BackedUpFile) {
        self.files.push(file);
    }

    /// Set the detected framework information
    pub fn set_framework(&mut self, framework: DetectedFramework) {
        self.detected_framework = Some(framework);
    }

    /// Save the manifest to a TOML file
    pub fn save_to_file(&self, path: &std::path::Path) -> Result<()> {
        let toml_string = toml::to_string_pretty(self)
            .context("Failed to serialize backup manifest to TOML")?;

        std::fs::write(path, toml_string)
            .with_context(|| format!("Failed to write backup manifest to {}", path.display()))?;

        // Set permissions to 600 (owner read/write only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = std::fs::Permissions::from_mode(0o600);
            std::fs::set_permissions(path, permissions)
                .with_context(|| format!("Failed to set permissions on manifest at {}", path.display()))?;
        }

        Ok(())
    }

    /// Load a backup manifest from a TOML file
    pub fn load_from_file(path: &std::path::Path) -> Result<Self> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read backup manifest from {}", path.display()))?;

        toml::from_str(&content)
            .context("Failed to parse backup manifest TOML")
    }
}

/// Metadata about the backup creation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupMetadata {
    /// Timestamp when the backup was created
    pub created_at: DateTime<Utc>,
    /// Zsh version at backup time (e.g., "zsh 5.9")
    pub zsh_version: String,
    /// Operating system (e.g., "Darwin", "Linux")
    pub os: String,
    /// zprof version that created the backup
    pub zprof_version: String,
}

/// Information about a detected zsh framework
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedFramework {
    /// Framework name (e.g., "oh-my-zsh", "zimfw")
    pub name: String,
    /// Path to the framework installation
    pub path: PathBuf,
    /// Framework configuration files
    pub config_files: Vec<PathBuf>,
}

/// Information about a backed up file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackedUpFile {
    /// Path relative to HOME directory
    pub path: PathBuf,
    /// File size in bytes
    pub size: u64,
    /// SHA256 checksum (hex encoded)
    pub checksum: String,
    /// Unix permissions mode
    pub permissions: u32,
    /// Whether the file is a symlink
    pub is_symlink: bool,
    /// Target path if the file is a symlink
    pub symlink_target: Option<PathBuf>,
}

impl BackedUpFile {
    /// Create a BackedUpFile entry from a file path
    ///
    /// # Arguments
    ///
    /// * `path` - Path relative to HOME directory
    /// * `absolute_path` - Absolute path to the file for reading metadata
    pub fn from_path(path: PathBuf, absolute_path: &std::path::Path) -> Result<Self> {
        use sha2::{Digest, Sha256};
        use std::fs;
        use std::io::Read;

        // Get file metadata
        let metadata = fs::metadata(absolute_path)
            .with_context(|| format!("Failed to get metadata for {}", absolute_path.display()))?;

        let size = metadata.len();

        // Get Unix permissions
        #[cfg(unix)]
        let permissions = {
            use std::os::unix::fs::PermissionsExt;
            metadata.permissions().mode()
        };
        #[cfg(not(unix))]
        let permissions = 0o644; // Default on non-Unix systems

        // Check if symlink
        let is_symlink = metadata.file_type().is_symlink();
        let symlink_target = if is_symlink {
            Some(fs::read_link(absolute_path)
                .with_context(|| format!("Failed to read symlink target for {}", absolute_path.display()))?)
        } else {
            None
        };

        // Calculate SHA256 checksum
        let checksum = if !is_symlink {
            let mut file = fs::File::open(absolute_path)
                .with_context(|| format!("Failed to open file for checksum: {}", absolute_path.display()))?;

            let mut hasher = Sha256::new();
            let mut buffer = [0u8; 8192];

            loop {
                let count = file.read(&mut buffer)
                    .with_context(|| format!("Failed to read file for checksum: {}", absolute_path.display()))?;
                if count == 0 {
                    break;
                }
                hasher.update(&buffer[..count]);
            }

            format!("{:x}", hasher.finalize())
        } else {
            // For symlinks, checksum the target path string
            let target_str = symlink_target.as_ref().unwrap().to_string_lossy();
            let mut hasher = Sha256::new();
            hasher.update(target_str.as_bytes());
            format!("{:x}", hasher.finalize())
        };

        Ok(Self {
            path,
            size,
            checksum,
            permissions,
            is_symlink,
            symlink_target,
        })
    }

    /// Verify that the file matches the checksummed version
    /// This will be used in Story 3.7 for backup validation
    #[allow(dead_code)]
    pub fn verify_checksum(&self, absolute_path: &std::path::Path) -> Result<bool> {
        let current_file = Self::from_path(self.path.clone(), absolute_path)?;
        Ok(current_file.checksum == self.checksum)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_backup_manifest_creation() {
        let manifest = BackupManifest::new(
            "zsh 5.9".to_string(),
            "Darwin".to_string(),
            "0.1.0".to_string(),
        );

        assert_eq!(manifest.metadata.zsh_version, "zsh 5.9");
        assert_eq!(manifest.metadata.os, "Darwin");
        assert_eq!(manifest.metadata.zprof_version, "0.1.0");
        assert!(manifest.detected_framework.is_none());
        assert!(manifest.files.is_empty());
    }

    #[test]
    fn test_backup_manifest_save_and_load() {
        let temp_dir = TempDir::new().unwrap();
        let manifest_path = temp_dir.path().join("backup-manifest.toml");

        let mut manifest = BackupManifest::new(
            "zsh 5.9".to_string(),
            "Darwin".to_string(),
            "0.1.0".to_string(),
        );

        manifest.set_framework(DetectedFramework {
            name: "oh-my-zsh".to_string(),
            path: PathBuf::from("/home/user/.oh-my-zsh"),
            config_files: vec![PathBuf::from(".zshrc")],
        });

        manifest.save_to_file(&manifest_path).unwrap();
        assert!(manifest_path.exists());

        let loaded = BackupManifest::load_from_file(&manifest_path).unwrap();
        assert_eq!(loaded.metadata.zsh_version, "zsh 5.9");
        assert!(loaded.detected_framework.is_some());
        assert_eq!(loaded.detected_framework.unwrap().name, "oh-my-zsh");
    }

    #[test]
    fn test_backed_up_file_from_path() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join(".zshrc");
        fs::write(&test_file, "# test content\n").unwrap();

        let backed_up = BackedUpFile::from_path(
            PathBuf::from(".zshrc"),
            &test_file,
        ).unwrap();

        assert_eq!(backed_up.path, PathBuf::from(".zshrc"));
        assert_eq!(backed_up.size, 15); // "# test content\n" is 15 bytes
        assert!(!backed_up.checksum.is_empty());
        assert!(!backed_up.is_symlink);
        assert!(backed_up.symlink_target.is_none());
    }

    #[test]
    fn test_backed_up_file_checksum_verification() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join(".zshrc");
        fs::write(&test_file, "# test content\n").unwrap();

        let backed_up = BackedUpFile::from_path(
            PathBuf::from(".zshrc"),
            &test_file,
        ).unwrap();

        // Verify original file matches
        assert!(backed_up.verify_checksum(&test_file).unwrap());

        // Modify file and verify checksum fails
        fs::write(&test_file, "# modified content\n").unwrap();
        assert!(!backed_up.verify_checksum(&test_file).unwrap());
    }

    #[test]
    #[cfg(unix)]
    fn test_backed_up_file_preserves_permissions() {
        use std::os::unix::fs::PermissionsExt;

        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join(".zshrc");
        fs::write(&test_file, "# test\n").unwrap();

        // Set specific permissions
        let permissions = fs::Permissions::from_mode(0o600);
        fs::set_permissions(&test_file, permissions).unwrap();

        let backed_up = BackedUpFile::from_path(
            PathBuf::from(".zshrc"),
            &test_file,
        ).unwrap();

        // Check that permissions were captured (mode includes file type bits, so mask them)
        assert_eq!(backed_up.permissions & 0o777, 0o600);
    }
}
