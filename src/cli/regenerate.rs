//! Regenerate shell configuration files from manifest
//!
//! This module implements the CLI command for Story 2.2: Generate Shell Configuration from TOML
//! It provides a command to regenerate .zshrc and .zshenv files from the profile.toml manifest.

use anyhow::{Context, Result};
use clap::Args;

use crate::core::manifest;
use crate::shell::generator;

#[derive(Debug, Args)]
pub struct RegenerateArgs {
    /// Name of the profile to regenerate
    pub profile_name: String,
}

/// Execute the regenerate command
///
/// Loads the profile's manifest, validates it, and regenerates the shell
/// configuration files (.zshrc and .zshenv) from the manifest.
///
/// This implements AC #4: Re-generation from manifest overwrites previous generated files
pub fn execute(args: RegenerateArgs) -> Result<()> {
    // Load and validate manifest (Story 2.1 integration)
    let manifest_obj = manifest::load_and_validate(&args.profile_name)
        .context("Cannot regenerate from invalid manifest")?;

    // Generate shell files (Story 2.2 core functionality)
    generator::write_generated_files(&args.profile_name, &manifest_obj)
        .context("Failed to generate shell configuration files")?;

    // Display success message
    println!();
    println!("✓ Shell configuration regenerated successfully");
    println!();
    println!("  Profile: {}", args.profile_name);
    println!("  Framework: {}", manifest_obj.profile.framework);
    println!("  Files updated:");
    println!("    - .zshrc");
    println!("    - .zshenv");
    println!();
    println!("  → Run 'zprof use {}' to activate changes", args.profile_name);
    println!();

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_regenerate_args_parsing() {
        // Test that we can construct the args struct
        let args = RegenerateArgs {
            profile_name: "work".to_string(),
        };
        assert_eq!(args.profile_name, "work");
    }
}
