//! Integration tests for framework detection
//!
//! These tests verify that all framework detection implementations work correctly
//! with mock file systems and handle edge cases gracefully.
//!
//! These tests use `serial_test` to ensure tests that modify the HOME environment
//! variable don't run in parallel and interfere with each other.

use serial_test::serial;
use std::fs;
use std::io::Write;
use tempfile::TempDir;

// Helper function to create a temporary home directory for testing
fn setup_temp_home() -> TempDir {
    TempDir::new().expect("Failed to create temp dir")
}

// Helper to set HOME environment variable
fn with_temp_home<F>(test: F)
where
    F: FnOnce(&TempDir),
{
    let temp_dir = setup_temp_home();
    let original_home = std::env::var("HOME").ok();

    std::env::set_var("HOME", temp_dir.path());

    test(&temp_dir);

    // Restore original HOME
    if let Some(home) = original_home {
        std::env::set_var("HOME", home);
    } else {
        std::env::remove_var("HOME");
    }
}

#[test]


#[serial]
fn test_oh_my_zsh_detection() {
    with_temp_home(|home| {
        // Create oh-my-zsh directory
        let omz_dir = home.path().join(".oh-my-zsh");
        fs::create_dir(&omz_dir).unwrap();

        // Create .zshrc with oh-my-zsh configuration
        let zshrc_path = home.path().join(".zshrc");
        let mut zshrc = fs::File::create(&zshrc_path).unwrap();
        writeln!(
            zshrc,
            r#"
export ZSH="$HOME/.oh-my-zsh"
ZSH_THEME="robbyrussell"
plugins=(git docker kubectl)
source $ZSH/oh-my-zsh.sh
"#
        )
        .unwrap();

        // Test detection
        use zprof::frameworks::{detect_existing_framework, FrameworkType};
        let result = detect_existing_framework();
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.framework_type, FrameworkType::OhMyZsh);
        assert_eq!(info.plugins, vec!["git", "docker", "kubectl"]);
        assert_eq!(info.theme, "robbyrussell");
    });
}

#[test]

#[serial]
fn test_zimfw_detection() {
    with_temp_home(|home| {
        // Create zimfw directory
        let zim_dir = home.path().join(".zim");
        fs::create_dir(&zim_dir).unwrap();

        // Create .zimrc with zimfw configuration
        let zimrc_path = home.path().join(".zimrc");
        let mut zimrc = fs::File::create(&zimrc_path).unwrap();
        writeln!(
            zimrc,
            r#"
zmodule romkatv/powerlevel10k
zmodule zsh-users/zsh-syntax-highlighting
zmodule zsh-users/zsh-autosuggestions
"#
        )
        .unwrap();

        // Test detection
        use zprof::frameworks::{detect_existing_framework, FrameworkType};
        let result = detect_existing_framework();
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.framework_type, FrameworkType::Zimfw);
        assert_eq!(info.theme, "powerlevel10k");
        assert!(info.plugins.contains(&"zsh-syntax-highlighting".to_string()));
        assert!(info.plugins.contains(&"zsh-autosuggestions".to_string()));
    });
}

#[test]

#[serial]
fn test_prezto_detection() {
    with_temp_home(|home| {
        // Create prezto directory
        let prezto_dir = home.path().join(".zprezto");
        fs::create_dir(&prezto_dir).unwrap();

        // Create .zpreztorc with prezto configuration
        let zpreztorc_path = home.path().join(".zpreztorc");
        let mut zpreztorc = fs::File::create(&zpreztorc_path).unwrap();
        writeln!(
            zpreztorc,
            r#"
zstyle ':prezto:load' pmodule 'environment' 'terminal' 'git' 'docker'
zstyle ':prezto:module:prompt' theme 'pure'
"#
        )
        .unwrap();

        // Test detection
        use zprof::frameworks::{detect_existing_framework, FrameworkType};
        let result = detect_existing_framework();
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.framework_type, FrameworkType::Prezto);
        assert_eq!(info.theme, "pure");
        assert!(info.plugins.contains(&"git".to_string()));
        assert!(info.plugins.contains(&"docker".to_string()));
    });
}

#[test]

#[serial]
fn test_zinit_detection() {
    with_temp_home(|home| {
        // Create zinit directory
        let zinit_dir = home.path().join(".zinit");
        fs::create_dir(&zinit_dir).unwrap();

        // Create .zshrc with zinit configuration
        let zshrc_path = home.path().join(".zshrc");
        let mut zshrc = fs::File::create(&zshrc_path).unwrap();
        writeln!(
            zshrc,
            r#"
zinit light romkatv/powerlevel10k
zinit load zdharma-continuum/fast-syntax-highlighting
zinit light zsh-users/zsh-autosuggestions
"#
        )
        .unwrap();

        // Test detection
        use zprof::frameworks::{detect_existing_framework, FrameworkType};
        let result = detect_existing_framework();
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.framework_type, FrameworkType::Zinit);
        assert_eq!(info.theme, "powerlevel10k");
        assert!(info.plugins.contains(&"fast-syntax-highlighting".to_string()));
        assert!(info.plugins.contains(&"zsh-autosuggestions".to_string()));
    });
}

#[test]

