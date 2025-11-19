//! Framework and plugin installation orchestration
//!
//! This module handles installing zsh frameworks and their plugins to profile directories.
//! It provides progress indicators and graceful error handling.

use anyhow::{Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::path::Path;
use std::fs;

use crate::frameworks::{FrameworkType};
use crate::git::clone_repository;

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
/// Downloads and installs the actual framework from its git repository.
/// Each framework has its own installation procedure and directory structure.
pub fn install_framework(framework: &FrameworkType, profile_path: &Path) -> Result<()> {
    match framework {
        FrameworkType::OhMyZsh => install_oh_my_zsh(profile_path),
        FrameworkType::Zap => install_zap(profile_path),
        FrameworkType::Zimfw => install_zimfw_placeholder(profile_path),
        FrameworkType::Prezto => install_prezto_placeholder(profile_path),
        FrameworkType::Zinit => install_zinit_placeholder(profile_path),
    }
}

/// Install Oh-My-Zsh framework from GitHub
fn install_oh_my_zsh(profile_path: &Path) -> Result<()> {
    let framework_dir = profile_path.join(".oh-my-zsh");
    let repo_url = "https://github.com/ohmyzsh/ohmyzsh.git";
    
    log::info!("Installing Oh-My-Zsh to {}", framework_dir.display());
    
    // Clone the repository (this will handle progress tracking internally)
    clone_repository(repo_url, &framework_dir, None)
        .context("Failed to clone Oh-My-Zsh repository")?;
    
    log::info!("Oh-My-Zsh installation completed successfully");
    Ok(())
}

/// Install Zap framework from GitHub  
fn install_zap(profile_path: &Path) -> Result<()> {
    // For profile-scoped installation, we put zap directly in the profile
    // This keeps each profile's framework isolated
    let framework_dir = profile_path.join(".zap");
    let repo_url = "https://github.com/zap-zsh/zap.git";
    
    log::info!("Installing Zap to {}", framework_dir.display());
    
    // Clone the repository
    clone_repository(repo_url, &framework_dir, None)
        .context("Failed to clone Zap repository")?;
    
    log::info!("Zap installation completed successfully");
    Ok(())
}

/// Install Zimfw framework (placeholder - still creates directories)
/// TODO: Implement real zimfw installation in Phase 2
fn install_zimfw_placeholder(profile_path: &Path) -> Result<()> {
    let framework_dir = profile_path.join(".zimfw");
    
    // Create framework directory
    fs::create_dir_all(&framework_dir).with_context(|| {
        format!(
            "Failed to create framework directory at {}",
            framework_dir.display()
        )
    })?;
    
    fs::create_dir_all(framework_dir.join("modules"))?;
    
    log::info!("Created Zimfw directory structure for {:?} (placeholder)", FrameworkType::Zimfw);
    Ok(())
}

/// Install Prezto framework (placeholder - still creates directories)  
/// TODO: Implement real prezto installation in Phase 2
fn install_prezto_placeholder(profile_path: &Path) -> Result<()> {
    let framework_dir = profile_path.join(".zprezto");
    
    // Create framework directory
    fs::create_dir_all(&framework_dir).with_context(|| {
        format!(
            "Failed to create framework directory at {}",
            framework_dir.display()
        )
    })?;
    
    fs::create_dir_all(framework_dir.join("modules"))?;
    
    log::info!("Created Prezto directory structure for {:?} (placeholder)", FrameworkType::Prezto);
    Ok(())
}

/// Install Zinit framework (placeholder - still creates directories)
/// TODO: Implement real zinit installation in Phase 2  
fn install_zinit_placeholder(profile_path: &Path) -> Result<()> {
    let framework_dir = profile_path.join(".zinit");
    
    // Create framework directory
    fs::create_dir_all(&framework_dir).with_context(|| {
        format!(
            "Failed to create framework directory at {}",
            framework_dir.display()
        )
    })?;
    
    fs::create_dir_all(framework_dir.join("plugins"))?;
    
    log::info!("Created Zinit directory structure for {:?} (placeholder)", FrameworkType::Zinit);
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
    fn test_install_framework_placeholder_types() {
        let temp_dir = TempDir::new().unwrap();
        let profile_path = temp_dir.path();

        // Test placeholder frameworks that still create directories
        let placeholder_frameworks = vec![
            (FrameworkType::Zimfw, ".zimfw"),
            (FrameworkType::Prezto, ".zprezto"), 
            (FrameworkType::Zinit, ".zinit"),
        ];

        for (framework, expected_dir) in placeholder_frameworks {
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

        // Test with a placeholder framework (zimfw) since oh-my-zsh now requires git
        install_framework(&FrameworkType::Zimfw, profile_path).unwrap();

        // Then install plugin
        install_plugin(&FrameworkType::Zimfw, "git", profile_path).unwrap();
        assert!(profile_path
            .join(".zimfw/modules/git")
            .exists());
    }

    #[test]
    fn test_install_profile_creates_framework_and_plugins() {
        let temp_dir = TempDir::new().unwrap();
        let profile_path = temp_dir.path();

        // Use zimfw for testing since it's still placeholder-based
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

    // Integration tests for real git installations (marked ignore to avoid network dependency)
    #[test]
    #[ignore]
    fn test_install_oh_my_zsh_real() {
        let temp_dir = TempDir::new().unwrap();
        let profile_path = temp_dir.path();

        install_framework(&FrameworkType::OhMyZsh, profile_path).unwrap();
        
        // Verify actual oh-my-zsh was cloned
        assert!(profile_path.join(".oh-my-zsh").exists());
        assert!(profile_path.join(".oh-my-zsh/.git").exists());
        assert!(profile_path.join(".oh-my-zsh/oh-my-zsh.sh").exists());
        assert!(profile_path.join(".oh-my-zsh/plugins").exists());
        assert!(profile_path.join(".oh-my-zsh/themes").exists());
    }

    #[test]
    #[ignore]
    fn test_install_zap_real() {
        let temp_dir = TempDir::new().unwrap();
        let profile_path = temp_dir.path();

        install_framework(&FrameworkType::Zap, profile_path).unwrap();
        
        // Verify actual zap was cloned
        assert!(profile_path.join(".zap").exists());
        assert!(profile_path.join(".zap/.git").exists());
        assert!(profile_path.join(".zap/zap.zsh").exists());
    }
}
