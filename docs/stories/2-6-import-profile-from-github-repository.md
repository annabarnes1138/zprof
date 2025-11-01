# Story 2.6: Import Profile from GitHub Repository

Status: ready-for-dev

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

- [ ] Extend import CLI to support GitHub syntax (AC: #1)
  - [ ] Modify `cli/import.rs` to detect github: prefix
  - [ ] Parse github:user/repo format
  - [ ] Extract username and repository name
  - [ ] Validate format (must have user and repo)
  - [ ] Route to GitHub import logic vs local import

- [ ] Create GitHub import module (AC: All)
  - [ ] Create `archive/github.rs` submodule
  - [ ] Define import_from_github() function
  - [ ] Follow architecture patterns
  - [ ] Add comprehensive logging
  - [ ] Use git2 0.20 crate per architecture

- [ ] Implement repository cloning (AC: #1, #6, #8)
  - [ ] Construct GitHub URL: https://github.com/<user>/<repo>
  - [ ] Create temp directory for clone
  - [ ] Use git2 to clone repository to temp directory
  - [ ] Handle authentication for private repos (git credential helpers)
  - [ ] Show progress during clone (indicatif progress bar)
  - [ ] Handle network errors with clear messages
  - [ ] Handle repository not found (404)
  - [ ] Handle authentication failures
  - [ ] Clean up temp directory on error

- [ ] Search for profile manifest (AC: #2, #8)
  - [ ] Search repo root for profile.toml
  - [ ] Also check for alternate locations: .zprof/profile.toml, profile.toml
  - [ ] If not found: show clear error with suggestions
  - [ ] If multiple found: use root, or prompt user
  - [ ] Validate found file is actually a TOML manifest

- [ ] Validate manifest from repository (AC: #3)
  - [ ] Load profile.toml using manifest::load_manifest_from_path()
  - [ ] Validate manifest schema
  - [ ] Display profile details (name, framework, description if present)
  - [ ] Show repository metadata (URL, author)
  - [ ] Handle invalid manifests with specific errors

- [ ] Handle name conflicts (AC: #3)
  - [ ] Get profile name from manifest
  - [ ] Check if profile already exists locally
  - [ ] Reuse name conflict logic from Story 2.5
  - [ ] Prompt: [R]ename, [O]verwrite, or [C]ancel
  - [ ] Support --name flag to override name
  - [ ] Support --force flag to overwrite without prompt

- [ ] Create profile from GitHub source (AC: #5)
  - [ ] Create profile directory: ~/.zsh-profiles/profiles/<name>/
  - [ ] Copy profile.toml from repo to profile directory
  - [ ] Copy any additional config files from repo root
  - [ ] Skip .git directory and GitHub-specific files (.github/, README.md, LICENSE)
  - [ ] Log which files are copied
  - [ ] Preserve file permissions

- [ ] Install framework and regenerate (AC: #4)
  - [ ] Install framework per manifest (reuse from Story 2.5)
  - [ ] Install plugins per manifest
  - [ ] Regenerate .zshrc and .zshenv using generator::write_generated_files()
  - [ ] Handle installation failures

- [ ] Store GitHub source reference (AC: #7)
  - [ ] Add source_url to profile metadata (extend manifest schema or create .zprof-meta file)
  - [ ] Store: github_url, clone_date, commit_hash (HEAD)
  - [ ] This enables future updates (pull latest from repo)
  - [ ] Optional: store in profile.toml [metadata] section or separate file

- [ ] Display success message with source attribution (AC: #7)
  - [ ] Show profile imported successfully
  - [ ] Display profile details (name, framework)
  - [ ] Show source repository URL
  - [ ] Show commit hash imported
  - [ ] Provide next steps: `zprof use <name>` to activate
  - [ ] Use consistent success format (✓ symbol)

- [ ] Handle edge cases and errors (AC: #8)
  - [ ] Invalid GitHub URL format: show format example
  - [ ] Repository doesn't exist (404): clear error
  - [ ] Repository is private and no auth: explain credential setup
  - [ ] Authentication failed: show git credential helper info
  - [ ] Network offline: detect and show helpful message
  - [ ] profile.toml not found in repo: suggest where to add it
  - [ ] Invalid manifest in repo: show validation errors
  - [ ] Git not installed: check and show installation instructions
  - [ ] Timeout on large repos: show progress, allow cancellation

- [ ] Write comprehensive tests (AC: All)
  - [ ] Unit test GitHub URL parsing (user/repo extraction)
  - [ ] Unit test manifest search in repo structure
  - [ ] Integration test clone public repository (mock or real)
  - [ ] Integration test manifest validation from repo
  - [ ] Integration test profile creation from GitHub source
  - [ ] Test name conflict handling
  - [ ] Test error handling (network, auth, missing manifest)
  - [ ] Test --name and --force flags
  - [ ] Manual test with real public GitHub repo
  - [ ] Manual test with private repo (auth)

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

<!-- Will be populated by dev agent during implementation -->

### Debug Log References

<!-- Will be populated by dev agent during implementation -->

### Completion Notes List

<!-- Will be populated by dev agent during implementation -->

### File List

<!-- Will be populated by dev agent during implementation -->

## Change Log

- 2025-10-31: Story drafted by SM agent (Bob)
