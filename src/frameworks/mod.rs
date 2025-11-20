//! Framework detection and management for zsh frameworks
//!
//! This module provides a unified interface for detecting and working with
//! various zsh framework installations (oh-my-zsh, zimfw, prezto, zinit, zap).

mod detector;
pub mod installer;
pub mod oh_my_zsh;
pub mod plugin;
pub mod prezto;
pub mod theme;
pub mod zap;
pub mod zimfw;
pub mod zinit;

pub use detector::detect_existing_framework;
pub use detector::FrameworkInfo;
pub use detector::FrameworkType;

use anyhow::Result;
use std::path::{Path, PathBuf};

/// Framework trait defining the standard interface for all zsh framework implementations
///
/// NOTE: This trait is currently unused as the implementation uses FrameworkType enum
/// and free functions instead. It's kept for potential future refactoring to a
/// trait-based architecture.
///
/// Each framework would implement this trait to provide consistent detection,
/// installation, and configuration capabilities.
#[allow(dead_code)]
pub trait Framework {
    /// Returns the human-readable name of the framework
    fn name(&self) -> &str;

    /// Attempts to detect this framework on the system
    ///
    /// Returns Some(FrameworkInfo) if detected, None if not found or detection fails.
    /// Detection failures should log warnings but not error.
    fn detect() -> Option<FrameworkInfo>;

    /// Installs this framework to the given profile path (not used in this story)
    fn install(profile_path: &Path) -> Result<()>;

    /// Gets available plugins for this framework (not used in this story)
    fn get_plugins() -> Vec<Plugin>;

    /// Gets available themes for this framework (not used in this story)
    fn get_themes() -> Vec<Theme>;
}

/// Plugin data model for framework plugins
#[derive(Debug, Clone)]
pub struct Plugin {
    pub name: &'static str,
    pub description: &'static str,
    // NOTE: Category is currently unused but kept for future plugin filtering/organization
    #[allow(dead_code)]
    pub category: PluginCategory,
    pub compatibility: PluginCompatibility,
}

/// Plugin categories for organizing plugins
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginCategory {
    Git,
    Docker,
    Kubernetes,
    Language,
    Utility,
}

/// Theme data model for framework themes
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    pub description: &'static str,
    pub preview: &'static str,
    pub compatibility: ThemeCompatibility,
}

/// Plugin compatibility metadata
#[derive(Debug, Clone)]
pub struct PluginCompatibility {
    /// Which managers can install this plugin
    pub supported_managers: &'static [ManagerSupport],
    /// External dependencies required for this plugin (e.g., other plugins/libraries)
    #[allow(dead_code)]
    pub dependencies: &'static [&'static str],
}

/// Theme compatibility metadata
#[derive(Debug, Clone)]
pub struct ThemeCompatibility {
    /// Which managers can install this theme
    pub supported_managers: &'static [ManagerSupport],
    /// External dependencies required for this theme (e.g., other plugins/libraries)
    #[allow(dead_code)]
    pub dependencies: &'static [&'static str],
}

/// Framework-specific support information
#[derive(Debug, Clone)]
pub struct ManagerSupport {
    /// The framework that supports this plugin/theme
    pub framework: FrameworkType,
    /// Repository URL (required for Zap, optional for others)
    pub repo_url: Option<&'static str>,
    /// Whether this plugin/theme is recommended for this framework
    pub recommended: bool,
}

impl PluginCompatibility {
    /// Check if this plugin supports the given framework
    pub fn supports_framework(&self, framework: &FrameworkType) -> bool {
        self.supported_managers
            .iter()
            .any(|m| &m.framework == framework)
    }

    /// Get the repository URL for a specific framework (if available)
    pub fn repo_url_for(&self, framework: &FrameworkType) -> Option<&str> {
        self.supported_managers
            .iter()
            .find(|m| &m.framework == framework)
            .and_then(|m| m.repo_url.as_deref())
    }

    /// Check if this plugin is recommended for the given framework
    pub fn is_recommended_for(&self, framework: &FrameworkType) -> bool {
        self.supported_managers
            .iter()
            .find(|m| &m.framework == framework)
            .map(|m| m.recommended)
            .unwrap_or(false)
    }
}

impl ThemeCompatibility {
    /// Check if this theme supports the given framework
    pub fn supports_framework(&self, framework: &FrameworkType) -> bool {
        self.supported_managers
            .iter()
            .any(|m| &m.framework == framework)
    }

    /// Get the repository URL for a specific framework (if available)
    pub fn repo_url_for(&self, framework: &FrameworkType) -> Option<&str> {
        self.supported_managers
            .iter()
            .find(|m| &m.framework == framework)
            .and_then(|m| m.repo_url.as_deref())
    }

    /// Check if this theme is recommended for the given framework
    pub fn is_recommended_for(&self, framework: &FrameworkType) -> bool {
        self.supported_managers
            .iter()
            .find(|m| &m.framework == framework)
            .map(|m| m.recommended)
            .unwrap_or(false)
    }
}

/// Helper to get home directory, respecting HOME env var for testing
/// Validates path to prevent directory traversal attacks
///
/// Returns None if:
/// - HOME env var and dirs::home_dir() both fail
/// - Path is not absolute
/// - Path contains parent directory components (..)
pub(crate) fn get_home_dir() -> Option<PathBuf> {
    let home = std::env::var("HOME")
        .ok()
        .map(PathBuf::from)
        .or_else(dirs::home_dir)?;

    // Validate home is an absolute path without parent directory components
    if !home.is_absolute()
        || home
            .components()
            .any(|c| matches!(c, std::path::Component::ParentDir))
    {
        return None;
    }

    Some(home)
}
