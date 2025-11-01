//! Framework and plugin installation orchestration
//!
//! This module handles installing zsh frameworks and their plugins to profile directories.
//! It provides progress indicators and graceful error handling.

use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::fs;

use crate::frameworks::{FrameworkType};

/// Wizard state containing all user selections for profile creation
#[derive(Debug, Clone)]
pub struct WizardState {
    pub profile_name: String,
    pub framework: FrameworkType,
    pub plugins: Vec<String>,
    pub theme: String,
}

/// Install framework and plugins to the given profile directory
///
/// This function orchestrates the complete installation process:
/// 1. Install framework to profile directory
/// 2. Install selected plugins
/// 3. Show progress indicators throughout (AC #7)
///
/// Currently implements a simplified installation that creates framework directories.
/// Future enhancement: Full framework installation with git clone and setup scripts.
pub fn install_profile(wizard_state: &WizardState, profile_path: &Path) -> Result<()> {
    let total_steps = 2 + wizard_state.plugins.len();
    let pb = ProgressBar::new(total_steps as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    // Step 1: Install framework
    pb.set_message(format!("Installing {}...", wizard_state.framework.name()));
    install_framework(&wizard_state.framework, profile_path)
        .context("Failed to install framework")?;
    pb.inc(1);

    // Step 2: Install plugins
    for plugin in &wizard_state.plugins {
        pb.set_message(format!("Installing plugin: {}...", plugin));
        install_plugin(&wizard_state.framework, plugin, profile_path)
            .context(format!("Failed to install plugin: {}", plugin))?;
        pb.inc(1);
    }

    // Step 3: Finalize
    pb.set_message("Finalizing installation...");
    pb.inc(1);

    pb.finish_with_message("Installation complete!");
    Ok(())
}

/// Install a framework to the profile directory
///
/// Creates the framework directory structure. For MVP, this creates placeholder
/// directories. Future enhancement: Clone from GitHub and run framework-specific
/// installation scripts.
fn install_framework(framework: &FrameworkType, profile_path: &Path) -> Result<()> {
    let framework_dir = match framework {
        FrameworkType::OhMyZsh => profile_path.join(".oh-my-zsh"),
        FrameworkType::Zimfw => profile_path.join(".zimfw"),
        FrameworkType::Prezto => profile_path.join(".zprezto"),
        FrameworkType::Zinit => profile_path.join(".zinit"),
        FrameworkType::Zap => profile_path.join(".zap"),
    };

    // Create framework directory
    fs::create_dir_all(&framework_dir).with_context(|| {
        format!(
            "Failed to create framework directory at {}",
            framework_dir.display()
        )
    })?;

    // Create framework-specific subdirectories
    match framework {
        FrameworkType::OhMyZsh => {
            fs::create_dir_all(framework_dir.join("plugins"))?;
            fs::create_dir_all(framework_dir.join("themes"))?;
            fs::create_dir_all(framework_dir.join("custom"))?;
        }
        FrameworkType::Zimfw => {
            fs::create_dir_all(framework_dir.join("modules"))?;
        }
        FrameworkType::Prezto => {
            fs::create_dir_all(framework_dir.join("modules"))?;
        }
        FrameworkType::Zinit => {
            fs::create_dir_all(framework_dir.join("plugins"))?;
        }
        FrameworkType::Zap => {
            fs::create_dir_all(framework_dir.join("plugins"))?;
        }
    }

    log::info!("Created framework directory structure for {:?}", framework);
    Ok(())
}

/// Install a plugin for the given framework
///
/// Creates plugin directory structure. For MVP, this creates placeholder directories.
/// Future enhancement: Clone plugin repositories and install dependencies.
fn install_plugin(
    framework: &FrameworkType,
    plugin_name: &str,
    profile_path: &Path,
) -> Result<()> {
    let plugins_dir = match framework {
        FrameworkType::OhMyZsh => profile_path.join(".oh-my-zsh/custom/plugins"),
        FrameworkType::Zimfw => profile_path.join(".zimfw/modules"),
        FrameworkType::Prezto => profile_path.join(".zprezto/modules"),
        FrameworkType::Zinit => profile_path.join(".zinit/plugins"),
        FrameworkType::Zap => profile_path.join(".zap/plugins"),
    };

    // Create plugins directory if it doesn't exist
    fs::create_dir_all(&plugins_dir)?;

    // Create plugin directory (for custom plugins)
    let plugin_dir = plugins_dir.join(plugin_name);
    if !plugin_dir.exists() {
        fs::create_dir_all(&plugin_dir).with_context(|| {
            format!(
                "Failed to create plugin directory at {}",
                plugin_dir.display()
            )
        })?;
    }

    log::info!(
        "Created plugin directory for {} in {:?}",
        plugin_name,
        framework
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_install_framework_creates_directories() {
        let temp_dir = TempDir::new().unwrap();
        let profile_path = temp_dir.path();

        // Test oh-my-zsh
        install_framework(&FrameworkType::OhMyZsh, profile_path).unwrap();
        assert!(profile_path.join(".oh-my-zsh").exists());
        assert!(profile_path.join(".oh-my-zsh/plugins").exists());
        assert!(profile_path.join(".oh-my-zsh/themes").exists());
        assert!(profile_path.join(".oh-my-zsh/custom").exists());
    }

    #[test]
    fn test_install_framework_all_types() {
        let temp_dir = TempDir::new().unwrap();
        let profile_path = temp_dir.path();

        let frameworks = vec![
            (FrameworkType::OhMyZsh, ".oh-my-zsh"),
            (FrameworkType::Zimfw, ".zimfw"),
            (FrameworkType::Prezto, ".zprezto"),
            (FrameworkType::Zinit, ".zinit"),
            (FrameworkType::Zap, ".zap"),
        ];

        for (framework, expected_dir) in frameworks {
            let framework_profile = profile_path.join(framework.name());
            fs::create_dir_all(&framework_profile).unwrap();

            install_framework(&framework, &framework_profile).unwrap();
            assert!(
                framework_profile.join(expected_dir).exists(),
                "Framework directory {} should exist for {:?}",
                expected_dir,
                framework
            );
        }
    }

    #[test]
    fn test_install_plugin_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let profile_path = temp_dir.path();

        // First install framework
        install_framework(&FrameworkType::OhMyZsh, profile_path).unwrap();

        // Then install plugin
        install_plugin(&FrameworkType::OhMyZsh, "git", profile_path).unwrap();
        assert!(profile_path
            .join(".oh-my-zsh/custom/plugins/git")
            .exists());
    }

    #[test]
    fn test_install_profile_creates_framework_and_plugins() {
        let temp_dir = TempDir::new().unwrap();
        let profile_path = temp_dir.path();

        let wizard_state = WizardState {
            profile_name: "test-profile".to_string(),
            framework: FrameworkType::Zimfw,
            plugins: vec!["git".to_string(), "docker".to_string()],
            theme: "pure".to_string(),
        };

        install_profile(&wizard_state, profile_path).unwrap();

        // Verify framework directory exists
        assert!(profile_path.join(".zimfw").exists());
        assert!(profile_path.join(".zimfw/modules").exists());

        // Verify plugin directories exist
        assert!(profile_path.join(".zimfw/modules/git").exists());
        assert!(profile_path.join(".zimfw/modules/docker").exists());
    }
}
