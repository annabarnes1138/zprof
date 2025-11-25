//! Preset configurations for quick profile setup
//!
//! This module provides a data-driven preset system that allows users to quickly
//! create profiles with pre-configured settings for common use cases. Presets are
//! defined as static constants, making it easy to add new presets without code changes.

use crate::core::manifest::PromptMode;
use crate::frameworks::FrameworkType;

/// A preset configuration template for creating profiles
///
/// Presets provide a complete configuration for common use cases, making it easy
/// for users to get started without manually configuring every option.
#[derive(Debug, Clone, PartialEq)]
pub struct Preset {
    /// Unique identifier for the preset (e.g., "minimal", "developer")
    pub id: &'static str,
    /// Display name shown to users
    pub name: &'static str,
    /// Icon or emoji for visual representation
    pub icon: &'static str,
    /// Description of what this preset is for
    pub description: &'static str,
    /// Target audience for this preset
    pub target_user: &'static str,
    /// The configuration details
    pub config: PresetConfig,
}

/// Configuration data that can be used to generate a Manifest
///
/// This struct contains all the settings needed to create a complete profile
/// manifest from a preset.
#[derive(Debug, Clone, PartialEq)]
pub struct PresetConfig {
    /// The zsh framework to use
    pub framework: FrameworkType,
    /// Prompt engine name (if using prompt engine mode)
    pub prompt_engine: Option<&'static str>,
    /// Framework theme name (if using framework theme mode)
    pub framework_theme: Option<&'static str>,
    /// List of plugins to enable
    pub plugins: &'static [&'static str],
    /// Environment variables to set
    pub env_vars: &'static [(&'static str, &'static str)],
    /// Shell options to configure
    pub shell_options: &'static [&'static str],
}

impl PresetConfig {
    /// Get the PromptMode for this preset
    pub fn prompt_mode(&self) -> PromptMode {
        if let Some(engine) = self.prompt_engine {
            PromptMode::PromptEngine {
                engine: engine.to_string(),
            }
        } else if let Some(theme) = self.framework_theme {
            PromptMode::FrameworkTheme {
                theme: theme.to_string(),
            }
        } else {
            // Default to empty framework theme
            PromptMode::FrameworkTheme {
                theme: String::new(),
            }
        }
    }
}

/// Find a preset by its ID (case-insensitive)
///
/// Returns the matching preset if found, or None if no preset with that ID exists.
///
/// # Examples
///
/// ```
/// use zprof::presets::find_preset_by_id;
///
/// let preset = find_preset_by_id("minimal").expect("Minimal preset should exist");
/// assert_eq!(preset.id, "minimal");
///
/// // Case-insensitive lookup
/// let preset = find_preset_by_id("PERFORMANCE").expect("Performance preset should exist");
/// assert_eq!(preset.id, "performance");
///
/// // Not found
/// assert!(find_preset_by_id("nonexistent").is_none());
/// ```
pub fn find_preset_by_id(id: &str) -> Option<&'static Preset> {
    PRESET_REGISTRY.iter().find(|preset| preset.id.eq_ignore_ascii_case(id))
}

