//! Integration tests for GitHub import functionality
//!
//! Tests the complete workflow of importing profiles from GitHub repositories.
//! Uses a mix of unit tests for parsing and integration tests for the full import flow.

// Helper to create test GitHub import options
#[allow(dead_code)]
fn create_test_options(username: &str, repo: &str) -> zprof::archive::github::GitHubImportOptions {
    zprof::archive::github::GitHubImportOptions {
        username: username.to_string(),
        repo_name: repo.to_string(),
        profile_name_override: None,
        force_overwrite: false,
    }
}

#[test]
fn test_parse_github_url_valid_formats() {
    use zprof::archive::github::parse_github_url;

    // Standard format
    let result = parse_github_url("github:user/repo");
    assert!(result.is_ok());
    let (username, repo) = result.unwrap();
    assert_eq!(username, "user");
    assert_eq!(repo, "repo");

    // With hyphens
    let result = parse_github_url("github:zsh-users/zsh-syntax-highlighting");
    assert!(result.is_ok());
    let (username, repo) = result.unwrap();
    assert_eq!(username, "zsh-users");
    assert_eq!(repo, "zsh-syntax-highlighting");

    // With underscores
    let result = parse_github_url("github:my_user/my_repo");
    assert!(result.is_ok());
    let (username, repo) = result.unwrap();
    assert_eq!(username, "my_user");
    assert_eq!(repo, "my_repo");

    // With numbers
    let result = parse_github_url("github:user123/repo456");
    assert!(result.is_ok());
    let (username, repo) = result.unwrap();
    assert_eq!(username, "user123");
    assert_eq!(repo, "repo456");
}

#[test]
fn test_parse_github_url_invalid_formats() {
    use zprof::archive::github::parse_github_url;

    // Missing prefix
    let result = parse_github_url("user/repo");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Invalid GitHub import format"));

    // Missing slash
    let result = parse_github_url("github:user-repo");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Invalid GitHub format"));

    // Too many slashes
    let result = parse_github_url("github:user/repo/extra");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("Invalid GitHub format"));

    // Empty username
    let result = parse_github_url("github:/repo");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("username cannot be empty"));

    // Empty repo
    let result = parse_github_url("github:user/");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("repository name cannot be empty"));

    // Just github:
    let result = parse_github_url("github:");
    assert!(result.is_err());

    // Whitespace username
    let result = parse_github_url("github:   /repo");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("username cannot be empty"));

    // Whitespace repo
    let result = parse_github_url("github:user/   ");
    assert!(result.is_err());
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("repository name cannot be empty"));
}

#[test]
fn test_parse_github_url_edge_cases() {
    use zprof::archive::github::parse_github_url;

    // Extra whitespace (should be trimmed)
    let result = parse_github_url("github: user / repo ");
    assert!(result.is_ok());
    let (username, repo) = result.unwrap();
    assert_eq!(username, "user");
    assert_eq!(repo, "repo");

    // Case sensitivity preserved
    let result = parse_github_url("github:MyUser/MyRepo");
    assert!(result.is_ok());
    let (username, repo) = result.unwrap();
    assert_eq!(username, "MyUser");
    assert_eq!(repo, "MyRepo");
}

// Note: The following tests require network access and valid GitHub repositories.
// They are marked with #[ignore] by default. Run with --ignored to execute them.

#[test]
#[ignore]
fn test_clone_nonexistent_repository() {
    // This test attempts to clone a repository that doesn't exist
    // Expected: Should fail with clear error message
    use zprof::archive::github::import_from_github;

    let options = create_test_options("nonexistent-user-12345", "nonexistent-repo-67890");

    let result = import_from_github(options);
    assert!(result.is_err());

    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("Repository not found") || error_msg.contains("clone"),
        "Error message should mention repository not found or clone failure: {}",
        error_msg
    );
}

#[test]
#[ignore]
fn test_clone_repo_without_manifest() {
    // This test clones a real public repo that doesn't have profile.toml
    // Expected: Should fail with clear error about missing manifest
    // Using a popular repo that's unlikely to have profile.toml
    use zprof::archive::github::import_from_github;

    let options = create_test_options("torvalds", "linux");

    let result = import_from_github(options);
    assert!(result.is_err());

    let error_msg = result.unwrap_err().to_string();
    assert!(
        error_msg.contains("profile.toml not found"),
        "Error message should mention profile.toml not found: {}",
        error_msg
    );
}

// Manual test instructions:
// To fully test GitHub import functionality:
//
// 1. Create a test GitHub repository with a valid profile.toml:
//    ```
//    mkdir test-zprof-profile
//    cd test-zprof-profile
//    cat > profile.toml << EOF
//    [profile]
//    name = "test-profile"
//    framework = "oh-my-zsh"
//    description = "Test profile for zprof GitHub import"
//    EOF
//    git init
//    git add profile.toml
//    git commit -m "Add test profile"
//    gh repo create --public --source=. --push
//    ```
//
// 2. Test import:
//    ```
//    cargo build
//    ./target/debug/zprof import github:YOUR_USERNAME/test-zprof-profile
//    ```
//
// 3. Verify:
//    - Profile was created in ~/.zsh-profiles/profiles/test-profile/
//    - profile.toml was copied
//    - .zprof-source metadata file exists with correct GitHub URL and commit hash
//    - Success message shows repository URL
//
// 4. Test name conflict:
//    ```
//    ./target/debug/zprof import github:YOUR_USERNAME/test-zprof-profile
//    # Should prompt for rename/overwrite/cancel
//    ```
//
// 5. Test --name flag:
//    ```
//    ./target/debug/zprof import github:YOUR_USERNAME/test-zprof-profile --name custom-name
//    ```
//
// 6. Test --force flag:
//    ```
//    ./target/debug/zprof import github:YOUR_USERNAME/test-zprof-profile --force
//    # Should overwrite without prompting
//    ```
//
// 7. Test with private repository (requires git credentials):
//    - Create private repo
//    - Test that git credential helper is invoked
//    - Verify authentication works
//
// 8. Test error cases:
//    - Network offline (disconnect network)
//    - Invalid repository URL
//    - Repository with invalid manifest
