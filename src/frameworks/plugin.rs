//! Plugin registry with compatibility metadata
//!
//! Central registry of all available plugins with framework-specific compatibility data.

use super::{FrameworkType, ManagerSupport, Plugin, PluginCategory, PluginCompatibility};

// RECOMMENDATION CRITERIA (as of 2025-01):
// - GitHub stars > 10k OR widely included in framework defaults
// - Included in oh-my-zsh/prezto/zimfw default/popular plugins
// - Referenced in "awesome-zsh-plugins" curated lists
// - Known stable and actively maintained
// - Commonly used based on community surveys and adoption

/// Get plugins compatible with a specific framework
pub fn get_plugins_for_framework(framework: &FrameworkType) -> Vec<Plugin> {
    PLUGIN_REGISTRY
        .iter()
        .filter(|p| p.compatibility.supports_framework(framework))
        .cloned()
        .collect()
}

/// Central plugin registry with full compatibility metadata
pub const PLUGIN_REGISTRY: &[Plugin] = &[
    // === Core Utility Plugins (Recommended for most users) ===
    Plugin {
        name: "zsh-autosuggestions",
        description: "Command suggestions as you type",
        category: PluginCategory::Utility,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::OhMyZsh,
                    repo_url: None, // oh-my-zsh uses plugin names directly
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Zimfw,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Prezto,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Zinit,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Zap,
                    repo_url: Some("zsh-users/zsh-autosuggestions"),
                    recommended: true,
                },
            ],
            dependencies: &[],
        },
    },
    Plugin {
        name: "zsh-syntax-highlighting",
        description: "Syntax highlighting in terminal",
        category: PluginCategory::Utility,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::OhMyZsh,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Zimfw,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Prezto,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Zinit,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Zap,
                    repo_url: Some("zsh-users/zsh-syntax-highlighting"),
                    recommended: true,
                },
            ],
            dependencies: &[],
        },
    },
    Plugin {
        name: "fzf",
        description: "Fuzzy finder for files and history",
        category: PluginCategory::Utility,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::OhMyZsh,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Zimfw,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Prezto,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Zinit,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Zap,
                    repo_url: Some("zap-zsh/fzf"),
                    recommended: true,
                },
            ],
            dependencies: &[],
        },
    },

    // === Git Plugin ===
    Plugin {
        name: "git",
        description: "Git integration and aliases",
        category: PluginCategory::Git,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::OhMyZsh,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Zimfw,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Prezto,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Zinit,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Zap,
                    repo_url: Some("chivalryq/git-alias"),
                    recommended: true,
                },
            ],
            dependencies: &[],
        },
    },

    // === Docker & Kubernetes ===
    Plugin {
        name: "docker",
        description: "Docker aliases and completion",
        category: PluginCategory::Docker,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::OhMyZsh,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zimfw,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Prezto,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zinit,
                    repo_url: None,
                    recommended: false,
                },
                // Zap: Docker CLI provides its own completions, no plugin needed
            ],
            dependencies: &[],
        },
    },
    Plugin {
        name: "kubectl",
        description: "Kubernetes kubectl with 100+ aliases and completions",
        category: PluginCategory::Kubernetes,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::OhMyZsh,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zimfw,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Prezto,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zinit,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zap,
                    repo_url: Some("chrishrb/zsh-kubectl"),
                    recommended: false,
                },
            ],
            dependencies: &[],
        },
    },
    Plugin {
        name: "kubectx",
        description: "Fast way to switch between Kubernetes clusters and namespaces",
        category: PluginCategory::Kubernetes,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::OhMyZsh,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Zimfw,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Prezto,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Zinit,
                    repo_url: None,
                    recommended: true,
                },
                ManagerSupport {
                    framework: FrameworkType::Zap,
                    repo_url: Some("unixorn/kubectx-zshplugin"),
                    recommended: true,
                },
            ],
            dependencies: &[],
        },
    },
    Plugin {
        name: "kube-ps1",
        description: "Display current Kubernetes context and namespace in prompt",
        category: PluginCategory::Kubernetes,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::OhMyZsh,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zimfw,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Prezto,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zinit,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zap,
                    repo_url: Some("jonmosco/kube-ps1"),
                    recommended: false,
                },
            ],
            dependencies: &[],
        },
    },

    // === Language-Specific Plugins ===
    Plugin {
        name: "rust",
        description: "Rust development utilities",
        category: PluginCategory::Language,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::OhMyZsh,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zimfw,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zinit,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zap,
                    repo_url: Some("wintermi/zsh-rust"),
                    recommended: false,
                },
            ],
            dependencies: &[],
        },
    },
    Plugin {
        name: "node",
        description: "Node.js and npm utilities",
        category: PluginCategory::Language,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::OhMyZsh,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zimfw,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Prezto,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zinit,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zap,
                    repo_url: Some("zap-zsh/nvm"),
                    recommended: false,
                },
            ],
            dependencies: &[],
        },
    },
    Plugin {
        name: "python",
        description: "Python environment management",
        category: PluginCategory::Language,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::OhMyZsh,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zimfw,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Prezto,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zinit,
                    repo_url: None,
                    recommended: false,
                },
                // Zap: Python completions built-in to zsh, no plugin needed
            ],
            dependencies: &[],
        },
    },

    // === Modern CLI Tools ===
    Plugin {
        name: "zsh-vi-mode",
        description: "Vi mode improvements for zsh",
        category: PluginCategory::Utility,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::OhMyZsh,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zimfw,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zinit,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zap,
                    repo_url: Some("jeffreytse/zsh-vi-mode"),
                    recommended: false,
                },
            ],
            dependencies: &[],
        },
    },
    Plugin {
        name: "eza",
        description: "Modern ls replacement with git integration",
        category: PluginCategory::Utility,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::OhMyZsh,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zimfw,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zinit,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zap,
                    repo_url: Some("zap-zsh/exa"),
                    recommended: false,
                },
            ],
            dependencies: &[],
        },
    },
    Plugin {
        name: "bat",
        description: "Cat clone with syntax highlighting",
        category: PluginCategory::Utility,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::OhMyZsh,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zimfw,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zinit,
                    repo_url: None,
                    recommended: false,
                },
                // Zap: Bat doesn't need a plugin, tool installed directly
            ],
            dependencies: &[],
        },
    },
    Plugin {
        name: "ripgrep",
        description: "Fast recursive grep",
        category: PluginCategory::Utility,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::OhMyZsh,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zimfw,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zinit,
                    repo_url: None,
                    recommended: false,
                },
                // Zap: Ripgrep doesn't need a plugin
            ],
            dependencies: &[],
        },
    },
    Plugin {
        name: "zoxide",
        description: "Smarter cd command",
        category: PluginCategory::Utility,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::OhMyZsh,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zimfw,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Prezto,
                    repo_url: None,
                    recommended: false,
                },
                ManagerSupport {
                    framework: FrameworkType::Zinit,
                    repo_url: None,
                    recommended: false,
                },
                // Zap: Zoxide auto-inits, no plugin needed
            ],
            dependencies: &[],
        },
    },

    // === Zap-Specific Plugins ===
    Plugin {
        name: "supercharge",
        description: "Zap supercharge plugin (optional: exa for color ls)",
        category: PluginCategory::Utility,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::Zap,
                    repo_url: Some("zap-zsh/supercharge"),
                    recommended: true,
                },
            ],
            dependencies: &[],
        },
    },
    Plugin {
        name: "completions",
        description: "Enhanced completion system for Zsh",
        category: PluginCategory::Utility,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::Zap,
                    repo_url: Some("zap-zsh/completions"),
                    recommended: false,
                },
            ],
            dependencies: &[],
        },
    },
    Plugin {
        name: "vim",
        description: "Better Vi mode for Zsh",
        category: PluginCategory::Utility,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::Zap,
                    repo_url: Some("zap-zsh/vim"),
                    recommended: false,
                },
            ],
            dependencies: &[],
        },
    },
    Plugin {
        name: "sudo",
        description: "Press ESC twice to add sudo to previous command",
        category: PluginCategory::Utility,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::Zap,
                    repo_url: Some("zap-zsh/sudo"),
                    recommended: false,
                },
            ],
            dependencies: &[],
        },
    },
    Plugin {
        name: "magic-enter",
        description: "Make your enter key magical by running commands on empty prompt",
        category: PluginCategory::Utility,
        compatibility: PluginCompatibility {
            supported_managers: &[
                ManagerSupport {
                    framework: FrameworkType::Zap,
                    repo_url: Some("zap-zsh/magic-enter"),
                    recommended: false,
                },
            ],
            dependencies: &[],
        },
    },
];