/// Registry of all available presets
///
/// This constant array contains all preset definitions. Adding a new preset
/// is as simple as adding another entry to this array.
pub const PRESET_REGISTRY: &[Preset] = &[
    // Minimal preset - fast startup, essential features only
    Preset {
        id: "minimal",
        name: "Minimal",
        icon: "✨",
        description: "Fast startup, clean prompt, essential plugins only",
        target_user: "Beginners who want simplicity",
        config: PresetConfig {
            framework: FrameworkType::Zap,
            prompt_engine: Some("pure"),
            framework_theme: None,
            plugins: &["zsh-autosuggestions", "zsh-syntax-highlighting", "git"],
            env_vars: &[],
            shell_options: &["HIST_IGNORE_DUPS", "AUTO_CD"],
        },
    },
    // Performance preset - optimized for speed with turbo mode
    Preset {
        id: "performance",
        name: "Performance",
        icon: "🚀",
        description: "Blazing fast, async prompt, optimized loading with turbo mode",
        target_user: "Users with slow shells",
        config: PresetConfig {
            framework: FrameworkType::Zinit,
            prompt_engine: Some("starship"),
            framework_theme: None,
            plugins: &[
                "git",
                "zsh-autosuggestions",
                "fast-syntax-highlighting",
                "fzf",
                "history-substring-search",
            ],
            env_vars: &[],
            shell_options: &["HIST_IGNORE_DUPS", "HIST_FIND_NO_DUPS"],
        },
    },
    // Fancy preset - beautiful UI with lots of features
    Preset {
        id: "fancy",
        name: "Fancy",
        icon: "✨",
        description: "Beautiful terminal with rich features and visual enhancements",
        target_user: "Make my terminal Instagram-worthy",
        config: PresetConfig {
            framework: FrameworkType::OhMyZsh,
            prompt_engine: None,
            framework_theme: Some("powerlevel10k"),
            plugins: &[
                "git",
                "docker",
                "kubectl",
                "node",
                "npm",
                "zsh-autosuggestions",
                "zsh-syntax-highlighting",
                "colored-man-pages",
                "web-search",
                "jsontools",
                "extract",
                "command-not-found",
            ],
            env_vars: &[],
            shell_options: &[
                "HIST_IGNORE_DUPS",
                "HIST_IGNORE_SPACE",
                "SHARE_HISTORY",
                "AUTO_CD",
            ],
        },
    },
    // Developer preset - tools for software development
    Preset {
        id: "developer",
        name: "Developer",
        icon: "👨‍💻",
        description: "Development-focused setup with common programming tools",
        target_user: "Professional devs who code daily",
        config: PresetConfig {
            framework: FrameworkType::Zimfw,
            prompt_engine: Some("starship"),
            framework_theme: None,
            plugins: &[
                "git",
                "docker",
                "kubectl",
                "fzf",
                "ripgrep",
                "node",
                "zsh-autosuggestions",
                "zsh-syntax-highlighting",
            ],
            env_vars: &[],
            shell_options: &[
                "HIST_IGNORE_DUPS",
                "HIST_IGNORE_SPACE",
                "SHARE_HISTORY",
                "AUTO_CD",
            ],
        },
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preset_registry_count() {
        assert_eq!(PRESET_REGISTRY.len(), 4, "Should have exactly 4 presets");
    }

    #[test]
    fn test_preset_ids_are_unique() {
        let mut ids = Vec::new();
        for preset in PRESET_REGISTRY {
            assert!(
                !ids.contains(&preset.id),
                "Duplicate preset id: {}",
                preset.id
            );
            ids.push(preset.id);
        }
    }

    #[test]
    fn test_preset_ids_match_expected() {
        let expected_ids = vec!["minimal", "performance", "fancy", "developer"];
        let actual_ids: Vec<&str> = PRESET_REGISTRY.iter().map(|p| p.id).collect();

        for expected in &expected_ids {
            assert!(
                actual_ids.contains(expected),
                "Missing expected preset: {expected}"
            );
        }
    }

    #[test]
    fn test_preset_has_all_required_fields() {
        for preset in PRESET_REGISTRY {
            // Verify all string fields are non-empty
            assert!(!preset.id.is_empty(), "Preset id cannot be empty");
            assert!(!preset.name.is_empty(), "Preset name cannot be empty");
            assert!(!preset.icon.is_empty(), "Preset icon cannot be empty");
            assert!(
                !preset.description.is_empty(),
                "Preset description cannot be empty"
            );
            assert!(
                !preset.target_user.is_empty(),
                "Preset target_user cannot be empty"
            );
        }
    }

    #[test]
    fn test_minimal_preset_structure() {
        let minimal = PRESET_REGISTRY
            .iter()
            .find(|p| p.id == "minimal")
            .expect("Minimal preset should exist");

        assert_eq!(minimal.name, "Minimal");
        assert_eq!(minimal.icon, "✨");
        assert_eq!(minimal.config.framework, FrameworkType::Zap);
        assert_eq!(minimal.config.prompt_engine, Some("pure"));
        assert_eq!(minimal.config.plugins.len(), 3);
        assert!(minimal.config.plugins.contains(&"zsh-autosuggestions"));
        assert!(minimal.config.plugins.contains(&"zsh-syntax-highlighting"));
        assert!(minimal.config.plugins.contains(&"git"));
    }

    #[test]
    fn test_developer_preset_structure() {
        let developer = PRESET_REGISTRY
            .iter()
            .find(|p| p.id == "developer")
            .expect("Developer preset should exist");

        assert_eq!(developer.name, "Developer");
        assert_eq!(developer.icon, "👨‍💻");
        assert_eq!(developer.config.framework, FrameworkType::Zimfw);
        assert_eq!(developer.config.prompt_engine, Some("starship"));
        assert_eq!(developer.config.plugins.len(), 8);
        assert!(developer.config.plugins.contains(&"git"));
        assert!(developer.config.plugins.contains(&"docker"));
        assert!(developer.config.plugins.contains(&"kubectl"));
        assert!(developer.config.plugins.contains(&"fzf"));
    }

    #[test]
    fn test_presets_have_valid_frameworks() {
        for preset in PRESET_REGISTRY {
            // Just verify the framework type is one of the valid enum variants
            // The enum itself ensures only valid values
            match preset.config.framework {
                FrameworkType::OhMyZsh
                | FrameworkType::Zimfw
                | FrameworkType::Prezto
                | FrameworkType::Zinit
                | FrameworkType::Zap => {
                    // Valid framework
                }
            }
        }
    }

    #[test]
    fn test_presets_have_valid_prompt_modes() {
        for preset in PRESET_REGISTRY {
            let prompt_mode = preset.config.prompt_mode();
            match &prompt_mode {
                PromptMode::PromptEngine { engine } => {
                    assert!(
                        !engine.is_empty(),
                        "Prompt engine cannot be empty for preset {}", preset.id
                    );
                }
                PromptMode::FrameworkTheme { theme } => {
                    // Theme can be empty, but if specified should be non-empty
                    if !theme.is_empty() {
                        assert!(
                            theme.trim() == theme,
                            "Theme should not have extra whitespace"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn test_preset_plugins_are_non_empty() {
        for preset in PRESET_REGISTRY {
            for plugin in preset.config.plugins {
                assert!(
                    !plugin.is_empty(),
                    "Plugin name cannot be empty in preset {}", preset.id
                );
            }
        }
    }

    #[test]
    fn test_preset_env_vars_valid() {
        for preset in PRESET_REGISTRY {
            for (key, value) in preset.config.env_vars {
                assert!(
                    !key.is_empty(),
                    "Env var key cannot be empty in preset {}", preset.id
                );
                assert!(
                    key.chars().all(|c| c.is_alphanumeric() || c == '_'),
                    "Env var key '{key}' contains invalid characters in preset {}", preset.id
                );
                assert!(
                    !value.is_empty(),
                    "Env var value cannot be empty in preset {}", preset.id
                );
            }
        }
    }

    #[test]
    fn test_preset_shell_options_non_empty() {
        for preset in PRESET_REGISTRY {
            for option in preset.config.shell_options {
                assert!(
                    !option.is_empty(),
                    "Shell option cannot be empty in preset {}", preset.id
                );
            }
        }
    }

    #[test]
    fn test_performance_preset_uses_fast_framework() {
        let performance = PRESET_REGISTRY
            .iter()
            .find(|p| p.id == "performance")
            .expect("Performance preset should exist");

        // Zinit is one of the fastest frameworks
        assert_eq!(performance.config.framework, FrameworkType::Zinit);
    }

    #[test]
    fn test_performance_preset_has_five_plugins() {
        let performance = PRESET_REGISTRY
            .iter()
            .find(|p| p.id == "performance")
            .expect("Performance preset should exist");

        assert_eq!(
            performance.config.plugins.len(),
            5,
            "Performance preset should have exactly 5 plugins"
        );
        assert_eq!(performance.config.framework, FrameworkType::Zinit);
        assert_eq!(performance.config.prompt_engine, Some("starship"));
    }

    #[test]
    fn test_fancy_preset_has_twelve_plugins() {
        let fancy = PRESET_REGISTRY
            .iter()
            .find(|p| p.id == "fancy")
            .expect("Fancy preset should exist");

        assert_eq!(
            fancy.config.plugins.len(),
            12,
            "Fancy preset should have exactly 12 plugins"
        );
        assert_eq!(fancy.config.framework, FrameworkType::OhMyZsh);
        assert_eq!(fancy.config.framework_theme, Some("powerlevel10k"));
    }

    #[test]
    fn test_prompt_mode_helper() {
        let minimal = PRESET_REGISTRY
            .iter()
            .find(|p| p.id == "minimal")
            .expect("Minimal preset should exist");

        let prompt_mode = minimal.config.prompt_mode();
        match prompt_mode {
            PromptMode::PromptEngine { engine } => {
                assert_eq!(engine, "pure");
            }
            _ => panic!("Expected PromptEngine mode"),
        }
    }

    #[test]
    fn test_all_presets_meet_plugin_count_requirements() {
        let minimal = PRESET_REGISTRY
            .iter()
            .find(|p| p.id == "minimal")
            .expect("Minimal preset should exist");
        assert_eq!(minimal.config.plugins.len(), 3);

        let performance = PRESET_REGISTRY
            .iter()
            .find(|p| p.id == "performance")
            .expect("Performance preset should exist");
        assert_eq!(performance.config.plugins.len(), 5);

        let fancy = PRESET_REGISTRY
            .iter()
            .find(|p| p.id == "fancy")
            .expect("Fancy preset should exist");
        assert_eq!(fancy.config.plugins.len(), 12);

        let developer = PRESET_REGISTRY
            .iter()
            .find(|p| p.id == "developer")
            .expect("Developer preset should exist");
        assert_eq!(developer.config.plugins.len(), 8);
    }

    #[test]
    fn test_find_preset_by_id_exact_match() {
        let preset = find_preset_by_id("minimal");
        assert!(preset.is_some());
        assert_eq!(preset.unwrap().id, "minimal");

        let preset = find_preset_by_id("performance");
        assert!(preset.is_some());
        assert_eq!(preset.unwrap().id, "performance");

        let preset = find_preset_by_id("developer");
        assert!(preset.is_some());
        assert_eq!(preset.unwrap().id, "developer");

        let preset = find_preset_by_id("fancy");
        assert!(preset.is_some());
        assert_eq!(preset.unwrap().id, "fancy");
    }

    #[test]
    fn test_find_preset_by_id_case_insensitive() {
        // Uppercase
        let preset = find_preset_by_id("MINIMAL");
        assert!(preset.is_some());
        assert_eq!(preset.unwrap().id, "minimal");

        // Mixed case
        let preset = find_preset_by_id("Performance");
        assert!(preset.is_some());
        assert_eq!(preset.unwrap().id, "performance");

        // All caps
        let preset = find_preset_by_id("DEVELOPER");
        assert!(preset.is_some());
        assert_eq!(preset.unwrap().id, "developer");
    }

    #[test]
    fn test_find_preset_by_id_not_found() {
        let preset = find_preset_by_id("nonexistent");
        assert!(preset.is_none());

        let preset = find_preset_by_id("invalid");
        assert!(preset.is_none());

        let preset = find_preset_by_id("");
        assert!(preset.is_none());
    }
}
