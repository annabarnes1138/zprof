use anyhow::{Context, Result};
use clap::Args;
use std::io::{self, Write};
use std::path::Path;

use crate::core::{filesystem, profile};

#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Name of the profile to delete
    pub profile_name: String,
}

pub fn execute(args: DeleteArgs) -> Result<()> {
    // Step 1: Validate profile exists
    let profile_path = profile::get_profile_path(&args.profile_name)?;
    profile::validate_profile(&profile_path)?;

    // Step 2: Ensure profile is not currently active (AC: #4)
    profile::validate_not_active(&args.profile_name)
        .context("Cannot delete active profile")?;

    // Step 3: Prompt for confirmation (AC: #1, #2)
    if !confirm_deletion(&args.profile_name, &profile_path)? {
        println!();
        println!("Deletion cancelled. Profile '{}' was not deleted.", args.profile_name);
        return Ok(());
    }

    // Step 4: Delete profile directory (AC: #3)
    filesystem::safe_delete_directory(
        &profile_path,
        &format!("User requested deletion of profile '{}'", args.profile_name)
    ).context("Failed to delete profile")?;

    // Step 5: Display success message (AC: #5)
    println!();
    println!("✓ Profile '{}' deleted successfully", args.profile_name);
    println!();
    println!("  Shared history and other profiles remain unaffected.");
    println!("  Backup retained at: ~/.zsh-profiles/cache/backups/");
    println!();

    Ok(())
}

/// Display confirmation prompt and get user response
fn confirm_deletion(profile_name: &str, profile_path: &Path) -> Result<bool> {
    println!();
    println!("⚠️  WARNING: This action is irreversible!");
    println!();
    println!("  Profile to delete: '{profile_name}'");
    println!("  Path: {profile_path:?}");
    println!();
    print!("Delete profile '{profile_name}'? (y/n): ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let response = input.trim().to_lowercase();
    Ok(response == "y" || response == "yes")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    fn create_test_profile_structure(profiles_dir: &Path, name: &str) -> Result<()> {
        let profile_dir = profiles_dir.join(name);
        fs::create_dir(&profile_dir)?;

        // Create profile.toml
        let manifest = format!(
            r#"[profile]
name = "{name}"
framework = "oh-my-zsh"
theme = "robbyrussell"
"#
        );
        fs::write(profile_dir.join("profile.toml"), manifest)?;

        // Create .zshrc
        fs::write(profile_dir.join(".zshrc"), "# test zshrc")?;

        Ok(())
    }

    #[test]
    fn test_confirm_deletion_acceptance() {
        // This test would require mocking stdin/stdout
        // In practice, integration tests would cover this
    }

    #[test]
    fn test_profile_structure_validation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let profiles_dir = temp_dir.path().join("profiles");
        fs::create_dir(&profiles_dir)?;

        create_test_profile_structure(&profiles_dir, "test-profile")?;

        let profile_path = profiles_dir.join("test-profile");
        assert!(profile_path.exists());
        assert!(profile_path.join("profile.toml").exists());
        assert!(profile_path.join(".zshrc").exists());

        Ok(())
    }
}
