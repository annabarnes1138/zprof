//! GitHub repository import functionality
//!
//! This module handles importing profiles directly from GitHub repositories.
//! Repositories are cloned to a temp directory, validated, then installed into
//! ~/.zsh-profiles/profiles/ with framework installation and shell regeneration.

use anyhow::{bail, ensure, Context, Result};
use git2::{FetchOptions, Repository};
use std::fs;
use std::io::Write as IoWrite;
use std::path::{Path, PathBuf};

use crate::archive::import;
use crate::shell::generator;

/// GitHub import options
pub struct GitHubImportOptions {
    pub username: String,
    pub repo_name: String,
    pub profile_name_override: Option<String>,
    pub force_overwrite: bool,
}

/// Import a profile from a GitHub repository
///
/// This function performs the complete GitHub import workflow:
/// 1. Parse and validate GitHub URL format
/// 2. Clone repository to temporary directory
/// 3. Search for profile.toml manifest in repo
/// 4. Load and validate manifest
/// 5. Handle name conflicts (prompt or force overwrite)
/// 6. Create profile directory
/// 7. Copy files from repo (exclude .git, GitHub-specific files)
/// 8. Store GitHub source metadata
/// 9. Install framework and plugins
/// 10. Regenerate shell configuration
/// 11. Clean up temporary directory
///
/// # Arguments
///
/// * `options` - GitHub import configuration options
///
/// # Returns
///
/// The final profile name (may differ from manifest if renamed)
///
/// # Errors
///
/// Returns error if:
/// - GitHub URL format is invalid
/// - Repository doesn't exist or network error
/// - Authentication fails (for private repos)
/// - Manifest not found in repository
/// - Manifest validation fails
/// - User cancels on name conflict
/// - Framework installation fails
/// - Shell configuration generation fails
pub fn import_from_github(options: GitHubImportOptions) -> Result<String> {
    log::info!(
        "Importing from GitHub: {}/{}",
        options.username,
        options.repo_name
    );

    // 1. Construct GitHub URL
    let repo_url = format!(
        "https://github.com/{}/{}",
        options.username, options.repo_name
    );
    println!("→ Cloning repository: {repo_url}");

    // 2. Create temp directory for clone
    let temp_dir = create_temp_clone_dir()?;
    log::info!("Cloning to temp dir: {temp_dir:?}");

    // 3. Clone repository
    let clone_result = clone_repository(&repo_url, &temp_dir);
    if let Err(e) = clone_result {
        // Clean up temp dir on clone failure
        let _ = fs::remove_dir_all(&temp_dir);
        return Err(e).context(format!("Failed to clone repository: {repo_url}"));
    }

    // 4. Search for profile.toml
    let manifest_path = match find_manifest_in_repo(&temp_dir) {
        Ok(p) => p,
        Err(e) => {
            // Clean up temp dir on manifest search failure
            let _ = fs::remove_dir_all(&temp_dir);
            return Err(e).context("Failed to find profile.toml in repository");
        }
    };

    println!("✓ Found profile manifest");

    // 5. Load and validate manifest (reuse from import module)
    let mut manifest = match import::load_manifest_from_path(&manifest_path) {
        Ok(m) => m,
        Err(e) => {
            // Clean up temp dir on manifest load failure
            let _ = fs::remove_dir_all(&temp_dir);
            return Err(e).context("Failed to load manifest from repository");
        }
    };

    println!("  Profile: {}", manifest.profile.name);
    println!("  Framework: {}", manifest.profile.framework);
    println!();

    // 6. Determine profile name
    let profile_name = options
        .profile_name_override
        .unwrap_or(manifest.profile.name.clone());

    // 7. Handle name conflicts (reuse from Story 2.5)
    let profile_name = match import::handle_name_conflict(&profile_name, options.force_overwrite) {
        Ok(name) => name,
        Err(e) => {
            // Clean up temp dir on conflict resolution failure
            let _ = fs::remove_dir_all(&temp_dir);
            return Err(e);
        }
    };

    // Update manifest profile name to match final name
    manifest.profile.name = profile_name.clone();

    // 8. Create profile directory
    let profile_dir = get_profile_dir(&profile_name)?;
    if let Err(e) = fs::create_dir_all(&profile_dir) {
        // Clean up temp dir on profile creation failure
        let _ = fs::remove_dir_all(&temp_dir);
        return Err(e).with_context(|| {
            format!("Failed to create profile directory: {profile_dir:?}")
        });
    }

    // 9. Copy files from repo to profile directory
    if let Err(e) = copy_repo_files(&temp_dir, &profile_dir) {
        // Clean up both temp dir and partial profile on copy failure
        let _ = fs::remove_dir_all(&temp_dir);
        let _ = fs::remove_dir_all(&profile_dir);
        return Err(e).context("Failed to copy profile files");
    }

    // 10. Store GitHub source metadata
    if let Err(e) = store_github_metadata(&profile_dir, &repo_url, &temp_dir) {
        // Clean up both temp dir and partial profile on metadata failure
        let _ = fs::remove_dir_all(&temp_dir);
        let _ = fs::remove_dir_all(&profile_dir);
        return Err(e).context("Failed to store GitHub metadata");
    }

    // 11. Install framework and plugins
    println!("→ Installing {} framework...", manifest.profile.framework);
    if let Err(e) = import::install_framework(&profile_dir, &manifest) {
        // Clean up both temp dir and partial profile on install failure
        let _ = fs::remove_dir_all(&temp_dir);
        let _ = fs::remove_dir_all(&profile_dir);
        return Err(e).context("Framework installation failed");
    }

    // 12. Regenerate shell configuration
    println!("→ Generating shell configuration...");
    if let Err(e) = generator::write_generated_files(&profile_name, &manifest) {
        // Clean up both temp dir and partial profile on generation failure
        let _ = fs::remove_dir_all(&temp_dir);
        let _ = fs::remove_dir_all(&profile_dir);
        return Err(e).context("Failed to generate shell configuration");
    }

    // 13. Clean up temp directory
    fs::remove_dir_all(&temp_dir).context("Failed to clean up temp directory")?;

    log::info!("GitHub import completed: {profile_name}");
    Ok(profile_name)
}

