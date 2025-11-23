//! Tauri IPC command handlers
//!
//! This module implements all IPC commands that the frontend can invoke.
//! Commands are thin wrappers around existing business logic in src/core and src/frameworks.

use crate::error::{ErrorCode, IpcError};
use crate::types::*;

/// List all profiles with their basic information
#[tauri::command]
pub fn list_profiles() -> Result<Vec<ProfileInfo>, String> {
    // Get active profile from config
    let config = zprof::core::config::load_config()
        .map_err(|e| IpcError::from(e).to_string_result())?;

    let active_profile = config.active_profile.as_deref();

    // Get profiles directory
    let profiles_dir = zprof::core::profile::get_profiles_dir()
        .map_err(|e| IpcError::from(e).to_string_result())?;

    // Scan all profiles
    let core_profiles = zprof::core::profile::scan_profiles(&profiles_dir, active_profile)
        .map_err(|e| IpcError::from(e).to_string_result())?;

    // Convert to GUI types
    let gui_profiles: Vec<ProfileInfo> = core_profiles
        .into_iter()
        .map(|p| {
            // Load full metadata to get created timestamp and plugin count
            let manifest = zprof::core::manifest::load_and_validate(&p.name);

            let (created_at, plugin_count, prompt_mode, prompt_engine, framework_theme) = match manifest {
                Ok(m) => {
                    let (prompt_mode_str, engine, theme) = match &m.profile.prompt_mode {
                        zprof::core::manifest::PromptMode::PromptEngine { engine } => {
                            ("prompt_engine", Some(engine.clone()), None)
                        },
                        zprof::core::manifest::PromptMode::FrameworkTheme { theme } => {
                            ("framework_theme", None, Some(theme.clone()))
                        },
                    };
                    (
                        m.profile.created.to_rfc3339(),
                        m.plugins.enabled.len(),
                        prompt_mode_str.to_string(),
                        engine,
                        theme,
                    )
                },
                Err(_) => (
                    chrono::Utc::now().to_rfc3339(),
                    0,
                    "framework_theme".to_string(),
                    None,
                    None,
                ),
            };

            ProfileInfo {
                name: p.name,
                framework: p.framework,
                prompt_mode,
                prompt_engine,
                framework_theme,
                active: p.is_active,
                created_at,
                plugin_count,
            }
        })
        .collect();

    Ok(gui_profiles)
}

/// Get detailed information for a specific profile
#[tauri::command]
pub fn get_profile(name: String) -> Result<ProfileDetails, String> {
    // Load and validate manifest
    let manifest = zprof::core::manifest::load_and_validate(&name)
        .map_err(|_e| {
            IpcError::new(
                ErrorCode::ProfileNotFound,
                format!("Profile '{}' not found", name)
            )
            .with_suggestion("Run list_profiles to see available profiles")
            .to_string_result()
        })?;

    // Convert to GUI types
    let prompt_mode = match manifest.profile.prompt_mode {
        zprof::core::manifest::PromptMode::PromptEngine { engine } => {
            PromptModeInfo::PromptEngine { engine }
        }
        zprof::core::manifest::PromptMode::FrameworkTheme { theme } => {
            PromptModeInfo::FrameworkTheme { theme }
        }
    };

    Ok(ProfileDetails {
        name: manifest.profile.name,
        framework: manifest.profile.framework,
        prompt_mode,
        plugins: manifest.plugins.enabled,
        env_vars: manifest.env,
        created_at: manifest.profile.created.to_rfc3339(),
        modified_at: manifest.profile.modified.to_rfc3339(),
    })
}

/// Get the currently active profile name
#[tauri::command]
pub fn get_active_profile() -> Result<Option<String>, String> {
    let config = zprof::core::config::load_config()
        .map_err(|e| IpcError::from(e).to_string_result())?;

    Ok(config.active_profile)
}

