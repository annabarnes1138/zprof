//! Prompt engine installation
//!
//! This module handles installation of standalone prompt engines like Starship,
//! Powerlevel10k, Oh-My-Posh, Pure, and Spaceship.
//!
//! Installation methods vary by engine:
//! - Binary download (Starship, Oh-My-Posh)
//! - Git clone (Powerlevel10k, Pure, Spaceship)
//! - Framework plugin (some engines)

use anyhow::{anyhow, bail, Context, Result};
use std::path::PathBuf;
use std::process::Command;

use crate::prompts::engine::{InstallMethod, PromptEngine};

/// Prompt engine installer
///
/// Handles installation of prompt engines with different installation methods.
/// Checks if engines are already installed before attempting installation.
#[allow(dead_code)] // Will be integrated into CLI in future stories
pub struct EngineInstaller {
    home_dir: PathBuf,
}

#[allow(dead_code)] // Will be integrated into CLI in future stories
impl EngineInstaller {
    /// Create a new engine installer
    ///
    /// # Errors
    ///
    /// Returns error if home directory cannot be determined
    pub fn new() -> Result<Self> {
        let home_dir = dirs::home_dir().ok_or_else(|| anyhow!("Could not determine home directory"))?;
        Ok(Self { home_dir })
    }

    /// Install a prompt engine
    ///
    /// Checks if the engine is already installed and skips installation if so.
    /// Otherwise, installs using the appropriate method for the engine.
    ///
    /// # Arguments
    ///
    /// * `engine` - The prompt engine to install
    ///
    /// # Errors
    ///
    /// Returns error if:
    /// - Installation command fails
    /// - Network is unavailable for downloads
    /// - Git clone fails
    /// - Required dependencies are missing
    pub fn install(&self, engine: &PromptEngine) -> Result<()> {
        // Check if already installed
        if self.is_installed(engine)? {
            log::info!("{} is already installed, skipping...", engine.name());
            return Ok(());
        }

        log::info!("Installing {}...", engine.name());

        let metadata = engine.metadata();
        match metadata.installation {
            InstallMethod::Binary { url } => self.install_binary(engine, url),
            InstallMethod::GitClone { repo } => self.install_git(engine, repo),
            InstallMethod::FrameworkPlugin { plugin_name } => {
                self.install_framework_plugin(engine, plugin_name)
            }
        }
    }

    /// Install engine via binary download/installer script
    ///
    /// # Arguments
    ///
    /// * `engine` - The prompt engine being installed
    /// * `url` - URL to the installation script or binary
    ///
    /// # Errors
    ///
    /// Returns error if the installation script fails or network is unavailable
    fn install_binary(&self, engine: &PromptEngine, url: &str) -> Result<()> {
        log::debug!("Installing {} via binary from {}", engine.name(), url);

        // For Starship, use the official installer
        if engine.name() == "Starship" {
            let output = Command::new("sh")
                .arg("-c")
                .arg("curl -sS https://starship.rs/install.sh | sh -s -- --yes")
                .output()
                .context("Failed to run Starship installer")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                bail!("Starship installation failed: {stderr}");
            }

            log::info!("✓ Starship installed successfully");
            return Ok(());
        }

        // For Oh-My-Posh, check for package manager
        if engine.name() == "Oh-My-Posh" {
            // Try homebrew first
            if Command::new("which")
                .arg("brew")
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
            {
                let output = Command::new("brew")
                    .args(["install", "jandedobbeleer/oh-my-posh/oh-my-posh"])
                    .output()
                    .context("Failed to run brew install for Oh-My-Posh")?;

                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    bail!("Oh-My-Posh installation via brew failed: {stderr}");
                }

                log::info!("✓ Oh-My-Posh installed successfully via brew");
                return Ok(());
            }