/// Parse GitHub URL in format: github:user/repo
///
/// # Arguments
///
/// * `input` - Input string in format "github:user/repo"
///
/// # Returns
///
/// Tuple of (username, repo_name)
///
/// # Errors
///
/// Returns error if:
/// - Input doesn't start with "github:"
/// - Format is not user/repo (missing slash, too many slashes)
/// - Username or repo name is empty
pub fn parse_github_url(input: &str) -> Result<(String, String)> {
    // Validate github: prefix
    ensure!(
        input.starts_with("github:"),
        "Invalid GitHub import format. Use: github:user/repo"
    );

    // Strip prefix
    let path = input
        .strip_prefix("github:")
        .context("Failed to strip github: prefix")?;

    // Split into parts
    let parts: Vec<&str> = path.split('/').collect();
    ensure!(
        parts.len() == 2,
        "Invalid GitHub format. Expected: github:user/repo\n  → Got: {input}\n  → Make sure to use the format: github:username/repository"
    );

    let username = parts[0].trim().to_string();
    let repo = parts[1].trim().to_string();

    ensure!(
        !username.is_empty(),
        "GitHub username cannot be empty\n  → Use format: github:username/repository"
    );
    ensure!(
        !repo.is_empty(),
        "GitHub repository name cannot be empty\n  → Use format: github:username/repository"
    );

    Ok((username, repo))
}

/// Create temporary directory for repository clone
fn create_temp_clone_dir() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;

    let temp_dir = home
        .join(".zsh-profiles")
        .join("cache")
        .join("github_clone")
        .join(format!("clone_{}", chrono::Utc::now().timestamp()));

    fs::create_dir_all(&temp_dir).context("Failed to create temp clone directory")?;

    Ok(temp_dir)
}

