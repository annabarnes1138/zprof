//! Prompt engine types and metadata
//!
//! Defines the supported prompt engines and their installation/configuration metadata.
//! Each engine is cross-framework compatible but may have specific requirements
//! (like Nerd Fonts or being zsh-only).

use serde::{Deserialize, Serialize};

/// Supported standalone prompt engines
///
/// These engines replace the framework's theme system and provide their own
/// prompt rendering. They are generally more feature-rich than framework themes
/// and work across multiple frameworks.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PromptEngine {
    /// Starship - Fast, cross-shell, Rust-powered prompt
    Starship,
    /// Powerlevel10k - Highly customizable, zsh-only, very fast
    Powerlevel10k,
    /// Oh-My-Posh - Cross-shell with many themes
    OhMyPosh,
    /// Pure - Minimal, async, fast zsh prompt
    Pure,
    /// Spaceship - Feature-rich, pretty zsh prompt
    Spaceship,
}

/// Installation method for a prompt engine
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InstallMethod {
    /// Install as a binary (via package manager or direct download)
    Binary { url: &'static str },
    /// Install via git clone
    GitClone { repo: &'static str },
    /// Install as a framework plugin
    FrameworkPlugin { plugin_name: &'static str },
}

/// Metadata describing a prompt engine's characteristics and installation
#[derive(Debug, Clone)]
pub struct EngineMetadata {
    /// Human-readable name
    pub name: &'static str,
    /// Brief description of the engine
    pub description: &'static str,
    /// Whether this engine requires Nerd Fonts to display properly
    pub requires_nerd_font: bool,
    /// How to install this engine
    #[allow(dead_code)] // Used in installer module, will be integrated in future stories
    pub installation: InstallMethod,
    /// Shell initialization command (e.g., `eval "$(starship init zsh)"`)
    #[allow(dead_code)] // Used in installer module, will be integrated in future stories
    pub init_command: &'static str,
    /// Whether this engine works across multiple shells (bash, zsh, fish)
    pub cross_shell: bool,
}

impl PromptEngine {
    /// Get metadata for this prompt engine
    pub fn metadata(&self) -> EngineMetadata {
        match self {
            PromptEngine::Starship => EngineMetadata {
                name: "Starship",
                description: "Cross-shell, Rust-powered, async prompt with extensive customization",
                requires_nerd_font: true,
                installation: InstallMethod::Binary {
                    url: "https://starship.rs/install.sh",
                },
                init_command: "eval \"$(starship init zsh)\"",
                cross_shell: true,
            },
            PromptEngine::Powerlevel10k => EngineMetadata {
                name: "Powerlevel10k",
                description: "Highly customizable zsh-only prompt with configuration wizard",
                requires_nerd_font: true,
                installation: InstallMethod::GitClone {
                    repo: "https://github.com/romkatv/powerlevel10k.git",
                },
                init_command: "source ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k/powerlevel10k.zsh-theme",
                cross_shell: false,
            },
            PromptEngine::OhMyPosh => EngineMetadata {
                name: "Oh-My-Posh",
                description: "Cross-shell prompt with many pre-built themes",
                requires_nerd_font: true,
                installation: InstallMethod::Binary {
                    url: "https://ohmyposh.dev/docs/installation/linux",
                },
                init_command: "eval \"$(oh-my-posh init zsh)\"",
                cross_shell: true,
            },
            PromptEngine::Pure => EngineMetadata {
                name: "Pure",
                description: "Minimal, async, fast zsh prompt with clean design",
                requires_nerd_font: false,
                installation: InstallMethod::FrameworkPlugin {
                    plugin_name: "sindresorhus/pure",
                },
                init_command: "autoload -U promptinit; promptinit; prompt pure",
                cross_shell: false,
            },
            PromptEngine::Spaceship => EngineMetadata {
                name: "Spaceship",
                description: "Feature-rich zsh prompt with git, docker, and language support",
                requires_nerd_font: true,
                installation: InstallMethod::GitClone {
                    repo: "https://github.com/spaceship-prompt/spaceship-prompt.git",
                },
                init_command: "source $HOME/.zprof/engines/spaceship-prompt/spaceship.zsh",
                cross_shell: false,
            },
        }
    }

    /// Returns the human-readable name of the engine
    #[allow(dead_code)] // Will be used when installer is integrated in future stories
    pub fn name(&self) -> &str {
        self.metadata().name
    }

    /// Check if this engine requires Nerd Fonts
    pub fn requires_nerd_font(&self) -> bool {
        self.metadata().requires_nerd_font
    }

    /// Check if this engine works across multiple shells
    #[allow(dead_code)] // Will be used when installer is integrated in future stories
    pub fn is_cross_shell(&self) -> bool {
        self.metadata().cross_shell
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_engines_represented() {
        // Ensure all 5 engines can be instantiated
        let engines = [
            PromptEngine::Starship,
            PromptEngine::Powerlevel10k,
            PromptEngine::OhMyPosh,
            PromptEngine::Pure,
            PromptEngine::Spaceship,
        ];
        assert_eq!(engines.len(), 5);
    }

    #[test]
    fn test_engine_serialization() {
        let engine = PromptEngine::Starship;
        let json = serde_json::to_string(&engine).unwrap();
        let deserialized: PromptEngine = serde_json::from_str(&json).unwrap();
        assert_eq!(engine, deserialized);
    }

    #[test]
    fn test_metadata_completeness() {
        // Test that each engine has non-empty metadata
        let engines = vec![
            PromptEngine::Starship,
            PromptEngine::Powerlevel10k,
            PromptEngine::OhMyPosh,
            PromptEngine::Pure,
            PromptEngine::Spaceship,
        ];

        for engine in engines {
            let meta = engine.metadata();
            assert!(!meta.name.is_empty(), "Engine name should not be empty");
            assert!(
                !meta.description.is_empty(),
                "Engine description should not be empty"
            );
            assert!(
                !meta.init_command.is_empty(),
                "Engine init command should not be empty"
            );

            // Verify installation method has valid data
            match meta.installation {
                InstallMethod::Binary { url } => {
                    assert!(!url.is_empty(), "Binary URL should not be empty");
                }
                InstallMethod::GitClone { repo } => {
                    assert!(!repo.is_empty(), "Git repo should not be empty");
                }
                InstallMethod::FrameworkPlugin { plugin_name } => {
                    assert!(!plugin_name.is_empty(), "Plugin name should not be empty");
                }
            }
        }
    }

    #[test]
    fn test_nerd_font_requirements() {
        // Test known Nerd Font requirements
        assert!(PromptEngine::Starship.requires_nerd_font());
        assert!(PromptEngine::Powerlevel10k.requires_nerd_font());
        assert!(PromptEngine::Spaceship.requires_nerd_font());
        assert!(!PromptEngine::Pure.requires_nerd_font());
        assert!(PromptEngine::OhMyPosh.requires_nerd_font());
    }

    #[test]
    fn test_cross_shell_compatibility() {
        assert!(PromptEngine::Starship.is_cross_shell());
        assert!(!PromptEngine::Powerlevel10k.is_cross_shell());
        assert!(PromptEngine::OhMyPosh.is_cross_shell());
        assert!(!PromptEngine::Pure.is_cross_shell());
        assert!(!PromptEngine::Spaceship.is_cross_shell());
    }

    #[test]
    fn test_name_accessor() {
        assert_eq!(PromptEngine::Starship.name(), "Starship");
        assert_eq!(PromptEngine::Powerlevel10k.name(), "Powerlevel10k");
        assert_eq!(PromptEngine::OhMyPosh.name(), "Oh-My-Posh");
        assert_eq!(PromptEngine::Pure.name(), "Pure");
        assert_eq!(PromptEngine::Spaceship.name(), "Spaceship");
    }

    #[test]
    fn test_installation_methods() {
        // Verify installation method types
        assert!(matches!(
            PromptEngine::Starship.metadata().installation,
            InstallMethod::Binary { .. }
        ));
        assert!(matches!(
            PromptEngine::Powerlevel10k.metadata().installation,
            InstallMethod::GitClone { .. }
        ));
        assert!(matches!(
            PromptEngine::Pure.metadata().installation,
            InstallMethod::FrameworkPlugin { .. }
        ));
    }
}
