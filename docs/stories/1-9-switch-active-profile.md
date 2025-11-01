# Story 1.9: Switch Active Profile

Status: ready-for-dev

## Story

As a developer,
I want to switch between my profiles quickly,
so that I can change my shell environment for different contexts.

## Acceptance Criteria

1. `zprof use <profile-name>` updates ZDOTDIR to point to selected profile
2. New shell instance is launched with selected profile active
3. Switching completes in under 500ms
4. Shared command history is accessible in new profile
5. Clear confirmation message shows which profile is now active
6. Handles invalid profile names with helpful error message

## Tasks / Subtasks

- [ ] Implement profile validation (AC: #6)
  - [ ] Create function to check if profile exists in `~/.zsh-profiles/profiles/`
  - [ ] Verify profile directory contains required files (profile.toml, .zshrc)
  - [ ] Return user-friendly error if profile not found
  - [ ] List available profiles in error message for discoverability
  - [ ] Handle case-sensitive profile name matching
- [ ] Implement ZDOTDIR update (AC: #1)
  - [ ] Create `shell/zdotdir.rs` module
  - [ ] Implement function to get profile directory path
  - [ ] Update ZDOTDIR environment variable to profile directory
  - [ ] Verify ZDOTDIR is exported correctly for child shell
  - [ ] Log ZDOTDIR change with env_logger
- [ ] Update active profile configuration (AC: #5)
  - [ ] Update `~/.zsh-profiles/config.toml` with new active_profile
  - [ ] Load existing config.toml if present
  - [ ] Create config.toml if it doesn't exist (first profile switch)
  - [ ] Write updated config with new active_profile value
  - [ ] Use serde + toml for config read/write per architecture
  - [ ] Handle config read/write errors gracefully
- [ ] Launch new shell instance (AC: #2, #3)
  - [ ] Use std::process::Command to execute 'zsh'
  - [ ] Use .exec() to replace current process (not spawn child) per ADR-004
  - [ ] Set ZDOTDIR in environment before exec
  - [ ] Ensure exec() meets < 500ms requirement (AC: #3)
  - [ ] Handle exec failure (shouldn't happen, but error gracefully)
  - [ ] Note: exec() never returns on success (replaces process)
- [ ] Verify shared history integration (AC: #4)
  - [ ] Ensure HISTFILE in .zshenv points to `~/.zsh-profiles/shared/.zsh_history`
  - [ ] Verify generated .zshenv from Story 1.8 sets correct HISTFILE
  - [ ] Test history is accessible across profile switches
  - [ ] Document shared history in user-facing messages/docs
- [ ] Implement CLI command (AC: All)
  - [ ] Create `cli/use_cmd.rs` module (use is Rust keyword)
  - [ ] Define UseCmdArgs struct with profile_name parameter
  - [ ] Implement execute() function following Pattern 1
  - [ ] Validate profile exists (AC: #6)
  - [ ] Update config.toml active_profile
  - [ ] Display confirmation before exec (AC: #5)
  - [ ] Set ZDOTDIR and exec new shell
- [ ] Handle edge cases and errors (AC: #6)
  - [ ] Profile doesn't exist: show error + list available profiles
  - [ ] Profile directory exists but missing .zshrc: show error + suggest repair
  - [ ] config.toml is corrupted: show error, offer to recreate
  - [ ] No profiles exist: show error, suggest creating first profile
  - [ ] Already using requested profile: show warning but allow switch
  - [ ] Use anyhow::Context for all error messages per Pattern 2
- [ ] Write comprehensive tests (AC: All)
  - [ ] Unit test profile validation logic
  - [ ] Unit test ZDOTDIR path generation
  - [ ] Unit test config.toml update logic
  - [ ] Integration test profile switching flow (validate -> update config -> exec setup)
  - [ ] Test error handling for invalid profile names (AC: #6)
  - [ ] Test error messages include available profiles
  - [ ] Performance test switching completes setup in < 500ms (AC: #3)
  - [ ] Manual test actual shell switching with multiple profiles
  - [ ] Manual test shared history works across switches (AC: #4)
  - [ ] Manual test confirmation message is clear (AC: #5)

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `cli/use_cmd.rs`, `shell/zdotdir.rs`
- Secondary: `core/config.rs` (config.toml management), `core/profile.rs` (validation)
- Follow Pattern 1 (CLI Command Structure)
- Follow Pattern 2 (Error Handling) with anyhow::Result
- Meets NFR001: < 500ms profile switching
- Implements ADR-004: exec() for profile switching (not subshell)

**Profile Validation Pattern:**
```rust
// core/profile.rs
use anyhow::{bail, Context, Result};
use std::path::{Path, PathBuf};

pub fn get_profile_path(profile_name: &str) -> Result<PathBuf> {
    let profiles_dir = get_profiles_directory();
    let profile_path = profiles_dir.join(profile_name);

    if !profile_path.exists() {
        let available = list_available_profiles()?;
        bail!(
            "✗ Error: Profile '{}' not found\n  Available profiles:\n{}",
            profile_name,
            format_profile_list(&available)
        );
    }

    Ok(profile_path)
}

pub fn validate_profile(profile_path: &Path) -> Result<()> {
    let zshrc = profile_path.join(".zshrc");
    let manifest = profile_path.join("profile.toml");

    if !zshrc.exists() {
        bail!(
            "✗ Error: Profile is incomplete - missing .zshrc\n  Path: {:?}\n  → Run 'zprof edit {}' to regenerate configuration",
            profile_path,
            profile_path.file_name().unwrap().to_string_lossy()
        );
    }

    if !manifest.exists() {
        bail!(
            "✗ Error: Profile is incomplete - missing profile.toml\n  Path: {:?}",
            profile_path
        );
    }

    Ok(())
}

fn list_available_profiles() -> Result<Vec<String>> {
    let profiles_dir = get_profiles_directory();
    let mut profiles = Vec::new();

    for entry in std::fs::read_dir(&profiles_dir)
        .context("Failed to read profiles directory")? {
        let entry = entry?;
        if entry.path().is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                profiles.push(name.to_string());
            }
        }
    }

    if profiles.is_empty() {
        bail!(
            "No profiles found. Create your first profile:\n  zprof create <name>"
        );
    }

    profiles.sort();
    Ok(profiles)
}

fn format_profile_list(profiles: &[String]) -> String {
    profiles.iter()
        .map(|p| format!("    - {}", p))
        .collect::<Vec<_>>()
        .join("\n")
}
```

**Config Update Pattern:**
```rust
// core/config.rs
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZprofConfig {
    pub active_profile: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_framework: Option<String>,
}

pub fn get_config_path() -> PathBuf {
    dirs::home_dir()
        .expect("Could not find home directory")
        .join(".zsh-profiles")
        .join("config.toml")
}

pub fn load_config() -> Result<ZprofConfig> {
    let config_path = get_config_path();

    if !config_path.exists() {
        // Return default config if file doesn't exist
        return Ok(ZprofConfig {
            active_profile: String::new(),
            default_framework: None,
        });
    }

    let content = std::fs::read_to_string(&config_path)
        .context("Failed to read config.toml")?;

    toml::from_str(&content)
        .context("Failed to parse config.toml - file may be corrupted")
}

pub fn save_config(config: &ZprofConfig) -> Result<()> {
    let config_path = get_config_path();

    let toml_string = toml::to_string_pretty(config)
        .context("Failed to serialize config")?;

    std::fs::write(&config_path, toml_string)
        .context(format!("Failed to write config to {:?}", config_path))?;

    log::debug!("Updated config.toml: active_profile = {}", config.active_profile);
    Ok(())
}

pub fn update_active_profile(profile_name: &str) -> Result<()> {
    let mut config = load_config()?;
    config.active_profile = profile_name.to_string();
    save_config(&config)?;
    Ok(())
}
```

**ZDOTDIR and Shell Exec Pattern (ADR-004):**
```rust
// shell/zdotdir.rs
use anyhow::{Context, Result};
use std::path::Path;
use std::os::unix::process::CommandExt;  // For exec()

pub fn switch_to_profile(profile_path: &Path) -> Result<()> {
    // Set ZDOTDIR to profile directory
    let zdotdir = profile_path.to_str()
        .context("Profile path contains invalid UTF-8")?;

    log::debug!("Setting ZDOTDIR={}", zdotdir);

    // Execute new zsh shell with ZDOTDIR set
    // This replaces the current process - never returns on success
    let err = std::process::Command::new("zsh")
        .env("ZDOTDIR", zdotdir)
        .exec();  // Unix-only: replaces current process

    // If we reach here, exec() failed
    Err(anyhow::anyhow!(
        "Failed to execute zsh shell: {}",
        err
    ))
}
```

**Complete CLI Command Implementation:**
```rust
// cli/use_cmd.rs
use anyhow::{Context, Result};
use clap::Args;
use crate::core::{config, profile};
use crate::shell::zdotdir;

#[derive(Debug, Args)]
pub struct UseCmdArgs {
    /// Name of the profile to activate
    pub profile_name: String,
}

pub fn execute(args: UseCmdArgs) -> Result<()> {
    // Step 1: Validate profile exists and is complete (AC: #6)
    let profile_path = profile::get_profile_path(&args.profile_name)?;
    profile::validate_profile(&profile_path)?;

    // Step 2: Update config.toml with new active profile
    config::update_active_profile(&args.profile_name)
        .context("Failed to update active profile in config")?;

    // Step 3: Display confirmation message (AC: #5)
    println!("✓ Switching to profile '{}'", args.profile_name);
    println!();
    println!("  Location: {:?}", profile_path);
    println!("  Shared history: enabled");
    println!();

    // Step 4: Launch new shell with ZDOTDIR set (AC: #1, #2)
    // This call never returns on success (exec replaces process)
    zdotdir::switch_to_profile(&profile_path)?;

    // Unreachable on success
    unreachable!("exec() should have replaced the process");
}
```

**Usage Example:**
```bash
$ zprof use experimental
✓ Switching to profile 'experimental'

  Location: /Users/anna/.zsh-profiles/profiles/experimental
  Shared history: enabled

# New shell launches immediately with experimental profile
# Current process is replaced, no return to previous shell
```

**Error Message Examples:**
```bash
# Invalid profile name (AC: #6)
$ zprof use nonexistent
✗ Error: Profile 'nonexistent' not found
  Available profiles:
    - experimental
    - work
    - minimal

# Incomplete profile
$ zprof use broken
✗ Error: Profile is incomplete - missing .zshrc
  Path: "/Users/anna/.zsh-profiles/profiles/broken"
  → Run 'zprof edit broken' to regenerate configuration

# No profiles exist
$ zprof use anything
✗ Error: No profiles found. Create your first profile:
  zprof create <name>
```

**Performance Considerations (NFR001):**
```rust
// Profile switching is extremely fast:
// 1. Validate profile: ~1-5ms (directory check + file existence)
// 2. Update config.toml: ~5-10ms (small file write)
// 3. Display message: ~1ms (stdout write)
// 4. exec() zsh: ~10-50ms (OS exec syscall)
// Total: ~20-70ms << 500ms requirement ✓

// The shell startup time (loading framework, plugins) is NOT counted
// in switching time - that's framework overhead, not zprof overhead
```

**Shared History Verification:**
```rust
// Generated .zshenv (from Story 1.8) already sets:
// export HISTFILE="$HOME/.zsh-profiles/shared/.zsh_history"

// This ensures all profiles share the same history file (AC: #4)
// No additional work needed in Story 1.9 - just verify it works
```

**Integration with Other Stories:**
- Story 1.1 created `~/.zsh-profiles/` directory structure
- Story 1.2 lists profiles (uses same profile enumeration logic)
- Story 1.3 shows current profile (reads config.toml active_profile)
- Story 1.8 generates .zshenv with shared HISTFILE
- Story 1.10 will prevent deleting active profile (uses config.toml check)

### Project Structure Notes

**New Files Created:**
- `src/cli/use_cmd.rs` - Profile switching CLI command
- `src/shell/zdotdir.rs` - ZDOTDIR manipulation and shell exec
- `src/core/config.rs` - config.toml read/write utilities (if not existing)

**Modified Files:**
- `src/main.rs` - Register `use` subcommand with Clap
- `src/core/profile.rs` - Add validation functions (if not existing from earlier stories)
- `src/cli/mod.rs` - Export use_cmd module
- `src/shell/mod.rs` - Export zdotdir module

**Learnings from Previous Stories:**

**From Story 1.8: TUI Wizard Theme Selection and Profile Generation (Status: drafted)**

Story 1.8 generates the .zshenv file that sets HISTFILE to the shared history location. Story 1.9 relies on this to ensure AC #4 (shared command history) works correctly:

- **Shared History Setup**: .zshenv from Story 1.8 sets `HISTFILE="$HOME/.zsh-profiles/shared/.zsh_history"`
- **No Additional Work**: Story 1.9 doesn't need to modify history configuration
- **Verification Only**: Test that history is accessible after switching profiles

**From Story 1.1: Initialize zprof Directory Structure (Status: ready-for-dev)**
- Directory structure: `~/.zsh-profiles/profiles/<profile-name>/`
- config.toml location: `~/.zsh-profiles/config.toml`
- Shared history file: `~/.zsh-profiles/shared/.zsh_history`

**Integration Requirements:**
- Story 1.9 reads profiles from the directory structure created in Story 1.1
- Updates config.toml (created in Story 1.1 or first profile switch)
- Uses ZDOTDIR to point to profile directory (standard zsh mechanism)
- exec() replaces current process per ADR-004 (no subshell, instant switch)

**Expected Workflow After Story 1.9:**
```bash
# Create profiles (Stories 1.5-1.8)
$ zprof create work      # Imports existing oh-my-zsh setup
$ zprof create experimental  # TUI wizard creates zimfw profile

# Switch between profiles (Story 1.9)
$ zprof use experimental  # < 500ms, launches new shell with zimfw
# ... work in experimental profile ...
$ exit
$ zprof use work         # Back to oh-my-zsh setup
# ... command history from experimental is still accessible ...
```

**Critical Implementation Detail (ADR-004):**
Using `exec()` instead of spawning a child shell means:
- Current shell process is **replaced** (not nested)
- No "return to previous shell" - user must `zprof use` to switch again
- Process tree stays clean (no shell nesting)
- Matches behavior of nvm, pyenv, rbenv (industry standard)

**Alternative Considered and Rejected:**
```rust
// DON'T DO THIS (creates nested shell):
std::process::Command::new("zsh")
    .env("ZDOTDIR", zdotdir)
    .spawn()?  // Wrong - creates child process
    .wait()?;  // Wrong - returns to parent shell after exit

// DO THIS (replaces current process):
std::process::Command::new("zsh")
    .env("ZDOTDIR", zdotdir)
    .exec();  // Correct - replaces process, never returns
```

### References

- [Source: docs/epics.md#Story-1.9]
- [Source: docs/PRD.md#FR003-switch-active-profile]
- [Source: docs/PRD.md#NFR001-sub-500ms-switching]
- [Source: docs/PRD.md#FR018-shared-command-history]
- [Source: docs/architecture.md#ADR-004-exec-for-switching]
- [Source: docs/architecture.md#Pattern-5-Shell-Integration]
- [Source: docs/architecture.md#Epic-1-Story-1.9-Mapping]
- [Source: docs/architecture.md#Performance-NFR001]
- [Source: docs/stories/1-8-tui-wizard-theme-selection-and-profile-generation.md#zshenv-generation]

## Dev Agent Record

### Context Reference

- docs/stories/1-9-switch-active-profile.context.xml

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