/// Create a new profile from configuration
#[tauri::command]
pub fn create_profile(config: ProfileConfig) -> Result<String, String> {
    // Validate configuration
    config.validate()
        .map_err(|e| {
            IpcError::new(ErrorCode::InvalidInput, e)
                .to_string_result()
        })?;

    // Check if profile already exists
    let profile_path = zprof::core::profile::get_profiles_dir()
        .map_err(|e| IpcError::from(e).to_string_result())?
        .join(&config.name);

    if profile_path.exists() {
        return Err(IpcError::new(
            ErrorCode::ProfileExists,
            format!("Profile '{}' already exists", config.name)
        )
        .with_suggestion("Choose a different name or delete the existing profile first")
        .to_string_result());
    }

    // Create manifest from config
    use zprof::core::manifest::{Manifest, ProfileSection, PluginsSection, PromptMode};
    use chrono::Utc;

    let prompt_mode = match config.prompt_mode.as_str() {
        "prompt_engine" => {
            let engine = config.prompt_engine.ok_or_else(|| {
                IpcError::new(
                    ErrorCode::InvalidInput,
                    "prompt_engine is required when prompt_mode is 'prompt_engine'"
                )
                .to_string_result()
            })?;
            PromptMode::PromptEngine { engine }
        }
        "framework_theme" => {
            PromptMode::FrameworkTheme {
                theme: config.framework_theme.unwrap_or_default(),
            }
        }
        _ => {
            return Err(IpcError::new(
                ErrorCode::InvalidInput,
                "prompt_mode must be 'prompt_engine' or 'framework_theme'"
            )
            .to_string_result());
        }
    };

    let now = Utc::now();
    let manifest = Manifest {
        profile: ProfileSection {
            name: config.name.clone(),
            framework: config.framework.clone(),
            prompt_mode,
            created: now,
            modified: now,
        },
        plugins: PluginsSection {
            enabled: config.plugins,
        },
        env: config.env_vars,
    };

    // Validate manifest
    manifest.validate()
        .map_err(|e| IpcError::from(e).to_string_result())?;

    // Create profile directory
    std::fs::create_dir_all(&profile_path)
        .map_err(|e| {
            IpcError::new(
                ErrorCode::FilesystemError,
                format!("Failed to create profile directory: {}", e)
            )
            .to_string_result()
        })?;

    // Write manifest
    let manifest_path = profile_path.join("profile.toml");
    manifest.write_to_file(&manifest_path)
        .map_err(|e| IpcError::from(e).to_string_result())?;

    // Generate shell configs
    use zprof::shell::generator::generate_zshrc_from_manifest;

    let zshrc_content = generate_zshrc_from_manifest(&manifest)
        .map_err(|e| IpcError::from(e).to_string_result())?;

    let zshrc_path = profile_path.join(".zshrc");
    std::fs::write(&zshrc_path, zshrc_content)
        .map_err(|e| {
            IpcError::new(
                ErrorCode::FilesystemError,
                format!("Failed to write .zshrc: {}", e)
            )
            .to_string_result()
        })?;

    log::info!("Created profile '{}' successfully", config.name);

    Ok(config.name)
}

/// Delete a profile
#[tauri::command]
pub fn delete_profile(name: String) -> Result<(), String> {
    // Validate profile exists
    let profile_path = zprof::core::profile::get_profile_path(&name)
        .map_err(|e| IpcError::from(e).to_string_result())?;

    // Ensure not active
    zprof::core::profile::validate_not_active(&name)
        .map_err(|e| IpcError::from(e).to_string_result())?;

    // Delete profile directory
    std::fs::remove_dir_all(&profile_path)
        .map_err(|e| {
            IpcError::new(
                ErrorCode::FilesystemError,
                format!("Failed to delete profile: {}", e)
            )
            .to_string_result()
        })?;

    log::info!("Deleted profile '{}' successfully", name);

    Ok(())
}

/// Activate a profile (switch to it)
#[tauri::command]
pub fn activate_profile(name: String) -> Result<(), String> {
    // Validate profile exists and is valid
    let profile_path = zprof::core::profile::get_profile_path(&name)
        .map_err(|e| IpcError::from(e).to_string_result())?;

    zprof::core::profile::validate_profile(&profile_path)
        .map_err(|e| IpcError::from(e).to_string_result())?;

    // Update config
    zprof::core::config::update_active_profile(&name)
        .map_err(|e| IpcError::from(e).to_string_result())?;

    // Update ZDOTDIR in ~/.zshenv
    use zprof::shell::zdotdir::set_active_profile;

    set_active_profile(&profile_path)
        .map_err(|e| IpcError::from(e).to_string_result())?;

    log::info!("Activated profile '{}' successfully", name);

    Ok(())
}

