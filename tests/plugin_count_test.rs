//! Integration test for plugin counts across all frameworks
//!
//! Verifies AC #3: At least 10-15 popular plugins per framework

use zprof::frameworks::{
    Framework,
    oh_my_zsh::OhMyZsh,
    prezto::Prezto,
    zap::Zap,
    zimfw::Zimfw,
    zinit::Zinit,
};

#[test]
fn test_oh_my_zsh_has_min_10_plugins() {
    let plugins = OhMyZsh::get_plugins();
    assert!(
        plugins.len() >= 10 && plugins.len() <= 15,
        "oh-my-zsh should have 10-15 plugins, has {}",
        plugins.len()
    );
}

#[test]
fn test_zimfw_has_min_10_plugins() {
    let plugins = Zimfw::get_plugins();
    assert!(
        plugins.len() >= 10 && plugins.len() <= 15,
        "zimfw should have 10-15 plugins, has {}",
        plugins.len()
    );
}

#[test]
fn test_prezto_has_min_10_plugins() {
    let plugins = Prezto::get_plugins();
    assert!(
        plugins.len() >= 10 && plugins.len() <= 15,
        "prezto should have 10-15 plugins, has {}",
        plugins.len()
    );
}

#[test]
fn test_zinit_has_min_10_plugins() {
    let plugins = Zinit::get_plugins();
    assert!(
        plugins.len() >= 10 && plugins.len() <= 15,
        "zinit should have 10-15 plugins, has {}",
        plugins.len()
    );
}

#[test]
fn test_zap_has_min_10_plugins() {
    let plugins = Zap::get_plugins();
    assert!(
        plugins.len() >= 10 && plugins.len() <= 15,
        "zap should have 10-15 plugins, has {}",
        plugins.len()
    );
}

#[test]
fn test_all_plugins_have_descriptions() {
    let frameworks = vec![
        ("oh-my-zsh", OhMyZsh::get_plugins()),
        ("zimfw", Zimfw::get_plugins()),
        ("prezto", Prezto::get_plugins()),
        ("zinit", Zinit::get_plugins()),
        ("zap", Zap::get_plugins()),
    ];

    for (framework_name, plugins) in frameworks {
        for plugin in plugins {
            assert!(
                !plugin.name.is_empty(),
                "{}: Plugin has empty name",
                framework_name
            );
            assert!(
                !plugin.description.is_empty(),
                "{}: Plugin '{}' has empty description",
                framework_name,
                plugin.name
            );
        }
    }
}
