use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Helper to get the compiled binary path
fn get_binary_path() -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("target");
    path.push("debug");
    path.push("zprof");
    path
}

/// Helper to create a test profile directory with manifest
fn create_test_profile(base_dir: &std::path::Path, name: &str, framework: &str) -> Result<()> {
    let profile_dir = base_dir
        .join(".zsh-profiles")
        .join("profiles")
        .join(name);
    fs::create_dir_all(&profile_dir)?;

    let manifest = format!(
        r#"[profile]
name = "{name}"
framework = "{framework}"
theme = "robbyrussell"
created = "2025-10-31T14:30:00Z"
modified = "2025-10-31T14:30:00Z"
"#
    );

    fs::write(profile_dir.join("profile.toml"), manifest)?;
    Ok(())
}

/// Helper to create config.toml with active profile
fn create_config(base_dir: &std::path::Path, active_profile: Option<&str>) -> Result<()> {
    let config_path = base_dir.join(".zsh-profiles").join("config.toml");

    let content = if let Some(active) = active_profile {
        format!(r#"active_profile = "{active}""#)
    } else {
        String::new()
    };

    fs::write(config_path, content)?;
    Ok(())
}

#[test]
fn test_list_multiple_profiles_with_active() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let zprof_dir = temp_dir.path().join(".zsh-profiles");
    fs::create_dir_all(zprof_dir.join("profiles"))?;

    // Create test profiles
    create_test_profile(temp_dir.path(), "work", "oh-my-zsh")?;
    create_test_profile(temp_dir.path(), "experimental", "zimfw")?;
    create_test_profile(temp_dir.path(), "minimal", "zinit")?;

    // Set active profile
    create_config(temp_dir.path(), Some("work"))?;

    // Run zprof list
    let output = Command::new(get_binary_path())
        .arg("list")
        .env("HOME", temp_dir.path())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify output
    assert!(output.status.success(), "Command should succeed");
    assert!(stdout.contains("Available profiles:"));
    assert!(stdout.contains("experimental"));
    assert!(stdout.contains("zimfw"));
    assert!(stdout.contains("minimal"));
    assert!(stdout.contains("zinit"));
    assert!(stdout.contains("work"));
    assert!(stdout.contains("oh-my-zsh"));

    // Verify active indicator (→) appears for work profile
    assert!(stdout.contains("→ work"));

    // Verify non-active profiles don't have indicator
    assert!(stdout.contains("  experimental") || stdout.contains("experimental"));
    assert!(stdout.contains("  minimal") || stdout.contains("minimal"));

    // Take snapshot
    insta::assert_snapshot!("list_multiple_profiles_with_active", stdout.trim());

    Ok(())
}

#[test]
fn test_list_empty_profiles_directory() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let zprof_dir = temp_dir.path().join(".zsh-profiles");
    fs::create_dir_all(zprof_dir.join("profiles"))?;

    // Run zprof list with empty profiles directory
    let output = Command::new(get_binary_path())
        .arg("list")
        .env("HOME", temp_dir.path())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify helpful message is shown
    assert!(output.status.success(), "Command should succeed even with empty directory");
    assert!(stdout.contains("No profiles found"));
    assert!(stdout.contains("zprof create"));

    // Take snapshot
    insta::assert_snapshot!("list_empty_profiles_directory", stdout.trim());

    Ok(())
}

#[test]
fn test_list_no_active_profile() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let zprof_dir = temp_dir.path().join(".zsh-profiles");
    fs::create_dir_all(zprof_dir.join("profiles"))?;

    // Create profiles but no active profile in config
    create_test_profile(temp_dir.path(), "work", "oh-my-zsh")?;
    create_test_profile(temp_dir.path(), "minimal", "zinit")?;
    create_config(temp_dir.path(), None)?;

    // Run zprof list
    let output = Command::new(get_binary_path())
        .arg("list")
        .env("HOME", temp_dir.path())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify output
    assert!(output.status.success());
    assert!(stdout.contains("Available profiles:"));
    assert!(stdout.contains("work"));
    assert!(stdout.contains("minimal"));

    // Verify no → indicator appears (no active profile)
    assert!(!stdout.contains("→"));

    // Take snapshot
    insta::assert_snapshot!("list_no_active_profile", stdout.trim());

    Ok(())
}

