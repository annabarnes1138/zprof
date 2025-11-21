use anyhow::Result;
use serial_test::serial;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

use zprof::archive::export;
use zprof::core::manifest::{Manifest, PluginsSection, ProfileSection};

/// Helper to create a test profile directory with manifest
fn create_test_profile(base_dir: &std::path::Path, name: &str) -> Result<PathBuf> {
    let profile_dir = base_dir
        .join(".zsh-profiles")
        .join("profiles")
        .join(name);
    fs::create_dir_all(&profile_dir)?;

    // Create manifest
    let manifest = Manifest {
        profile: ProfileSection {
            name: name.to_string(),
            framework: "oh-my-zsh".to_string(),
            prompt_mode: zprof::core::manifest::PromptMode::FrameworkTheme {
                theme: "robbyrussell".to_string(),
            },
            created: chrono::Utc::now(),
            modified: chrono::Utc::now(),
        },
        plugins: PluginsSection {
            enabled: vec!["git".to_string(), "docker".to_string()],
        },
        env: std::collections::HashMap::new(),
    };

    let toml = manifest.to_toml_string()?;
    fs::write(profile_dir.join("profile.toml"), toml)?;

    // Create generated files
    fs::write(profile_dir.join(".zshrc"), "# Generated zshrc\n")?;
    fs::write(profile_dir.join(".zshenv"), "# Generated zshenv\n")?;

    // Create a custom file
    fs::write(profile_dir.join("custom.sh"), "# Custom script\n")?;

    Ok(profile_dir)
}

#[test]
#[serial]
fn test_export_creates_archive() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let profile_name = "test-export";
    let original_home = std::env::var("HOME").ok();

    // Set HOME to temp directory
    std::env::set_var("HOME", temp_dir.path());

    // Create test profile
    create_test_profile(temp_dir.path(), profile_name)?;

    // Create output directory
    let output_dir = temp_dir.path().join("output");
    fs::create_dir(&output_dir)?;

    let output_path = output_dir.join(format!("{}.zprof", profile_name));

    // Export profile
    let archive_path = export::export_profile(profile_name, Some(output_path.clone()))?;

    // Verify archive was created
    assert!(archive_path.exists());
    assert_eq!(archive_path, output_path);

    // Verify archive is not empty
    let metadata = fs::metadata(&archive_path)?;
    assert!(metadata.len() > 0);

    // Restore original HOME
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    }

    Ok(())
}

#[test]
#[serial]
fn test_archive_contains_required_files() -> Result<()> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let temp_dir = TempDir::new()?;
    let profile_name = "test-contents";
    let original_home = std::env::var("HOME").ok();

    std::env::set_var("HOME", temp_dir.path());
    create_test_profile(temp_dir.path(), profile_name)?;

    let output_dir = temp_dir.path().join("output");
    fs::create_dir(&output_dir)?;
    let output_path = output_dir.join(format!("{}.zprof", profile_name));

    // Export profile
    let archive_path = export::export_profile(profile_name, Some(output_path))?;

    // Extract and verify contents
    let tar_file = fs::File::open(&archive_path)?;
    let decoder = GzDecoder::new(tar_file);
    let mut archive = Archive::new(decoder);

    let mut found_files = Vec::new();
    for entry in archive.entries()? {
        let entry = entry?;
        let path = entry.path()?.to_string_lossy().to_string();
        found_files.push(path);
    }

    // Verify required files
    assert!(found_files.contains(&"metadata.json".to_string()));
    assert!(found_files.contains(&"profile.toml".to_string()));
    assert!(found_files.contains(&".zshrc".to_string()));
    assert!(found_files.contains(&".zshenv".to_string()));
    assert!(found_files.contains(&"custom.sh".to_string()));

    // Restore original HOME
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    }

    Ok(())
}

#[test]
#[serial]
fn test_metadata_json_valid() -> Result<()> {
    use flate2::read::GzDecoder;
    use tar::Archive;
    use std::io::Read;

    let temp_dir = TempDir::new()?;
    let profile_name = "test-metadata";
    let original_home = std::env::var("HOME").ok();

    std::env::set_var("HOME", temp_dir.path());
    create_test_profile(temp_dir.path(), profile_name)?;

    let output_dir = temp_dir.path().join("output");
    fs::create_dir(&output_dir)?;
    let output_path = output_dir.join(format!("{}.zprof", profile_name));

    // Export profile
    let archive_path = export::export_profile(profile_name, Some(output_path))?;

    // Extract metadata.json
    let tar_file = fs::File::open(&archive_path)?;
    let decoder = GzDecoder::new(tar_file);
    let mut archive = Archive::new(decoder);

    for entry in archive.entries()? {
        let mut entry = entry?;
        let path = entry.path()?.to_string_lossy().to_string();

        if path == "metadata.json" {
            let mut contents = String::new();
            entry.read_to_string(&mut contents)?;

            // Parse as JSON
            let metadata: serde_json::Value = serde_json::from_str(&contents)?;

            // Verify required fields
            assert_eq!(metadata["profile_name"], profile_name);
            assert_eq!(metadata["framework"], "oh-my-zsh");
            assert!(metadata["export_date"].is_string());
            assert!(metadata["zprof_version"].is_string());
            assert!(metadata["exported_by"].is_string());

            // Restore HOME before returning
            if let Some(home) = original_home {
                std::env::set_var("HOME", home);
            }

            return Ok(());
        }
    }

    // Restore HOME before panicking
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    }

    panic!("metadata.json not found in archive");
}

