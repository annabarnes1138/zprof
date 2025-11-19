//! Git operations using system git command
//! 
//! This module provides git functionality by calling the system git command
//! to avoid authentication complexity with git2 library.

use anyhow::{Context, Result};
use indicatif::ProgressBar;
use git2::Repository;
use std::path::Path;
use std::process::Command;

/// Clone a git repository using the system git command
/// This bypasses git2 authentication issues for public repositories
pub fn clone_repository(
    url: &str,
    destination: &Path,
    progress_bar: Option<&ProgressBar>,
) -> Result<Repository> {
    // Validate inputs
    if url.is_empty() {
        anyhow::bail!("Repository URL cannot be empty");
    }

    if destination.exists() {
        anyhow::bail!(
            "Destination directory already exists: {}",
            destination.display()
        );
    }

    // Create parent directory if it doesn't exist
    if let Some(parent) = destination.parent() {
        std::fs::create_dir_all(parent).with_context(|| {
            format!("Failed to create parent directory: {}", parent.display())
        })?;
    }

    log::info!("Cloning repository: {} -> {}", url, destination.display());
    
    if let Some(pb) = progress_bar {
        pb.set_message("Cloning repository...");
    }

    // Use system git with explicit configuration to disable SSH URL rewriting
    let mut cmd = Command::new("git");
    cmd.arg("clone")
       .arg("--progress") // Enable progress output
       .arg("-c").arg("url.ssh://git@github.com/.insteadof=") // Disable SSH rewriting
       .arg(url)
       .arg(destination);

    log::debug!("Executing: {:?}", cmd);
    
    let output = cmd.output()
        .with_context(|| "Failed to execute git clone command")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        anyhow::bail!("Git clone failed:\nStderr: {}\nStdout: {}", stderr, stdout);
    }

    if let Some(pb) = progress_bar {
        pb.finish_with_message("Clone complete");
    }

    // Open the cloned repository using git2
    let repo = Repository::open(destination).with_context(|| {
        format!("Failed to open cloned repository at {}", destination.display())
    })?;

    log::info!("Successfully cloned repository to {}", destination.display());
    Ok(repo)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_clone_repository_invalid_url() {
        let temp_dir = TempDir::new().unwrap();
        let dest = temp_dir.path().join("test-clone");

        let result = clone_repository("", &dest, None);
        assert!(result.is_err());
        let error_msg = format!("{}", result.err().unwrap());
        assert!(error_msg.contains("cannot be empty"));
    }

    #[test]
    fn test_clone_repository_existing_destination() {
        let temp_dir = TempDir::new().unwrap();
        let dest = temp_dir.path().join("existing");

        // Create the destination directory
        fs::create_dir_all(&dest).unwrap();

        let result = clone_repository("https://github.com/user/repo.git", &dest, None);
        assert!(result.is_err());
        let error_msg = format!("{}", result.err().unwrap());
        assert!(error_msg.contains("already exists"));
    }

    #[test]
    #[ignore]
    fn test_clone_real_repository() {
        let temp_dir = TempDir::new().unwrap();
        let dest = temp_dir.path().join("test-clone");

        // Use a small, stable repository for testing
        let url = "https://github.com/octocat/Hello-World.git";
        let result = clone_repository(url, &dest, None);

        assert!(result.is_ok());
        assert!(dest.exists());
        assert!(dest.join("README").exists());
    }
}