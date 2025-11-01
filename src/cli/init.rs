use anyhow::{Context, Result};
use clap::Args;

use crate::core::config::Config;
use crate::core::filesystem;

/// Initialize zprof directory structure
#[derive(Debug, Args)]
pub struct InitArgs {}

/// Execute the init command
pub fn execute(_args: InitArgs) -> Result<()> {
    // Check if already initialized
    if filesystem::is_initialized()? {
        eprintln!("→ Warning: zprof directory already exists at ~/.zsh-profiles/");
        eprintln!("→ Skipping initialization to preserve existing data");
        return Ok(());
    }

    // Create directory structure
    let base_dir = filesystem::create_zprof_structure()
        .context("Failed to create zprof directory structure")?;

    println!("✓ Created directory structure at {}", base_dir.display());
    println!("  ├── profiles/");
    println!("  ├── shared/");
    println!("  └── cache/");

    // Create shared history file
    let history_file = filesystem::create_shared_history()
        .context("Failed to create shared history file")?;
    println!("✓ Created shared history file: {}", history_file.display());

    // Create default config.toml
    let config_file = base_dir.join("config.toml");
    let config = Config::new();
    config.write_to_file(config_file.clone())
        .context("Failed to write default configuration file")?;
    println!("✓ Created configuration file: {}", config_file.display());

    println!("\nzprof initialized successfully!");
    println!("Run 'zprof --help' to see available commands");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_args_creation() {
        let args = InitArgs {};
        // Just verify the struct can be created
        assert!(format!("{:?}", args).contains("InitArgs"));
    }
}