#[test]
fn test_list_alphabetical_sorting() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let zprof_dir = temp_dir.path().join(".zsh-profiles");
    fs::create_dir_all(zprof_dir.join("profiles"))?;

    // Create profiles in non-alphabetical order
    create_test_profile(temp_dir.path(), "zebra", "oh-my-zsh")?;
    create_test_profile(temp_dir.path(), "alpha", "zimfw")?;
    create_test_profile(temp_dir.path(), "middle", "zinit")?;
    create_config(temp_dir.path(), Some("alpha"))?;

    // Run zprof list
    let output = Command::new(get_binary_path())
        .arg("list")
        .env("HOME", temp_dir.path())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify alphabetical order
    assert!(output.status.success());
    let lines: Vec<&str> = stdout.lines().collect();

    // Find profile lines (skip header)
    let profile_lines: Vec<&str> = lines
        .iter()
        .filter(|l| l.contains("oh-my-zsh") || l.contains("zimfw") || l.contains("zinit"))
        .copied()
        .collect();

    // Verify order: alpha, middle, zebra
    assert!(profile_lines[0].contains("alpha"));
    assert!(profile_lines[1].contains("middle"));
    assert!(profile_lines[2].contains("zebra"));

    // Take snapshot
    insta::assert_snapshot!("list_alphabetical_sorting", stdout.trim());

    Ok(())
}

#[test]
fn test_list_missing_zprof_directory() -> Result<()> {
    let temp_dir = TempDir::new()?;

    // Don't create .zsh-profiles directory

    // Run zprof list
    let output = Command::new(get_binary_path())
        .arg("list")
        .env("HOME", temp_dir.path())
        .output()?;

    let stderr = String::from_utf8_lossy(&output.stderr);

    // Verify error message suggests running init
    assert!(!output.status.success(), "Command should fail");
    assert!(stderr.contains("zprof directory not found") || stderr.contains("zprof init"));

    Ok(())
}

#[test]
fn test_list_handles_missing_profile_toml() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let zprof_dir = temp_dir.path().join(".zsh-profiles");
    fs::create_dir_all(zprof_dir.join("profiles"))?;

    // Create valid profile
    create_test_profile(temp_dir.path(), "valid", "oh-my-zsh")?;

    // Create invalid profile (directory but no profile.toml)
    let invalid_dir = zprof_dir.join("profiles").join("invalid");
    fs::create_dir_all(&invalid_dir)?;

    create_config(temp_dir.path(), Some("valid"))?;

    // Run zprof list
    let output = Command::new(get_binary_path())
        .arg("list")
        .env("HOME", temp_dir.path())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Command should succeed but show warning
    assert!(output.status.success());
    assert!(stdout.contains("valid"));

    // Should show warning about missing profile.toml
    assert!(stderr.contains("Warning") || stderr.contains("missing profile.toml"));

    Ok(())
}

#[test]
fn test_list_handles_malformed_toml() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let zprof_dir = temp_dir.path().join(".zsh-profiles");
    fs::create_dir_all(zprof_dir.join("profiles"))?;

    // Create valid profile
    create_test_profile(temp_dir.path(), "valid", "oh-my-zsh")?;

    // Create profile with malformed TOML
    let malformed_dir = zprof_dir.join("profiles").join("malformed");
    fs::create_dir_all(&malformed_dir)?;
    fs::write(malformed_dir.join("profile.toml"), "this is not valid TOML!!!")?;

    create_config(temp_dir.path(), Some("valid"))?;

    // Run zprof list
    let output = Command::new(get_binary_path())
        .arg("list")
        .env("HOME", temp_dir.path())
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Command should succeed and show valid profile
    assert!(output.status.success());
    assert!(stdout.contains("valid"));

    // Should show warning about malformed TOML
    assert!(stderr.contains("Warning") || stderr.contains("Failed"));

    Ok(())
}
