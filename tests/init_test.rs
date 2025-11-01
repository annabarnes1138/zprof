use std::path::PathBuf;
use std::process::Command;
use tempfile::TempDir;

/// Helper to set up a test environment with a temporary home directory
struct TestEnv {
    _temp_dir: TempDir,
    home_dir: PathBuf,
}

impl TestEnv {
    fn new() -> Self {
        let temp_dir = TempDir::new().unwrap();
        let home_dir = temp_dir.path().to_path_buf();
        Self {
            _temp_dir: temp_dir,
            home_dir,
        }
    }

    fn zprof_dir(&self) -> PathBuf {
        self.home_dir.join(".zsh-profiles")
    }

    fn run_init(&self) -> std::process::Output {
        Command::new(env!("CARGO_BIN_EXE_zprof"))
            .arg("init")
            .env("HOME", &self.home_dir)
            .output()
            .expect("Failed to execute zprof init")
    }
}

#[test]
fn test_fresh_initialization_creates_directories() {
    // AC#1: `zprof init` creates directory structure with subdirectories
    let env = TestEnv::new();

    let output = env.run_init();
    assert!(output.status.success(), "init command should succeed");

    let zprof_dir = env.zprof_dir();
    assert!(zprof_dir.exists(), "~/.zsh-profiles should exist");
    assert!(zprof_dir.join("profiles").exists(), "profiles/ should exist");
    assert!(zprof_dir.join("shared").exists(), "shared/ should exist");
    assert!(zprof_dir.join("cache").exists(), "cache/ should exist");
    assert!(zprof_dir.join("cache/backups").exists(), "cache/backups/ should exist");
    assert!(zprof_dir.join("cache/downloads").exists(), "cache/downloads/ should exist");
}

#[test]
fn test_shared_history_file_created() {
    // AC#2: Shared command history file .zsh_history is created in shared/
    let env = TestEnv::new();

    let output = env.run_init();
    assert!(output.status.success(), "init command should succeed");

    let history_file = env.zprof_dir().join("shared/.zsh_history");
    assert!(history_file.exists(), ".zsh_history should exist in shared/");
    assert!(history_file.is_file(), ".zsh_history should be a file");

    // Verify file permissions on Unix systems
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let metadata = std::fs::metadata(&history_file).unwrap();
        let mode = metadata.permissions().mode();
        // Check that permissions are 0600 (user read/write only)
        assert_eq!(mode & 0o777, 0o600, "History file should have 0600 permissions");
    }
}

#[test]
fn test_config_toml_created_with_defaults() {
    // AC#3: Global configuration file config.toml is created with sensible defaults
    let env = TestEnv::new();

    let output = env.run_init();
    assert!(output.status.success(), "init command should succeed");

    let config_file = env.zprof_dir().join("config.toml");
    assert!(config_file.exists(), "config.toml should exist");

    let content = std::fs::read_to_string(&config_file).unwrap();
    // Config should be valid TOML (may be empty since all fields are None and skipped)
    // Empty string is valid TOML
    let _: toml::Value = toml::from_str(&content).expect("config.toml should be valid TOML");
}

#[test]
fn test_success_message_output() {
    // AC#4: Command outputs success message confirming initialization
    let env = TestEnv::new();

    let output = env.run_init();
    assert!(output.status.success(), "init command should succeed");

    let stdout = String::from_utf8_lossy(&output.stdout);

    // Check for success indicators
    assert!(stdout.contains("✓"), "Output should contain success symbol");
    assert!(stdout.contains(".zsh-profiles"), "Output should mention directory path");
    assert!(stdout.contains("profiles/"), "Output should list profiles subdirectory");
    assert!(stdout.contains("shared/"), "Output should list shared subdirectory");
    assert!(stdout.contains("cache/"), "Output should list cache subdirectory");
    assert!(stdout.contains("initialized successfully"), "Output should confirm success");

    // Snapshot test for exact output format (normalize temp paths)
    let normalized = stdout.replace(env.home_dir.to_str().unwrap(), "$HOME");
    insta::assert_snapshot!("init_success_output", normalized);
}

#[test]
fn test_reinit_warns_without_corruption() {
    // AC#5: Running `zprof init` when already initialized warns user but does not corrupt data
    let env = TestEnv::new();

    // First initialization
    let output1 = env.run_init();
    assert!(output1.status.success(), "First init should succeed");

    // Create a test file to verify no corruption
    let test_file = env.zprof_dir().join("profiles/test.txt");
    std::fs::write(&test_file, "test data").unwrap();

    // Second initialization (re-init)
    let output2 = env.run_init();
    assert!(output2.status.success(), "Re-init should succeed (not error)");

    let stderr = String::from_utf8_lossy(&output2.stderr);

    // Check for warning message
    assert!(stderr.contains("Warning") || stderr.contains("already exists"),
            "Re-init should show warning");
    assert!(stderr.contains("→"), "Warning should use → symbol");

    // Verify no corruption - test file should still exist with original content
    assert!(test_file.exists(), "Existing files should not be deleted");
    let content = std::fs::read_to_string(&test_file).unwrap();
    assert_eq!(content, "test data", "Existing file content should not be modified");

    // Snapshot test for warning message
    insta::assert_snapshot!("reinit_warning_output", stderr.to_string());
}

#[test]
fn test_all_subdirectories_created() {
    // Comprehensive test to verify all expected subdirectories
    let env = TestEnv::new();

    let output = env.run_init();
    assert!(output.status.success(), "init command should succeed");

    let zprof_dir = env.zprof_dir();

    // Verify all top-level directories
    let expected_dirs = vec![
        "profiles",
        "shared",
        "cache",
        "cache/backups",
        "cache/downloads",
    ];

    for dir in expected_dirs {
        let dir_path = zprof_dir.join(dir);
        assert!(dir_path.exists(), "{} should exist", dir);
        assert!(dir_path.is_dir(), "{} should be a directory", dir);
    }
}