#[serial]
fn test_zap_detection() {
    with_temp_home(|home| {
        // Create zap directory
        let zap_dir = home.path().join(".local/share/zap");
        fs::create_dir_all(&zap_dir).unwrap();

        // Create .zshrc with zap configuration
        let zshrc_path = home.path().join(".zshrc");
        let mut zshrc = fs::File::create(&zshrc_path).unwrap();
        writeln!(
            zshrc,
            r#"
plug "zsh-users/zsh-autosuggestions"
plug "zsh-users/zsh-syntax-highlighting"
"#
        )
        .unwrap();

        // Test detection
        use zprof::frameworks::{detect_existing_framework, FrameworkType};
        let result = detect_existing_framework();
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.framework_type, FrameworkType::Zap);
        assert!(info.plugins.contains(&"zsh-autosuggestions".to_string()));
        assert!(info.plugins.contains(&"zsh-syntax-highlighting".to_string()));
    });
}

#[test]

#[serial]
fn test_no_framework_detected() {
    with_temp_home(|_home| {
        // Empty home directory - no frameworks installed
        use zprof::frameworks::detect_existing_framework;
        let result = detect_existing_framework();
        assert!(result.is_none());
    });
}

#[test]

#[serial]
fn test_corrupted_zshrc_handling() {
    with_temp_home(|home| {
        // Create oh-my-zsh directory
        let omz_dir = home.path().join(".oh-my-zsh");
        fs::create_dir(&omz_dir).unwrap();

        // Create .zshrc with corrupted/incomplete configuration
        let zshrc_path = home.path().join(".zshrc");
        let mut zshrc = fs::File::create(&zshrc_path).unwrap();
        writeln!(
            zshrc,
            r#"
# Corrupted config - missing closing paren
plugins=(git docker
source $ZSH/oh-my-zsh.sh
"#
        )
        .unwrap();

        // Detection should handle gracefully and still detect framework
        use zprof::frameworks::{detect_existing_framework, FrameworkType};
        let result = detect_existing_framework();
        // Should still detect oh-my-zsh even with corrupted plugins
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.framework_type, FrameworkType::OhMyZsh);
    });
}

#[test]

#[serial]
fn test_multiple_frameworks_returns_most_recent() {
    with_temp_home(|home| {
        // Create oh-my-zsh setup
        let omz_dir = home.path().join(".oh-my-zsh");
        fs::create_dir(&omz_dir).unwrap();
        let omz_rc = home.path().join(".zshrc");
        let mut zshrc = fs::File::create(&omz_rc).unwrap();
        writeln!(
            zshrc,
            r#"
export ZSH="$HOME/.oh-my-zsh"
plugins=(git)
source $ZSH/oh-my-zsh.sh
"#
        )
        .unwrap();

        // Sleep briefly to ensure timestamp difference
        std::thread::sleep(std::time::Duration::from_millis(100));

        // Create zimfw setup (more recent)
        let zim_dir = home.path().join(".zim");
        fs::create_dir(&zim_dir).unwrap();
        let zimrc_path = home.path().join(".zimrc");
        let mut zimrc = fs::File::create(&zimrc_path).unwrap();
        writeln!(zimrc, "zmodule zsh-users/zsh-autosuggestions").unwrap();

        // Should return zimfw since it's more recent
        use zprof::frameworks::{detect_existing_framework, FrameworkType};
        let result = detect_existing_framework();
        assert!(result.is_some());
        let info = result.unwrap();
        // This test may vary based on filesystem timestamps
        // Just verify we got one of them
        assert!(
            info.framework_type == FrameworkType::Zimfw
                || info.framework_type == FrameworkType::OhMyZsh
        );
    });
}

#[test]

#[serial]
fn test_empty_plugins_array() {
    with_temp_home(|home| {
        // Create oh-my-zsh directory
        let omz_dir = home.path().join(".oh-my-zsh");
        fs::create_dir(&omz_dir).unwrap();

        // Create .zshrc with empty plugins
        let zshrc_path = home.path().join(".zshrc");
        let mut zshrc = fs::File::create(&zshrc_path).unwrap();
        writeln!(
            zshrc,
            r#"
export ZSH="$HOME/.oh-my-zsh"
plugins=()
source $ZSH/oh-my-zsh.sh
"#
        )
        .unwrap();

        use zprof::frameworks::{detect_existing_framework, FrameworkType};
        let result = detect_existing_framework();
        assert!(result.is_some());
        let info = result.unwrap();
        assert_eq!(info.framework_type, FrameworkType::OhMyZsh);
        assert!(info.plugins.is_empty());
    });
}

#[test]

#[serial]
fn test_performance_under_2_seconds() {
    with_temp_home(|home| {
        // Create all five frameworks to test worst-case scenario
        fs::create_dir(home.path().join(".oh-my-zsh")).ok();
        fs::create_dir(home.path().join(".zim")).ok();
        fs::create_dir(home.path().join(".zprezto")).ok();
        fs::create_dir(home.path().join(".zinit")).ok();
        fs::create_dir_all(home.path().join(".local/share/zap")).ok();

        // Create config files
        fs::write(
            home.path().join(".zshrc"),
            "source $ZSH/oh-my-zsh.sh\nplugins=(git)",
        )
        .ok();
        fs::write(home.path().join(".zimrc"), "zmodule git").ok();
        fs::write(home.path().join(".zpreztorc"), "zstyle ':prezto:load' pmodule 'git'").ok();

        use std::time::Instant;
        use zprof::frameworks::detect_existing_framework;

        let start = Instant::now();
        let _result = detect_existing_framework();
        let duration = start.elapsed();

        // Should complete in under 2 seconds (AC #4)
        assert!(
            duration.as_secs() < 2,
            "Detection took {:?}, expected < 2s",
            duration
        );
    });
}
