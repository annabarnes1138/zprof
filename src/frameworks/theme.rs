//! Theme registry with compatibility metadata
//!
//! Central registry of all available themes with framework-specific compatibility data.

use super::{FrameworkType, ManagerSupport, Theme, ThemeCompatibility};

// RECOMMENDATION CRITERIA (as of 2025-01):
// - GitHub stars > 5k OR widely used in the community
// - Fast startup time and performance
// - Active maintenance and compatibility
// - Good documentation and customization options
// - Popular in framework defaults or community recommendations

/// Get themes compatible with a specific framework
pub fn get_themes_for_framework(framework: &FrameworkType) -> Vec<Theme> {
    THEME_REGISTRY
        .iter()
        .filter(|t| t.compatibility.supports_framework(framework))
        .cloned()
        .collect()
}

/// Central theme registry with full compatibility metadata
pub const THEME_REGISTRY: &[Theme] = &[
    // === Recommended Modern Themes ===
    Theme {
        name: "starship",
        description: "Cross-shell Rust-powered prompt (recommended)",
        preview: "Minimal, fast, highly configurable",
        compatibility: ThemeCompatibility {
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
                    repo_url: Some("wintermi/zsh-starship"),
                    recommended: true,
                },
            ],
        },
    },
    Theme {
        name: "powerlevel10k",
        description: "Fast, feature-rich powerline theme",
        preview: "Instant prompt, rich customization",
        compatibility: ThemeCompatibility {
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
                    repo_url: Some("romkatv/powerlevel10k"),
                    recommended: true,
                },
            ],
        },
    },

    // === Minimal & Clean Themes ===
    Theme {
        name: "pure",
        description: "Minimal, asynchronous prompt",
        preview: "Clean single-line with git status",
        compatibility: ThemeCompatibility {
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
                    repo_url: Some("sindresorhus/pure"),
                    recommended: false,
                },
            ],
        },
    },
    Theme {
        name: "minimal",
        description: "Ultra-minimal prompt",
        preview: "Just path and git branch",
        compatibility: ThemeCompatibility {
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
                    repo_url: Some("subnixr/minimal"),
                    recommended: false,
                },
            ],
        },
    },
    Theme {
        name: "geometry",
        description: "Minimal git-aware prompt with colors",
        preview: "Single-line, color-coded git info",
        compatibility: ThemeCompatibility {
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
                    repo_url: Some("geometry-zsh/geometry"),
                    recommended: false,
                },
            ],
        },
    },

    // === Feature-Rich Themes ===
    Theme {
        name: "spaceship",
        description: "Modern developer-focused prompt",
        preview: "Git, versions, execution time",
        compatibility: ThemeCompatibility {
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
                    repo_url: Some("spaceship-prompt/spaceship-prompt"),
                    recommended: false,
                },
            ],
        },
    },
    Theme {
        name: "typewritten",
        description: "Two-line informative prompt",
        preview: "Top: path, git | Bottom: input",
        compatibility: ThemeCompatibility {
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
                    repo_url: Some("reobin/typewritten"),
                    recommended: false,
                },
            ],
        },
    },

    // === Classic Themes ===
    Theme {
        name: "robbyrussell",
        description: "Classic oh-my-zsh default theme",
        preview: "➜ user@host:~/dir (git:main)",
        compatibility: ThemeCompatibility {
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
                    repo_url: Some("zap-zsh/zap-prompt"),
                    recommended: false,
                },
            ],
        },
    },
    Theme {
        name: "agnoster",
        description: "Powerline theme with segments",
        preview: "User, path, git status segments",
        compatibility: ThemeCompatibility {
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
                // Zap: No direct zap equivalent
            ],
        },
    },
    Theme {
        name: "lambda",
        description: "Simple prompt with lambda symbol",
        preview: "λ path (branch) >",
        compatibility: ThemeCompatibility {
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
                    repo_url: Some("halfo/lambda-mod-zsh-theme"),
                    recommended: false,
                },
            ],
        },
    },
];
