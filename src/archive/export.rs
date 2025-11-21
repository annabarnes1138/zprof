//! Profile export functionality
//!
//! This module handles exporting profiles to portable .zprof archives (tar.gz format).
//! Archives contain the profile manifest and custom configuration files, excluding
//! framework binaries per the manifest-as-source-of-truth principle.

use anyhow::{bail, Context, Result};
use chrono::Utc;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use tar::Builder;

use crate::core::manifest;

/// Archive metadata structure
///
/// Serialized to JSON and included in archive as metadata.json
#[derive(Debug, Serialize, Deserialize)]
pub struct ArchiveMetadata {
    pub profile_name: String,
    pub framework: String,
    pub export_date: String, // ISO 8601 format
    pub zprof_version: String,
    pub framework_version: Option<String>,
    pub exported_by: String,
}

/// Export a profile to a .zprof archive
///
/// Creates a tar.gz archive containing the profile manifest, generated shell files,
/// and any custom configuration files. Framework binaries are excluded to keep
/// the archive size small and maintain portability.
///
/// # Arguments
///
/// * `profile_name` - Name of the profile to export
/// * `output_path` - Optional custom output path (defaults to `<profile-name>.zprof` in cwd)
///
/// # Returns
///
/// The path to the created archive file
///
/// # Errors
///
/// Returns error if:
/// - Profile doesn't exist
/// - Profile manifest is invalid
/// - Output file already exists (without --force)
/// - Insufficient disk space
/// - Permission denied
pub fn export_profile(profile_name: &str, output_path: Option<PathBuf>) -> Result<PathBuf> {
    log::info!("Exporting profile: {}", profile_name);

    // 1. Get profile directory
    let profile_dir = get_profile_dir(profile_name)?;
    if !profile_dir.exists() {
        bail!(
            "✗ Profile '{}' not found\n  → Run 'zprof list' to see available profiles",
            profile_name
        );
    }

    // 2. Load manifest for metadata and validation
    let manifest = manifest::load_and_validate(profile_name)
        .context("Cannot export profile with invalid manifest")?;

    // 3. Collect files to include
    let files_to_include = collect_files(&profile_dir)?;
    log::info!("Collected {} files for export", files_to_include.len());

    if files_to_include.is_empty() {
        log::warn!("Profile directory is empty, but will create archive anyway");
    }

    // 4. Create metadata
    let metadata = create_metadata(&manifest)?;

    // 5. Determine output path
    let archive_path = output_path.unwrap_or_else(|| {
        let cwd = std::env::current_dir().unwrap_or_default();
        cwd.join(format!("{}.zprof", profile_name))
    });

    // 6. Check if output file exists
    if archive_path.exists() {
        bail!(
            "✗ Archive already exists: {}\n  → Use --force to overwrite or specify a different path with --output",
            archive_path.display()
        );
    }

    // 7. Create tar.gz archive
    create_archive(&archive_path, &profile_dir, &files_to_include, &metadata)
        .context("Failed to create archive")?;

    // 8. Validate archive
    validate_archive(&archive_path).context("Archive validation failed")?;

    log::info!("Export completed: {:?}", archive_path);
    Ok(archive_path)
}

/// Get the profile directory path
fn get_profile_dir(profile_name: &str) -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;

    Ok(home
        .join(".zsh-profiles")
        .join("profiles")
        .join(profile_name))
}

