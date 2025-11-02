//! Profile import functionality
//!
//! This module handles importing profiles from .zprof archives (tar.gz format).
//! Archives are extracted to a temp directory, validated, then installed into
//! ~/.zsh-profiles/profiles/ with framework installation and shell regeneration.

use anyhow::{bail, ensure, Context, Result};
use flate2::read::GzDecoder;
use serde_json;
use std::fs::{self, File};
use std::io::{self, Write as IoWrite};
use std::path::{Path, PathBuf};
use tar::Archive;

use crate::archive::export::ArchiveMetadata;
use crate::core::manifest::Manifest;
use crate::shell::generator;

/// Import options for profile import
pub struct ImportOptions {
    pub archive_path: PathBuf,
    pub profile_name_override: Option<String>,
    pub force_overwrite: bool,
}

/// Import a profile from a .zprof archive
///
/// This function performs the complete import workflow:
/// 1. Extract archive to temporary directory
/// 2. Validate archive contents (metadata.json, profile.toml)
/// 3. Handle name conflicts (prompt or force overwrite)
/// 4. Create profile directory
/// 5. Copy files from archive
/// 6. Install framework and plugins
/// 7. Regenerate shell configuration
/// 8. Clean up temporary directory
///
/// # Arguments
///
/// * `options` - Import configuration options
///
/// # Returns
///
/// The final profile name (may differ from archive if renamed)
///
/// # Errors
///
/// Returns error if:
/// - Archive doesn't exist or is corrupted
/// - Archive validation fails
/// - User cancels on name conflict
/// - Framework installation fails
/// - Shell configuration generation fails
pub fn import_profile(options: ImportOptions) -> Result<String> {
    log::info!("Importing profile from: {:?}", options.archive_path);

    // 1. Verify archive exists
    ensure!(
        options.archive_path.exists(),
        "✗ Archive not found: {}\n  → Check the file path and try again",
        options.archive_path.display()
    );

    // 2. Create temp directory for extraction
    let temp_dir = create_temp_extraction_dir()?;
    log::info!("Extracting to temp dir: {:?}", temp_dir);

    // 3. Extract archive
    let extract_result = extract_archive(&options.archive_path, &temp_dir);
    if let Err(e) = extract_result {
        // Clean up temp dir on extraction failure
        let _ = fs::remove_dir_all(&temp_dir);
        return Err(e).context("Failed to extract archive");
    }

    // 4. Validate archive contents
    let metadata = match validate_archive_contents(&temp_dir) {
        Ok(m) => m,
        Err(e) => {
            // Clean up temp dir on validation failure
            let _ = fs::remove_dir_all(&temp_dir);
            return Err(e).context("Archive validation failed");
        }
    };

    println!("→ Found profile: {}", metadata.profile_name);
    println!("  Framework: {}", metadata.framework);
    println!("  Exported: {}", metadata.export_date);
    println!("  Exported by: {}", metadata.exported_by);
    println!();

    // 5. Determine final profile name
    let profile_name = options
        .profile_name_override
        .unwrap_or(metadata.profile_name.clone());

    // 6. Handle name conflicts
    let profile_name = match handle_name_conflict(&profile_name, options.force_overwrite) {
        Ok(name) => name,
        Err(e) => {
            // Clean up temp dir on conflict resolution failure
            let _ = fs::remove_dir_all(&temp_dir);
            return Err(e);
        }
    };

    // 7. Load manifest from temp directory
    let manifest_path = temp_dir.join("profile.toml");
    let mut manifest = match load_manifest_from_path(&manifest_path) {
        Ok(m) => m,
        Err(e) => {
            // Clean up temp dir on manifest load failure
            let _ = fs::remove_dir_all(&temp_dir);
            return Err(e).context("Failed to load manifest from archive");
        }
    };

    // Update manifest profile name to match final name
    manifest.profile.name = profile_name.clone();

    // 8. Create profile directory
    let profile_dir = get_profile_dir(&profile_name)?;
    if let Err(e) = fs::create_dir_all(&profile_dir) {
        // Clean up temp dir on profile creation failure
        let _ = fs::remove_dir_all(&temp_dir);
        return Err(e).with_context(|| {
            format!("Failed to create profile directory: {:?}", profile_dir)
        });
    }

    // 9. Copy files from temp to profile directory
    if let Err(e) = copy_profile_files(&temp_dir, &profile_dir) {
        // Clean up both temp dir and partial profile on copy failure
        let _ = fs::remove_dir_all(&temp_dir);
        let _ = fs::remove_dir_all(&profile_dir);
        return Err(e).context("Failed to copy profile files");
    }

    // 10. Install framework and plugins
    println!("→ Installing {} framework...", manifest.profile.framework);
    if let Err(e) = install_framework(&profile_dir, &manifest) {
        // Clean up both temp dir and partial profile on install failure
        let _ = fs::remove_dir_all(&temp_dir);
        let _ = fs::remove_dir_all(&profile_dir);
        return Err(e).context("Framework installation failed");
    }

    // 11. Regenerate shell configuration
    println!("→ Generating shell configuration...");
    if let Err(e) = generator::write_generated_files(&profile_name, &manifest) {
        // Clean up both temp dir and partial profile on generation failure
        let _ = fs::remove_dir_all(&temp_dir);
        let _ = fs::remove_dir_all(&profile_dir);
        return Err(e).context("Failed to generate shell configuration");
    }

    // 12. Clean up temp directory
    fs::remove_dir_all(&temp_dir).context("Failed to clean up temp directory")?;

    log::info!("Import completed successfully: {}", profile_name);
    Ok(profile_name)
}

