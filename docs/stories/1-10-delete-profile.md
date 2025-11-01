# Story 1.10: Delete Profile

Status: ready-for-dev

## Story

As a developer,
I want to delete profiles I no longer need,
so that I can keep my profile collection clean and manageable.

## Acceptance Criteria

1. `zprof delete <profile-name>` prompts for confirmation before deletion
2. Confirmation shows profile name and warns action is irreversible
3. On confirmation, removes profile directory and all contents
4. Cannot delete currently active profile (requires switching first)
5. Success message confirms deletion
6. Shared history and other profiles remain unaffected

## Tasks / Subtasks

- [ ] Implement profile deletion validation (AC: #4, #6)
  - [ ] Check if profile exists using `core/profile.rs::get_profile_path()`
  - [ ] Load current active profile from config.toml
  - [ ] Compare requested profile with active profile
  - [ ] Return error if trying to delete active profile
  - [ ] Suggest switching to different profile first in error message
  - [ ] List available profiles to switch to
  - [ ] Handle case where profile doesn't exist with helpful error
- [ ] Implement confirmation prompt (AC: #1, #2)
  - [ ] Display profile name being deleted
  - [ ] Show full profile path for transparency
  - [ ] Display irreversibility warning
  - [ ] Prompt "Delete profile 'X'? (y/n): " and wait for input
  - [ ] Accept y/yes (case insensitive) for confirmation
  - [ ] Accept n/no (case insensitive) or any other input for cancellation
  - [ ] Allow Ctrl+C to cancel without error
  - [ ] Display cancellation message if user declines
- [ ] Implement safe directory deletion (AC: #3, #6)
  - [ ] Use Pattern 3 (Safe File Operations) from architecture
  - [ ] Create backup of profile directory before deletion
  - [ ] Store backup in `~/.zsh-profiles/cache/backups/<profile-name>-<timestamp>/`
  - [ ] Remove entire profile directory recursively
  - [ ] Use std::fs::remove_dir_all() for recursive deletion
  - [ ] Verify deletion completed successfully
  - [ ] Log deletion with env_logger
  - [ ] Clean up backup after successful deletion (optional, keep for safety)
  - [ ] Restore from backup if deletion fails mid-operation
- [ ] Verify isolation from shared resources (AC: #6)
  - [ ] Confirm shared history file is NOT deleted
  - [ ] Verify other profile directories remain untouched
  - [ ] Ensure config.toml is not corrupted
  - [ ] Only delete files within the specific profile directory
  - [ ] Use path validation to prevent directory traversal issues
- [ ] Implement CLI command (AC: All)
  - [ ] Create `cli/delete.rs` module
  - [ ] Define DeleteArgs struct with profile_name parameter
  - [ ] Implement execute() function following Pattern 1
  - [ ] Validate profile exists
  - [ ] Check profile is not active (AC: #4)
  - [ ] Display confirmation prompt (AC: #1, #2)
  - [ ] Delete profile directory on confirmation (AC: #3)
  - [ ] Display success message (AC: #5)
  - [ ] Handle all error cases gracefully
  - [ ] Use anyhow::Context for error messages per Pattern 2
- [ ] Write comprehensive tests (AC: All)
  - [ ] Unit test active profile detection
  - [ ] Unit test confirmation prompt logic
  - [ ] Unit test directory deletion
  - [ ] Integration test full deletion flow
  - [ ] Test cannot delete active profile (AC: #4)
  - [ ] Test confirmation required before deletion (AC: #1)
  - [ ] Test declining confirmation cancels operation
  - [ ] Test shared history unaffected (AC: #6)
  - [ ] Test other profiles unaffected (AC: #6)
  - [ ] Test error handling for non-existent profile
  - [ ] Test backup creation before deletion
  - [ ] Manual test full workflow with real profiles

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `cli/delete.rs`, `core/profile.rs`
- Secondary: `core/filesystem.rs` (safe file operations)
- Follow Pattern 1 (CLI Command Structure)
- Follow Pattern 2 (Error Handling) with anyhow::Result
- Follow Pattern 3 (Safe File Operations) - NFR002 compliance
- Implements ADR-004: Cannot delete active profile (shell process is using it)

**Active Profile Check Pattern:**
```rust
// core/profile.rs
use anyhow::{bail, Context, Result};

pub fn validate_not_active(profile_name: &str) -> Result<()> {
    let config = crate::core::config::load_config()?;

    if config.active_profile == profile_name {
        // Get list of other available profiles for suggestion
        let all_profiles = list_available_profiles()?;
        let other_profiles: Vec<_> = all_profiles.iter()
            .filter(|p| *p != profile_name)
            .collect();

        let suggestion = if other_profiles.is_empty() {
            "  → Create a new profile with 'zprof create <name>' first".to_string()
        } else {
            format!(
                "  → Switch to another profile first:\n{}",
                other_profiles.iter()
                    .map(|p| format!("      zprof use {}", p))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        };

        bail!(
            "✗ Error: Cannot delete active profile '{}'\n\n{}",
            profile_name,
            suggestion
        );
    }

    Ok(())
}
```

**Confirmation Prompt Pattern:**
```rust
// core/profile.rs or cli/delete.rs
use std::io::{self, Write};

fn confirm_deletion(profile_name: &str, profile_path: &Path) -> Result<bool> {
    println!();
    println!("⚠️  WARNING: This action is irreversible!");
    println!();
    println!("  Profile to delete: '{}'", profile_name);
    println!("  Path: {:?}", profile_path);
    println!();
    print!("Delete profile '{}'? (y/n): ", profile_name);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let response = input.trim().to_lowercase();
    Ok(response == "y" || response == "yes")
}
```

**Safe Directory Deletion Pattern (NFR002):**
```rust
// core/filesystem.rs
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};
use chrono::Utc;

pub fn safe_delete_directory(dir_path: &Path, reason: &str) -> Result<()> {
    ensure!(dir_path.exists(), "Directory does not exist: {:?}", dir_path);
    ensure!(dir_path.is_dir(), "Path is not a directory: {:?}", dir_path);

    // Step 1: Create backup
    let backup_dir = create_backup_directory()?;
    let timestamp = Utc::now().format("%Y%m%d-%H%M%S");
    let dir_name = dir_path.file_name()
        .context("Invalid directory path")?;
    let backup_path = backup_dir.join(format!("{}-{}",
        dir_name.to_string_lossy(), timestamp));

    log::debug!("Creating backup at {:?}", backup_path);
    copy_dir_recursive(dir_path, &backup_path)
        .context("Failed to create backup before deletion")?;

    // Step 2: Delete original directory
    match std::fs::remove_dir_all(dir_path) {
        Ok(_) => {
            log::info!("Deleted directory: {:?} (reason: {})", dir_path, reason);
            // Optionally clean up backup after successful deletion
            // For safety, we keep it for now
            log::debug!("Backup retained at {:?}", backup_path);
            Ok(())
        }
        Err(e) => {
            log::error!("Failed to delete directory, backup preserved at {:?}", backup_path);
            Err(e).context(format!("Failed to delete directory {:?}", dir_path))
        }
    }
}

fn create_backup_directory() -> Result<PathBuf> {
    let backup_dir = dirs::home_dir()
        .context("Could not find home directory")?
        .join(".zsh-profiles")
        .join("cache")
        .join("backups");

    std::fs::create_dir_all(&backup_dir)
        .context("Failed to create backups directory")?;

    Ok(backup_dir)
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    std::fs::create_dir_all(dst)?;

    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());

        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            std::fs::copy(&src_path, &dst_path)?;
        }
    }

    Ok(())
}
```

**Complete CLI Command Implementation:**
```rust
// cli/delete.rs
use anyhow::{Context, Result};
use clap::Args;
use crate::core::{filesystem, profile};

#[derive(Debug, Args)]
pub struct DeleteArgs {
    /// Name of the profile to delete
    pub profile_name: String,
}

pub fn execute(args: DeleteArgs) -> Result<()> {
    // Step 1: Validate profile exists (AC: error handling)
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

use std::io::{self, Write};
use std::path::Path;

fn confirm_deletion(profile_name: &str, profile_path: &Path) -> Result<bool> {
    println!();
    println!("⚠️  WARNING: This action is irreversible!");
    println!();
    println!("  Profile to delete: '{}'", profile_name);
    println!("  Path: {:?}", profile_path);
    println!();
    print!("Delete profile '{}'? (y/n): ", profile_name);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let response = input.trim().to_lowercase();
    Ok(response == "y" || response == "yes")
}
```

**Usage Example:**
```bash
# Try to delete active profile (AC: #4)
$ zprof current
experimental

$ zprof delete experimental
✗ Error: Cannot delete active profile 'experimental'

  → Switch to another profile first:
      zprof use work
      zprof use minimal

# Delete non-active profile (AC: #1-6)
$ zprof use work
✓ Switching to profile 'work'
...

$ zprof delete experimental

⚠️  WARNING: This action is irreversible!

  Profile to delete: 'experimental'
  Path: "/Users/anna/.zsh-profiles/profiles/experimental"

Delete profile 'experimental'? (y/n): y

✓ Profile 'experimental' deleted successfully

  Shared history and other profiles remain unaffected.
  Backup retained at: ~/.zsh-profiles/cache/backups/

# Verify deletion
$ zprof list
* work
  minimal

$ cat ~/.zsh-profiles/shared/.zsh_history
# History still contains commands from all profiles (AC: #6)
```

**Error Message Examples:**
```bash
# Profile doesn't exist
$ zprof delete nonexistent
✗ Error: Profile 'nonexistent' not found
  Available profiles:
    - experimental
    - work
    - minimal

# Decline confirmation (AC: #1)
$ zprof delete experimental

⚠️  WARNING: This action is irreversible!

  Profile to delete: 'experimental'
  Path: "/Users/anna/.zsh-profiles/profiles/experimental"

Delete profile 'experimental'? (y/n): n

Deletion cancelled. Profile 'experimental' was not deleted.

# Active profile protection (AC: #4)
$ zprof delete work
✗ Error: Cannot delete active profile 'work'

  → Switch to another profile first:
      zprof use experimental
```

### Project Structure Notes

**New Files Created:**
- `src/cli/delete.rs` - Profile deletion CLI command
- `src/core/filesystem.rs` - Safe file operations with backup (if not existing from earlier stories)

**Modified Files:**
- `src/main.rs` - Register `delete` subcommand with Clap
- `src/core/profile.rs` - Add `validate_not_active()` function
- `src/cli/mod.rs` - Export delete module
- `src/core/mod.rs` - Export filesystem module

**Learnings from Previous Story:**

**From Story 1.9: Switch Active Profile (Status: drafted)**

Story 1.9 establishes the config.toml pattern for tracking the active profile. Story 1.10 relies on this to prevent deleting the currently active profile:

- **Active Profile Tracking**: config.toml stores `active_profile = "profile-name"`
- **Load Pattern**: Use `core/config.rs::load_config()` to read active profile
- **Validation Pattern**: Compare requested profile with active_profile before deletion
- **Error Pattern**: Provide helpful suggestions for switching to another profile
- **Directory Structure**: Profiles located at `~/.zsh-profiles/profiles/<profile-name>/`
- **Shared Resources**: Shared history at `~/.zsh-profiles/shared/.zsh_history` must NOT be deleted

**Critical Integration Point:**
Story 1.9 uses `exec()` to launch a new shell with ZDOTDIR pointing to the active profile directory. If we deleted the active profile directory, the current shell process would break. Therefore, AC #4 (cannot delete currently active profile) is a hard requirement for system stability.

**Expected Workflow After Story 1.10:**
```bash
# Create multiple profiles (Stories 1.5-1.8)
$ zprof create work
$ zprof create experimental
$ zprof create minimal

# List profiles (Story 1.2)
$ zprof list
* work
  experimental
  minimal

# Switch to different profile (Story 1.9)
$ zprof use experimental

# Clean up unused profiles (Story 1.10)
$ zprof delete minimal   # Safe - not active
✓ Profile 'minimal' deleted successfully

$ zprof delete experimental   # Error - currently active
✗ Error: Cannot delete active profile 'experimental'
  → Switch to another profile first

$ zprof use work
$ zprof delete experimental   # Now safe - not active
✓ Profile 'experimental' deleted successfully

# Final state
$ zprof list
* work
```

### References

- [Source: docs/epics.md#Story-1.10]
- [Source: docs/PRD.md#FR005-delete-profiles]
- [Source: docs/PRD.md#NFR002-non-destructive-operations]
- [Source: docs/architecture.md#Pattern-3-Safe-File-Operations]
- [Source: docs/architecture.md#Epic-1-Story-1.10-Mapping]
- [Source: docs/stories/1-9-switch-active-profile.md#config-toml-pattern]

## Dev Agent Record

### Context Reference

- docs/stories/1-10-delete-profile.context.xml

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
