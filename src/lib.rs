//! zprof - Manage multiple zsh profiles with ease
//!
//! This library provides functionality for managing zsh profiles,
//! including framework detection, profile management, and configuration.

pub mod cli;
pub mod core;
pub mod frameworks;
pub mod tui;

// Re-export commonly used items
pub use frameworks::{detect_existing_framework, FrameworkInfo, FrameworkType};