/// Collect files to include in the archive
///
/// Includes:
/// - profile.toml (required manifest)
/// - .zshrc (generated file, for reference)
/// - .zshenv (generated file, for reference)
/// - Any custom configuration files
///
/// Excludes:
/// - Framework installations (.oh-my-zsh/, .zimfw/, etc.)
/// - Cache and temporary files (*.tmp, *.cache, *.log)
/// - Editor backup files (.swp, .swo, *~)
fn collect_files(profile_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    // Required file
    let manifest_path = profile_dir.join("profile.toml");
    if !manifest_path.exists() {
        bail!("profile.toml not found in profile directory");
    }
    files.push(manifest_path);

    // Optional generated files (for reference)
    let zshrc_path = profile_dir.join(".zshrc");
    if zshrc_path.exists() {
        files.push(zshrc_path);
    }

    let zshenv_path = profile_dir.join(".zshenv");
    if zshenv_path.exists() {
        files.push(zshenv_path);
    }

    // Walk directory for custom files
    for entry in fs::read_dir(profile_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Skip directories (framework installations)
        if path.is_dir() {
            log::debug!("Excluding directory: {:?}", path);
            continue;
        }

        // Skip already-added files
        if files.contains(&path) {
            continue;
        }

        // Skip excluded patterns
        if should_exclude(&path) {
            log::debug!("Excluding file: {:?}", path);
            continue;
        }

        // Include custom file
        log::debug!("Including custom file: {:?}", path);
        files.push(path);
    }

    Ok(files)
}

/// Determine if a file should be excluded from the archive
fn should_exclude(path: &Path) -> bool {
    let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

    // Framework directories (shouldn't reach here as directories are filtered earlier)
    if filename.starts_with(".oh-my-zsh")
        || filename.starts_with(".zimfw")
        || filename.starts_with(".zprezto")
        || filename.starts_with(".zinit")
        || filename.starts_with(".zap")
    {
        return true;
    }

    // Cache and temporary files
    if filename.ends_with(".tmp")
        || filename.ends_with(".cache")
        || filename.ends_with(".log")
        || filename.ends_with(".swp")
        || filename.ends_with(".swo")
        || filename.ends_with("~")
    {
        return true;
    }

    false
}

/// Create archive metadata from manifest
fn create_metadata(manifest: &manifest::Manifest) -> Result<ArchiveMetadata> {
    let export_date = Utc::now().to_rfc3339();
    let zprof_version = env!("CARGO_PKG_VERSION").to_string();

    // Try to get username
    let exported_by = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());

    Ok(ArchiveMetadata {
        profile_name: manifest.profile.name.clone(),
        framework: manifest.profile.framework.clone(),
        export_date,
        zprof_version,
        framework_version: None, // TODO: Detect framework version if installed
        exported_by,
    })
}

/// Create the tar.gz archive
fn create_archive(
    archive_path: &Path,
    profile_dir: &Path,
    files: &[PathBuf],
    metadata: &ArchiveMetadata,
) -> Result<()> {
    // Create tar.gz file
    let tar_file = File::create(archive_path)
        .with_context(|| format!("Failed to create archive: {}", archive_path.display()))?;

    let encoder = GzEncoder::new(tar_file, Compression::default());
    let mut tar = Builder::new(encoder);

    // Add metadata.json
    let metadata_json = serde_json::to_string_pretty(&metadata)
        .context("Failed to serialize metadata")?;

    let metadata_bytes = metadata_json.as_bytes();
    let mut header = tar::Header::new_gnu();
    header.set_size(metadata_bytes.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();

    tar.append_data(&mut header, "metadata.json", metadata_bytes)
        .context("Failed to add metadata to archive")?;

    // Add profile files
    for file_path in files {
        // Get relative path within profile directory
        let relative_path = file_path
            .strip_prefix(profile_dir)
            .with_context(|| format!("File not in profile directory: {:?}", file_path))?;

        let mut file = File::open(file_path)
            .with_context(|| format!("Failed to open file: {:?}", file_path))?;

        tar.append_file(relative_path, &mut file)
            .with_context(|| format!("Failed to add file to archive: {:?}", file_path))?;

        log::debug!("Added to archive: {:?}", relative_path);
    }

    // Finalize archive
    tar.finish().context("Failed to finalize tar archive")?;

    Ok(())
}

/// Validate the created archive
fn validate_archive(archive_path: &Path) -> Result<()> {
    // Check archive exists and has reasonable size
    let metadata = fs::metadata(archive_path)
        .context("Archive file not found after creation")?;

    let size = metadata.len();

    if size == 0 {
        bail!("Archive is empty");
    }

    if size > 10 * 1024 * 1024 {
        // 10 MB
        log::warn!("Archive is larger than expected: {} bytes", size);
    }

    log::info!("Archive size: {} bytes", size);
    Ok(())
}

/// Format file size in human-readable format
pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;

    if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}