/// Clone GitHub repository using git2
///
/// Clones with progress callbacks and handles authentication
/// via git credential helpers.
///
/// # Arguments
///
/// * `url` - Full GitHub repository URL (https://github.com/user/repo)
/// * `dest` - Destination directory for clone
///
/// # Errors
///
/// Returns error if:
/// - Repository doesn't exist (404)
/// - Network error
/// - Authentication fails (for private repos)
/// - git2 clone operation fails
fn clone_repository(url: &str, dest: &Path) -> Result<()> {
    // Set up progress callbacks
    let mut callbacks = git2::RemoteCallbacks::new();

    // Progress callback for showing clone progress
    callbacks.transfer_progress(|progress| {
        let received = progress.received_objects();
        let total = progress.total_objects();
        if total > 0 {
            print!("\r  Receiving objects: {received}/{total}");
            std::io::stdout().flush().ok();
        }
        true
    });

    // Fetch options with callbacks
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    // Clone options
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);

    // Perform clone
    match builder.clone(url, dest) {
        Ok(_) => {
            println!("\r✓ Repository cloned successfully                    ");
            Ok(())
        }
        Err(e) => {
            // Provide helpful error messages based on error type
            let error_msg = if e.code() == git2::ErrorCode::NotFound {
                format!(
                    "✗ Repository not found: {url}\n  → Check that the repository exists and is spelled correctly\n  → For private repos, ensure you have access and git credentials are configured"
                )
            } else if e.message().contains("authentication") || e.message().contains("credentials") {
                "✗ Authentication failed\n  → This may be a private repository requiring authentication\n  → Configure git credentials: git config --global credential.helper\n  → For GitHub, you may need a Personal Access Token".to_string()
            } else {
                format!(
                    "✗ Git clone failed\n  → Error: {}\n  → Check repository URL and network connection",
                    e.message()
                )
            };

            bail!(error_msg)
        }
    }
}

/// Search for profile.toml manifest in repository
///
/// Searches in the following locations (in order):
/// 1. profile.toml (repo root)
/// 2. .zprof/profile.toml
/// 3. zprof/profile.toml
///
/// # Arguments
///
/// * `repo_dir` - Root directory of cloned repository
///
/// # Returns
///
/// Path to profile.toml manifest
///
/// # Errors
///
/// Returns error if manifest not found in any search location
fn find_manifest_in_repo(repo_dir: &Path) -> Result<PathBuf> {
    // Search locations in order of preference
    let search_paths = vec![
        repo_dir.join("profile.toml"),        // Root
        repo_dir.join(".zprof/profile.toml"), // Hidden directory
        repo_dir.join("zprof/profile.toml"),  // Subdirectory
    ];

    for path in &search_paths {
        if path.exists() {
            log::info!("Found manifest at: {path:?}");
            return Ok(path.clone());
        }
    }

    bail!(
        "profile.toml not found in repository.\n\
         Searched:\n\
         - profile.toml (root)\n\
         - .zprof/profile.toml\n\
         - zprof/profile.toml\n\
         \n\
         Add a profile.toml manifest to the repository to import it as a zprof profile."
    );
}

/// Copy profile files from repository to profile directory
///
/// Copies:
/// - profile.toml (required)
/// - Any custom configuration files from repo root
///
/// Skips:
/// - .git directory
/// - .github directory
/// - Hidden files (.gitignore, etc.)
/// - README.md
/// - LICENSE files
/// - CHANGELOG files
///
/// # Arguments
///
/// * `repo_dir` - Root directory of cloned repository
/// * `profile_dir` - Destination profile directory
///
/// # Errors
///
/// Returns error if file copy operations fail
fn copy_repo_files(repo_dir: &Path, profile_dir: &Path) -> Result<()> {
    // Find and copy profile.toml
    let src_manifest = find_manifest_in_repo(repo_dir)?;
    let dst_manifest = profile_dir.join("profile.toml");
    fs::copy(&src_manifest, &dst_manifest).context("Failed to copy profile.toml")?;
    log::info!("Copied: profile.toml");

    // Copy additional config files from repo root only (not subdirectories)
    for entry in fs::read_dir(repo_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Skip directories
        if path.is_dir() {
            continue;
        }

        let filename = path.file_name().and_then(|n| n.to_str()).unwrap_or("");

        // Skip GitHub-specific and common files
        if filename == "profile.toml" // Already copied
            || filename.starts_with('.') // Hidden files (.git, .gitignore, etc.)
            || filename.eq_ignore_ascii_case("readme.md")
            || filename.eq_ignore_ascii_case("readme")
            || filename.eq_ignore_ascii_case("license")
            || filename.eq_ignore_ascii_case("license.txt")
            || filename.eq_ignore_ascii_case("license.md")
            || filename.eq_ignore_ascii_case("changelog")
            || filename.eq_ignore_ascii_case("changelog.md")
        {
            log::debug!("Skipping file: {filename}");
            continue;
        }

        // Copy custom config file
        let dst_path = profile_dir.join(filename);
        fs::copy(&path, &dst_path).with_context(|| format!("Failed to copy {filename}"))?;
        log::info!("Copied custom file: {filename}");
    }

    Ok(())
}

