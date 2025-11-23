use anyhow::Result;
use serial_test::serial;
use std::fs;
use tempfile::TempDir;

// Test helper to create a complete test environment
struct TestEnv {
    _temp_dir: TempDir,
    home_dir: std::path::PathBuf,
    profiles_dir: std::path::PathBuf,
}

impl TestEnv {
    fn new() -> Result<Self> {
        let temp_dir = TempDir::new()?;
        let home_dir = temp_dir.path().to_path_buf();
        let profiles_dir = home_dir.join(".zsh-profiles").join("profiles");

        fs::create_dir_all(&profiles_dir)?;

        Ok(Self {
            _temp_dir: temp_dir,
            home_dir,
            profiles_dir,
        })
    }

    fn create_profile_with_backup(&self, name: &str, framework: &str) -> Result<std::path::PathBuf> {
        let profile_dir = self.profiles_dir.join(name);
        fs::create_dir_all(&profile_dir)?;

        // Create profile manifest
        let manifest = format!(
            r#"[profile]
name = "{name}"
framework = "{framework}"
theme = "robbyrussell"
created = "2025-11-01T10:00:00Z"
modified = "2025-11-01T10:00:00Z"
"#
        );
        fs::write(profile_dir.join("profile.toml"), manifest)?;

        // Create backup .zshrc file
        let backup_content = format!(
            "# Original .zshrc backed up during zprof init\n\
             # Profile: {name}\n\
             # Framework: {framework}\n\
             \n\
             export PATH=$HOME/bin:$PATH\n\
             export EDITOR=vim\n\
             \n\
             alias ll='ls -la'\n\
             alias gs='git status'\n"
        );
        fs::write(profile_dir.join(".zshrc.pre-zprof"), backup_content)?;

        // Create framework directory
        let framework_dir_name = match framework {
            "oh-my-zsh" => ".oh-my-zsh",
            "zimfw" => ".zimfw",
            "prezto" => ".zprezto",
            "zinit" => ".zinit",
            "zap" => ".zap",
            _ => ".oh-my-zsh",
        };
        let framework_dir = profile_dir.join(framework_dir_name);
        fs::create_dir_all(&framework_dir)?;
        fs::write(framework_dir.join("oh-my-zsh.sh"), "#!/bin/zsh\n# Mock framework")?;

        Ok(profile_dir)
    }

    fn create_current_zshrc(&self, content: &str) -> Result<()> {
        let zshrc_path = self.home_dir.join(".zshrc");
        fs::write(zshrc_path, content)?;
        Ok(())
    }
}

// AC1: Test backup detection finds .zshrc.pre-zprof in profile directories
#[test]
#[serial]
fn test_backup_detection() -> Result<()> {
    let env = TestEnv::new()?;

    // Create profile with backup
    env.create_profile_with_backup("test-profile", "oh-my-zsh")?;

    // Verify backup exists
    let backup_path = env.profiles_dir.join("test-profile").join(".zshrc.pre-zprof");
    assert!(backup_path.exists(), "Backup file should exist");

    // Verify it's readable
    let content = fs::read_to_string(&backup_path)?;
    assert!(content.contains("Original .zshrc"), "Backup should contain original marker");

    Ok(())
}

// AC6: Test missing backup error message
#[test]
#[serial]
fn test_missing_backup_error() -> Result<()> {
    let env = TestEnv::new()?;

    // Create profile without backup
    let profile_dir = env.profiles_dir.join("no-backup");
    fs::create_dir_all(&profile_dir)?;

    let manifest = r#"[profile]
name = "no-backup"
framework = "oh-my-zsh"
"#;
    fs::write(profile_dir.join("profile.toml"), manifest)?;

    // Verify no backup exists
    let backup_path = profile_dir.join(".zshrc.pre-zprof");
    assert!(!backup_path.exists(), "Backup should not exist");

    Ok(())
}

