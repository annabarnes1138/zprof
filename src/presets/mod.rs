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

/// Registry of all available presets
///
/// This constant array contains all preset definitions. Adding a new preset
/// is as simple as adding another entry to this array.
pub const PRESET_REGISTRY: &[Preset] = &[
    // Minimal preset - fast startup, essential features only
    Preset {
        id: "minimal",
        name: "Minimal",
        icon: "⚡",
        description: "Lightweight setup with fast startup time",
        target_user: "Users who prioritize speed and simplicity",
        config: PresetConfig {
            framework: FrameworkType::Zimfw,
            prompt_engine: Some("starship"),
            framework_theme: None,
            plugins: &["git", "sudo"],
            env_vars: &[("EDITOR", "vim")],
            shell_options: &["HIST_IGNORE_DUPS", "SHARE_HISTORY"],
        },
    },
    // Performance preset - optimized for speed
    Preset {
        id: "performance",
        name: "Performance",
        icon: "🚀",
        description: "Optimized for maximum speed with minimal overhead",
        target_user: "Power users who need blazing fast shell startup",
        config: PresetConfig {
            framework: FrameworkType::Zinit,
            prompt_engine: Some("starship"),
            framework_theme: None,
            plugins: &["git", "zsh-autosuggestions"],
            env_vars: &[("EDITOR", "nvim"), ("ZSH_AUTOSUGGEST_BUFFER_MAX_SIZE", "20")],
            shell_options: &["HIST_IGNORE_DUPS", "HIST_FIND_NO_DUPS"],
        },
    },
    // Fancy preset - beautiful UI with lots of features
    Preset {
        id: "fancy",
        name: "Fancy",
        icon: "✨",
        description: "Beautiful terminal with rich features and visual enhancements",
        target_user: "Users who want a visually appealing and feature-rich environment",
        config: PresetConfig {
            framework: FrameworkType::OhMyZsh,
            prompt_engine: None,
            framework_theme: Some("powerlevel10k"),
            plugins: &[
                "git",
                "docker",
                "kubectl",
                "zsh-autosuggestions",
                "zsh-syntax-highlighting",
                "colored-man-pages",
            ],
            env_vars: &[("EDITOR", "code"), ("LS_COLORS", "auto")],
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
        target_user: "Software developers and engineers",
        config: PresetConfig {
            framework: FrameworkType::OhMyZsh,
            prompt_engine: Some("starship"),
            framework_theme: None,
            plugins: &[
                "git",
                "docker",
                "kubectl",
                "node",
                "python",
                "rust",
                "zsh-autosuggestions",
                "zsh-syntax-highlighting",
            ],
            env_vars: &[("EDITOR", "nvim"), ("VISUAL", "code"), ("PAGER", "less")],
            shell_options: &[
                "HIST_IGNORE_DUPS",
                "HIST_IGNORE_SPACE",
                "SHARE_HISTORY",
                "AUTO_CD",
                "CORRECT",
            ],
        },
    },
    // Beginner preset - user-friendly with helpful features
    Preset {
        id: "beginner",
        name: "Beginner",
        icon: "🌱",
        description: "Friendly setup with helpful hints and safety features",
        target_user: "New terminal users and beginners",
        config: PresetConfig {
            framework: FrameworkType::OhMyZsh,
            prompt_engine: None,
            framework_theme: Some("robbyrussell"),
            plugins: &[
                "git",
                "colored-man-pages",
                "command-not-found",
                "zsh-autosuggestions",
            ],
            env_vars: &[("EDITOR", "nano")],
            shell_options: &["CORRECT", "CORRECT_ALL", "SHARE_HISTORY"],
        },
    },
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preset_registry_count() {
        assert_eq!(
            PRESET_REGISTRY.len(),
            5,
            "Should have exactly 5 presets"
        );
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
        let expected_ids = vec!["minimal", "performance", "fancy", "developer", "beginner"];
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
        assert_eq!(minimal.icon, "⚡");
        assert_eq!(minimal.config.framework, FrameworkType::Zimfw);
        assert_eq!(minimal.config.prompt_engine, Some("starship"));
        assert!(!minimal.config.plugins.is_empty());
    }

    #[test]
    fn test_developer_preset_structure() {
        let developer = PRESET_REGISTRY
            .iter()
            .find(|p| p.id == "developer")
            .expect("Developer preset should exist");

        assert_eq!(developer.name, "Developer");
        assert_eq!(developer.icon, "👨‍💻");
        assert_eq!(developer.config.framework, FrameworkType::OhMyZsh);
        assert!(developer.config.plugins.contains(&"git"));
        assert!(developer.config.plugins.contains(&"docker"));
        assert!(developer.config.env_vars.contains(&("EDITOR", "nvim")));
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
    fn test_beginner_preset_has_helpful_features() {
        let beginner = PRESET_REGISTRY
            .iter()
            .find(|p| p.id == "beginner")
            .expect("Beginner preset should exist");

        // Should have beginner-friendly features
        assert!(beginner.config.plugins.contains(&"colored-man-pages"));
        assert!(beginner.config.shell_options.contains(&"CORRECT"));
        assert!(
            beginner.config.env_vars.contains(&("EDITOR", "nano")),
            "Beginners should use nano by default"
        );
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
                assert_eq!(engine, "starship");
            }
            _ => panic!("Expected PromptEngine mode"),
        }
    }
}
