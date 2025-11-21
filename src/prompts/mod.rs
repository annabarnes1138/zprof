//! Prompt engine management for cross-framework prompt configurations
//!
//! This module provides a unified interface for working with standalone prompt
//! engines like Starship, Powerlevel10k, etc. These engines replace the framework's
//! built-in theme system and work across different shell frameworks.

mod engine;

pub use engine::{EngineMetadata, InstallMethod, PromptEngine};