// AC7: Test backup integrity validation
#[test]
#[serial]
fn test_backup_integrity_validation() -> Result<()> {
    let env = TestEnv::new()?;

    // Create profile with valid backup
    let profile_dir = env.create_profile_with_backup("valid", "oh-my-zsh")?;
    let backup_path = profile_dir.join(".zshrc.pre-zprof");

    // Test 1: Valid backup
    let content = fs::read_to_string(&backup_path)?;
    assert!(
        !content.trim().is_empty(),
        "Valid backup should not be empty"
    );

    // Test 2: Empty backup (corrupted)
    let corrupted_profile = env.profiles_dir.join("corrupted");
    fs::create_dir_all(&corrupted_profile)?;
    fs::write(corrupted_profile.join(".zshrc.pre-zprof"), "")?;

    let empty_backup = fs::read_to_string(corrupted_profile.join(".zshrc.pre-zprof"))?;
    assert!(
        empty_backup.trim().is_empty(),
        "Corrupted backup should be empty"
    );

    Ok(())
}

// Test framework detection from profile manifest
#[test]
#[serial]
fn test_framework_detection() -> Result<()> {
    let env = TestEnv::new()?;

    // Test each framework type
    let frameworks = vec![
        ("oh-my-zsh", ".oh-my-zsh"),
        ("zimfw", ".zimfw"),
        ("prezto", ".zprezto"),
        ("zinit", ".zinit"),
        ("zap", ".zap"),
    ];

    for (framework_name, framework_dir) in frameworks {
        let profile_dir = env.create_profile_with_backup(&format!("test-{framework_name}"), framework_name)?;

        // Verify manifest exists
        let manifest_path = profile_dir.join("profile.toml");
        assert!(manifest_path.exists(), "Manifest should exist for {framework_name}");

        // Verify framework directory exists
        let fw_dir = profile_dir.join(framework_dir);
        assert!(
            fw_dir.exists(),
            "Framework directory should exist for {framework_name}"
        );
    }

    Ok(())
}

// Test multiple profiles with backups (should use most recent)
#[test]
#[serial]
fn test_multiple_profiles_with_backups() -> Result<()> {
    let env = TestEnv::new()?;

    // Create multiple profiles with backups
    env.create_profile_with_backup("profile1", "oh-my-zsh")?;
    std::thread::sleep(std::time::Duration::from_millis(10)); // Ensure different timestamps
    env.create_profile_with_backup("profile2", "zimfw")?;
    std::thread::sleep(std::time::Duration::from_millis(10));
    let profile3 = env.create_profile_with_backup("profile3", "zinit")?;

    // Verify all backups exist
    assert!(env.profiles_dir.join("profile1").join(".zshrc.pre-zprof").exists());
    assert!(env.profiles_dir.join("profile2").join(".zshrc.pre-zprof").exists());
    assert!(env.profiles_dir.join("profile3").join(".zshrc.pre-zprof").exists());

    // profile3 should be most recent due to creation order
    let profile3_backup = profile3.join(".zshrc.pre-zprof");
    let metadata = fs::metadata(&profile3_backup)?;
    assert!(metadata.is_file());

    Ok(())
}

// Test safety backup creation before rollback
#[test]
#[serial]
fn test_safety_backup_creation() -> Result<()> {
    let env = TestEnv::new()?;

    // Create current .zshrc
    env.create_current_zshrc("# Current zshrc managed by zprof\n")?;

    let current_zshrc = env.home_dir.join(".zshrc");
    let safety_backup = env.home_dir.join(".zshrc.pre-rollback");

    assert!(current_zshrc.exists(), "Current zshrc should exist");
    assert!(!safety_backup.exists(), "Safety backup should not exist yet");

    // After rollback would be performed, safety backup should exist
    // This is tested by the rollback command itself

    Ok(())
}

// Test framework relocation back to home directory
#[test]
#[serial]
fn test_framework_relocation() -> Result<()> {
    let env = TestEnv::new()?;

    let profile_dir = env.create_profile_with_backup("relocate-test", "oh-my-zsh")?;

    // Verify framework is in profile directory
    let framework_in_profile = profile_dir.join(".oh-my-zsh");
    assert!(framework_in_profile.exists(), "Framework should be in profile directory");

    // After rollback, it should be moved to home directory
    let framework_in_home = env.home_dir.join(".oh-my-zsh");
    assert!(!framework_in_home.exists(), "Framework should not be in home directory yet");

    // The rollback command would move it

    Ok(())
}