/// Store GitHub source metadata in profile directory
///
/// Creates a .zprof-source file with:
/// - source_url: GitHub repository URL
/// - commit_hash: HEAD commit hash at time of import
/// - imported_date: ISO 8601 timestamp of import
///
/// This enables future features like profile updates from GitHub.
///
/// # Arguments
///
/// * `profile_dir` - Profile directory to store metadata in
/// * `repo_url` - GitHub repository URL
/// * `repo_dir` - Cloned repository directory (to read git metadata)
///
/// # Errors
///
/// Returns error if:
/// - Cannot open git repository
/// - Cannot read HEAD or commit
/// - Cannot write metadata file
fn store_github_metadata(profile_dir: &Path, repo_url: &str, repo_dir: &Path) -> Result<()> {
    // Open cloned repository
    let repo = Repository::open(repo_dir).context("Failed to open cloned repository")?;

    // Get current commit hash
    let head = repo.head().context("Failed to get HEAD")?;
    let commit = head
        .peel_to_commit()
        .context("Failed to get commit from HEAD")?;
    let commit_hash = commit.id().to_string();

    // Create metadata content
    let metadata = format!(
        "# GitHub Source Metadata\n\
         # This profile was imported from GitHub\n\
         \n\
         source_url = \"{}\"\n\
         commit_hash = \"{}\"\n\
         imported_date = \"{}\"\n",
        repo_url,
        commit_hash,
        chrono::Utc::now().to_rfc3339()
    );

    // Write metadata file
    let metadata_path = profile_dir.join(".zprof-source");
    fs::write(&metadata_path, metadata).context("Failed to write source metadata")?;

    log::info!("Stored GitHub metadata: {commit_hash}");
    Ok(())
}

/// Get the profile directory path
fn get_profile_dir(profile_name: &str) -> Result<PathBuf> {
    let home = dirs::home_dir().context("Could not find home directory")?;

    Ok(home
        .join(".zsh-profiles")
        .join("profiles")
        .join(profile_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_github_url_valid() {
        let result = parse_github_url("github:user/repo");
        assert!(result.is_ok());
        let (username, repo) = result.unwrap();
        assert_eq!(username, "user");
        assert_eq!(repo, "repo");
    }

    #[test]
    fn test_parse_github_url_with_hyphens() {
        let result = parse_github_url("github:zsh-users/zsh-syntax-highlighting");
        assert!(result.is_ok());
        let (username, repo) = result.unwrap();
        assert_eq!(username, "zsh-users");
        assert_eq!(repo, "zsh-syntax-highlighting");
    }

    #[test]
    fn test_parse_github_url_missing_prefix() {
        let result = parse_github_url("user/repo");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid GitHub import format"));
    }

    #[test]
    fn test_parse_github_url_missing_slash() {
        let result = parse_github_url("github:user-repo");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid GitHub format"));
    }

    #[test]
    fn test_parse_github_url_too_many_slashes() {
        let result = parse_github_url("github:user/repo/extra");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("Invalid GitHub format"));
    }

    #[test]
    fn test_parse_github_url_empty_username() {
        let result = parse_github_url("github:/repo");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("username cannot be empty"));
    }

    #[test]
    fn test_parse_github_url_empty_repo() {
        let result = parse_github_url("github:user/");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("repository name cannot be empty"));
    }

    #[test]
    fn test_get_profile_dir() {
        let result = get_profile_dir("test-profile");
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.to_string_lossy().contains(".zsh-profiles"));
        assert!(path.to_string_lossy().contains("profiles"));
        assert!(path.to_string_lossy().contains("test-profile"));
    }
}
