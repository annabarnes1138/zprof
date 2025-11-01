//! Shell configuration generation module
//!
//! Handles generation of .zshrc and .zshenv files from profile manifests.

pub mod generator;

pub use generator::generate_shell_configs;