/// Create temporary directory for archive extraction
fn create_temp_extraction_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;

    let temp_dir = home
        .join(".zsh-profiles")
        .join("cache")
        .join("import_temp")
        .join(format!("import_{}", chrono::Utc::now().timestamp()));

    fs::create_dir_all(&temp_dir).context("Failed to create temp extraction directory")?;

    Ok(temp_dir)
}

/// Extract tar.gz archive to destination directory
fn extract_archive(archive_path: &Path, dest_dir: &Path) -> Result<()> {
    let tar_file = File::open(archive_path).with_context(|| {
        format!(
            "✗ Failed to open archive: {}\n  → Ensure the file exists and you have read permissions",
            archive_path.display()
        )
    })?;

    let decoder = GzDecoder::new(tar_file);
    let mut archive = Archive::new(decoder);

    archive.unpack(dest_dir).with_context(|| {
        format!(
            "✗ Failed to unpack archive. Archive may be corrupted.\n  → Try re-downloading or re-creating the archive"
        )
    })?;

    Ok(())
}

/// Validate archive contents
///
/// Checks that required files exist and are valid:
/// - metadata.json must exist and parse correctly
/// - profile.toml must exist
fn validate_archive_contents(temp_dir: &Path) -> Result<ArchiveMetadata> {
    // Check metadata.json exists
    let metadata_path = temp_dir.join("metadata.json");
    ensure!(
        metadata_path.exists(),
        "✗ Invalid archive: metadata.json not found\n  → This may not be a valid .zprof archive"
    );

    // Parse metadata
    let metadata_json = fs::read_to_string(&metadata_path)
        .context("Failed to read metadata.json")?;

    let metadata: ArchiveMetadata = serde_json::from_str(&metadata_json).with_context(|| {
        "✗ Failed to parse metadata.json. Archive may be corrupted.\n  → Try re-downloading or re-creating the archive"
    })?;

    // Check profile.toml exists
    let manifest_path = temp_dir.join("profile.toml");
    ensure!(
        manifest_path.exists(),
        "✗ Invalid archive: profile.toml not found\n  → This may not be a valid .zprof archive"
    );

    Ok(metadata)
}

/// Load and validate manifest from arbitrary path
///
/// Similar to manifest::load_and_validate but works with any path
/// (used for temp directory manifests during import)
///
/// This function is public to allow reuse by GitHub import module
pub fn load_manifest_from_path(manifest_path: &Path) -> Result<Manifest> {
    ensure!(
        manifest_path.exists(),
        "Manifest not found at: {:?}",
        manifest_path
    );

    let toml_content = fs::read_to_string(manifest_path)
        .with_context(|| format!("Failed to read manifest: {:?}", manifest_path))?;

    let manifest: Manifest = toml::from_str(&toml_content).with_context(|| {
        format!(
            "✗ Failed to parse manifest TOML\n  → The profile.toml in the archive is invalid"
        )
    })?;

    // Validate framework is supported
    let supported_frameworks = ["oh-my-zsh", "zimfw", "prezto", "zinit", "zap"];
    ensure!(
        supported_frameworks.contains(&manifest.profile.framework.as_str()),
        "✗ Unsupported framework: {}\n  → Supported frameworks: {}",
        manifest.profile.framework,
        supported_frameworks.join(", ")
    );

    Ok(manifest)
}

