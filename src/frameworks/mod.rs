//! Framework detection and management for zsh frameworks
//!
//! This module provides a unified interface for detecting and working with
//! various zsh framework installations (oh-my-zsh, zimfw, prezto, zinit, zap).

mod detector;
mod oh_my_zsh;
mod prezto;
mod zap;
mod zimfw;
mod zinit;

pub use detector::detect_existing_framework;
pub use detector::FrameworkInfo;
pub use detector::FrameworkType;

use anyhow::Result;
use std::path::{Path, PathBuf};

/// Framework trait defining the standard interface for all zsh framework implementations
///
/// Each framework must implement this trait to provide consistent detection,
/// installation, and configuration capabilities.
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

/// Placeholder for Plugin type (to be implemented in future stories)
#[derive(Debug, Clone)]
pub struct Plugin {
    pub name: String,
}

/// Placeholder for Theme type (to be implemented in future stories)
#[derive(Debug, Clone)]
pub struct Theme {
    pub name: String,
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
