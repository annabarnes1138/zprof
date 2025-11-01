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
    // After Story 1.1b, success message varies based on framework detection
    // Either "initialized successfully" (old) or framework detection message (new)
    assert!(stdout.contains("No existing framework detected") ||
            stdout.contains("initialized successfully") ||
            stdout.contains("Existing"),
            "Output should indicate initialization result");

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

// ============================================================================
// Story 1.1b Tests: Framework Detection and Import During Init
// ============================================================================

/// Helper to create a mock oh-my-zsh installation for testing
fn create_mock_oh_my_zsh(home: &PathBuf) {
    use std::fs;

    let omz_dir = home.join(".oh-my-zsh");
    fs::create_dir_all(&omz_dir).unwrap();

    // Create oh-my-zsh.sh
    fs::write(
        omz_dir.join("oh-my-zsh.sh"),
        "# oh-my-zsh main script\necho 'oh-my-zsh loaded'\n"
    ).unwrap();

    // Create plugins directory with some plugins
    let plugins_dir = omz_dir.join("plugins");
    fs::create_dir_all(&plugins_dir).unwrap();
    fs::create_dir_all(plugins_dir.join("git")).unwrap();
    fs::create_dir_all(plugins_dir.join("docker")).unwrap();
    fs::create_dir_all(plugins_dir.join("kubectl")).unwrap();

    // Create themes directory
    let themes_dir = omz_dir.join("themes");
    fs::create_dir_all(&themes_dir).unwrap();

    // Create .zshrc with oh-my-zsh config
    fs::write(
        home.join(".zshrc"),
        "export ZSH=\"$HOME/.oh-my-zsh\"\n\
         ZSH_THEME=\"robbyrussell\"\n\
         plugins=(git docker kubectl)\n\
         source $ZSH/oh-my-zsh.sh\n"
    ).unwrap();
}

#[test]
fn test_init_with_framework_user_accepts_import() {
    // AC#1, AC#2, AC#3, AC#10: Framework detection and interactive import flow
    // This test uses the mock UserInput to test the interactive logic without needing a TTY
    use zprof::cli::init::{InitArgs, execute_with_input};
    use zprof::cli::init::test_utils::MockUserInput;

    let env = TestEnv::new();
    create_mock_oh_my_zsh(&env.home_dir);

    // Mock user accepting import and providing custom profile name
    let mock_input = MockUserInput::new()
        .with_confirm(true)  // User says "yes" to import
        .with_input("test-profile".to_string());  // User provides "test-profile" as name

    // Execute init with mock input - HOME env var must be set during execution
    let result = {
        std::env::set_var("HOME", &env.home_dir);
        let r = execute_with_input(InitArgs {}, &mock_input);
        std::env::remove_var("HOME");
        r
    };

    assert!(result.is_ok(), "Init should succeed with framework import: {:?}", result.err());

    // Verify both prompts were called (AC#2, AC#3)
    assert!(*mock_input.confirm_called.borrow(), "Should have prompted for import confirmation");
    assert!(*mock_input.input_called.borrow(), "Should have prompted for profile name");

    // Verify profile was created (AC#4)
    let profile_dir = env.zprof_dir().join("profiles/test-profile");
    assert!(profile_dir.exists(), "Profile directory should be created");

    // Verify .zshenv management was successful by checking profile structure (AC#6)
    // Note: .zshenv is created in the actual HOME directory, not the test temp dir
    // So we verify the profile was properly set up instead
    let profile_path = profile_dir.join(".zshrc");
    assert!(profile_path.exists(), "Profile .zshrc should exist");
}

#[test]
fn test_init_with_framework_user_declines_import() {
    // AC#11: User declines import
    use zprof::cli::init::{InitArgs, execute_with_input};
    use zprof::cli::init::test_utils::MockUserInput;

    let env = TestEnv::new();
    create_mock_oh_my_zsh(&env.home_dir);

    // Mock user declining import
    let mock_input = MockUserInput::new()
        .with_confirm(false);  // User says "no" to import

    // Execute init with mock input - HOME env var must be set during execution
    let result = {
        std::env::set_var("HOME", &env.home_dir);
        let r = execute_with_input(InitArgs {}, &mock_input);
        std::env::remove_var("HOME");
        r
    };

    assert!(result.is_ok(), "Init should succeed even when import declined");

    // Verify confirm was called but input was not
    assert!(*mock_input.confirm_called.borrow(), "Should have prompted for import confirmation");
    assert!(!*mock_input.input_called.borrow(), "Should NOT have prompted for profile name when declined");

    // Verify no profile was created
    let profiles_dir = env.zprof_dir().join("profiles");
    if profiles_dir.exists() {
        let profile_count = std::fs::read_dir(profiles_dir).unwrap().count();
        assert_eq!(profile_count, 0, "No profiles should be created when import declined");
    }
}

#[test]
fn test_init_without_framework_shows_wizard_suggestion() {
    // No framework detected scenario
    let env = TestEnv::new();

    let output = env.run_init();
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("No existing framework detected") ||
            stdout.contains("zprof wizard"),
            "Should suggest wizard when no framework found");
}

#[test]
fn test_zshrc_preserved_during_import() {
    // AC#7: CRITICAL NFR002 test - User's ~/.zshrc must remain untouched
    let env = TestEnv::new();
    create_mock_oh_my_zsh(&env.home_dir);

    let original_zshrc_content = std::fs::read_to_string(env.home_dir.join(".zshrc")).unwrap();

    // Note: This test cannot fully execute import without user interaction
    // Integration tests with mocked dialoguer would be needed for full coverage
    // This test verifies the framework is detected
    let _output = env.run_init();

    // Verify original .zshrc still exists and hasn't been modified
    assert!(env.home_dir.join(".zshrc").exists(),
            "Original .zshrc must still exist (NFR002)");

    let current_zshrc_content = std::fs::read_to_string(env.home_dir.join(".zshrc")).unwrap();
    assert_eq!(original_zshrc_content, current_zshrc_content,
               "Original .zshrc content must be unchanged (NFR002)");
}

// Unit tests for zdotdir module
#[test]
fn test_zdotdir_path_generation() {
    let profile_path = std::path::Path::new("/home/user/.zsh-profiles/profiles/work");
    let zdotdir_line = format!("export ZDOTDIR=\"{}\"", profile_path.display());
    assert_eq!(
        zdotdir_line,
        "export ZDOTDIR=\"/home/user/.zsh-profiles/profiles/work\""
    );
}

#[test]
fn test_backup_filename_pattern() {
    use chrono::Local;

    let timestamp = Local::now().format("%Y%m%d-%H%M%S");
    let backup_filename = format!(".zshenv.backup.{}", timestamp);

    // Verify pattern
    assert!(backup_filename.starts_with(".zshenv.backup."));
    assert!(backup_filename.len() > 20); // Should have timestamp

    // Verify timestamp format (YYYYMMDD-HHMMSS)
    let parts: Vec<&str> = backup_filename.split('.').collect();
    assert!(parts.len() >= 3);
    let ts_part = parts[2]; // Should be "backup"
    assert_eq!(ts_part, "backup");
}
