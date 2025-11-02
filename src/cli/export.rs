use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;

use crate::archive::export;

/// Export a profile to a .zprof archive
#[derive(Debug, Args)]
pub struct ExportArgs {
    /// Name of the profile to export
    pub profile_name: String,

    /// Output path for .zprof archive (default: ./<profile-name>.zprof)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Overwrite existing archive without prompting
    #[arg(short, long)]
    pub force: bool,
}

pub fn execute(args: ExportArgs) -> Result<()> {
    // Check for force flag and handle existing file
    let output_path = args.output.clone().or_else(|| {
        let cwd = std::env::current_dir().ok()?;
        Some(cwd.join(format!("{}.zprof", args.profile_name)))
    });

    // If force flag is set and output exists, delete it first
    if args.force {
        if let Some(ref path) = output_path {
            if path.exists() {
                std::fs::remove_file(path)
                    .with_context(|| format!("Failed to remove existing archive: {}", path.display()))?;
            }
        }
    }

    // Export profile to archive
    let archive_path = export::export_profile(&args.profile_name, args.output)
        .context("Failed to export profile")?;

    // Get archive metadata
    let metadata = std::fs::metadata(&archive_path)?;
    let size = export::format_file_size(metadata.len());

    // Count files in archive (read tar to count)
    let file_count = export::count_archive_files(&archive_path)?;

    // Display success message
    println!("✓ Profile exported successfully");
    println!();
    println!("  Profile: {}", args.profile_name);
    println!("  Archive: {}", archive_path.display());
    println!("  Size: {}", size);
    println!("  Files: {}", file_count);
    println!();
    println!("  → Share this archive with teammates or import on another machine:");
    println!("    zprof import {}", archive_path.display());

    Ok(())
}
