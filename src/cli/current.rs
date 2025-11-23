use anyhow::{Context, Result};
use clap::Args;

use crate::core::{config::Config, profile};

/// Display the currently active profile
#[derive(Debug, Args)]
pub struct CurrentArgs {}

pub fn execute(_args: CurrentArgs) -> Result<()> {
    // Get config path
    let config_path = profile::get_config_path()?;

    // Check if zsh-profiles directory exists
    if !config_path.parent().unwrap().exists() {
        anyhow::bail!(
            "✗ zprof directory not found\n\n\
             Run 'zprof init' to initialize the directory structure first."
        );
    }

    // Check if config file exists
    if !config_path.exists() {
        println!("No active profile. Use 'zprof use <name>' to activate a profile.");
        return Ok(());
    }

    // Load config
    let config = Config::load_from_file(config_path.clone())
        .with_context(|| {
            format!(
                "✗ Failed to read config file\n\n\
                 The config file at {} may be corrupted.\n\
                 Try running 'zprof init' to reinitialize.",
                config_path.display()
            )
        })?;

    // Check if there's an active profile
    let active_profile_name = match config.active_profile {
        Some(name) => name,
        None => {
            println!("No active profile. Use 'zprof use <name>' to activate a profile.");
            return Ok(());
        }
    };

    // Load profile metadata
    let metadata = profile::load_profile_metadata(&active_profile_name)
        .context("✗ Failed to load active profile")?;

    // Display profile information
    println!("Current profile: {}\n", metadata.name);
    println!("Framework: {}", metadata.framework);

    // Format and display creation date if available
    if let Some(created) = metadata.created {
        match format_date(&created) {
            Ok(formatted_date) => println!("Created: {formatted_date}"),
            Err(_) => println!("Created: {created}"), // Fallback to raw timestamp if parsing fails
        }
    }

    Ok(())
}

/// Format ISO 8601 timestamp to human-readable format (e.g., "Oct 31, 2025")
fn format_date(iso_timestamp: &str) -> Result<String> {
    use chrono::{DateTime, Utc};

    let datetime = DateTime::parse_from_rfc3339(iso_timestamp)
        .context("Failed to parse timestamp")?;
    let datetime_utc: DateTime<Utc> = datetime.into();

    Ok(datetime_utc.format("%b %d, %Y").to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_date() -> Result<()> {
        let formatted = format_date("2025-10-31T14:30:00Z")?;
        assert_eq!(formatted, "Oct 31, 2025");
        Ok(())
    }

    #[test]
    fn test_format_date_different_month() -> Result<()> {
        let formatted = format_date("2025-01-15T10:00:00Z")?;
        assert_eq!(formatted, "Jan 15, 2025");
        Ok(())
    }
}