            // Fallback: manual installation instructions
            bail!(
                "Oh-My-Posh installation requires homebrew or manual installation.\n\
                 Please install homebrew or visit: {url}"
            );
        }

        bail!(
            "Binary installation not yet implemented for {}. URL: {}",
            engine.name(),
            url
        )
    }

    /// Install engine via git clone
    ///
    /// # Arguments
    ///
    /// * `engine` - The prompt engine being installed
    /// * `repo` - Git repository URL to clone
    ///
    /// # Errors
    ///
    /// Returns error if git clone fails or git is not available
    fn install_git(&self, engine: &PromptEngine, repo: &str) -> Result<()> {
        let install_dir = self.get_install_dir(engine);

        if install_dir.exists() {
            log::info!("{} directory already exists, skipping git clone", engine.name());
            return Ok(());
        }

        log::debug!("Cloning {} from {}", engine.name(), repo);

        // Create parent directory
        if let Some(parent) = install_dir.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create engines directory")?;
        }

        let output = Command::new("git")
            .args([
                "clone",
                "--depth=1",
                repo,
                install_dir.to_str().unwrap(),
            ])
            .output()
            .context("Failed to run git clone (is git installed?)")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("Git clone failed for {}: {}", engine.name(), stderr);
        }

        log::info!("✓ {} cloned successfully", engine.name());
        Ok(())
    }

    /// Install engine as a framework plugin
    ///
    /// # Arguments
    ///
    /// * `engine` - The prompt engine being installed
    /// * `plugin_name` - Name of the plugin/theme
    ///
    /// # Errors
    ///
    /// Returns error if plugin installation fails
    fn install_framework_plugin(&self, engine: &PromptEngine, plugin_name: &str) -> Result<()> {
        log::debug!(
            "Installing {} as framework plugin: {}",
            engine.name(),
            plugin_name
        );

        // For Pure, we can install via npm or git
        if engine.name() == "Pure" {
            let install_dir = self.get_install_dir(engine);

            if install_dir.exists() {
                log::info!("Pure directory already exists, skipping installation");
                return Ok(());
            }

            // Clone the pure repository
            std::fs::create_dir_all(install_dir.parent().unwrap())
                .context("Failed to create engines directory")?;

            let output = Command::new("git")
                .args([
                    "clone",
                    "--depth=1",
                    "https://github.com/sindresorhus/pure.git",
                    install_dir.to_str().unwrap(),
                ])
                .output()
                .context("Failed to clone Pure repository")?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                bail!("Pure installation failed: {stderr}");
            }

            log::info!("✓ Pure installed successfully");
            return Ok(());
        }

        bail!(
            "Framework plugin installation not yet implemented for {}",
            engine.name()
        )
    }

    /// Check if a prompt engine is installed
    ///
    /// # Arguments
    ///
    /// * `engine` - The prompt engine to check
    ///
    /// # Returns
    ///
    /// Returns Ok(true) if installed, Ok(false) if not, or error if check fails
    pub fn is_installed(&self, engine: &PromptEngine) -> Result<bool> {
        match engine {
            PromptEngine::Starship => {
                // Check if starship binary exists in PATH
                Command::new("which")
                    .arg("starship")
                    .output()
                    .map(|o| o.status.success())
                    .context("Failed to check for starship binary")
            }
            PromptEngine::Powerlevel10k => {
                // Check if p10k directory exists
                let p10k_dir = self
                    .home_dir
                    .join(".oh-my-zsh/custom/themes/powerlevel10k");
                Ok(p10k_dir.exists())
            }
            PromptEngine::OhMyPosh => {
                // Check if oh-my-posh binary exists in PATH
                Command::new("which")
                    .arg("oh-my-posh")
                    .output()
                    .map(|o| o.status.success())
                    .context("Failed to check for oh-my-posh binary")
            }
            PromptEngine::Pure => {
                let pure_dir = self.get_install_dir(engine);
                Ok(pure_dir.exists())
            }
            PromptEngine::Spaceship => {
                let spaceship_dir = self.get_install_dir(engine);
                Ok(spaceship_dir.exists())
            }
        }
    }

    /// Get installation directory for an engine
    ///
    /// # Arguments
    ///
    /// * `engine` - The prompt engine
    ///
    /// # Returns
    ///
    /// Path to where the engine should be installed
    fn get_install_dir(&self, engine: &PromptEngine) -> PathBuf {
        match engine {
            PromptEngine::Powerlevel10k => {
                self.home_dir
                    .join(".oh-my-zsh/custom/themes/powerlevel10k")
            }
            PromptEngine::Pure => self.home_dir.join(".zprof/engines/pure"),
            PromptEngine::Spaceship => {
                self.home_dir.join(".zprof/engines/spaceship-prompt")
            }
            // Binary installs don't have a specific directory
            _ => self.home_dir.join(".zprof/engines").join(engine.name().to_lowercase()),
        }
    }

    /// Attempt to rollback a failed installation
    ///
    /// Cleans up partial installations and provides user guidance.
    ///
    /// # Arguments
    ///
    /// * `engine` - The prompt engine whose installation failed
    ///
    /// # Errors
    ///
    /// Returns error if cleanup fails
    pub fn rollback(&self, engine: &PromptEngine) -> Result<()> {
        log::warn!("Rolling back failed installation of {}", engine.name());

        let install_dir = self.get_install_dir(engine);

        if install_dir.exists() {
            log::debug!("Removing partial installation at {install_dir:?}");
            std::fs::remove_dir_all(&install_dir).context("Failed to remove partial installation")?;
            log::info!("✓ Cleaned up partial installation");
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_installer_creation() {
        let installer = EngineInstaller::new();
        assert!(installer.is_ok(), "Should create installer successfully");
    }

    #[test]
    fn test_get_install_dir() {
        let installer = EngineInstaller::new().unwrap();

        let pure_dir = installer.get_install_dir(&PromptEngine::Pure);
        assert!(pure_dir.to_str().unwrap().contains("/.zprof/engines/pure"));

        let spaceship_dir = installer.get_install_dir(&PromptEngine::Spaceship);
        assert!(
            spaceship_dir
                .to_str()
                .unwrap()
                .contains("/.zprof/engines/spaceship-prompt")
        );

        let p10k_dir = installer.get_install_dir(&PromptEngine::Powerlevel10k);
        assert!(p10k_dir
            .to_str()
            .unwrap()
            .contains("/.oh-my-zsh/custom/themes/powerlevel10k"));
    }

    #[test]
    fn test_is_installed_check() -> Result<()> {
        let installer = EngineInstaller::new()?;

        // These may return true or false depending on system state, but should not error
        let _starship_installed = installer.is_installed(&PromptEngine::Starship)?;
        let _pure_installed = installer.is_installed(&PromptEngine::Pure)?;
        let _p10k_installed = installer.is_installed(&PromptEngine::Powerlevel10k)?;

        Ok(())
    }

    #[test]
    #[ignore] // Requires network and modifies filesystem
    fn test_install_starship() {
        let installer = EngineInstaller::new().unwrap();
        let result = installer.install(&PromptEngine::Starship);

        // Should either succeed or already be installed
        if let Err(e) = result {
            println!("Starship installation failed (may require manual setup): {e}");
        }
    }

    #[test]
    fn test_rollback() -> Result<()> {
        let installer = EngineInstaller::new()?;

        // Rollback should not fail even if nothing is installed
        installer.rollback(&PromptEngine::Pure)?;

        Ok(())
    }
}
