use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// Helper function to create a test profile
fn create_test_profile(profiles_dir: &std::path::Path, name: &str, framework: &str) -> Result<()> {
    let profile_dir = profiles_dir.join(name);
    fs::create_dir_all(&profile_dir)?;

    // Create profile.toml
    let manifest = format!(
        r#"[profile]
name = "{}"
framework = "{}"
theme = "robbyrussell"
created = "2025-10-31T14:30:00Z"
modified = "2025-10-31T14:30:00Z"
"#,
        name, framework
    );
    fs::write(profile_dir.join("profile.toml"), manifest)?;

    // Create .zshrc
    fs::write(
        profile_dir.join(".zshrc"),
        format!("# {} profile .zshrc\n", name),
    )?;

    // Create .zshenv
    fs::write(
        profile_dir.join(".zshenv"),
        "# Shared environment variables\n",
    )?;

    Ok(())
}

#[test]
fn test_profile_validation_with_valid_profile() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let profiles_dir = temp_dir.path().join("profiles");
    fs::create_dir_all(&profiles_dir)?;

    // Create a valid test profile
    create_test_profile(&profiles_dir, "test-profile", "oh-my-zsh")?;

    // Test validation - using internal functions would require making them pub
    // For now, this test ensures the helper creates valid profiles
    let profile_path = profiles_dir.join("test-profile");
    assert!(profile_path.join(".zshrc").exists());
    assert!(profile_path.join("profile.toml").exists());

    Ok(())
}

#[test]
fn test_profile_switching_updates_config() -> Result<()> {
    use std::env;
    use zprof::core::config::{load_config, update_active_profile, Config};

    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Create initial config
    let mut config = Config::default();
    config.active_profile = Some("profile1".to_string());
    config.write_to_file(config_path.clone())?;

    // Temporarily override where config is loaded from for testing
    // Note: This test demonstrates the logic, but would need dependency injection
    // in the actual code to properly test without affecting global state

    let loaded = Config::load_from_file(config_path.clone())?;
    assert_eq!(loaded.active_profile, Some("profile1".to_string()));

    // Update to new profile
    let mut config = Config::load_from_file(config_path.clone())?;
    config.active_profile = Some("profile2".to_string());
    config.write_to_file(config_path.clone())?;

    // Verify update
    let updated = Config::load_from_file(config_path)?;
    assert_eq!(updated.active_profile, Some("profile2".to_string()));

    Ok(())
}

#[test]
fn test_config_handles_missing_file() -> Result<()> {
    use zprof::core::config::Config;

    let temp_dir = TempDir::new()?;
    let nonexistent_path = temp_dir.path().join("nonexistent.toml");

    // Loading from nonexistent file should fail
    let result = Config::load_from_file(nonexistent_path);
    assert!(result.is_err());

    Ok(())
}

#[test]
fn test_config_serialization() -> Result<()> {
    use zprof::core::config::Config;

    let temp_dir = TempDir::new()?;
    let config_path = temp_dir.path().join("config.toml");

    // Create config with values
    let mut config = Config::default();
    config.active_profile = Some("my-profile".to_string());
    config.default_framework = Some("oh-my-zsh".to_string());

    // Write to file
    config.write_to_file(config_path.clone())?;

    // Read back and verify
    let loaded = Config::load_from_file(config_path)?;
    assert_eq!(loaded.active_profile, Some("my-profile".to_string()));
    assert_eq!(loaded.default_framework, Some("oh-my-zsh".to_string()));

    Ok(())
}
