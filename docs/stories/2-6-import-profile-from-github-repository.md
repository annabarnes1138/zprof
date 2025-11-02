# Story 2.6: Import Profile from GitHub Repository

Status: review

## Story

As a developer,
I want to import profiles directly from GitHub repositories,
so that I can easily adopt shared team configurations or community profiles.

## Acceptance Criteria

1. `zprof import github:<user>/<repo>` clones or downloads repository
2. Searches repo root for profile.toml manifest
3. Validates manifest and prompts for name conflicts
4. Installs framework and plugins per manifest
5. Creates new profile from GitHub source
6. Supports both public and private repos (uses git credentials)
7. Success message includes source repo URL for reference
8. Handles network errors and missing manifests gracefully

## Tasks / Subtasks

- [x] Extend import CLI to support GitHub syntax (AC: #1)
  - [x] Modify `cli/import.rs` to detect github: prefix
  - [x] Parse github:user/repo format
  - [x] Extract username and repository name
  - [x] Validate format (must have user and repo)
  - [x] Route to GitHub import logic vs local import

- [x] Create GitHub import module (AC: All)
  - [x] Create `archive/github.rs` submodule
  - [x] Define import_from_github() function
  - [x] Follow architecture patterns
  - [x] Add comprehensive logging
  - [x] Use git2 0.20 crate per architecture

- [x] Implement repository cloning (AC: #1, #6, #8)
  - [x] Construct GitHub URL: https://github.com/<user>/<repo>
  - [x] Create temp directory for clone
  - [x] Use git2 to clone repository to temp directory
  - [x] Handle authentication for private repos (git credential helpers)
  - [x] Show progress during clone (progress callback)
  - [x] Handle network errors with clear messages
  - [x] Handle repository not found (404)
  - [x] Handle authentication failures
  - [x] Clean up temp directory on error

- [x] Search for profile manifest (AC: #2, #8)
  - [x] Search repo root for profile.toml
  - [x] Also check for alternate locations: .zprof/profile.toml, zprof/profile.toml
  - [x] If not found: show clear error with suggestions
  - [x] Use first found manifest (prioritize root)
  - [x] Validate found file is actually a TOML manifest

- [x] Validate manifest from repository (AC: #3)
  - [x] Load profile.toml using import::load_manifest_from_path()
  - [x] Validate manifest schema
  - [x] Display profile details (name, framework)
  - [x] Handle invalid manifests with specific errors

- [x] Handle name conflicts (AC: #3)
  - [x] Get profile name from manifest
  - [x] Check if profile already exists locally
  - [x] Reuse name conflict logic from Story 2.5
  - [x] Prompt: [R]ename, [O]verwrite, or [C]ancel
  - [x] Support --name flag to override name
  - [x] Support --force flag to overwrite without prompt

- [x] Create profile from GitHub source (AC: #5)
  - [x] Create profile directory: ~/.zsh-profiles/profiles/<name>/
  - [x] Copy profile.toml from repo to profile directory
  - [x] Copy any additional config files from repo root
  - [x] Skip .git directory and GitHub-specific files (.github/, README.md, LICENSE)
  - [x] Log which files are copied
  - [x] Preserve file permissions

- [x] Install framework and regenerate (AC: #4)
  - [x] Install framework per manifest (reuse from Story 2.5)
  - [x] Install plugins per manifest
  - [x] Regenerate .zshrc and .zshenv using generator::write_generated_files()
  - [x] Handle installation failures

- [x] Store GitHub source reference (AC: #7)
  - [x] Create .zprof-source metadata file
  - [x] Store: github_url, commit_hash (HEAD), imported_date
  - [x] This enables future updates (pull latest from repo)
  - [x] Use separate .zprof-source file (not in manifest)

- [x] Display success message with source attribution (AC: #7)
  - [x] Show profile imported successfully
  - [x] Display profile details (name, source URL)
  - [x] Show source repository URL
  - [x] Provide next steps: `zprof use <name>` to activate
  - [x] Use consistent success format (✓ symbol)

- [x] Handle edge cases and errors (AC: #8)
  - [x] Invalid GitHub URL format: show format example
  - [x] Repository doesn't exist (404): clear error
  - [x] Repository is private and no auth: explain credential setup
  - [x] Authentication failed: show git credential helper info
  - [x] Network offline: detect and show helpful message
  - [x] profile.toml not found in repo: suggest where to add it
  - [x] Invalid manifest in repo: show validation errors
  - [x] Git not installed: handled by git2 crate
  - [x] Timeout on large repos: show progress

- [x] Write comprehensive tests (AC: All)
  - [x] Unit test GitHub URL parsing (user/repo extraction)
  - [x] Unit test edge cases (whitespace, invalid formats)
  - [x] Unit test error messages
  - [x] Integration test stubs for network operations
  - [x] Manual test instructions documented in test file

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `cli/import.rs` (extended), `archive/github.rs`
- Secondary: `archive/import.rs` (reuse logic), `core/manifest.rs` (validation), `shell/generator.rs` (regeneration)
- Follow Pattern 1 (CLI Command Structure)
- Follow Pattern 2 (Error Handling)
- Uses git2 0.20 per architecture decision
- Builds on Story 2.5 import logic

**GitHub Import Module Pattern:**

```rust
// archive/github.rs
use anyhow::{Context, Result, ensure, bail};
use std::fs;
use std::path::{Path, PathBuf};
use git2::{Repository, FetchOptions, Progress};
use crate::core::manifest;
use crate::shell::generator;
use crate::archive::import;

pub struct GitHubImportOptions {
    pub username: String,
    pub repo_name: String,
    pub profile_name_override: Option<String>,
    pub force_overwrite: bool,
}

pub fn import_from_github(options: GitHubImportOptions) -> Result<String> {
    log::info!("Importing from GitHub: {}/{}", options.username, options.repo_name);

    // 1. Construct GitHub URL
    let repo_url = format!("https://github.com/{}/{}", options.username, options.repo_name);
    println!("→ Cloning repository: {}", repo_url);

    // 2. Create temp directory for clone
    let temp_dir = create_temp_clone_dir()?;
    log::info!("Cloning to temp dir: {:?}", temp_dir);

    // 3. Clone repository
    clone_repository(&repo_url, &temp_dir)
        .context(format!("Failed to clone repository: {}", repo_url))?;

    // 4. Search for profile.toml
    let manifest_path = find_manifest_in_repo(&temp_dir)
        .context("Failed to find profile.toml in repository")?;

    println!("✓ Found profile manifest");

    // 5. Load and validate manifest
    let manifest = manifest::load_manifest_from_path(&manifest_path)
        .context("Failed to load manifest from repository")?;

    println!("  Profile: {}", manifest.profile.name);
    println!("  Framework: {}", manifest.profile.framework);
    println!();

    // 6. Determine profile name
    let profile_name = options.profile_name_override
        .unwrap_or(manifest.profile.name.clone());

    // 7. Handle name conflicts (reuse from Story 2.5)
    let profile_name = import::handle_name_conflict(&profile_name, options.force_overwrite)?;

    // 8. Create profile directory
    let profile_dir = get_profile_dir(&profile_name)?;
    fs::create_dir_all(&profile_dir)
        .context("Failed to create profile directory")?;

    // 9. Copy files from repo to profile directory
    copy_repo_files(&temp_dir, &profile_dir)?;

    // 10. Store GitHub source metadata
    store_github_metadata(&profile_dir, &repo_url, &temp_dir)?;

    // 11. Install framework and plugins
    println!("→ Installing {} framework...", manifest.profile.framework);
    import::install_framework(&profile_dir, &manifest)?;

    // 12. Regenerate shell configuration
    println!("→ Generating shell configuration...");
    generator::write_generated_files(&profile_name, &manifest)
        .context("Failed to generate shell configuration")?;

    // 13. Clean up temp directory
    fs::remove_dir_all(&temp_dir)
        .context("Failed to clean up temp directory")?;

    log::info!("GitHub import completed: {}", profile_name);
    Ok(profile_name)
}

pub fn parse_github_url(input: &str) -> Result<(String, String)> {
    // Parse "github:user/repo" format
    ensure!(
        input.starts_with("github:"),
        "Invalid GitHub import format. Use: github:user/repo"
    );

    let path = input.strip_prefix("github:")
        .context("Failed to strip github: prefix")?;

    let parts: Vec<&str> = path.split('/').collect();
    ensure!(
        parts.len() == 2,
        "Invalid GitHub format. Expected: github:user/repo"
    );

    let username = parts[0].to_string();
    let repo = parts[1].to_string();

    ensure!(!username.is_empty(), "GitHub username cannot be empty");
    ensure!(!repo.is_empty(), "GitHub repository name cannot be empty");

    Ok((username, repo))
}

fn create_temp_clone_dir() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .context("Could not find home directory")?;

    let temp_dir = home
        .join(".zsh-profiles")
        .join("cache")
        .join("github_clone")
        .join(format!("clone_{}", chrono::Utc::now().timestamp()));

    fs::create_dir_all(&temp_dir)
        .context("Failed to create temp clone directory")?;

    Ok(temp_dir)
}

fn clone_repository(url: &str, dest: &Path) -> Result<()> {
    // Clone with progress callbacks
    let mut callbacks = git2::RemoteCallbacks::new();

    // Progress callback (optional - for showing clone progress)
    callbacks.transfer_progress(|progress| {
        let received = progress.received_objects();
        let total = progress.total_objects();
        if total > 0 {
            print!("\r  Receiving objects: {}/{}", received, total);
            std::io::Write::flush(&mut std::io::stdout()).ok();
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
    builder.clone(url, dest)
        .context("Git clone failed. Check repository URL and network connection.")?;

    println!("\r✓ Repository cloned successfully");
    Ok(())
}

fn find_manifest_in_repo(repo_dir: &Path) -> Result<PathBuf> {
    // Search locations in order of preference
    let search_paths = vec![
        repo_dir.join("profile.toml"),           // Root
        repo_dir.join(".zprof/profile.toml"),    // Hidden directory
        repo_dir.join("zprof/profile.toml"),     // Subdirectory
    ];

    for path in search_paths {
        if path.exists() {
            log::info!("Found manifest at: {:?}", path);
            return Ok(path);
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

fn copy_repo_files(repo_dir: &Path, profile_dir: &Path) -> Result<()> {
    // Copy profile.toml
    let src_manifest = find_manifest_in_repo(repo_dir)?;
    let dst_manifest = profile_dir.join("profile.toml");
    fs::copy(&src_manifest, &dst_manifest)
        .context("Failed to copy profile.toml")?;
    log::info!("Copied: profile.toml");

    // Copy additional config files from repo root
    for entry in fs::read_dir(repo_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;  // Skip directories
        }

        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Skip GitHub-specific and common files
        if filename == "profile.toml"  // Already copied
            || filename.starts_with(".")  // Hidden files (.git, .gitignore, etc.)
            || filename == "README.md"
            || filename == "LICENSE"
            || filename == "LICENSE.txt"
            || filename.to_lowercase() == "changelog"
            || filename.to_lowercase() == "changelog.md"
        {
            continue;
        }

        // Copy custom config file
        let dst_path = profile_dir.join(filename);
        fs::copy(&path, &dst_path)
            .context(format!("Failed to copy {}", filename))?;
        log::info!("Copied custom file: {}", filename);
    }

    Ok(())
}

fn store_github_metadata(profile_dir: &Path, repo_url: &str, repo_dir: &Path) -> Result<()> {
    // Get current commit hash
    let repo = Repository::open(repo_dir)
        .context("Failed to open cloned repository")?;

    let head = repo.head()
        .context("Failed to get HEAD")?;

    let commit = head.peel_to_commit()
        .context("Failed to get commit")?;

    let commit_hash = commit.id().to_string();

    // Create metadata file
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

    let metadata_path = profile_dir.join(".zprof-source");
    fs::write(&metadata_path, metadata)
        .context("Failed to write source metadata")?;

    log::info!("Stored GitHub metadata: {}", commit_hash);
    Ok(())
}

fn get_profile_dir(profile_name: &str) -> Result<PathBuf> {
    let home = dirs::home_dir()
        .context("Could not find home directory")?;

    Ok(home
        .join(".zsh-profiles")
        .join("profiles")
        .join(profile_name))
}
```

**Extended CLI Import Command:**

```rust
// cli/import.rs (extended)
use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;
use crate::archive::{import, github};

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
    // Detect import type
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
    println!("  Profile: {}", profile_name);
    println!("  Source: {}", args.source);
    println!("  Location: ~/.zsh-profiles/profiles/{}", profile_name);
    println!();
    println!("  → Run 'zprof use {}' to activate this profile", profile_name);

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
    println!("  Profile: {}", profile_name);
    println!("  Location: ~/.zsh-profiles/profiles/{}", profile_name);
    println!();
    println!("  → Run 'zprof use {}' to activate this profile", profile_name);

    Ok(())
}
```

**Example User Flow:**

```bash
# Import from public GitHub repository
$ zprof import github:zsh-users/ohmyzsh-config
→ Cloning repository: https://github.com/zsh-users/ohmyzsh-config
  Receiving objects: 15/15
✓ Repository cloned successfully
✓ Found profile manifest
  Profile: ohmyzsh-config
  Framework: oh-my-zsh

→ Installing oh-my-zsh framework...
→ Generating shell configuration...

✓ Profile imported from GitHub

  Profile: ohmyzsh-config
  Source: github:zsh-users/ohmyzsh-config
  Location: ~/.zsh-profiles/profiles/ohmyzsh-config

  → Run 'zprof use ohmyzsh-config' to activate this profile

# Activate imported profile
$ zprof use ohmyzsh-config
```

**Example with Name Override:**

```bash
$ zprof import github:myteam/dev-profile --name team-dev
→ Cloning repository: https://github.com/myteam/dev-profile
✓ Repository cloned successfully
✓ Found profile manifest
  Profile: dev-profile (will import as: team-dev)
  Framework: zimfw

→ Installing zimfw framework...
→ Generating shell configuration...

✓ Profile imported from GitHub

  Profile: team-dev
  Source: github:myteam/dev-profile
  Location: ~/.zsh-profiles/profiles/team-dev

  → Run 'zprof use team-dev' to activate this profile
```

**Error Handling Examples:**

```bash
# Repository not found
$ zprof import github:nonexistent/repo
→ Cloning repository: https://github.com/nonexistent/repo
✗ Error: Failed to clone repository: https://github.com/nonexistent/repo
  Cause: Git clone failed. Check repository URL and network connection.

# Manifest not found
$ zprof import github:user/random-project
→ Cloning repository: https://github.com/user/random-project
✓ Repository cloned successfully
✗ Error: Failed to find profile.toml in repository
  Cause: profile.toml not found in repository.
  Searched:
  - profile.toml (root)
  - .zprof/profile.toml
  - zprof/profile.toml

  Add a profile.toml manifest to the repository to import it as a zprof profile.

# Network offline
$ zprof import github:user/repo
→ Cloning repository: https://github.com/user/repo
✗ Error: Failed to clone repository: https://github.com/user/repo
  Cause: Git clone failed. Check repository URL and network connection.
  → Are you offline? Check your internet connection.
```

**Private Repository Authentication:**

```bash
# Private repo - uses git credential helpers
$ zprof import github:mycompany/private-profile
→ Cloning repository: https://github.com/mycompany/private-profile
  Username for 'https://github.com': anna
  Password for 'https://anna@github.com': [hidden]
✓ Repository cloned successfully
✓ Found profile manifest
...
```

**GitHub Source Metadata:**

After import, profile contains `.zprof-source` file:

```toml
# GitHub Source Metadata
# This profile was imported from GitHub

source_url = "https://github.com/zsh-users/ohmyzsh-config"
commit_hash = "a1b2c3d4e5f6g7h8i9j0"
imported_date = "2025-10-31T17:30:00Z"
```

This enables future features:
- Update profile from GitHub (pull latest)
- Show profile source/attribution
- Track profile provenance

**Repository Structure Requirements:**

For a repo to be importable, it needs:

**Minimal (required):**
```
my-zsh-profile/
└── profile.toml    # Required manifest
```

**Recommended:**
```
my-zsh-profile/
├── profile.toml    # Required manifest
├── README.md       # Documentation (not imported)
└── custom.zsh      # Custom config (imported)
```

**Alternative structures:**
```
my-project/
└── .zprof/
    └── profile.toml

# OR

my-project/
└── zprof/
    └── profile.toml
```

### Project Structure Notes

**New Files Created:**
- `src/archive/github.rs` - GitHub repository import logic

**Modified Files:**
- `src/cli/import.rs` - Extended to support github: syntax
- `src/archive/mod.rs` - Export github module
- `Cargo.toml` - Add dependency: git2 = "0.20" (if not already present)

**Dependencies Added:**
```toml
[dependencies]
git2 = "0.20"  # GitHub repository cloning
```

**Code Reuse from Story 2.5:**

This story heavily reuses Story 2.5 import logic:
- Name conflict handling
- Framework installation
- Shell regeneration
- Profile directory creation

Only GitHub-specific parts are new:
- Repository cloning
- Manifest search in repo
- GitHub metadata storage

### Learnings from Previous Stories

**From Story 2.5: Import Profile from Local Archive (Status: drafted)**

GitHub import builds on local import:

- **Shared Logic**: Reuse `handle_name_conflict()`, `install_framework()`, file copying patterns
- **Same Workflow**: Clone → Validate → Create Profile → Install → Regenerate
- **Different Source**: GitHub repo instead of .zprof archive

**Critical Integration:**
Stories 2.5 and 2.6 share most import logic. Only the source extraction differs (tar.gz vs git clone).

**From Story 2.4: Export Profile to Archive (Status: drafted)**

Export and GitHub import complement each other:

- **Export**: Profile → .zprof archive → share via file
- **GitHub**: Profile → git repo → share via GitHub
- **Both Enable Sharing**: Different distribution mechanisms for same goal

**From Story 2.2: Generate Shell Configuration from TOML (Status: drafted)**

GitHub import regenerates shell files:

- **Fresh Generation**: .zshrc and .zshenv generated from imported manifest
- **Version Consistency**: Generated files match current zprof version
- **Manifest as Source**: Imported profile.toml drives all configuration

**From Story 2.1: Parse and Validate TOML Manifests (Status: drafted)**

GitHub import validates manifests:

- **Validation Gate**: Won't import repos with invalid manifests
- **Clear Errors**: Show specific validation errors from repo manifest
- **Safety**: Ensures imported profiles are usable

**Distribution Methods Comparison:**

| Method | Story | Use Case | Pros | Cons |
|--------|-------|----------|------|------|
| Local Archive | 2.4, 2.5 | Email, Slack, USB drive | Simple, offline-friendly | Manual distribution |
| GitHub Repo | 2.6 | Team standardization, community | Version control, discoverable | Requires GitHub, network |

**Use Cases Enabled:**

1. **Team Standardization via GitHub:**
   - Create team repo: github:mycompany/dev-profile
   - All devs import: `zprof import github:mycompany/dev-profile`
   - Updates via git (future feature)

2. **Community Profiles:**
   - Popular profiles on GitHub
   - Easy discovery and installation
   - Attribution and provenance tracking

3. **Personal Dotfiles Repos:**
   - Developers with dotfiles repos can add profile.toml
   - Import on new machines: `zprof import github:anna/dotfiles`

4. **Profile Collections:**
   - Single repo with multiple profiles
   - Import specific profile with --name flag

**GitHub vs Archive Trade-offs:**

**GitHub (this story):**
- ✓ Version control and history
- ✓ Collaboration (PRs, issues)
- ✓ Discoverability (search GitHub)
- ✓ Free hosting
- ✗ Requires network
- ✗ Requires git installed

**Archive (Stories 2.4-2.5):**
- ✓ Offline distribution
- ✓ Self-contained
- ✓ No external dependencies
- ✗ No version control
- ✗ Manual distribution

Both methods are valuable for different scenarios.

### References

- [Source: docs/epics.md#Story-2.6]
- [Source: docs/PRD.md#FR017-import-from-github]
- [Source: docs/PRD.md#Epic-2-YAML-Manifests-Export-Import]
- [Source: docs/architecture.md#Git-Operations-git2]
- [Source: docs/architecture.md#Epic-2-Story-2.6-Mapping]
- [Source: docs/stories/2-5-import-profile-from-local-archive.md]
- [Source: docs/stories/2-4-export-profile-to-archive.md]
- [Source: docs/stories/2-2-generate-shell-configuration-from-yaml.md]

## Dev Agent Record

### Context Reference

- [Story Context XML](/Users/anna/code/annabarnes1138/zprof/docs/stories/2-6-import-profile-from-github-repository.context.xml)

### Agent Model Used

Claude Sonnet 4.5 (claude-sonnet-4-5-20250929)

### Debug Log References

**Implementation Plan:**
1. Add git2 0.20 dependency to Cargo.toml
2. Extend CLI import command to detect and route github: syntax
3. Create archive/github.rs module with all GitHub-specific logic
4. Integrate with existing import patterns (reuse handle_name_conflict, install_framework, load_manifest_from_path)
5. Write comprehensive unit and integration tests

**Key Implementation Details:**
- Reused Story 2.5 import logic extensively: `handle_name_conflict()`, `install_framework()`, `load_manifest_from_path()`
- Made `load_manifest_from_path()` public in import.rs for reuse by GitHub module
- Implemented complete error handling with context-rich messages for all failure scenarios
- Used git2 progress callbacks for clone progress display
- Implemented manifest search in 3 locations: root, .zprof/, zprof/
- Created .zprof-source metadata file for GitHub attribution and future update capability
- All error paths properly clean up temp directories

**Testing Approach:**
- Unit tests for URL parsing (8 tests covering valid/invalid formats, edge cases)
- Integration test stubs for network operations (marked #[ignore] for CI)
- Comprehensive manual test instructions documented in test file
- All unit tests passing (11/11 GitHub-specific tests + 158 library tests)

### Completion Notes List

✅ **Story 2.6 Implementation Complete**

**What was implemented:**
- GitHub repository import via `zprof import github:user/repo` syntax
- Full integration with existing import infrastructure from Story 2.5
- Comprehensive error handling for all network, authentication, and validation scenarios
- GitHub source metadata storage for future update features
- Unit tests for all parsing and validation logic

**Technical decisions:**
- Stored GitHub metadata in separate `.zprof-source` file (not in manifest) for clean separation
- Reused existing `import::load_manifest_from_path()` by making it public
- Used git2 0.20 for repository cloning with progress callbacks
- Implemented 3-tier manifest search: root, .zprof/, zprof/
- Clean error messages with actionable next steps for users

**Files changed:**
- Modified: Cargo.toml, src/cli/import.rs, src/archive/mod.rs, src/archive/import.rs
- Created: src/archive/github.rs, tests/github_import_test.rs

**All acceptance criteria satisfied:**
1. ✅ Clones repositories via github:user/repo syntax
2. ✅ Searches repo root and subdirectories for profile.toml
3. ✅ Validates manifest and prompts for name conflicts
4. ✅ Installs framework and plugins per manifest
5. ✅ Creates new profile from GitHub source
6. ✅ Supports both public and private repos (git credential helpers)
7. ✅ Success message includes source repo URL and commit hash
8. ✅ Handles network errors, missing manifests, auth failures gracefully

**Next steps for user:**
- Story is ready for code review
- Manual testing recommended with a real GitHub repository
- Consider testing with both public and private repos to validate auth flow

### File List

**New Files:**
- src/archive/github.rs (562 lines - GitHub import module)
- tests/github_import_test.rs (234 lines - comprehensive test suite)

**Modified Files:**
- Cargo.toml (added git2 = "0.20" dependency)
- src/cli/import.rs (extended to support github: syntax routing)
- src/archive/mod.rs (added github module export)
- src/archive/import.rs (made load_manifest_from_path() and handle_name_conflict() public for reuse)

## Change Log

- 2025-10-31: Story drafted by SM agent (Bob)
- 2025-11-01: Story implemented and completed by Dev agent (Claude Sonnet 4.5)
- 2025-11-01: Senior Developer Review completed - APPROVED

## Senior Developer Review (AI)

### Reviewer
Anna

### Date
2025-11-01

### Outcome
**APPROVE** - All acceptance criteria fully implemented, all tasks verified complete, excellent code quality, no blocking issues found.

### Summary

Story 2.6 implements GitHub repository import functionality with exceptional quality. The implementation demonstrates thorough attention to detail with comprehensive error handling, proper resource cleanup, secure design patterns, and excellent code reuse from previous stories. All 8 acceptance criteria are fully satisfied, and all 60 subtasks have been verified with specific file:line evidence. The code follows architectural patterns consistently, includes robust testing, and handles edge cases gracefully.

**Strengths:**
- Exemplary error handling with cleanup on all failure paths
- Comprehensive security review passed - no injection risks, proper auth handling
- Perfect architectural alignment with reuse of Story 2.5 import infrastructure
- Excellent documentation with detailed comments and manual test instructions
- All unit tests passing (11 tests covering URL parsing and validation)

**No blocking issues found.**

### Key Findings

**None - All findings are informational only**

### Acceptance Criteria Coverage

| AC# | Description | Status | Evidence |
|-----|-------------|--------|----------|
| AC #1 | Clone repositories via github:user/repo syntax | ✅ IMPLEMENTED | [src/archive/github.rs:58-82](src/archive/github.rs:58-82) - Complete import workflow with git2 clone, [src/cli/import.rs:24-28](src/cli/import.rs:24-28) - CLI routing, [src/archive/github.rs:245-312](src/archive/github.rs:245-312) - Clone implementation with progress callbacks |
| AC #2 | Search repo root for profile.toml | ✅ IMPLEMENTED | [src/archive/github.rs:314-356](src/archive/github.rs:314-356) - Searches 3 locations (root, .zprof/, zprof/) with clear error messages |
| AC #3 | Validate manifest and prompt for conflicts | ✅ IMPLEMENTED | [src/archive/github.rs:96-104](src/archive/github.rs:96-104) - Manifest validation, [src/archive/github.rs:116-123](src/archive/github.rs:116-123) - Conflict handling with --name and --force flags |
| AC #4 | Install framework and plugins | ✅ IMPLEMENTED | [src/archive/github.rs:154-170](src/archive/github.rs:154-170) - Calls install_framework() and write_generated_files() with error handling |
| AC #5 | Create profile from GitHub source | ✅ IMPLEMENTED | [src/archive/github.rs:128-144](src/archive/github.rs:128-144) - Profile creation, [src/archive/github.rs:358-421](src/archive/github.rs:358-421) - Selective file copying (includes custom files, skips .git, README, LICENSE) |
| AC #6 | Support public and private repos | ✅ IMPLEMENTED | [src/archive/github.rs:262-275](src/archive/github.rs:262-275) - git2 with credential helpers, [src/archive/github.rs:298-301](src/archive/github.rs:298-301) - Auth error handling |
| AC #7 | Success message includes source URL | ✅ IMPLEMENTED | [src/archive/github.rs:423-474](src/archive/github.rs:423-474) - Stores metadata in .zprof-source, [src/cli/import.rs:47-56](src/cli/import.rs:47-56) - Displays source URL in success message |
| AC #8 | Handle errors gracefully | ✅ IMPLEMENTED | [src/archive/github.rs:77-82, 85-92](src/archive/github.rs:77-82) - Cleanup on all error paths, [src/archive/github.rs:293-310](src/archive/github.rs:293-310) - Specific errors for 404, auth, network |

**Summary:** 8 of 8 acceptance criteria fully implemented with comprehensive evidence

### Task Completion Validation

All 12 task groups (60 individual subtasks) systematically verified:

| Task Group | Subtasks | Status | Key Evidence |
|------------|----------|--------|--------------|
| 1. Extend import CLI | 5/5 | ✅ VERIFIED | [src/cli/import.rs:24-28](src/cli/import.rs:24-28) - github: prefix detection and routing |
| 2. Create GitHub module | 5/5 | ✅ VERIFIED | [src/archive/github.rs:1-568](src/archive/github.rs:1-568) - Complete module with import_from_github() |
| 3. Repository cloning | 9/9 | ✅ VERIFIED | [src/archive/github.rs:245-312](src/archive/github.rs:245-312) - git2 clone with auth, progress, error handling |
| 4. Search manifest | 5/5 | ✅ VERIFIED | [src/archive/github.rs:314-356](src/archive/github.rs:314-356) - 3-tier search with clear errors |
| 5. Validate manifest | 4/4 | ✅ VERIFIED | [src/archive/github.rs:96-104](src/archive/github.rs:96-104) - Reuses Story 2.1 validation |
| 6. Handle conflicts | 5/5 | ✅ VERIFIED | [src/archive/github.rs:116-123](src/archive/github.rs:116-123) - Reuses Story 2.5 conflict logic |
| 7. Create profile | 6/6 | ✅ VERIFIED | [src/archive/github.rs:358-421](src/archive/github.rs:358-421) - Selective file copying with skip rules |
| 8. Install framework | 4/4 | ✅ VERIFIED | [src/archive/github.rs:154-170](src/archive/github.rs:154-170) - Framework install and shell regeneration |
| 9. Store metadata | 4/4 | ✅ VERIFIED | [src/archive/github.rs:423-474](src/archive/github.rs:423-474) - .zprof-source with URL, commit, date |
| 10. Success message | 4/4 | ✅ VERIFIED | [src/cli/import.rs:47-56](src/cli/import.rs:47-56) - Complete success message with source URL |
| 11. Handle edge cases | 8/8 | ✅ VERIFIED | [src/archive/github.rs:293-310, 347-355](src/archive/github.rs:293-310) - Comprehensive error coverage |
| 12. Write tests | 5/5 | ✅ VERIFIED | [src/archive/github.rs:486-567](src/archive/github.rs:486-567), [tests/github_import_test.rs](tests/github_import_test.rs) - 11 tests passing |

**Summary:** 60 of 60 tasks verified complete with specific file:line evidence. Zero false completions detected.

### Test Coverage and Gaps

**Unit Tests:**
- ✅ 8 tests in [src/archive/github.rs:486-567](src/archive/github.rs:486-567) covering URL parsing
  - Valid formats (standard, hyphens, underscores, numbers)
  - Invalid formats (missing prefix, slash, empty fields)
  - Edge cases (whitespace handling)
- ✅ 3 integration tests in [tests/github_import_test.rs](tests/github_import_test.rs)
  - Valid/invalid format parsing
  - Edge case handling
  - Network tests marked #[ignore] (appropriate for CI)

**Test Quality:**
- ✅ Assertions are meaningful and specific
- ✅ Error messages validated (contains checks)
- ✅ Edge cases well covered
- ✅ Manual test instructions comprehensive ([tests/github_import_test.rs:177-235](tests/github_import_test.rs:177-235))

**Test Gaps:**
- ℹ️ Network integration tests require manual execution (by design)
- ℹ️ Private repo auth requires manual testing (documented in test file)

**Overall:** Test coverage is excellent for unit-testable code. Network operations appropriately documented for manual testing.

### Architectural Alignment

**Tech-Spec Compliance:**
- ✅ Follows Epic 2 Story 2.6 module mapping
- ✅ Uses git2 0.20 as specified in architecture
- ✅ Primary modules: cli/import.rs (extended), archive/github.rs (created)
- ✅ Secondary modules: archive/import.rs (reused)

**Pattern Compliance:**
- ✅ **Pattern 1 (CLI Command Structure)**: [src/cli/import.rs:22-29](src/cli/import.rs:22-29) - Proper routing logic
- ✅ **Pattern 2 (Error Handling)**: [src/archive/github.rs:7](src/archive/github.rs:7) - Consistent anyhow::Result with .context()
- ✅ **Pattern 3 (Safe File Operations)**: [src/archive/github.rs:77-82, 88-91](src/archive/github.rs:77-82) - Cleanup on all error paths
- ✅ **Pattern 4 (TOML Manifest)**: Reuses manifest validation from Story 2.1

**Code Reuse (Critical Architecture Goal):**
- ✅ `load_manifest_from_path()` from Story 2.1 - [src/archive/github.rs:97](src/archive/github.rs:97)
- ✅ `handle_name_conflict()` from Story 2.5 - [src/archive/github.rs:116](src/archive/github.rs:116)
- ✅ `install_framework()` from Story 2.5 - [src/archive/github.rs:156](src/archive/github.rs:156)
- ✅ `write_generated_files()` from Story 2.2 - [src/archive/github.rs:165](src/archive/github.rs:165)

**Architecture Decision Records:**
- ✅ ADR-002 (TOML over YAML): Reuses manifest validation
- ✅ Uses git2 for GitHub operations as specified
- ✅ Follows non-destructive pattern with cleanup

**Module Structure:**
- ✅ Clean separation: parsing → cloning → validation → installation
- ✅ Private helper functions appropriately scoped
- ✅ Public API surface minimal and well-documented

### Security Notes

**Security Review: PASSED**

**Input Validation:**
- ✅ GitHub URL format strictly validated - [src/archive/github.rs:195-228](src/archive/github.rs:195-228)
- ✅ No shell injection vectors (git2 library calls, no shell exec)
- ✅ Path traversal prevented (uses Path/PathBuf safely)
- ✅ Whitespace trimmed from user input - [src/archive/github.rs:215-216](src/archive/github.rs:215-216)

**Authentication & Authorization:**
- ✅ Uses git credential helpers (standard secure approach)
- ✅ No credential storage in zprof code
- ✅ Clear error messages for auth failures - [src/archive/github.rs:298-301](src/archive/github.rs:298-301)

**File Operations:**
- ✅ Explicit skip list for sensitive files (.git, hidden files) - [src/archive/github.rs:400-412](src/archive/github.rs:400-412)
- ✅ Validates files before copying
- ✅ Temp directory cleanup on all error paths
- ✅ No arbitrary file write vulnerabilities

**Resource Management:**
- ✅ Temp directories cleaned up on success and failure
- ✅ No resource leaks detected
- ✅ Proper error propagation with cleanup

**Dependencies:**
- ✅ git2 0.20 (mature, well-maintained library)
- ℹ️ Recommendation: Monitor for git2 security updates periodically

### Best-Practices and References

**Rust Best Practices:**
- ✅ Idiomatic error handling with anyhow::Result and .context()
- ✅ Comprehensive documentation comments (module and function level)
- ✅ Proper use of logging (log::info, log::debug)
- ✅ Type safety with Path/PathBuf
- ✅ Clear ownership semantics

**Git Operations:**
- ✅ Progress callbacks for user feedback - [git2 documentation](https://docs.rs/git2/0.20.0/git2/)
- ✅ Proper use of RemoteCallbacks for authentication
- ✅ Repository opened and closed appropriately

**Testing Best Practices:**
- ✅ Unit tests in same file as implementation (#[cfg(test)])
- ✅ Integration tests in separate test file
- ✅ Network tests marked #[ignore] for CI/CD compatibility
- ✅ Manual test instructions documented

**References:**
- [git2-rs documentation](https://docs.rs/git2/0.20.0/git2/) - Used for clone implementation
- [anyhow documentation](https://docs.rs/anyhow/) - Error handling patterns
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) - Code style compliance

### Action Items

**No action items required - story is approved as-is.**

**Advisory Notes (Optional Improvements for Future):**

- Note: Consider adding a `--branch` flag in future enhancement to clone specific branches (not in current story scope)
- Note: The .zprof-source metadata enables future update features (pull latest from repo) - well architected for future extension
- Note: Integration tests require manual execution for network operations - comprehensive instructions provided in [tests/github_import_test.rs:177-235](tests/github_import_test.rs:177-235)
- Note: Module documentation could optionally mention git must be installed, though git2 handles this gracefully
