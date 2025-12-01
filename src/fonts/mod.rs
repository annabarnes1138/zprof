//! Font management for Nerd Fonts auto-installation
//!
//! This module provides font detection, download, installation, and configuration
//! for Nerd Fonts required by modern prompt engines (Starship, Powerlevel10k, etc.).

pub mod detector;
pub mod nerd_fonts;

// Re-export main detector API for convenient access
pub use detector::{detect_nerd_fonts, DetectionResult};
