//! Integration tests for GUI IPC commands
//!
//! These tests verify that the Tauri IPC layer correctly wraps
//! the existing business logic and returns properly serialized results.

use anyhow::Result;
use zprof_tauri::*;

#[test]
fn test_get_frameworks() -> Result<()> {
    let frameworks = get_frameworks().map_err(|e| anyhow::anyhow!(e))?;

    assert_eq!(frameworks.len(), 5);

    // Check that all expected frameworks are present
    let framework_names: Vec<&str> = frameworks.iter().map(|f| f.name.as_str()).collect();
    assert!(framework_names.contains(&"oh-my-zsh"));
    assert!(framework_names.contains(&"zimfw"));
    assert!(framework_names.contains(&"prezto"));
    assert!(framework_names.contains(&"zinit"));
    assert!(framework_names.contains(&"zap"));

    // Verify framework info structure
    let omz = frameworks.iter().find(|f| f.name == "oh-my-zsh").unwrap();
    assert!(!omz.description.is_empty());
    assert!(omz.supports_themes);
    assert!(omz.supports_plugins);

    Ok(())
}

#[test]
fn test_get_prompt_engines() -> Result<()> {
    let engines = get_prompt_engines().map_err(|e| anyhow::anyhow!(e))?;

    assert_eq!(engines.len(), 5);

    // Check that all expected engines are present
    let engine_names: Vec<&str> = engines.iter().map(|e| e.name.as_str()).collect();
    assert!(engine_names.contains(&"Starship"));
    assert!(engine_names.contains(&"Powerlevel10k"));
    assert!(engine_names.contains(&"Oh-My-Posh"));
    assert!(engine_names.contains(&"Pure"));
    assert!(engine_names.contains(&"Spaceship"));

    // Verify engine info structure
    let starship = engines.iter().find(|e| e.name == "Starship").unwrap();
    assert!(!starship.description.is_empty());
    assert!(starship.nerd_font_required);

    let pure = engines.iter().find(|e| e.name == "Pure").unwrap();
    assert!(!pure.nerd_font_required); // Pure doesn't require Nerd Fonts

    Ok(())
}

#[test]
fn test_profile_config_validation() {
    let valid_config = ProfileConfig {
        name: "test-profile".to_string(),
        framework: "oh-my-zsh".to_string(),
        prompt_mode: "prompt_engine".to_string(),
        prompt_engine: Some("starship".to_string()),
        framework_theme: None,
        plugins: vec!["git".to_string()],
        env_vars: std::collections::HashMap::new(),
    };

    assert!(valid_config.validate().is_ok());

    // Invalid: empty name
    let invalid_config = ProfileConfig {
        name: "".to_string(),
        ..valid_config.clone()
    };
    assert!(invalid_config.validate().is_err());

    // Invalid: missing prompt_engine
    let invalid_config2 = ProfileConfig {
        prompt_engine: None,
        ..valid_config.clone()
    };
    assert!(invalid_config2.validate().is_err());

    // Invalid: unsupported framework
    let invalid_config3 = ProfileConfig {
        framework: "bash-it".to_string(),
        ..valid_config.clone()
    };
    assert!(invalid_config3.validate().is_err());
}

#[test]
fn test_ipc_error_classification() {
    use zprof_tauri::{ErrorCode, IpcError};

    // Test error code classification from anyhow errors
    let not_found_err: IpcError = anyhow::anyhow!("Profile not found").into();
    assert_eq!(not_found_err.code, ErrorCode::ProfileNotFound);

    let already_exists_err: IpcError = anyhow::anyhow!("Profile already exists").into();
    assert_eq!(already_exists_err.code, ErrorCode::ProfileExists);

    let active_err: IpcError = anyhow::anyhow!("Cannot delete active profile").into();
    assert_eq!(active_err.code, ErrorCode::ProfileActive);

    let invalid_err: IpcError = anyhow::anyhow!("Invalid profile name").into();
    assert_eq!(invalid_err.code, ErrorCode::InvalidInput);
}

#[test]
fn test_ipc_error_to_string() {
    use zprof_tauri::{ErrorCode, IpcError};

    let err = IpcError::new(ErrorCode::ProfileNotFound, "Profile 'test' not found")
        .with_suggestion("Run list_profiles to see available profiles");

    let result = err.to_string_result();
    assert!(result.contains("Profile 'test' not found"));
    assert!(result.contains("Suggestion: Run list_profiles"));
}

// Note: Tests that interact with filesystem (like list_profiles, create_profile, etc.)
// require either mocking the filesystem or setting up actual test profiles.
// These would ideally be in a separate test module with proper setup/teardown.

#[cfg(test)]
mod integration {
    use super::*;

    // These tests would require more complex setup to mock/isolate the filesystem
    // For now, we're testing the command handlers' type conversions and error handling
    // rather than full end-to-end workflows which would modify the user's system.

    #[test]
    fn test_type_serialization() {
        use serde_json;

        // Test that our types can be serialized/deserialized properly
        let profile_info = ProfileInfo {
            name: "work".to_string(),
            framework: "oh-my-zsh".to_string(),
            prompt_mode: "prompt_engine".to_string(),
            active: true,
            created_at: "2025-11-22T10:00:00Z".to_string(),
            plugin_count: 5,
        };

        let json = serde_json::to_string(&profile_info).expect("Serialization failed");
        let deserialized: ProfileInfo = serde_json::from_str(&json).expect("Deserialization failed");

        assert_eq!(deserialized.name, "work");
        assert_eq!(deserialized.framework, "oh-my-zsh");
        assert!(deserialized.active);
    }
}