#[test]
#[serial]
fn test_archive_excludes_framework_dirs() -> Result<()> {
    use flate2::read::GzDecoder;
    use tar::Archive;

    let temp_dir = TempDir::new()?;
    let profile_name = "test-exclusions";
    let original_home = std::env::var("HOME").ok();

    std::env::set_var("HOME", temp_dir.path());
    let profile_dir = create_test_profile(temp_dir.path(), profile_name)?;

    // Create framework directory (should be excluded)
    fs::create_dir(profile_dir.join(".oh-my-zsh"))?;
    fs::write(profile_dir.join(".oh-my-zsh").join("test.sh"), "framework file")?;

    // Create cache files (should be excluded)
    fs::write(profile_dir.join("test.tmp"), "temp file")?;
    fs::write(profile_dir.join("test.cache"), "cache file")?;
    fs::write(profile_dir.join("test.log"), "log file")?;

    let output_dir = temp_dir.path().join("output");
    fs::create_dir(&output_dir)?;
    let output_path = output_dir.join(format!("{}.zprof", profile_name));

    // Export profile
    let archive_path = export::export_profile(profile_name, Some(output_path))?;

    // Verify excluded files are not in archive
    let tar_file = fs::File::open(&archive_path)?;
    let decoder = GzDecoder::new(tar_file);
    let mut archive = Archive::new(decoder);

    let mut found_files = Vec::new();
    for entry in archive.entries()? {
        let entry = entry?;
        let path = entry.path()?.to_string_lossy().to_string();
        found_files.push(path);
    }

    // Verify excluded patterns
    assert!(!found_files.iter().any(|f| f.contains(".oh-my-zsh")));
    assert!(!found_files.iter().any(|f| f.ends_with(".tmp")));
    assert!(!found_files.iter().any(|f| f.ends_with(".cache")));
    assert!(!found_files.iter().any(|f| f.ends_with(".log")));

    // Restore original HOME
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    }

    Ok(())
}

#[test]
fn test_format_file_size() {
    assert_eq!(export::format_file_size(0), "0 bytes");
    assert_eq!(export::format_file_size(512), "512 bytes");
    assert_eq!(export::format_file_size(1024), "1.00 KB");
    assert_eq!(export::format_file_size(2048), "2.00 KB");
    assert_eq!(export::format_file_size(1024 * 1024), "1.00 MB");
    assert_eq!(export::format_file_size(2 * 1024 * 1024), "2.00 MB");
}

#[test]
#[serial]
fn test_count_archive_files() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let profile_name = "test-count";
    let original_home = std::env::var("HOME").ok();

    std::env::set_var("HOME", temp_dir.path());
    create_test_profile(temp_dir.path(), profile_name)?;

    let output_dir = temp_dir.path().join("output");
    fs::create_dir(&output_dir)?;
    let output_path = output_dir.join(format!("{}.zprof", profile_name));

    // Export profile
    let archive_path = export::export_profile(profile_name, Some(output_path))?;

    // Count files
    let count = export::count_archive_files(&archive_path)?;

    // Should have: metadata.json, profile.toml, .zshrc, .zshenv, custom.sh = 5 files
    assert_eq!(count, 5);

    // Restore original HOME
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    }

    Ok(())
}

#[test]
#[serial]
fn test_export_nonexistent_profile_fails() {
    let temp_dir = TempDir::new().unwrap();
    let original_home = std::env::var("HOME").ok();
    std::env::set_var("HOME", temp_dir.path());

    // Create zprof directory but no profiles
    fs::create_dir_all(temp_dir.path().join(".zsh-profiles").join("profiles")).unwrap();

    let output_dir = temp_dir.path().join("output");
    fs::create_dir(&output_dir).unwrap();
    let output_path = output_dir.join("nonexistent.zprof");

    let result = export::export_profile("nonexistent", Some(output_path));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("not found"));

    // Restore original HOME
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    }
}

#[test]
#[serial]
fn test_export_existing_file_fails() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let profile_name = "test-existing";
    let original_home = std::env::var("HOME").ok();

    std::env::set_var("HOME", temp_dir.path());
    create_test_profile(temp_dir.path(), profile_name)?;

    let output_dir = temp_dir.path().join("output");
    fs::create_dir(&output_dir)?;
    let output_path = output_dir.join(format!("{}.zprof", profile_name));

    // Create existing file
    fs::write(&output_path, "existing content")?;

    // Try to export (should fail)
    let result = export::export_profile(profile_name, Some(output_path));
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("already exists"));

    // Restore original HOME
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    }

    Ok(())
}
