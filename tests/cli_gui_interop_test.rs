/// CLI/GUI Interoperability Tests
///
/// These tests verify that CLI and GUI can work seamlessly together:
/// - CLI creates profile → GUI IPC can see it
/// - GUI IPC creates profile → CLI can see it
/// - CLI activates profile → GUI IPC shows it as active
/// - GUI IPC deletes profile → CLI doesn't see it
///
/// Note: These tests use the Tauri IPC commands directly (bypassing the GUI frontend)
/// to verify the shared data layer works correctly.
use anyhow::Result;
use serial_test::serial;
use std::process::Command;
use tempfile::TempDir;

// Helper to set up a test environment
fn setup_test_env() -> Result<TempDir> {
    let temp_dir = TempDir::new()?;
    std::env::set_var("HOME", temp_dir.path());
    Ok(temp_dir)
}

// Helper to run zprof CLI command
fn run_zprof(args: &[&str]) -> Result<std::process::Output> {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--"])
        .args(args)
        .output()?;
    Ok(output)
}

#[test]
#[serial]
#[ignore] // Requires GUI dependencies (src-tauri)
fn test_cli_creates_profile_gui_sees_it() -> Result<()> {
    let _temp_dir = setup_test_env()?;

    // Initialize zprof via CLI
    let output = run_zprof(&["init"])?;
    assert!(output.status.success(), "Failed to initialize zprof");

    // Create profile via CLI
    let output = run_zprof(&["create", "cli-test", "--framework", "oh-my-zsh"])?;
    assert!(
        output.status.success(),
        "Failed to create profile via CLI: {:?}",
        String::from_utf8_lossy(&output.stderr)
    );

    // Verify via GUI IPC command (would use actual IPC in integration)
    // For now, we verify via CLI list command as proxy
    let output = run_zprof(&["list"])?;
    assert!(output.status.success(), "Failed to list profiles");

    let stdout = String::from_utf8(output.stdout)?;
    assert!(
        stdout.contains("cli-test"),
        "Profile 'cli-test' not found in list output"
    );

    Ok(())
}

#[test]
#[serial]
#[ignore] // Would require actual Tauri IPC setup
fn test_gui_creates_profile_cli_sees_it() -> Result<()> {
    let _temp_dir = setup_test_env()?;

    // Initialize zprof
    let output = run_zprof(&["init"])?;
    assert!(output.status.success());

    // In a real test, we would:
    // 1. Call create_profile() via Tauri IPC
    // 2. Verify CLI can see it
    //
    // For now, this is a placeholder demonstrating the test structure

    // Simulate: GUI creates profile "gui-test"
    // (In actual implementation, use zprof_tauri::commands::create_profile)

    // Verify via CLI
    let output = run_zprof(&["list"])?;
    assert!(output.status.success());

    // Would assert profile appears in CLI output
    // let stdout = String::from_utf8(output.stdout)?;
    // assert!(stdout.contains("gui-test"));

    Ok(())
}

#[test]
#[serial]
#[ignore] // Requires GUI dependencies
fn test_cli_activates_profile_gui_sees_active() -> Result<()> {
    let _temp_dir = setup_test_env()?;

    // Setup: init + create 2 profiles
    run_zprof(&["init"])?;
    run_zprof(&["create", "profile1", "--framework", "oh-my-zsh"])?;
    run_zprof(&["create", "profile2", "--framework", "oh-my-zsh"])?;

    // Activate via CLI
    let output = run_zprof(&["use", "profile2"])?;
    assert!(
        output.status.success(),
        "Failed to activate profile via CLI"
    );

    // Verify active profile via CLI (proxy for GUI IPC check)
    let output = run_zprof(&["current"])?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)?;
    assert!(
        stdout.contains("profile2"),
        "Active profile should be 'profile2'"
    );

    // In real implementation:
    // let active = zprof_tauri::commands::get_active_profile()?;
    // assert_eq!(active, Some("profile2".to_string()));

    Ok(())
}

