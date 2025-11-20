use anyhow::{Context, Result};
use clap::Args;

use crate::core::{manifest, profile};

/// Show detailed information about a profile
#[derive(Debug, Args)]
pub struct ShowArgs {
    /// Name of the profile to show (defaults to current profile if not specified)
    pub profile_name: Option<String>,
}

pub fn execute(args: ShowArgs) -> Result<()> {
    // Determine which profile to show
    let profile_name = match args.profile_name {
        Some(name) => name,
        None => {
            // Get current profile
            let config_path = profile::get_config_path()?;
            if !config_path.exists() {
                anyhow::bail!(
                    "No active profile. Either specify a profile name or activate one with 'zprof use <name>'"
                );
            }
            let config = crate::core::config::Config::load_from_file(config_path)?;
            config.active_profile.ok_or_else(|| {
                anyhow::anyhow!(
                    "No active profile. Either specify a profile name or activate one with 'zprof use <name>'"
                )
            })?
        }
    };

    display_profile_details(&profile_name)?;

    Ok(())
}

/// Display detailed information about a profile
///
/// This is a public utility function that can be used by other CLI commands
/// to display profile information in a consistent format.
pub fn display_profile_details(profile_name: &str) -> Result<()> {
    // Load manifest to get detailed configuration
    let manifest_obj = manifest::load_and_validate(profile_name)
        .context("Failed to load profile manifest")?;

    // Load metadata for creation date
    let metadata = profile::load_profile_metadata(profile_name)
        .context("Failed to load profile metadata")?;

    // Display profile information
    println!();
    println!("Profile: {}", manifest_obj.profile.name);
    println!("Framework: {}", manifest_obj.profile.framework);
    println!("Theme: {}", manifest_obj.profile.theme);

    // Show creation date if available
    if let Some(created) = metadata.created {
        match format_date(&created) {
            Ok(formatted_date) => println!("Created: {}", formatted_date),
            Err(_) => println!("Created: {}", created),
        }
    }

    println!();

    // Display plugins
    if manifest_obj.plugins.enabled.is_empty() {
        println!("Plugins: (none)");
    } else {
        println!("Plugins ({}):", manifest_obj.plugins.enabled.len());
        for plugin in &manifest_obj.plugins.enabled {
            println!("  - {}", plugin);
        }
    }
    println!();

    // Display environment variables
    if !manifest_obj.env.is_empty() {
        println!("Environment Variables ({}):", manifest_obj.env.len());
        for (key, value) in &manifest_obj.env {
            println!("  {}={}", key, value);
        }
        println!();
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
}