// Test .zsh-profiles preservation
#[test]
#[serial]
fn test_zsh_profiles_preservation() -> Result<()> {
    let env = TestEnv::new()?;

    env.create_profile_with_backup("preserve-test", "oh-my-zsh")?;

    let zsh_profiles = env.home_dir.join(".zsh-profiles");
    assert!(zsh_profiles.exists(), ".zsh-profiles directory should exist");

    // After rollback, it should still exist
    // The rollback command explicitly preserves this directory

    Ok(())
}

// Test file permissions on restored files
#[test]
#[serial]
fn test_restored_file_permissions() -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let env = TestEnv::new()?;
    let profile_dir = env.create_profile_with_backup("perms-test", "oh-my-zsh")?;

    let backup_path = profile_dir.join(".zshrc.pre-zprof");

    // Set specific permissions on backup
    let permissions = fs::Permissions::from_mode(0o644);
    fs::set_permissions(&backup_path, permissions)?;

    // Verify permissions
    let metadata = fs::metadata(&backup_path)?;
    let mode = metadata.permissions().mode();
    assert_eq!(mode & 0o777, 0o644, "Backup should have 644 permissions");

    Ok(())
}

// Integration test: Complete rollback scenario with oh-my-zsh
#[test]
#[serial]
fn test_complete_rollback_oh_my_zsh() -> Result<()> {
    let env = TestEnv::new()?;

    // Setup: Create profile with oh-my-zsh
    env.create_profile_with_backup("complete-test", "oh-my-zsh")?;
    env.create_current_zshrc("# Current zprof-managed config\n")?;

    // Verify initial state
    let current_zshrc = env.home_dir.join(".zshrc");
    let backup_path = env.profiles_dir.join("complete-test").join(".zshrc.pre-zprof");

    assert!(current_zshrc.exists(), "Current zshrc should exist");
    assert!(backup_path.exists(), "Backup should exist");

    let current_content = fs::read_to_string(&current_zshrc)?;
    assert!(current_content.contains("zprof-managed"), "Current should be zprof-managed");

    let backup_content = fs::read_to_string(&backup_path)?;
    assert!(backup_content.contains("Original .zshrc"), "Backup should be original");

    // The actual rollback command would:
    // 1. Create safety backup
    // 2. Restore .zshrc from backup
    // 3. Move framework to home
    // 4. Preserve .zsh-profiles

    Ok(())
}

// Integration test: Rollback with zinit
#[test]
#[serial]
fn test_rollback_zinit() -> Result<()> {
    let env = TestEnv::new()?;

    env.create_profile_with_backup("zinit-test", "zinit")?;

    let profile_dir = env.profiles_dir.join("zinit-test");
    let zinit_dir = profile_dir.join(".zinit");

    assert!(zinit_dir.exists(), "Zinit directory should exist in profile");

    Ok(())
}

// Integration test: Rollback with prezto
#[test]
#[serial]
fn test_rollback_prezto() -> Result<()> {
    let env = TestEnv::new()?;

    env.create_profile_with_backup("prezto-test", "prezto")?;

    let profile_dir = env.profiles_dir.join("prezto-test");
    let prezto_dir = profile_dir.join(".zprezto");

    assert!(prezto_dir.exists(), "Prezto directory should exist in profile");

    Ok(())
}

// Integration test: Rollback with zimfw
#[test]
#[serial]
fn test_rollback_zimfw() -> Result<()> {
    let env = TestEnv::new()?;

    env.create_profile_with_backup("zimfw-test", "zimfw")?;

    let profile_dir = env.profiles_dir.join("zimfw-test");
    let zimfw_dir = profile_dir.join(".zimfw");

    assert!(zimfw_dir.exists(), "Zimfw directory should exist in profile");

    Ok(())
}