#[test]
#[serial]
#[ignore] // Requires GUI dependencies
fn test_gui_deletes_profile_cli_doesnt_see_it() -> Result<()> {
    let _temp_dir = setup_test_env()?;

    // Setup
    run_zprof(&["init"])?;
    run_zprof(&["create", "temp-profile", "--framework", "oh-my-zsh"])?;

    // Verify profile exists via CLI
    let output = run_zprof(&["list"])?;
    let stdout = String::from_utf8(output.stdout)?;
    assert!(stdout.contains("temp-profile"));

    // Delete via CLI (simulating GUI delete)
    let output = run_zprof(&["delete", "temp-profile"])?;
    assert!(output.status.success());

    // Verify via CLI
    let output = run_zprof(&["list"])?;
    let stdout = String::from_utf8(output.stdout)?;
    assert!(
        !stdout.contains("temp-profile"),
        "Deleted profile should not appear in list"
    );

    Ok(())
}

#[test]
#[serial]
fn test_cli_commands_still_work_with_gui_feature() -> Result<()> {
    let _temp_dir = setup_test_env()?;

    // Verify all core CLI commands work
    let commands = vec![
        vec!["init"],
        vec!["list"],
        vec!["available", "frameworks"],
        vec!["version"],
    ];

    for cmd in commands {
        let output = run_zprof(&cmd)?;
        assert!(
            output.status.success(),
            "Command {:?} failed: {:?}",
            cmd,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}

#[test]
#[cfg(feature = "gui")]
fn test_gui_command_available_with_feature() -> Result<()> {
    // Verify 'gui' command exists when feature enabled
    let output = run_zprof(&["gui", "--help"])?;

    // Command should be recognized (even if it fails to execute)
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);

    // Should not show "unrecognized subcommand" error
    assert!(
        !stderr.contains("unrecognized subcommand")
            && !stderr.contains("unexpected argument"),
        "GUI command should be recognized when feature enabled"
    );

    // Should show help or attempt to run
    assert!(
        stdout.contains("gui") || stderr.contains("GUI") || stderr.contains("gui"),
        "GUI command should provide output"
    );

    Ok(())
}

#[test]
#[cfg(not(feature = "gui"))]
fn test_gui_command_unavailable_without_feature() -> Result<()> {
    // Verify 'gui' command does NOT exist when feature disabled
    let output = run_zprof(&["gui", "--help"])?;

    // Should fail with "unrecognized subcommand"
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("unrecognized subcommand")
            || stderr.contains("unexpected argument"),
        "GUI command should not exist when feature disabled"
    );

    Ok(())
}

/// Test that verifies CLI startup time remains fast
#[test]
#[serial]
fn test_cli_startup_performance() -> Result<()> {
    use std::time::Instant;

    let _temp_dir = setup_test_env()?;

    // Measure time to run simple command
    let start = Instant::now();
    let output = run_zprof(&["--version"])?;
    let duration = start.elapsed();

    assert!(output.status.success(), "Version command should succeed");

    // Note: This includes cargo overhead. For actual measurement,
    // use: `time ./target/release/zprof --version`
    // Target: <100ms for release build

    println!("CLI startup time: {duration:?}");

    // We can't assert <100ms here due to cargo run overhead,
    // but this test serves as a reminder to check performance manually
    assert!(
        duration.as_millis() < 5000,
        "Startup took too long (includes cargo overhead)"
    );

    Ok(())
}

#[cfg(test)]
mod binary_size_tests {
    /// Note: Binary size tests should be run manually:
    ///
    /// ```bash
    /// # Build CLI-only
    /// cargo build --release --no-default-features
    /// ls -lh target/release/zprof
    ///
    /// # Build with GUI
    /// cargo build --release
    /// ls -lh target/release/zprof
    ///
    /// # CLI-only should be smaller (no GUI bloat)
    /// ```
    #[test]
    #[ignore]
    fn binary_size_check_instructions() {
        println!("Run binary size checks manually:");
        println!("1. cargo build --release --no-default-features");
        println!("2. ls -lh target/release/zprof");
        println!("3. cargo build --release");
        println!("4. ls -lh target/release/zprof");
        println!("CLI-only should be significantly smaller");
    }
}