/// Handle name conflict resolution
///
/// If profile already exists and not force mode:
/// - Prompt user for action: [R]ename, [O]verwrite, or [C]ancel
/// - Handle recursively for renamed profiles
///
/// Returns final profile name
pub fn handle_name_conflict(profile_name: &str, force: bool) -> Result<String> {
    let profile_dir = get_profile_dir(profile_name)?;

    if !profile_dir.exists() {
        // No conflict
        return Ok(profile_name.to_string());
    }

    if force {
        // Force overwrite - delete existing
        println!("⚠ Overwriting existing profile: {}", profile_name);
        fs::remove_dir_all(&profile_dir)
            .with_context(|| format!("Failed to remove existing profile: {}", profile_name))?;
        return Ok(profile_name.to_string());
    }

    // Prompt user
    println!("⚠ Profile '{}' already exists", profile_name);
    println!();
    let action = prompt_conflict_resolution()?;

    match action.as_str() {
        "r" | "rename" => {
            let new_name = prompt_new_name()?;
            // Recursive call to check new name
            handle_name_conflict(&new_name, force)
        }
        "o" | "overwrite" => {
            println!("→ Overwriting existing profile...");
            fs::remove_dir_all(&profile_dir)
                .with_context(|| format!("Failed to remove existing profile: {}", profile_name))?;
            Ok(profile_name.to_string())
        }
        "c" | "cancel" => {
            bail!("Import cancelled by user");
        }
        _ => unreachable!(),
    }
}

/// Prompt user for conflict resolution action
fn prompt_conflict_resolution() -> Result<String> {
    print!("  [R]ename, [O]verwrite, or [C]ancel? ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let choice = input.trim().to_lowercase();

    match choice.as_str() {
        "r" | "rename" => Ok("rename".to_string()),
        "o" | "overwrite" => Ok("overwrite".to_string()),
        "c" | "cancel" => Ok("cancel".to_string()),
        _ => {
            println!("  Invalid choice. Cancelling import.");
            Ok("cancel".to_string())
        }
    }
}

/// Prompt user for new profile name
fn prompt_new_name() -> Result<String> {
    print!("  Enter new profile name: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let name = input.trim().to_string();

    ensure!(!name.is_empty(), "Profile name cannot be empty");
    ensure!(
        name.chars()
            .all(|c| c.is_alphanumeric() || c == '-' || c == '_'),
        "Profile name must be alphanumeric (hyphens and underscores allowed)"
    );

    Ok(name)
}

/// Copy profile files from temp directory to profile directory
///
/// Copies:
/// - profile.toml (required)
/// - Any custom configuration files
///
/// Skips:
/// - .zshrc and .zshenv (will be regenerated)
/// - metadata.json (only used during import)
fn copy_profile_files(temp_dir: &Path, profile_dir: &Path) -> Result<()> {
    // Copy profile.toml
    let src_manifest = temp_dir.join("profile.toml");
    let dst_manifest = profile_dir.join("profile.toml");
    fs::copy(&src_manifest, &dst_manifest).context("Failed to copy profile.toml")?;
    log::info!("Copied: profile.toml");

    // Copy any custom files (skip .zshrc, .zshenv - will be regenerated)
    for entry in fs::read_dir(temp_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue; // Skip directories
        }

        let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Skip metadata and generated files
        if filename == "metadata.json"
            || filename == ".zshrc"
            || filename == ".zshenv"
            || filename == "profile.toml" // Already copied
        {
            continue;
        }

        // Copy custom file
        let dst_path = profile_dir.join(filename);
        fs::copy(&path, &dst_path)
            .with_context(|| format!("Failed to copy {}", filename))?;
        log::info!("Copied custom file: {}", filename);
    }

    Ok(())
}

/// Install framework and plugins per manifest
///
/// This is an integration point for framework installation.
/// For MVP, this is stubbed with a helpful message.
/// Future implementation will call framework-specific installation logic.
pub fn install_framework(_profile_dir: &Path, manifest: &Manifest) -> Result<()> {
    let framework = &manifest.profile.framework;

    // Framework installation integration point
    // Future: Call frameworks::install_framework(profile_dir, framework, &manifest.plugins)?;

    // Placeholder for MVP
    println!("  ℹ Framework installation not yet implemented");
    println!(
        "  → You'll need to manually install {} in this profile",
        framework
    );

    Ok(())
}

/// Get the profile directory path
fn get_profile_dir(profile_name: &str) -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;

    Ok(home
        .join(".zsh-profiles")
        .join("profiles")
        .join(profile_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_profile_dir() {
        let result = get_profile_dir("test-profile");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains(".zsh-profiles"));
        assert!(path.to_string_lossy().contains("profiles"));
        assert!(path.to_string_lossy().contains("test-profile"));
    }

    #[test]
    fn test_prompt_new_name_validation() {
        // Test name validation logic (without actual prompt)
        let valid_names = vec!["my-profile", "profile_123", "test-profile-1"];
        for name in valid_names {
            assert!(name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_'));
        }

        let invalid_names = vec!["my profile", "profile/test", "profile\\test", "test.profile"];
        for name in invalid_names {
            assert!(!name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '-' || c == '_'));
        }
    }
}
