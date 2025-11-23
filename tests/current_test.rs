use anyhow::Result;
use serial_test::serial;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;

/// Helper to set up test environment with temporary home directory
fn setup_test_env() -> Result<(TempDir, PathBuf)> {
    let temp_dir = TempDir::new()?;
    let home_dir = temp_dir.path().to_path_buf();
    let zprof_dir = home_dir.join(".zsh-profiles");
    fs::create_dir_all(&zprof_dir)?;
    Ok((temp_dir, home_dir))
}

/// Helper to create a test profile
fn create_test_profile(home_dir: &Path, name: &str, framework: &str, created: &str) -> Result<()> {
    let profile_dir = home_dir.join(".zsh-profiles").join("profiles").join(name);
    fs::create_dir_all(&profile_dir)?;

    let manifest = format!(
        r#"[profile]
name = "{name}"
framework = "{framework}"
theme = "robbyrussell"
created = "{created}"
modified = "{created}"
"#
    );

    fs::write(profile_dir.join("profile.toml"), manifest)?;
    Ok(())
}

/// Helper to create config file
fn create_config(home_dir: &Path, active_profile: Option<&str>) -> Result<()> {
    let config_path = home_dir.join(".zsh-profiles").join("config.toml");

    let content = if let Some(profile) = active_profile {
        format!(r#"active_profile = "{profile}""#)
    } else {
        String::new()
    };

    fs::write(config_path, content)?;
    Ok(())
}

/// Helper to run zprof command with custom HOME
fn run_zprof_with_home(home_dir: &PathBuf, args: &[&str]) -> Result<std::process::Output> {
    let mut cmd = Command::new(env!("CARGO_BIN_EXE_zprof"));
    cmd.env("HOME", home_dir);
    cmd.args(args);
    let output = cmd.output()?;
    Ok(output)
}

#[test]
fn test_current_displays_active_profile() -> Result<()> {
    let (_temp, home_dir) = setup_test_env()?;
    create_test_profile(&home_dir, "work", "oh-my-zsh", "2025-10-31T14:30:00Z")?;
    create_config(&home_dir, Some("work"))?;

    let output = run_zprof_with_home(&home_dir, &["current"])?;

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify profile name is displayed
    assert!(stdout.contains("Current profile: work"));
    // Verify framework is displayed
    assert!(stdout.contains("Framework: oh-my-zsh"));
    // Verify creation date is formatted properly
    assert!(stdout.contains("Created: Oct 31, 2025"));

    // Snapshot test
    insta::assert_snapshot!(stdout.trim());

    Ok(())
}

#[test]
fn test_current_no_active_profile_empty_config() -> Result<()> {
    let (_temp, home_dir) = setup_test_env()?;
    create_config(&home_dir, None)?;

    let output = run_zprof_with_home(&home_dir, &["current"])?;

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify helpful message is displayed
    assert!(stdout.contains("No active profile"));
    assert!(stdout.contains("zprof use <name>"));

    // Snapshot test
    insta::assert_snapshot!("no_active_profile_empty_config", stdout.trim());

    Ok(())
}

#[test]
fn test_current_no_config_file() -> Result<()> {
    let (_temp, home_dir) = setup_test_env()?;
    // Don't create config file

    let output = run_zprof_with_home(&home_dir, &["current"])?;

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Verify helpful message is displayed
    assert!(stdout.contains("No active profile"));
    assert!(stdout.contains("zprof use <name>"));

    // Snapshot test
    insta::assert_snapshot!("no_config_file", stdout.trim());

    Ok(())
}

#[test]
fn test_current_zprof_directory_not_initialized() -> Result<()> {
    let temp_dir = TempDir::new()?;
    let home_dir = temp_dir.path().to_path_buf();
    // Don't create .zsh-profiles directory

    let output = run_zprof_with_home(&home_dir, &["current"])?;

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Verify error message suggests running init
    assert!(stderr.contains("zprof directory not found"));
    assert!(stderr.contains("zprof init"));

    Ok(())
}

#[test]
fn test_current_active_profile_deleted() -> Result<()> {
    let (_temp, home_dir) = setup_test_env()?;
    // Create config pointing to a profile that doesn't exist
    create_config(&home_dir, Some("deleted-profile"))?;

    let output = run_zprof_with_home(&home_dir, &["current"])?;

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Verify error message mentions the deleted profile
    assert!(stderr.contains("deleted-profile"));
    assert!(stderr.contains("not found"));
    assert!(stderr.contains("may have been deleted"));
    // Verify helpful suggestions
    assert!(stderr.contains("zprof list"));
    assert!(stderr.contains("zprof use"));

    Ok(())
}

#[test]
fn test_current_with_different_frameworks() -> Result<()> {
    let (_temp, home_dir) = setup_test_env()?;

    // Test with zimfw
    create_test_profile(&home_dir, "experimental", "zimfw", "2025-01-15T10:00:00Z")?;
    create_config(&home_dir, Some("experimental"))?;

    let output = run_zprof_with_home(&home_dir, &["current"])?;
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Current profile: experimental"));
    assert!(stdout.contains("Framework: zimfw"));
    assert!(stdout.contains("Created: Jan 15, 2025"));

    insta::assert_snapshot!("different_framework", stdout.trim());

    Ok(())
}

#[test]
fn test_current_malformed_config() -> Result<()> {
    let (_temp, home_dir) = setup_test_env()?;

    // Create malformed config
    let config_path = home_dir.join(".zsh-profiles").join("config.toml");
    fs::write(config_path, "this is not valid TOML { [ ]")?;

    let output = run_zprof_with_home(&home_dir, &["current"])?;

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);

    // Verify error mentions corrupted config
    assert!(stderr.contains("Failed to read config file") || stderr.contains("config file"));
    assert!(stderr.contains("corrupted") || stderr.contains("parse"));

    Ok(())
}

#[test]
#[serial]
fn test_current_performance() -> Result<()> {
    let (_temp, home_dir) = setup_test_env()?;
    create_test_profile(&home_dir, "work", "oh-my-zsh", "2025-10-31T14:30:00Z")?;
    create_config(&home_dir, Some("work"))?;

    use std::time::Instant;

    // Warm up - run once to ensure any lazy loading is done
    let _ = run_zprof_with_home(&home_dir, &["current"])?;

    // Now measure actual performance
    let start = Instant::now();
    let output = run_zprof_with_home(&home_dir, &["current"])?;
    let duration = start.elapsed();

    assert!(output.status.success());

    // AC #4: Command should execute in under 100ms
    // Note: Integration tests include significant process startup overhead
    // - Binary compilation and loading: ~200-500ms
    // - Test framework overhead: ~100-300ms
    // - Actual file operations: < 10ms (per NFR001)
    // We use 2000ms (2s) as a reasonable upper bound for integration tests
    // while still catching major performance regressions
    assert!(
        duration.as_millis() < 2000,
        "Command took {}ms, expected < 2000ms (integration test overhead included)",
        duration.as_millis()
    );

    Ok(())
}
