use anyhow::{Context, Result};
use clap::Args;

use crate::core::{config, manifest, profile};
use crate::shell::zdotdir;

#[derive(Debug, Args)]
pub struct UseArgs {
    /// Name of the profile to activate
    pub profile_name: String,
}

pub fn execute(args: UseArgs) -> Result<()> {
    // Step 1: Validate profile exists and is complete (AC: #6)
    let profile_path = profile::get_profile_path(&args.profile_name)?;
    profile::validate_profile(&profile_path)?;

    // Step 1b: Validate manifest schema (Story 2.1 AC#5)
    // Ensures invalid manifests prevent profile activation
    manifest::load_and_validate(&args.profile_name)
        .context("Cannot switch to profile with invalid manifest")?;

    // Step 2: Update config.toml with new active profile (AC: #5)
    config::update_active_profile(&args.profile_name)
        .context("Failed to update active profile in config")?;

    // Step 3: Set ZDOTDIR in ~/.zshenv to point to the new profile (AC: #1)
    // This persists across all future shell sessions
    zdotdir::set_active_profile(&profile_path)
        .context("Failed to set ZDOTDIR for new profile")?;

    // Step 4: Display confirmation message (AC: #5)
    println!("✓ Switching to profile '{}'", args.profile_name);
    println!();
    println!("  Location: {}", profile_path.display());
    println!("  Shared history: enabled");
    println!();
    println!("  → Start a new shell session to activate: exec zsh");

    Ok(())
}
