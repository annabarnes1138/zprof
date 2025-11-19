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

/// Check if a directory contains a git repository
pub fn is_git_repository(path: &Path) -> bool {
    Repository::open(path).is_ok()
}

/// Get the current commit hash of a git repository
pub fn get_current_commit_hash(repo_path: &Path) -> Result<Option<String>> {
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(_) => return Ok(None), // Not a git repository
    };

    let head = match repo.head() {
        Ok(head) => head,
        Err(_) => return Ok(None), // No commits yet
    };

    if let Some(oid) = head.target() {
        Ok(Some(oid.to_string()))
    } else {
        Ok(None)
    }
}

/// Get the latest tag of a git repository
pub fn get_latest_tag(repo_path: &Path) -> Result<Option<String>> {
    let repo = match Repository::open(repo_path) {
        Ok(repo) => repo,
        Err(_) => return Ok(None), // Not a git repository
    };

    let mut latest_tag = None;
    let mut latest_time = 0;

    repo.tag_foreach(|oid, name| {
        if let Ok(tag_name) = std::str::from_utf8(name) {
            if let Ok(obj) = repo.find_object(oid, None) {
                if let Some(tag) = obj.as_tag() {
                    if let Some(tagger) = tag.tagger() {
                        if tagger.when().seconds() > latest_time {
                            latest_time = tagger.when().seconds();
                            latest_tag = Some(tag_name.trim_start_matches("refs/tags/").to_string());
                        }
                    }
                }
            }
        }
        true
    })?;

    Ok(latest_tag)
}

/// Validate if a string is a valid git URL
pub fn is_valid_git_url(url: &str) -> bool {
    // Check for git-specific patterns
    if url.starts_with("git@") || url.starts_with("ssh://") {
        return true;
    }
    
    // For HTTP/HTTPS, check if it looks like a git repository
    if url.starts_with("https://") || url.starts_with("http://") {
        // Common git hosting patterns
        return url.contains("github.com") || 
               url.contains("gitlab.com") || 
               url.contains("bitbucket.org") ||
               url.ends_with(".git");
    }
    
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use std::fs;

    #[test]
    fn test_is_valid_git_url() {
        assert!(is_valid_git_url("https://github.com/user/repo.git"));
        assert!(is_valid_git_url("git@github.com:user/repo.git"));
        assert!(is_valid_git_url("ssh://git@github.com/user/repo.git"));
        assert!(!is_valid_git_url("not-a-url"));
        assert!(!is_valid_git_url("http://not-git.com"));
    }

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
    fn test_is_git_repository() {
        let temp_dir = TempDir::new().unwrap();
        
        // Not a git repository
        assert!(!is_git_repository(temp_dir.path()));
        
        // Create a git repository
        let _repo = Repository::init(temp_dir.path()).unwrap();
        assert!(is_git_repository(temp_dir.path()));
    }

    #[test]
    fn test_get_current_commit_hash_no_repo() {
        let temp_dir = TempDir::new().unwrap();
        let result = get_current_commit_hash(temp_dir.path()).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_get_latest_tag_no_repo() {
        let temp_dir = TempDir::new().unwrap();
        let result = get_latest_tag(temp_dir.path()).unwrap();
        assert!(result.is_none());
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