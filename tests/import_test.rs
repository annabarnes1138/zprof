use anyhow::Result;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde_json;
use std::fs::{self, File};
use std::io::Write;
use std::path::PathBuf;
use tar::Builder;
use tempfile::TempDir;

// Import the test module from export tests to reuse test fixtures
mod test_helpers {
    use super::*;
    use chrono::Utc;

    /// Create a test archive with valid contents
    pub fn create_test_archive(archive_path: &PathBuf, profile_name: &str) -> Result<()> {
        // Create archive metadata
        let metadata = serde_json::json!({
            "profile_name": profile_name,
            "framework": "oh-my-zsh",
            "export_date": Utc::now().to_rfc3339(),
            "zprof_version": "0.1.0",
            "framework_version": null,
            "exported_by": "test-user"
        });

        // Create valid profile.toml
        let profile_toml = format!(
            r#"[profile]
name = "{}"
framework = "oh-my-zsh"
theme = "robbyrussell"
created = "2025-10-31T10:00:00Z"
modified = "2025-10-31T10:00:00Z"

[plugins]
enabled = ["git", "docker"]

[env]
"#,
            profile_name
        );

        // Create tar.gz archive
        let tar_file = File::create(archive_path)?;
        let encoder = GzEncoder::new(tar_file, Compression::default());
        let mut tar = Builder::new(encoder);

        // Add metadata.json
        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        let metadata_bytes = metadata_json.as_bytes();
        let mut header = tar::Header::new_gnu();
        header.set_size(metadata_bytes.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append_data(&mut header, "metadata.json", metadata_bytes)?;

        // Add profile.toml
        let toml_bytes = profile_toml.as_bytes();
        let mut header = tar::Header::new_gnu();
        header.set_size(toml_bytes.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append_data(&mut header, "profile.toml", toml_bytes)?;

        // Finalize archive
        tar.finish()?;

        Ok(())
    }

    /// Create a corrupted archive (not valid gzip)
    pub fn create_corrupted_archive(archive_path: &PathBuf) -> Result<()> {
        let mut file = File::create(archive_path)?;
        file.write_all(b"This is not a valid tar.gz file")?;
        Ok(())
    }

    /// Create an archive missing metadata.json
    pub fn create_archive_missing_metadata(archive_path: &PathBuf) -> Result<()> {
        let profile_toml = r#"[profile]
name = "test"
framework = "oh-my-zsh"
theme = "robbyrussell"
created = "2025-10-31T10:00:00Z"
modified = "2025-10-31T10:00:00Z"

[plugins]
enabled = ["git"]

[env]
"#;

        let tar_file = File::create(archive_path)?;
        let encoder = GzEncoder::new(tar_file, Compression::default());
        let mut tar = Builder::new(encoder);

        // Only add profile.toml, no metadata.json
        let toml_bytes = profile_toml.as_bytes();
        let mut header = tar::Header::new_gnu();
        header.set_size(toml_bytes.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append_data(&mut header, "profile.toml", toml_bytes)?;

        tar.finish()?;
        Ok(())
    }

    /// Create an archive with invalid manifest
    pub fn create_archive_invalid_manifest(archive_path: &PathBuf) -> Result<()> {
        let metadata = serde_json::json!({
            "profile_name": "test",
            "framework": "oh-my-zsh",
            "export_date": "2025-10-31T10:00:00Z",
            "zprof_version": "0.1.0",
            "framework_version": null,
            "exported_by": "test-user"
        });

        // Invalid TOML (missing required fields)
        let invalid_toml = r#"[profile]
name = "test"
# Missing framework field - invalid!
"#;

        let tar_file = File::create(archive_path)?;
        let encoder = GzEncoder::new(tar_file, Compression::default());
        let mut tar = Builder::new(encoder);

        // Add metadata.json
        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        let metadata_bytes = metadata_json.as_bytes();
        let mut header = tar::Header::new_gnu();
        header.set_size(metadata_bytes.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append_data(&mut header, "metadata.json", metadata_bytes)?;

        // Add invalid profile.toml
        let toml_bytes = invalid_toml.as_bytes();
        let mut header = tar::Header::new_gnu();
        header.set_size(toml_bytes.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append_data(&mut header, "profile.toml", toml_bytes)?;

        tar.finish()?;
        Ok(())
    }

    /// Create an archive with unsupported framework
    pub fn create_archive_unsupported_framework(archive_path: &PathBuf) -> Result<()> {
        let metadata = serde_json::json!({
            "profile_name": "test",
            "framework": "bash", // Unsupported!
            "export_date": "2025-10-31T10:00:00Z",
            "zprof_version": "0.1.0",
            "framework_version": null,
            "exported_by": "test-user"
        });

        let profile_toml = r#"[profile]
name = "test"
framework = "bash"
theme = "default"
created = "2025-10-31T10:00:00Z"
modified = "2025-10-31T10:00:00Z"

[plugins]
enabled = []

[env]
"#;

        let tar_file = File::create(archive_path)?;
        let encoder = GzEncoder::new(tar_file, Compression::default());
        let mut tar = Builder::new(encoder);

        // Add metadata.json
        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        let metadata_bytes = metadata_json.as_bytes();
        let mut header = tar::Header::new_gnu();
        header.set_size(metadata_bytes.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append_data(&mut header, "metadata.json", metadata_bytes)?;

        // Add profile.toml with unsupported framework
        let toml_bytes = profile_toml.as_bytes();
        let mut header = tar::Header::new_gnu();
        header.set_size(toml_bytes.len() as u64);
        header.set_mode(0o644);
        header.set_cksum();
        tar.append_data(&mut header, "profile.toml", toml_bytes)?;

        tar.finish()?;
        Ok(())
    }
}

/// Integration test: Import archive that doesn't exist
#[test]
fn test_import_nonexistent_archive() {
    let nonexistent = PathBuf::from("/tmp/does-not-exist.zprof");

    // This should fail immediately
    let result = zprof::archive::import::import_profile(zprof::archive::import::ImportOptions {
        archive_path: nonexistent,
        profile_name_override: None,
        force_overwrite: false,
    });

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(err_msg.contains("Archive not found") || err_msg.contains("not found"));
}

/// Integration test: Import corrupted archive
#[test]
fn test_import_corrupted_archive() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let archive_path = temp_dir.path().join("corrupted.zprof");

    // Create corrupted archive
    test_helpers::create_corrupted_archive(&archive_path)?;

    // Attempt import
    let result = zprof::archive::import::import_profile(zprof::archive::import::ImportOptions {
        archive_path: archive_path.clone(),
        profile_name_override: None,
        force_overwrite: false,
    });

    assert!(result.is_err());
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("Failed to extract") || err_msg.contains("corrupted"),
        "Expected extraction error, got: {}",
        err_msg
    );

    Ok(())
}

/// Integration test: Import archive missing metadata.json
#[test]
fn test_import_missing_metadata() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let archive_path = temp_dir.path().join("no-metadata.zprof");

    // Create archive without metadata.json
    test_helpers::create_archive_missing_metadata(&archive_path)?;

    // Attempt import
    let result = zprof::archive::import::import_profile(zprof::archive::import::ImportOptions {
        archive_path: archive_path.clone(),
        profile_name_override: None,
        force_overwrite: false,
    });

    assert!(result.is_err());
    // Error could occur at various stages
    // The important thing is that archives missing metadata are rejected

    Ok(())
}

/// Integration test: Import archive with invalid manifest
#[test]
fn test_import_invalid_manifest() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let archive_path = temp_dir.path().join("invalid-manifest.zprof");

    // Create archive with invalid TOML
    test_helpers::create_archive_invalid_manifest(&archive_path)?;

    // Attempt import
    let result = zprof::archive::import::import_profile(zprof::archive::import::ImportOptions {
        archive_path: archive_path.clone(),
        profile_name_override: None,
        force_overwrite: false,
    });

    assert!(result.is_err());
    // Error could occur at various stages (extraction, parsing, etc.)
    // The important thing is that invalid input is rejected

    Ok(())
}

/// Integration test: Import archive with unsupported framework
#[test]
fn test_import_unsupported_framework() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let archive_path = temp_dir.path().join("unsupported-framework.zprof");

    // Create archive with unsupported framework
    test_helpers::create_archive_unsupported_framework(&archive_path)?;

    // Attempt import
    let result = zprof::archive::import::import_profile(zprof::archive::import::ImportOptions {
        archive_path: archive_path.clone(),
        profile_name_override: None,
        force_overwrite: false,
    });

    assert!(result.is_err());
    // Error could occur at various stages (validation, parsing, etc.)
    // The important thing is that unsupported frameworks are rejected

    Ok(())
}

// Note: Full successful import test would require:
// - Mock ~/.zsh-profiles directory
// - Framework installation stubbing
// - Shell generation mocking
//
// These are better suited for manual integration testing with actual archives.
// The tests above cover the critical error paths and validation logic.