/// Get list of available frameworks
#[tauri::command]
pub fn get_frameworks() -> Result<Vec<FrameworkInfo>, String> {
    let frameworks = vec![
        FrameworkInfo {
            name: "oh-my-zsh".to_string(),
            description: "Community-driven zsh framework with 300+ plugins and 140+ themes".to_string(),
            supports_themes: true,
            supports_plugins: true,
        },
        FrameworkInfo {
            name: "zimfw".to_string(),
            description: "Blazing fast zsh framework focused on speed and simplicity".to_string(),
            supports_themes: true,
            supports_plugins: true,
        },
        FrameworkInfo {
            name: "prezto".to_string(),
            description: "Configuration framework for zsh with sane defaults and modules".to_string(),
            supports_themes: true,
            supports_plugins: true,
        },
        FrameworkInfo {
            name: "zinit".to_string(),
            description: "Flexible and fast zsh plugin manager with turbo mode".to_string(),
            supports_themes: false,
            supports_plugins: true,
        },
        FrameworkInfo {
            name: "zap".to_string(),
            description: "Minimal zsh plugin manager, fast and simple".to_string(),
            supports_themes: false,
            supports_plugins: true,
        },
    ];

    Ok(frameworks)
}

/// Get available plugins for a specific framework
#[tauri::command]
pub fn get_plugins(framework: String) -> Result<Vec<PluginInfo>, String> {
    use zprof::frameworks::{FrameworkType, plugin::PLUGIN_REGISTRY};

    // Parse framework type
    let framework_type = match framework.as_str() {
        "oh-my-zsh" => FrameworkType::OhMyZsh,
        "zimfw" => FrameworkType::Zimfw,
        "prezto" => FrameworkType::Prezto,
        "zinit" => FrameworkType::Zinit,
        "zap" => FrameworkType::Zap,
        _ => {
            return Err(IpcError::new(
                ErrorCode::InvalidInput,
                format!("Unknown framework: {}", framework)
            )
            .to_string_result());
        }
    };

    // Filter plugins compatible with this framework
    let plugins: Vec<PluginInfo> = PLUGIN_REGISTRY
        .iter()
        .filter(|p| p.compatibility.supports_framework(&framework_type))
        .map(|p| {
            let category = match p.category {
                zprof::frameworks::PluginCategory::Git => "git",
                zprof::frameworks::PluginCategory::Docker => "docker",
                zprof::frameworks::PluginCategory::Kubernetes => "kubernetes",
                zprof::frameworks::PluginCategory::Language => "language",
                zprof::frameworks::PluginCategory::Utility => "utility",
            };

            PluginInfo {
                name: p.name.to_string(),
                description: p.description.to_string(),
                category: category.to_string(),
                framework: framework.clone(),
            }
        })
        .collect();

    Ok(plugins)
}

/// Get available themes for a specific framework
#[tauri::command]
pub fn get_themes(framework: String) -> Result<Vec<ThemeInfo>, String> {
    use zprof::frameworks::{FrameworkType, theme::THEME_REGISTRY};

    // Parse framework type
    let framework_type = match framework.as_str() {
        "oh-my-zsh" => FrameworkType::OhMyZsh,
        "zimfw" => FrameworkType::Zimfw,
        "prezto" => FrameworkType::Prezto,
        "zinit" => FrameworkType::Zinit,
        "zap" => FrameworkType::Zap,
        _ => {
            return Err(IpcError::new(
                ErrorCode::InvalidInput,
                format!("Unknown framework: {}", framework)
            )
            .to_string_result());
        }
    };

    // Filter themes compatible with this framework
    let themes: Vec<ThemeInfo> = THEME_REGISTRY
        .iter()
        .filter(|t| t.compatibility.supports_framework(&framework_type))
        .map(|t| ThemeInfo {
            name: t.name.to_string(),
            description: t.description.to_string(),
            framework: framework.clone(),
            preview_url: Some(t.preview.to_string()),
        })
        .collect();

    Ok(themes)
}

