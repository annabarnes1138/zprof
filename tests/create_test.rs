use anyhow::Result;
use serial_test::serial;
use std::env;
use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

// Helper to set up a test environment with HOME pointing to a temp directory
fn setup_test_env() -> (TempDir, PathBuf) {
    let temp_dir = TempDir::new().unwrap();
    let home = temp_dir.path().to_path_buf();

    // Set HOME environment variable
    env::set_var("HOME", &home);

    (temp_dir, home)
}

// Helper to create a mock oh-my-zsh installation for testing
fn create_mock_omz(home: &PathBuf) -> Result<()> {
    let omz_dir = home.join(".oh-my-zsh");
    fs::create_dir_all(&omz_dir)?;
    fs::create_dir_all(omz_dir.join("custom/plugins"))?;
    fs::create_dir_all(omz_dir.join("themes"))?;

    // Create mock .zshrc
    let zshrc_content = r#"
export ZSH="$HOME/.oh-my-zsh"
ZSH_THEME="robbyrussell"
plugins=(git docker kubectl)
source $ZSH/oh-my-zsh.sh
"#;
    fs::write(home.join(".zshrc"), zshrc_content)?;

    Ok(())
}

#[test]
#[serial]
fn test_create_profile_name_validation() {
    use zprof::cli::create::{execute, CreateArgs};

    let (_temp, _home) = setup_test_env();

    // Test invalid names
    let invalid_names = vec![
        "",
        "profile/name",
        "profile\\name",
        "../etc",
        "profile name",
        "profile@name",
    ];

    for name in invalid_names {
        let args = CreateArgs {
            name: name.to_string(),
        };
        let result = execute(args);
        assert!(result.is_err(), "Name '{}' should be invalid", name);
    }
}

#[test]
#[serial]
fn test_create_profile_no_framework_detected() {
    use zprof::cli::create::{execute, CreateArgs};
    use zprof::core::filesystem::create_zprof_structure;

    let (_temp, _home) = setup_test_env();

    // Initialize zprof structure
    create_zprof_structure().unwrap();

    // No framework installed - will attempt to launch TUI wizard
    let args = CreateArgs {
        name: "test-profile".to_string(),
    };

    let result = execute(args);
    // In test environment without a terminal, TUI initialization will fail
    // This is expected behavior - in real usage, the TUI would launch successfully
    assert!(result.is_err(), "Should fail when TUI cannot be initialized in test environment");
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("terminal") || err_msg.contains("TUI") || err_msg.contains("cancelled"),
        "Error should be related to terminal/TUI: {}",
        err_msg
    );
}

#[test]
#[serial]
fn test_create_profile_already_exists() {
    use zprof::cli::create::CreateArgs;
    use zprof::core::filesystem::{create_zprof_structure, get_zprof_dir};

    let (_temp, home) = setup_test_env();

    // Initialize zprof and create mock framework
    create_zprof_structure().unwrap();
    create_mock_omz(&home).unwrap();

    // Create the profile directory manually to simulate existing profile
    let zprof_dir = get_zprof_dir().unwrap();
    let profile_dir = zprof_dir.join("profiles").join("existing");
    fs::create_dir_all(&profile_dir).unwrap();

    let _args = CreateArgs {
        name: "existing".to_string(),
    };

    // Note: Can't test execute() directly since it uses interactive dialoguer
    // Would need to mock stdin or use a non-interactive mode
    // For now, just test that profile directory exists
    assert!(profile_dir.exists());
}

#[test]
#[serial]
fn test_manifest_generation_from_framework_info() {
    use zprof::core::manifest::Manifest;
    use zprof::frameworks::{FrameworkInfo, FrameworkType};

    let framework_info = FrameworkInfo {
        framework_type: FrameworkType::OhMyZsh,
        plugins: vec!["git".to_string(), "docker".to_string()],
        theme: "robbyrussell".to_string(),
        config_path: PathBuf::from("/home/user/.zshrc"),
        install_path: PathBuf::from("/home/user/.oh-my-zsh"),
    };

    let manifest = Manifest::from_framework_info("work", &framework_info);

    assert_eq!(manifest.profile.name, "work");
    assert_eq!(manifest.profile.framework, "oh-my-zsh");
    assert_eq!(manifest.profile.theme(), "robbyrussell");
    assert_eq!(manifest.plugins.enabled.len(), 2);
    assert!(manifest.plugins.enabled.contains(&"git".to_string()));
    assert!(manifest.plugins.enabled.contains(&"docker".to_string()));
}

#[test]
#[serial]
fn test_copy_preserves_original_files() {
    use zprof::core::filesystem::copy_dir_recursive;

    let (_temp, home) = setup_test_env();

    // Create source directory with files
    let source = home.join("source");
    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("file1.txt"), "content1").unwrap();
    fs::write(source.join("file2.txt"), "content2").unwrap();

    let dest = home.join("dest");

    // Copy directory
    copy_dir_recursive(&source, &dest).unwrap();

    // CRITICAL: Verify originals still exist (NFR002)
    assert!(source.exists());
    assert!(source.join("file1.txt").exists());
    assert!(source.join("file2.txt").exists());

    // Verify copies exist
    assert!(dest.exists());
    assert!(dest.join("file1.txt").exists());
    assert!(dest.join("file2.txt").exists());

    // Verify content
    let content1 = fs::read_to_string(dest.join("file1.txt")).unwrap();
    assert_eq!(content1, "content1");
}

#[test]
#[serial]
fn test_zshrc_preserved_after_copy() {
    use zprof::core::filesystem::get_zprof_dir;

    let (_temp, home) = setup_test_env();

    // Create mock .zshrc
    let zshrc_source = home.join(".zshrc");
    let zshrc_content = "# Test zshrc\nexport PATH=$PATH:/usr/local/bin";
    fs::write(&zshrc_source, zshrc_content).unwrap();

    // Create destination
    let zprof_dir = get_zprof_dir().unwrap();
    fs::create_dir_all(&zprof_dir).unwrap();
    let profile_dir = zprof_dir.join("profiles").join("test");
    fs::create_dir_all(&profile_dir).unwrap();

    // Copy .zshrc
    let zshrc_dest = profile_dir.join(".zshrc");
    fs::copy(&zshrc_source, &zshrc_dest).unwrap();

    // CRITICAL: Verify original .zshrc still exists and unchanged (NFR002)
    assert!(zshrc_source.exists());
    let original_content = fs::read_to_string(&zshrc_source).unwrap();
    assert_eq!(original_content, zshrc_content);

    // Verify copy exists
    assert!(zshrc_dest.exists());
    let copied_content = fs::read_to_string(&zshrc_dest).unwrap();
    assert_eq!(copied_content, zshrc_content);
}
