# Story 2.3: Manual TOML Editing with Live Validation

Status: ready-for-dev

## Story

As a power user,
I want to manually edit my profile TOML and receive validation feedback,
so that I can quickly customize profiles without using the TUI wizard.

## Acceptance Criteria

1. `zprof edit <profile-name>` opens profile.toml in user's $EDITOR
2. After saving, system validates TOML and reports any errors
3. If valid, regenerates .zshrc and .zshenv from updated manifest
4. If invalid, preserves old configuration and shows validation errors
5. User can retry edit or cancel without breaking profile
6. Changes take effect on next `zprof use <profile-name>`

## Tasks / Subtasks

- [ ] Create edit command CLI interface (AC: #1)
  - [ ] Create `cli/edit.rs` module
  - [ ] Define EditArgs with profile_name parameter
  - [ ] Follow Pattern 1 (CLI Command Structure) from architecture
  - [ ] Add comprehensive error handling with anyhow::Context
  - [ ] Register command in main.rs subcommand list

- [ ] Implement $EDITOR detection and invocation (AC: #1)
  - [ ] Check $EDITOR environment variable first
  - [ ] Fallback to $VISUAL if $EDITOR not set
  - [ ] Fallback to "vim" if neither set (Unix default)
  - [ ] On Windows, fallback to "notepad"
  - [ ] Get profile directory path from profile_name
  - [ ] Construct path to profile.toml within profile directory
  - [ ] Verify profile exists before editing
  - [ ] Open editor as child process: `Command::new(editor).arg(manifest_path).status()`
  - [ ] Wait for editor to close before continuing
  - [ ] Handle editor launch failures with clear error messages

- [ ] Create backup before validation (AC: #4, #5, NFR002)
  - [ ] Use Pattern 3 (Safe File Operations) from architecture
  - [ ] Create backup of profile.toml to cache/backups/
  - [ ] Backup naming: `profile-<name>-profile.toml.backup.<timestamp>`
  - [ ] Store backup path for potential restoration
  - [ ] Log backup creation for debugging

- [ ] Implement post-edit validation loop (AC: #2, #4, #5)
  - [ ] Load edited profile.toml using manifest::load_and_validate()
  - [ ] If validation succeeds:
    - [ ] Proceed to regeneration step (AC: #3)
    - [ ] Delete backup (no longer needed)
  - [ ] If validation fails:
    - [ ] Display clear error messages from validation
    - [ ] Show line numbers and specific issues
    - [ ] Preserve original backup (don't corrupt profile)
    - [ ] Prompt user: "[R]etry edit, [C]ancel, or [Restore] backup?"
    - [ ] On Retry: reopen editor with current (invalid) file
    - [ ] On Cancel: keep invalid file, don't regenerate, warn user
    - [ ] On Restore: copy backup back to profile.toml, delete backup

- [ ] Implement shell configuration regeneration (AC: #3)
  - [ ] Call generator::write_generated_files() from Story 2.2
  - [ ] Pass profile_name and validated manifest
  - [ ] Regenerate .zshrc and .zshenv from updated manifest
  - [ ] Handle regeneration failures gracefully
  - [ ] If regeneration fails: restore backup, show error

- [ ] Display success message and next steps (AC: #6)
  - [ ] Confirm manifest updated successfully
  - [ ] List what changed (compare old vs new manifest)
  - [ ] Show which files were regenerated (.zshrc, .zshenv)
  - [ ] Remind user to run `zprof use <profile-name>` to activate
  - [ ] Use consistent success format (✓ symbol per architecture)

- [ ] Handle edge cases and errors (AC: All)
  - [ ] Profile doesn't exist: clear error with suggestion to create
  - [ ] No $EDITOR set (handled by fallbacks)
  - [ ] Editor crashes or returns non-zero: ask if changes should be kept
  - [ ] User makes no changes: detect and skip validation
  - [ ] TOML syntax errors: show specific line and column
  - [ ] Semantic validation errors: show which fields are invalid
  - [ ] Permission denied on file write: helpful error message
  - [ ] Concurrent edits (rare): warn user about potential conflicts

- [ ] Write comprehensive tests (AC: All)
  - [ ] Unit test $EDITOR detection logic
  - [ ] Unit test backup creation and restoration
  - [ ] Integration test successful edit flow (mock editor)
  - [ ] Integration test validation failure with retry
  - [ ] Integration test restoration from backup
  - [ ] Integration test cancellation preserves state
  - [ ] Test regeneration is called after valid edit
  - [ ] Test editor failure handling
  - [ ] Manual test with real $EDITOR (vim, nano, code, etc.)
  - [ ] Manual test retry/cancel/restore prompts work correctly

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `cli/edit.rs`
- Secondary: `core/manifest.rs` (validation), `shell/generator.rs` (regeneration), `core/filesystem.rs` (backups)
- Follow Pattern 1 (CLI Command Structure)
- Follow Pattern 2 (Error Handling)
- Follow Pattern 3 (Safe File Operations) - critical for NFR002
- Implements ADR-002 (TOML as source of truth)
- Uses functionality from Story 2.1 (manifest validation) and Story 2.2 (regeneration)

**Edit Command Pattern:**
```rust
// cli/edit.rs
use anyhow::{Context, Result, bail, ensure};
use clap::Args;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use chrono::Utc;
use crate::core::manifest;
use crate::shell::generator;

#[derive(Debug, Args)]
pub struct EditArgs {
    /// Name of the profile to edit
    pub profile_name: String,
}

pub fn execute(args: EditArgs) -> Result<()> {
    // 1. Validate profile exists
    let profile_dir = get_profile_dir(&args.profile_name)?;
    let manifest_path = profile_dir.join("profile.toml");

    ensure!(
        manifest_path.exists(),
        "Profile '{}' not found. Run 'zprof list' to see available profiles.",
        args.profile_name
    );

    // 2. Detect editor
    let editor = detect_editor()?;
    log::info!("Using editor: {}", editor);

    // 3. Create backup before editing (NFR002)
    let backup_path = create_backup(&manifest_path)?;
    log::info!("Created backup: {:?}", backup_path);

    // 4. Open editor
    println!("→ Opening {} in {}...", manifest_path.display(), editor);
    let edit_result = open_editor(&editor, &manifest_path);

    // Handle editor failure
    if let Err(e) = edit_result {
        println!("✗ Editor failed to launch: {}", e);
        println!("  → Restoring backup...");
        restore_backup(&backup_path, &manifest_path)?;
        bail!("Edit cancelled due to editor failure");
    }

    // 5. Validation loop
    loop {
        match manifest::load_and_validate(&args.profile_name) {
            Ok(manifest) => {
                // Validation succeeded
                println!("✓ TOML manifest validated successfully");

                // 6. Regenerate shell files
                println!("→ Regenerating shell configuration...");
                generator::write_generated_files(&args.profile_name, &manifest)
                    .context("Failed to regenerate shell configuration")?;

                // 7. Success
                println!("✓ Profile updated successfully");
                println!();
                println!("  Profile: {}", args.profile_name);
                println!("  Framework: {}", manifest.profile.framework);
                println!("  Files updated:");
                println!("    - profile.toml (manifest)");
                println!("    - .zshrc (regenerated)");
                println!("    - .zshenv (regenerated)");
                println!();
                println!("  → Run 'zprof use {}' to activate changes", args.profile_name);

                // Clean up backup
                fs::remove_file(&backup_path)
                    .context("Failed to remove backup")?;

                return Ok(());
            }
            Err(e) => {
                // Validation failed
                println!("✗ TOML validation failed:");
                println!("{:#}", e);
                println!();

                // Prompt for action
                let action = prompt_validation_failure()?;

                match action.as_str() {
                    "r" | "retry" => {
                        // Retry edit
                        println!("→ Reopening editor...");
                        open_editor(&editor, &manifest_path)?;
                        continue;
                    }
                    "restore" => {
                        // Restore backup
                        println!("→ Restoring original manifest...");
                        restore_backup(&backup_path, &manifest_path)?;
                        println!("✓ Original manifest restored");
                        return Ok(());
                    }
                    "c" | "cancel" => {
                        // Cancel - keep invalid file but don't regenerate
                        println!("→ Edit cancelled. Invalid manifest preserved.");
                        println!("  ⚠ Warning: Profile may not work until manifest is fixed");
                        println!("  → Run 'zprof edit {}' again to fix", args.profile_name);
                        return Ok(());
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

fn get_profile_dir(profile_name: &str) -> Result<PathBuf> {
    let home = dirs::home_dir()
        .context("Could not find home directory")?;

    Ok(home
        .join(".zsh-profiles")
        .join("profiles")
        .join(profile_name))
}

fn detect_editor() -> Result<String> {
    // Check $EDITOR first
    if let Ok(editor) = env::var("EDITOR") {
        if !editor.is_empty() {
            return Ok(editor);
        }
    }

    // Check $VISUAL
    if let Ok(visual) = env::var("VISUAL") {
        if !visual.is_empty() {
            return Ok(visual);
        }
    }

    // Platform-specific fallbacks
    if cfg!(target_os = "windows") {
        Ok("notepad".to_string())
    } else {
        // Unix/Linux/macOS: default to vim
        Ok("vim".to_string())
    }
}

fn open_editor(editor: &str, file_path: &Path) -> Result<()> {
    let status = Command::new(editor)
        .arg(file_path)
        .status()
        .context(format!("Failed to launch editor: {}", editor))?;

    if !status.success() {
        bail!("Editor exited with non-zero status: {}", status);
    }

    Ok(())
}

fn create_backup(file_path: &Path) -> Result<PathBuf> {
    // Create backups directory
    let home = dirs::home_dir()
        .context("Could not find home directory")?;

    let backups_dir = home
        .join(".zsh-profiles")
        .join("cache")
        .join("backups");

    fs::create_dir_all(&backups_dir)
        .context("Failed to create backups directory")?;

    // Generate backup filename with timestamp
    let timestamp = Utc::now().format("%Y%m%d-%H%M%S");
    let filename = file_path
        .file_name()
        .context("Invalid file path")?
        .to_str()
        .context("Invalid filename")?;

    let backup_filename = format!("{}.backup.{}", filename, timestamp);
    let backup_path = backups_dir.join(backup_filename);

    // Copy file to backup
    fs::copy(file_path, &backup_path)
        .context(format!("Failed to create backup at {:?}", backup_path))?;

    Ok(backup_path)
}

fn restore_backup(backup_path: &Path, dest_path: &Path) -> Result<()> {
    fs::copy(backup_path, dest_path)
        .context("Failed to restore backup")?;

    // Delete backup after successful restoration
    fs::remove_file(backup_path)
        .context("Failed to remove backup after restoration")?;

    Ok(())
}

fn prompt_validation_failure() -> Result<String> {
    use std::io::{self, Write};

    print!("  [R]etry edit, [Restore] backup, or [C]ancel? ");
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let choice = input.trim().to_lowercase();

    match choice.as_str() {
        "r" | "retry" => Ok("retry".to_string()),
        "restore" => Ok("restore".to_string()),
        "c" | "cancel" => Ok("cancel".to_string()),
        _ => {
            println!("  Invalid choice. Defaulting to cancel.");
            Ok("cancel".to_string())
        }
    }
}
```

**Example User Flow:**

```bash
# User wants to add a new plugin to their work profile
$ zprof edit work
→ Opening /Users/anna/.zsh-profiles/profiles/work/profile.toml in vim...

# User edits manifest in vim, adds "zsh-autosuggestions" to plugins
# Saves and exits vim (:wq)

✓ TOML manifest validated successfully
→ Regenerating shell configuration...
✓ Profile updated successfully

  Profile: work
  Framework: oh-my-zsh
  Files updated:
    - profile.toml (manifest)
    - .zshrc (regenerated)
    - .zshenv (regenerated)

  → Run 'zprof use work' to activate changes

# Activate profile to load new plugin
$ zprof use work
```

**Example Validation Failure Flow:**

```bash
$ zprof edit experimental
→ Opening /Users/anna/.zsh-profiles/profiles/experimental/profile.toml in vim...

# User makes typo in framework name: "oh-my-zsh" → "oh-my-zhs"

✗ TOML validation failed:
Error: Unsupported framework: oh-my-zhs

Supported frameworks:
  - oh-my-zsh
  - zimfw
  - prezto
  - zinit
  - zap

  [R]etry edit, [Restore] backup, or [C]ancel? r

→ Reopening editor...

# User fixes typo in vim, saves

✓ TOML manifest validated successfully
→ Regenerating shell configuration...
✓ Profile updated successfully

  Profile: experimental
  Framework: oh-my-zsh
  Files updated:
    - profile.toml (manifest)
    - .zshrc (regenerated)
    - .zshenv (regenerated)

  → Run 'zprof use experimental' to activate changes
```

**Backup Safety (NFR002 Compliance):**

This story implements critical safety features:

1. **Backup Before Edit**: Always create timestamped backup before opening editor
2. **Validation Gate**: Never regenerate shell files from invalid manifest
3. **Restoration Option**: User can restore backup if edit goes wrong
4. **Preserve on Cancel**: Invalid edits are kept (not lost) but not applied
5. **Atomic Operations**: Either full success or full rollback

**Workflow States:**

```
Initial State → Create Backup → Open Editor → User Edits → Save & Exit
                                                              ↓
                                                         Validate TOML
                                                              ↓
                                          ┌───────────────────┴────────────────────┐
                                          ↓                                        ↓
                                    Valid TOML                              Invalid TOML
                                          ↓                                        ↓
                              Regenerate Shell Files                    Prompt User Action
                                          ↓                                        ↓
                                   Delete Backup              ┌──────────────────┬┴─────────┐
                                          ↓                   ↓                  ↓          ↓
                                 Success Message         Retry Edit      Restore Backup  Cancel
                                                              ↓                  ↓          ↓
                                                      (Loop to Editor)   (Restore) (Preserve Invalid)
```

### Project Structure Notes

**New Files Created:**
- `src/cli/edit.rs` - CLI command for manual manifest editing

**Modified Files:**
- `src/main.rs` - Register `edit` subcommand
- `src/cli/mod.rs` - Export edit module

**User Experience:**
- Power users get direct access to manifest without TUI overhead
- Validation prevents broken profiles (safety)
- Retry loop enables quick iteration on manifest edits
- Backup system provides safety net per NFR002

### Learnings from Previous Story

**From Story 2.2: Generate Shell Configuration from TOML (Status: drafted)**

Story 2.3 builds directly on Story 2.2's regeneration capabilities:

- **Regeneration Function**: Use `generator::write_generated_files()` after valid edit
- **Manifest Validation**: Validate before regenerating (don't generate from invalid manifest)
- **Auto-generated Files**: .zshrc and .zshenv are regenerated, overwriting previous versions
- **Manifest as Source**: TOML is single source of truth - edits flow through manifest
- **Performance**: Regeneration completes in < 1 second, so edit flow is fast

**Critical Integration:**
Story 2.3 enables the "edit manifest → regenerate shell files" workflow that Story 2.2 makes possible.

**From Story 2.1: Parse and Validate TOML Manifests (Status: drafted)**

Story 2.3 relies on Story 2.1's validation system:

- **Validation Function**: Use `manifest::load_and_validate()` after edit
- **Error Messages**: Display validation errors to guide user fixes
- **Schema Validation**: Catches syntax errors, missing fields, invalid values
- **Framework Validation**: Ensures framework is one of 5 supported options

**From Story 1.5-1.8: Profile Creation Stories (Status: drafted)**

Edit workflow complements creation workflow:

- **Creation**: TUI wizard generates initial manifest
- **Editing**: Manual TOML editing for power users who outgrow wizard
- **Both Valid**: Both workflows produce validated manifests
- **Shared Backend**: Both use same manifest schema and validation

**Workflow Progression:**

1. **Story 1.5-1.8**: User creates profile via TUI wizard → manifest generated
2. **Story 2.2**: Manifest drives shell file generation
3. **Story 2.3**: User manually edits manifest → regeneration triggered
4. **Story 2.4-2.6**: User exports/imports manifests for sharing

**Edit Command Benefits:**

- **Faster than TUI**: No navigating menus for small changes
- **Version Control Friendly**: Edit manifest in git diff workflow
- **Power User Workflow**: Direct access to configuration source
- **Validation Safety**: Can't break profile with invalid edits
- **Learn by Example**: Users see TOML structure and learn syntax

**Common Edit Scenarios:**

- Add/remove plugins quickly
- Change theme
- Update environment variables
- Tweak framework-specific settings (future: custom fields)
- Bulk changes (easier in text editor than TUI)

### References

- [Source: docs/epics.md#Story-2.3]
- [Source: docs/PRD.md#FR014-manual-yaml-editing]
- [Source: docs/PRD.md#NFR002-non-destructive-operations]
- [Source: docs/architecture.md#ADR-002-TOML-not-YAML]
- [Source: docs/architecture.md#Pattern-2-Error-Handling]
- [Source: docs/architecture.md#Pattern-3-Safe-File-Operations]
- [Source: docs/stories/2-1-parse-and-validate-yaml-manifests.md#validation]
- [Source: docs/stories/2-2-generate-shell-configuration-from-yaml.md#regeneration]

## Dev Agent Record

### Context Reference

- [2-3-manual-toml-editing-with-live-validation.context.xml](docs/stories/2-3-manual-toml-editing-with-live-validation.context.xml)

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
