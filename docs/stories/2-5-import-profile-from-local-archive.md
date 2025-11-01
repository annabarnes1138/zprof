# Story 2.5: Import Profile from Local Archive

Status: ready-for-dev

## Story

As a developer,
I want to import a profile from a local .zprof archive,
so that I can use shared profiles on my machine.

## Acceptance Criteria

1. `zprof import <file.zprof>` extracts archive and validates contents
2. Validates profile.toml manifest within archive
3. Checks for name conflicts and prompts for resolution (rename/overwrite/cancel)
4. Installs specified framework and plugins per manifest
5. Creates new profile in `~/.zsh-profiles/profiles/`
6. Success message confirms import and lists profile details
7. Handles corrupted archives gracefully with clear error messages

## Tasks / Subtasks

- [ ] Create import command CLI interface (AC: #1)
  - [ ] Create `cli/import.rs` module
  - [ ] Define ImportArgs with archive_path parameter
  - [ ] Optional flags: --name (override profile name), --force (skip conflict prompt)
  - [ ] Follow Pattern 1 (CLI Command Structure) from architecture
  - [ ] Add comprehensive error handling with anyhow::Context
  - [ ] Register command in main.rs subcommand list

- [ ] Create import module structure (AC: All)
  - [ ] Create `archive/import.rs` submodule
  - [ ] Define import_profile() function
  - [ ] Follow architecture patterns for module organization
  - [ ] Add logging for debugging
  - [ ] Use functionality from export.rs (ArchiveMetadata struct)

- [ ] Implement archive extraction to temp directory (AC: #1, #7)
  - [ ] Create temporary directory for extraction
  - [ ] Use tar::Archive + flate2 per architecture
  - [ ] Extract archive to temp directory
  - [ ] Verify extraction succeeded
  - [ ] Handle corrupted tar files with clear error
  - [ ] Handle gzip decompression errors
  - [ ] Clean up temp directory on error

- [ ] Validate archive contents (AC: #1, #2, #7)
  - [ ] Check metadata.json exists in archive
  - [ ] Parse and validate metadata.json structure
  - [ ] Check profile.toml exists in archive
  - [ ] Load and validate profile.toml using manifest::load_and_validate()
  - [ ] Verify framework is supported (oh-my-zsh, zimfw, prezto, zinit, zap)
  - [ ] Display metadata (profile name, framework, export date, exported by)
  - [ ] Handle missing required files with helpful error
  - [ ] Handle malformed JSON/TOML with specific error messages

- [ ] Handle name conflicts (AC: #3)
  - [ ] Get profile name from manifest or --name flag
  - [ ] Check if profile already exists in ~/.zsh-profiles/profiles/
  - [ ] If exists and NOT --force:
    - [ ] Display conflict message with existing profile details
    - [ ] Prompt: "[R]ename, [O]verwrite, or [C]ancel?"
    - [ ] On Rename: prompt for new name, validate uniqueness
    - [ ] On Overwrite: warn about data loss, confirm, delete existing
    - [ ] On Cancel: clean up temp dir, exit gracefully
  - [ ] If exists and --force: overwrite without prompting
  - [ ] If not exists: proceed with import

- [ ] Create profile directory and copy files (AC: #5)
  - [ ] Create profile directory: ~/.zsh-profiles/profiles/<name>/
  - [ ] Copy profile.toml from temp directory to profile directory
  - [ ] Copy any custom configuration files from archive
  - [ ] Preserve file permissions from archive
  - [ ] Skip .zshrc and .zshenv (will be regenerated)
  - [ ] Log which files are copied

- [ ] Install framework and plugins (AC: #4)
  - [ ] Read framework from manifest
  - [ ] Call framework installation logic (from Story 1.8 or frameworks module)
  - [ ] Install framework to profile directory (e.g., .oh-my-zsh/)
  - [ ] Install plugins specified in manifest
  - [ ] Show progress indicators for long operations (using indicatif)
  - [ ] Handle installation failures gracefully
  - [ ] If installation fails: clean up partial profile, restore previous state

- [ ] Regenerate shell configuration (AC: #4)
  - [ ] Call generator::write_generated_files() from Story 2.2
  - [ ] Generate .zshrc and .zshenv from imported manifest
  - [ ] Validate generated files are syntactically correct
  - [ ] Handle regeneration failures

- [ ] Display import success message (AC: #6)
  - [ ] Confirm profile imported successfully
  - [ ] Display profile details (name, framework, plugin count)
  - [ ] Display import metadata (exported by, export date, zprof version)
  - [ ] Show profile location
  - [ ] Use consistent success format (✓ symbol per architecture)
  - [ ] Provide next steps: `zprof use <profile-name>` to activate

- [ ] Handle edge cases and errors (AC: #7)
  - [ ] Archive file doesn't exist: clear error
  - [ ] Archive is not a valid tar.gz: specific error
  - [ ] Archive missing required files: list what's missing
  - [ ] Manifest validation fails: show validation errors
  - [ ] Unsupported framework: show supported list
  - [ ] Framework installation fails: rollback profile creation
  - [ ] Disk space insufficient: check before extraction
  - [ ] Permission denied: helpful error message
  - [ ] Network required for framework install but offline: clear error

- [ ] Write comprehensive tests (AC: All)
  - [ ] Unit test archive extraction
  - [ ] Unit test metadata validation
  - [ ] Unit test manifest validation
  - [ ] Unit test name conflict detection
  - [ ] Integration test successful import (mock framework install)
  - [ ] Integration test import with name override
  - [ ] Integration test import with overwrite
  - [ ] Integration test corrupted archive handling
  - [ ] Integration test missing metadata.json
  - [ ] Integration test invalid manifest in archive
  - [ ] Manual test import actual exported archive
  - [ ] Manual test framework installation works

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `cli/import.rs`, `archive/import.rs`
- Secondary: `core/manifest.rs` (validation), `shell/generator.rs` (regeneration), `frameworks/mod.rs` (installation)
- Follow Pattern 1 (CLI Command Structure)
- Follow Pattern 2 (Error Handling)
- Follow Pattern 3 (Safe File Operations) - cleanup on failure
- Uses tar 0.4 + flate2 1.0 per architecture decision
- Complements export functionality from Story 2.4

**Import Module Pattern:**

```rust
// archive/import.rs
use anyhow::{Context, Result, ensure, bail};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::Read;
use tar::Archive;
use flate2::read::GzDecoder;
use serde_json;
use crate::core::manifest;
use crate::shell::generator;
use crate::archive::export::ArchiveMetadata;

pub struct ImportOptions {
    pub archive_path: PathBuf,
    pub profile_name_override: Option<String>,
    pub force_overwrite: bool,
}

pub fn import_profile(options: ImportOptions) -> Result<String> {
    log::info!("Importing profile from: {:?}", options.archive_path);

    // 1. Verify archive exists
    ensure!(
        options.archive_path.exists(),
        "Archive not found: {}",
        options.archive_path.display()
    );

    // 2. Create temp directory for extraction
    let temp_dir = create_temp_extraction_dir()?;
    log::info!("Extracting to temp dir: {:?}", temp_dir);

    // 3. Extract archive
    extract_archive(&options.archive_path, &temp_dir)
        .context("Failed to extract archive")?;

    // 4. Validate archive contents
    let metadata = validate_archive_contents(&temp_dir)
        .context("Archive validation failed")?;

    println!("→ Found profile: {}", metadata.profile_name);
    println!("  Framework: {}", metadata.framework);
    println!("  Exported: {}", metadata.export_date);
    println!("  Exported by: {}", metadata.exported_by);
    println!();

    // 5. Determine final profile name
    let profile_name = options.profile_name_override
        .unwrap_or(metadata.profile_name.clone());

    // 6. Handle name conflicts
    let profile_name = handle_name_conflict(&profile_name, options.force_overwrite)?;

    // 7. Load and validate manifest from temp directory
    let manifest_path = temp_dir.join("profile.toml");
    let manifest = manifest::load_manifest_from_path(&manifest_path)
        .context("Failed to load manifest from archive")?;

    // 8. Create profile directory
    let profile_dir = get_profile_dir(&profile_name)?;
    fs::create_dir_all(&profile_dir)
        .context(format!("Failed to create profile directory: {:?}", profile_dir))?;

    // 9. Copy files from temp to profile directory
    copy_profile_files(&temp_dir, &profile_dir)?;

    // 10. Install framework and plugins
    println!("→ Installing {} framework...", manifest.profile.framework);
    install_framework(&profile_dir, &manifest)?;

    // 11. Regenerate shell configuration
    println!("→ Generating shell configuration...");
    generator::write_generated_files(&profile_name, &manifest)
        .context("Failed to generate shell configuration")?;

    // 12. Clean up temp directory
    fs::remove_dir_all(&temp_dir)
        .context("Failed to clean up temp directory")?;

    log::info!("Import completed successfully: {}", profile_name);
    Ok(profile_name)
}

fn create_temp_extraction_dir() -> Result<PathBuf> {
    let home = dirs::home_dir()
        .context("Could not find home directory")?;

    let temp_dir = home
        .join(".zsh-profiles")
        .join("cache")
        .join("import_temp")
        .join(format!("import_{}", chrono::Utc::now().timestamp()));

    fs::create_dir_all(&temp_dir)
        .context("Failed to create temp extraction directory")?;

    Ok(temp_dir)
}

fn extract_archive(archive_path: &Path, dest_dir: &Path) -> Result<()> {
    let tar_file = File::open(archive_path)
        .context(format!("Failed to open archive: {:?}", archive_path))?;

    let decoder = GzDecoder::new(tar_file);
    let mut archive = Archive::new(decoder);

    archive.unpack(dest_dir)
        .context("Failed to unpack archive. Archive may be corrupted.")?;

    Ok(())
}

fn validate_archive_contents(temp_dir: &Path) -> Result<ArchiveMetadata> {
    // Check metadata.json exists
    let metadata_path = temp_dir.join("metadata.json");
    ensure!(
        metadata_path.exists(),
        "Invalid archive: metadata.json not found"
    );

    // Parse metadata
    let metadata_json = fs::read_to_string(&metadata_path)
        .context("Failed to read metadata.json")?;

    let metadata: ArchiveMetadata = serde_json::from_str(&metadata_json)
        .context("Failed to parse metadata.json. Archive may be corrupted.")?;

    // Check profile.toml exists
    let manifest_path = temp_dir.join("profile.toml");
    ensure!(
        manifest_path.exists(),
        "Invalid archive: profile.toml not found"
    );

    Ok(metadata)
}

fn handle_name_conflict(profile_name: &str, force: bool) -> Result<String> {
    let profile_dir = get_profile_dir(profile_name)?;

    if !profile_dir.exists() {
        // No conflict
        return Ok(profile_name.to_string());
    }

    if force {
        // Force overwrite - delete existing
        println!("⚠ Overwriting existing profile: {}", profile_name);
        fs::remove_dir_all(&profile_dir)
            .context("Failed to remove existing profile")?;
        return Ok(profile_name.to_string());
    }

    // Prompt user
    println!("⚠ Profile '{}' already exists", profile_name);
    println!();
    let action = prompt_conflict_resolution()?;

    match action.as_str() {
        "r" | "rename" => {
            let new_name = prompt_new_name()?;
            // Recursive call to check new name
            handle_name_conflict(&new_name, force)
        }
        "o" | "overwrite" => {
            println!("→ Overwriting existing profile...");
            fs::remove_dir_all(&profile_dir)
                .context("Failed to remove existing profile")?;
            Ok(profile_name.to_string())
        }
        "c" | "cancel" => {
            bail!("Import cancelled by user");
        }
        _ => unreachable!(),
    }
}

fn prompt_conflict_resolution() -> Result<String> {
    use std::io::{self, Write};

    print!("  [R]ename, [O]verwrite, or [C]ancel? ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let choice = input.trim().to_lowercase();

    match choice.as_str() {
        "r" | "rename" => Ok("rename".to_string()),
        "o" | "overwrite" => Ok("overwrite".to_string()),
        "c" | "cancel" => Ok("cancel".to_string()),
        _ => {
            println!("  Invalid choice. Cancelling import.");
            Ok("cancel".to_string())
        }
    }
}

fn prompt_new_name() -> Result<String> {
    use std::io::{self, Write};

    print!("  Enter new profile name: ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let name = input.trim().to_string();

    ensure!(!name.is_empty(), "Profile name cannot be empty");
    ensure!(
        name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_'),
        "Profile name must be alphanumeric (hyphens and underscores allowed)"
    );

    Ok(name)
}

fn copy_profile_files(temp_dir: &Path, profile_dir: &Path) -> Result<()> {
    // Copy profile.toml
    let src_manifest = temp_dir.join("profile.toml");
    let dst_manifest = profile_dir.join("profile.toml");
    fs::copy(&src_manifest, &dst_manifest)
        .context("Failed to copy profile.toml")?;
    log::info!("Copied: profile.toml");

    // Copy any custom files (skip .zshrc, .zshenv - will be regenerated)
    for entry in fs::read_dir(temp_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            continue;  // Skip directories
        }

        let filename = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("");

        // Skip metadata and generated files
        if filename == "metadata.json"
            || filename == ".zshrc"
            || filename == ".zshenv"
            || filename == "profile.toml"  // Already copied
        {
            continue;
        }

        // Copy custom file
        let dst_path = profile_dir.join(filename);
        fs::copy(&path, &dst_path)
            .context(format!("Failed to copy {}", filename))?;
        log::info!("Copied custom file: {}", filename);
    }

    Ok(())
}

fn install_framework(profile_dir: &Path, manifest: &manifest::ProfileManifest) -> Result<()> {
    // TODO: Implement framework installation
    // This will call framework-specific installation logic from frameworks module
    // For now, just create placeholder directory

    let framework = &manifest.profile.framework;

    // Framework installation would go here
    // Call frameworks::install_framework(profile_dir, framework, &manifest.plugins)?;

    // Placeholder for MVP
    println!("  ℹ Framework installation not yet implemented");
    println!("  → You'll need to manually install {} in this profile", framework);

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

**CLI Command Pattern:**

```rust
// cli/import.rs
use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;
use crate::archive::import;

#[derive(Debug, Args)]
pub struct ImportArgs {
    /// Path to .zprof archive file
    pub archive_path: PathBuf,

    /// Override profile name from archive
    #[arg(short, long)]
    pub name: Option<String>,

    /// Force overwrite existing profile without prompting
    #[arg(short, long)]
    pub force: bool,
}

pub fn execute(args: ImportArgs) -> Result<()> {
    let options = import::ImportOptions {
        archive_path: args.archive_path.clone(),
        profile_name_override: args.name.clone(),
        force_overwrite: args.force,
    };

    // Import profile
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
# Receive archive from teammate
$ ls
team-work-profile.zprof

# Import profile
$ zprof import team-work-profile.zprof
→ Found profile: work
  Framework: oh-my-zsh
  Exported: 2025-10-31T16:45:00Z
  Exported by: anna

→ Installing oh-my-zsh framework...
  ℹ Framework installation not yet implemented
  → You'll need to manually install oh-my-zsh in this profile
→ Generating shell configuration...

✓ Profile imported successfully

  Profile: work
  Location: ~/.zsh-profiles/profiles/work

  → Run 'zprof use work' to activate this profile

# Activate imported profile
$ zprof use work
```

**Example Name Conflict Flow:**

```bash
$ zprof import work.zprof
→ Found profile: work
  Framework: oh-my-zsh
  Exported: 2025-10-31T16:45:00Z
  Exported by: teammate

⚠ Profile 'work' already exists

  [R]ename, [O]verwrite, or [C]ancel? r
  Enter new profile name: work-team

→ Installing oh-my-zsh framework...
→ Generating shell configuration...

✓ Profile imported successfully

  Profile: work-team
  Location: ~/.zsh-profiles/profiles/work-team

  → Run 'zprof use work-team' to activate this profile
```

**Example Force Overwrite:**

```bash
$ zprof import --force work.zprof
→ Found profile: work
  Framework: oh-my-zsh
  Exported: 2025-10-31T16:45:00Z
  Exported by: anna

⚠ Overwriting existing profile: work
→ Installing oh-my-zsh framework...
→ Generating shell configuration...

✓ Profile imported successfully

  Profile: work
  Location: ~/.zsh-profiles/profiles/work

  → Run 'zprof use work' to activate this profile
```

**Error Handling Examples:**

```bash
# Corrupted archive
$ zprof import broken.zprof
✗ Error: Failed to extract archive
  Cause: Failed to unpack archive. Archive may be corrupted.

# Missing metadata
$ zprof import old-format.zprof
✗ Error: Archive validation failed
  Cause: Invalid archive: metadata.json not found

# Invalid manifest
$ zprof import invalid-manifest.zprof
✗ Error: Failed to load manifest from archive
  Cause: TOML parsing error at line 5: invalid value
```

**Archive Import Process:**

```
1. Extract to temp directory
2. Validate contents (metadata.json, profile.toml)
3. Check name conflict
4. Create profile directory
5. Copy files (profile.toml + custom files)
6. Install framework per manifest
7. Regenerate .zshrc and .zshenv
8. Clean up temp directory
```

**Safety Features:**

- Extraction to temp directory first (validate before committing)
- Name conflict detection and resolution
- Manifest validation before installation
- Framework installation error handling
- Temp directory cleanup on success or failure
- Rollback on any failure (don't leave partial profiles)

### Project Structure Notes

**New Files Created:**
- `src/archive/import.rs` - Profile import from .zprof archive

**Modified Files:**
- `src/cli/import.rs` - CLI command for import (if not exists, create)
- `src/main.rs` - Register `import` subcommand (if not already from 2.6)
- `src/archive/mod.rs` - Export import module
- `src/cli/mod.rs` - Export import module

**Dependencies Used:**
- tar = "0.4" (already added in Story 2.4)
- flate2 = "1.0" (already added in Story 2.4)
- serde_json = "1.0" (already added in Story 2.4)
- chrono (for temp directory naming)

**Framework Installation:**

Note: Full framework installation logic is deferred to implementation phase. This story creates the integration point, but actual framework download/install may be stubbed for MVP. Stories 1.5-1.8 will provide the framework installation implementations that this story calls.

### Learnings from Previous Stories

**From Story 2.4: Export Profile to Archive (Status: drafted)**

Import is the complement to export:

- **Archive Structure**: Understands .zprof format created by export
- **Metadata Format**: Reads ArchiveMetadata struct from metadata.json
- **File Exclusions**: Knows frameworks were excluded, so must install them
- **Manifest as Source**: Relies on profile.toml to rebuild profile

**Critical Integration:**
Export creates archives, import consumes them. They must agree on format.

**From Story 2.2: Generate Shell Configuration from TOML (Status: drafted)**

Import regenerates shell files:

- **Regeneration**: Use `generator::write_generated_files()` after import
- **Fresh Generation**: .zshrc and .zshenv regenerated from manifest (not copied)
- **Consistency**: Ensures generated files match current zprof version

**From Story 2.1: Parse and Validate TOML Manifests (Status: drafted)**

Import validates imported manifests:

- **Validation**: Use `manifest::load_and_validate()` on extracted manifest
- **Safety**: Won't create profile from invalid manifest
- **Error Messages**: Show specific validation errors to user

**From Story 1.5-1.8: Profile Creation and TUI Wizard (Status: drafted)**

Import uses framework installation:

- **Framework Install**: Calls same installation logic as TUI wizard
- **Plugin Install**: Installs plugins per manifest
- **Reusability**: Import and create share framework installation code

**Export + Import Workflow:**

```
Machine A:                          Machine B:
---------                           ---------
1. Create profile (Story 1.8)
2. Customize (Story 2.3)
3. Export (Story 2.4)
   → work.zprof                     4. Receive work.zprof
                                    5. Import (Story 2.5)
                                    6. Framework installed
                                    7. Shell files generated
                                    8. Profile ready to use
```

**Use Cases Enabled:**

1. **Team Standardization**: Share team profile with all developers
2. **Machine Migration**: Move profiles to new laptop
3. **Backup/Restore**: Export for safekeeping, import to restore
4. **Experimentation**: Try community profiles risk-free
5. **Onboarding**: New team members get standard setup instantly

**Import vs GitHub Import (Story 2.6):**

This story (2.5):
- Import from local .zprof file
- File path as input
- Direct archive extraction

Story 2.6:
- Import from GitHub repository
- github:user/repo as input
- Clone repo → find profile.toml → import

Both stories share most of the import logic (this story's `import.rs` module).

### References

- [Source: docs/epics.md#Story-2.5]
- [Source: docs/PRD.md#FR016-import-from-archive]
- [Source: docs/PRD.md#Epic-2-YAML-Manifests-Export-Import]
- [Source: docs/architecture.md#Archive-Format-tar-gz]
- [Source: docs/architecture.md#Epic-2-Story-2.5-Mapping]
- [Source: docs/stories/2-4-export-profile-to-archive.md]
- [Source: docs/stories/2-2-generate-shell-configuration-from-yaml.md]
- [Source: docs/stories/2-1-parse-and-validate-yaml-manifests.md]

## Dev Agent Record

### Context Reference

- [Story Context XML](/Users/anna/code/annabarnes1138/zprof/docs/stories/2-5-import-profile-from-local-archive.context.xml)

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
