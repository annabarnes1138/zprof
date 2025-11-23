use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;

use crate::archive::{github, import};

/// Import a profile from a .zprof archive or GitHub repository
#[derive(Debug, Args)]
pub struct ImportArgs {
    /// Path to .zprof archive file OR github:user/repo
    pub source: String,

    /// Override profile name from archive/repo
    #[arg(short, long)]
    pub name: Option<String>,

    /// Force overwrite existing profile without prompting
    #[arg(short, long)]
    pub force: bool,
}

pub fn execute(args: ImportArgs) -> Result<()> {
    // Detect import type based on source format
    if args.source.starts_with("github:") {
        execute_github_import(args)
    } else {
        execute_local_import(args)
    }
}

fn execute_github_import(args: ImportArgs) -> Result<()> {
    // Parse GitHub URL
    let (username, repo) = github::parse_github_url(&args.source)
        .context("Invalid GitHub import format")?;

    let options = github::GitHubImportOptions {
        username,
        repo_name: repo,
        profile_name_override: args.name,
        force_overwrite: args.force,
    };

    // Import from GitHub
    let profile_name = github::import_from_github(options)
        .context("Failed to import profile from GitHub")?;

    // Display success message
    println!();
    println!("✓ Profile imported from GitHub");
    println!();
    println!("  Profile: {profile_name}");
    println!("  Source: {}", args.source);
    println!("  Location: ~/.zsh-profiles/profiles/{profile_name}");
    println!();
    println!("  → Run 'zprof use {profile_name}' to activate this profile");

    Ok(())
}

fn execute_local_import(args: ImportArgs) -> Result<()> {
    let archive_path = PathBuf::from(&args.source);

    let options = import::ImportOptions {
        archive_path: archive_path.clone(),
        profile_name_override: args.name,
        force_overwrite: args.force,
    };

    // Import from local archive
    let profile_name = import::import_profile(options)
        .context("Failed to import profile")?;

    // Display success message
    println!();
    println!("✓ Profile imported successfully");
    println!();
    println!("  Profile: {profile_name}");
    println!("  Location: ~/.zsh-profiles/profiles/{profile_name}");
    println!();
    println!("  → Run 'zprof use {profile_name}' to activate this profile");

    Ok(())
}