/// Count files in an archive
///
/// Used to display file count in success message
pub fn count_archive_files(archive_path: &Path) -> Result<usize> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let tar_file = File::open(archive_path)
        .with_context(|| format!("Failed to open archive: {}", archive_path.display()))?;
    let decoder = GzDecoder::new(tar_file);
    let mut archive = Archive::new(decoder);

    let count = archive.entries()?.count();
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_should_exclude_cache_files() {
        assert!(should_exclude(Path::new("test.tmp")));
        assert!(should_exclude(Path::new("test.cache")));
        assert!(should_exclude(Path::new("test.log")));
    }

    #[test]
    fn test_should_exclude_editor_files() {
        assert!(should_exclude(Path::new(".test.swp")));
        assert!(should_exclude(Path::new(".test.swo")));
        assert!(should_exclude(Path::new("test~")));
    }

    #[test]
    fn test_should_exclude_framework_dirs() {
        assert!(should_exclude(Path::new(".oh-my-zsh")));
        assert!(should_exclude(Path::new(".zimfw")));
        assert!(should_exclude(Path::new(".zprezto")));
        assert!(should_exclude(Path::new(".zinit")));
        assert!(should_exclude(Path::new(".zap")));
    }

    #[test]
    fn test_should_not_exclude_normal_files() {
        assert!(!should_exclude(Path::new("profile.toml")));
        assert!(!should_exclude(Path::new(".zshrc")));
        assert!(!should_exclude(Path::new("custom.sh")));
    }

    #[test]
    fn test_format_file_size_bytes() {
        assert_eq!(format_file_size(0), "0 bytes");
        assert_eq!(format_file_size(512), "512 bytes");
        assert_eq!(format_file_size(1023), "1023 bytes");
    }

    #[test]
    fn test_format_file_size_kb() {
        assert_eq!(format_file_size(1024), "1.00 KB");
        assert_eq!(format_file_size(2048), "2.00 KB");
        assert_eq!(format_file_size(2560), "2.50 KB");
    }

    #[test]
    fn test_format_file_size_mb() {
        assert_eq!(format_file_size(1024 * 1024), "1.00 MB");
        assert_eq!(format_file_size(2 * 1024 * 1024), "2.00 MB");
        assert_eq!(format_file_size(1024 * 1024 + 512 * 1024), "1.50 MB");
    }

    #[test]
    fn test_create_metadata() {
        use crate::core::manifest::{Manifest, PluginsSection, ProfileSection};
        use chrono::Utc;
        use std::collections::HashMap;

        let manifest = Manifest {
            profile: ProfileSection {
                name: "test".to_string(),
                framework: "oh-my-zsh".to_string(),
                prompt_mode: crate::core::manifest::PromptMode::FrameworkTheme {
                    theme: "robbyrussell".to_string(),
                },
                created: Utc::now(),
                modified: Utc::now(),
            },
            plugins: PluginsSection {
                enabled: vec!["git".to_string()],
            },
            env: HashMap::new(),
        };

        let metadata = create_metadata(&manifest).unwrap();

        assert_eq!(metadata.profile_name, "test");
        assert_eq!(metadata.framework, "oh-my-zsh");
        assert_eq!(metadata.zprof_version, env!("CARGO_PKG_VERSION"));
        assert!(!metadata.export_date.is_empty());
        assert!(!metadata.exported_by.is_empty());
    }
}
