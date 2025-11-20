//! Property-based tests for plugin and theme metadata integrity
//!
//! These tests validate that the plugin and theme registries maintain
//! structural consistency and follow the required metadata rules.

use zprof::frameworks::{
    plugin::PLUGIN_REGISTRY, theme::THEME_REGISTRY, FrameworkType,
};

/// Property 1: No plugin appears in TUI if unsupported
/// For all plugins and frameworks: if visible, then supported
#[test]
fn test_plugin_visibility_implies_support() {
    for plugin in PLUGIN_REGISTRY.iter() {
        for framework in &[
            FrameworkType::OhMyZsh,
            FrameworkType::Zimfw,
            FrameworkType::Prezto,
            FrameworkType::Zinit,
            FrameworkType::Zap,
        ] {
            // If a plugin is in the registry and the framework exists in supported_managers,
            // then it must truly support that framework
            let has_entry = plugin
                .compatibility
                .supported_managers
                .iter()
                .any(|m| &m.framework == framework);

            let supports = plugin.compatibility.supports_framework(framework);

            assert_eq!(
                has_entry, supports,
                "Plugin '{}' has inconsistent support for {:?}",
                plugin.name, framework
            );
        }
    }
}

/// Property 2: Zap plugins must have repo URLs
/// For all plugins: if supports Zap, then must have repo_url
#[test]
fn test_zap_plugins_have_repo_urls() {
    for plugin in PLUGIN_REGISTRY.iter() {
        if plugin
            .compatibility
            .supports_framework(&FrameworkType::Zap)
        {
            let repo_url = plugin
                .compatibility
                .repo_url_for(&FrameworkType::Zap);

            assert!(
                repo_url.is_some() && !repo_url.unwrap().is_empty(),
                "Plugin '{}' supports Zap but has no repo URL",
                plugin.name
            );
        }
    }
}

/// Property 3: Recommended ⊆ Compatible
/// For all plugins and frameworks: if recommended, then must be supported
#[test]
fn test_recommended_implies_supported() {
    for plugin in PLUGIN_REGISTRY.iter() {
        for manager_support in plugin.compatibility.supported_managers.iter() {
            if manager_support.recommended {
                assert!(
                    plugin
                        .compatibility
                        .supports_framework(&manager_support.framework),
                    "Plugin '{}' is recommended for {:?} but not supported",
                    plugin.name, manager_support.framework
                );
            }
        }
    }
}

/// Property 4: Symmetric - if compatible data exists, framework must be in supported list
/// This test verifies that the supports_framework() method correctly reflects
/// the presence of a framework in the supported_managers list
#[test]
fn test_framework_support_symmetry() {
    for plugin in PLUGIN_REGISTRY.iter() {
        for framework in &[
            FrameworkType::OhMyZsh,
            FrameworkType::Zimfw,
            FrameworkType::Prezto,
            FrameworkType::Zinit,
            FrameworkType::Zap,
        ] {
            let in_list = plugin
                .compatibility
                .supported_managers
                .iter()
                .any(|m| &m.framework == framework);
            let supports = plugin.compatibility.supports_framework(framework);

            assert_eq!(
                in_list, supports,
                "Plugin '{}' has asymmetric support for {:?}: in_list={}, supports={}",
                plugin.name, framework, in_list, supports
            );
        }
    }
}

/// Same property tests for themes
mod theme_tests {
    use super::*;

    #[test]
    fn test_theme_visibility_implies_support() {
        for theme in THEME_REGISTRY.iter() {
            for framework in &[
                FrameworkType::OhMyZsh,
                FrameworkType::Zimfw,
                FrameworkType::Prezto,
                FrameworkType::Zinit,
                FrameworkType::Zap,
            ] {
                let has_entry = theme
                    .compatibility
                    .supported_managers
                    .iter()
                    .any(|m| &m.framework == framework);

                let supports = theme.compatibility.supports_framework(framework);

                assert_eq!(
                    has_entry, supports,
                    "Theme '{}' has inconsistent support for {:?}",
                    theme.name, framework
                );
            }
        }
    }

    #[test]
    fn test_zap_themes_have_repo_urls() {
        for theme in THEME_REGISTRY.iter() {
            if theme.compatibility.supports_framework(&FrameworkType::Zap) {
                let repo_url = theme.compatibility.repo_url_for(&FrameworkType::Zap);

                assert!(
                    repo_url.is_some() && !repo_url.unwrap().is_empty(),
                    "Theme '{}' supports Zap but has no repo URL",
                    theme.name
                );
            }
        }
    }

    #[test]
    fn test_theme_recommended_implies_supported() {
        for theme in THEME_REGISTRY.iter() {
            for manager_support in theme.compatibility.supported_managers.iter() {
                if manager_support.recommended {
                    assert!(
                        theme
                            .compatibility
                            .supports_framework(&manager_support.framework),
                        "Theme '{}' is recommended for {:?} but not supported",
                        theme.name, manager_support.framework
                    );
                }
            }
        }
    }

    #[test]
    fn test_theme_framework_support_symmetry() {
        for theme in THEME_REGISTRY.iter() {
            for framework in &[
                FrameworkType::OhMyZsh,
                FrameworkType::Zimfw,
                FrameworkType::Prezto,
                FrameworkType::Zinit,
                FrameworkType::Zap,
            ] {
                let in_list = theme
                    .compatibility
                    .supported_managers
                    .iter()
                    .any(|m| &m.framework == framework);
                let supports = theme.compatibility.supports_framework(framework);

                assert_eq!(
                    in_list, supports,
                    "Theme '{}' has asymmetric support for {:?}: in_list={}, supports={}",
                    theme.name, framework, in_list, supports
                );
            }
        }
    }
}
