//! zprof - Manage multiple zsh profiles with ease
//!
//! This library provides functionality for managing zsh profiles,
//! including framework detection, profile management, and configuration.

pub mod archive;
pub mod cli;
pub mod core;
pub mod frameworks;
pub mod git;
pub mod prompts;
pub mod shell;
pub mod tui;

// Re-export commonly used items
pub use frameworks::{detect_existing_framework, FrameworkInfo, FrameworkType};
pub use prompts::engine::{EngineMetadata, InstallMethod, PromptEngine};
pub use prompts::installer::EngineInstaller;