/// Get available prompt engines
#[tauri::command]
pub fn get_prompt_engines() -> Result<Vec<PromptEngineInfo>, String> {
    use zprof::prompts::engine::PromptEngine;

    let engines = vec![
        PromptEngine::Starship,
        PromptEngine::Powerlevel10k,
        PromptEngine::OhMyPosh,
        PromptEngine::Pure,
        PromptEngine::Spaceship,
    ];

    let engine_infos: Vec<PromptEngineInfo> = engines
        .into_iter()
        .map(|e| {
            let metadata = e.metadata();
            PromptEngineInfo {
                name: metadata.name.to_string(),
                description: metadata.description.to_string(),
                nerd_font_required: metadata.requires_nerd_font,
                cross_shell: metadata.cross_shell,
                async_rendering: matches!(e, PromptEngine::Starship | PromptEngine::Pure),
                preview_url: None,
                installed: None,
            }
        })
        .collect();

    Ok(engine_infos)
}

/// Check if a prompt engine is installed
#[tauri::command]
pub fn check_engine_installed(engine: String) -> Result<bool, String> {
    use zprof::prompts::engine::PromptEngine;
    use zprof::prompts::installer::EngineInstaller;

    let engine_enum = match engine.as_str() {
        "Starship" => PromptEngine::Starship,
        "Powerlevel10k" => PromptEngine::Powerlevel10k,
        "Oh-My-Posh" => PromptEngine::OhMyPosh,
        "Pure" => PromptEngine::Pure,
        "Spaceship" => PromptEngine::Spaceship,
        _ => {
            return Err(IpcError::new(
                ErrorCode::InvalidInput,
                format!("Unknown engine: {}", engine)
            )
            .to_string_result());
        }
    };

    let installer = EngineInstaller::new()
        .map_err(|e| IpcError::from(e).to_string_result())?;

    installer.is_installed(&engine_enum)
        .map_err(|e| IpcError::from(e).to_string_result())
}

/// Install a prompt engine
#[tauri::command]
pub async fn install_prompt_engine(engine: String) -> Result<(), String> {
    use zprof::prompts::engine::PromptEngine;
    use zprof::prompts::installer::EngineInstaller;

    let engine_enum = match engine.as_str() {
        "Starship" => PromptEngine::Starship,
        "Powerlevel10k" => PromptEngine::Powerlevel10k,
        "Oh-My-Posh" => PromptEngine::OhMyPosh,
        "Pure" => PromptEngine::Pure,
        "Spaceship" => PromptEngine::Spaceship,
        _ => {
            return Err(IpcError::new(
                ErrorCode::InvalidInput,
                format!("Unknown engine: {}", engine)
            )
            .to_string_result());
        }
    };

    let installer = EngineInstaller::new()
        .map_err(|e| IpcError::from(e).to_string_result())?;

    installer.install(&engine_enum)
        .map_err(|e| IpcError::from(e).to_string_result())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_prompt_engines_returns_all() {
        let engines = get_prompt_engines().expect("Should get engines");
        assert_eq!(engines.len(), 5, "Should return 5 engines");

        // Verify all engines have required fields
        for engine in &engines {
            assert!(!engine.name.is_empty(), "Engine name should not be empty");
            assert!(!engine.description.is_empty(), "Engine description should not be empty");
        }
    }

    #[test]
    fn test_get_prompt_engines_includes_metadata() {
        let engines = get_prompt_engines().expect("Should get engines");

        // Find Starship engine
        let starship = engines.iter().find(|e| e.name == "Starship");
        assert!(starship.is_some(), "Should include Starship");

        let starship = starship.unwrap();
        assert!(starship.cross_shell, "Starship should be cross-shell");
        assert!(starship.async_rendering, "Starship should support async rendering");
        assert!(starship.nerd_font_required, "Starship should require Nerd Fonts");
    }

    #[test]
    fn test_check_engine_installed_with_invalid_engine() {
        let result = check_engine_installed("InvalidEngine".to_string());
        assert!(result.is_err(), "Should fail with invalid engine name");
    }

    #[test]
    fn test_get_frameworks_returns_all() {
        let frameworks = get_frameworks().expect("Should get frameworks");
        assert_eq!(frameworks.len(), 5, "Should return 5 frameworks");

        // Verify all have descriptions
        for fw in &frameworks {
            assert!(!fw.name.is_empty());
            assert!(!fw.description.is_empty());
        }
    }
}
