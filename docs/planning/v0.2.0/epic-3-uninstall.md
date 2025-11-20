# Epic 3: Complete Uninstall System

**Priority:** P0 (Must Have)
**Estimated Effort:** 4 days
**Owner:** TBD

## Overview

Provide a comprehensive uninstall command that allows users to confidently remove zprof, with options to restore their original backed-up configuration, promote a profile to root, or simply clean up and start fresh.

## Problem Statement

Current limitations:
- No clean way to completely remove zprof
- Users can't restore their original pre-zprof shell configuration
- No option to "graduate" a profile to become the root config
- Unclear what happens to data (history, custom scripts) during removal

This creates hesitation for users to try zprof - they worry about getting "locked in" or breaking their shell setup.

## Goals

1. **Safe experimentation**: Users can try zprof knowing they can fully restore their original setup
2. **Flexible restoration**: Choose between original backup, promoting a profile, or clean removal
3. **Data preservation**: History and customizations are safely handled
4. **Clear communication**: Users understand exactly what will happen before confirmation
5. **Safety nets**: Final backup created before any destructive operations

## User Stories

### Story 3.1: Enhance Init to Create Pre-zprof Backup

**As a** user running `zprof init`
**I want** my current shell config automatically backed up
**So that** I can restore it later if needed

**Acceptance Criteria:**
- [ ] Create `.zsh-profiles/backups/pre-zprof/` directory during init
- [ ] Backup all root shell config files:
  - `.zshrc`
  - `.zshenv`
  - `.zprofile`
  - `.zlogin`
  - `.zlogout`
  - Any other `.zsh*` files in HOME
- [ ] Backup `.zsh_history` if it exists
- [ ] Create `backup-manifest.toml` with:
  - Timestamp of backup
  - List of backed up files with checksums
  - Shell version info
  - Detection of existing framework (if any)
- [ ] Skip backup if already exists (don't overwrite on re-init)
- [ ] Show confirmation message about backup location
- [ ] Add unit tests for backup logic

**Files:**
- `src/cli/init.rs`
- `src/backup/pre_zprof.rs` (NEW)
- `src/core/backup_manifest.rs` (NEW)

---

### Story 3.2: Move Root Configs After Backup

**As a** user completing `zprof init`
**I want** my root configs moved out of the way
**So that** I don't get confused about which config is active

**Acceptance Criteria:**
- [ ] After backing up, move (not copy) root configs to `.zsh-profiles/backups/pre-zprof/`
- [ ] Leave only zprof-generated `.zshenv` in root
- [ ] Create `.zshrc` symlink to active profile (if one exists)
- [ ] Document the move in output message:
  ```
  ✓ Backed up your existing shell config to ~/.zsh-profiles/backups/pre-zprof/
  ✓ Moved root config files to backup location
  ✓ Your original setup is safely preserved and can be restored at any time
  ```
- [ ] Handle edge cases:
  - Root configs are symlinks (resolve and backup target)
  - Files are read-only (preserve permissions)
  - Files don't exist (don't error)
- [ ] Update init integration test

**Files:**
- `src/cli/init.rs`
- `src/backup/pre_zprof.rs`

---

### Story 3.3: Create Uninstall Command with Restoration Options

**As a** user who wants to remove zprof
**I want** to choose what happens to my shell config
**So that** I can restore my original setup or promote a profile

**Acceptance Criteria:**
- [ ] Create `src/cli/uninstall.rs`
- [ ] Implement `zprof uninstall` command
- [ ] Show restoration options TUI:
  ```
  What would you like to do with your shell configuration?

  > Restore original (pre-zprof backup from Jan 15, 2025)
    Promote profile to root (choose which profile)
    Clean removal (no restoration, start fresh)
    Cancel
  ```
- [ ] If "Restore original" selected:
  - Copy files from `.zsh-profiles/backups/pre-zprof/` to HOME
  - Restore `.zsh_history` if backed up
  - Remove zprof `.zshenv`
- [ ] If "Promote profile":
  - Show profile selection TUI
  - Copy selected profile's configs to HOME root
  - Copy profile's history to `.zsh_history`
  - Merge shared history if enabled
- [ ] If "Clean removal":
  - Just remove zprof files, leave HOME clean
- [ ] Show summary of what will happen before final confirmation
- [ ] Add `--yes` flag for non-interactive mode
- [ ] Return error if no pre-zprof backup exists for "Restore original"

**Files:**
- `src/cli/uninstall.rs` (NEW)
- `src/tui/uninstall_select.rs` (NEW)

---

### Story 3.4: Implement Safety Backup Before Uninstall

**As a** user running uninstall
**I want** a final backup created before removal
**So that** I can recover if something goes wrong

**Acceptance Criteria:**
- [ ] Create `.zsh-profiles/backups/final-snapshot-{timestamp}.tar.gz` before any destructive operations
- [ ] Archive includes:
  - All profiles
  - Shared history
  - All backup directories
  - `.zsh-profiles/config.toml`
- [ ] Show backup location in output:
  ```
  Creating safety backup: ~/.zsh-profiles/backups/final-snapshot-20250120-143022.tar.gz
  ✓ Backup complete (2.3 MB)
  ```
- [ ] Abort uninstall if backup creation fails
- [ ] Add `--no-backup` flag to skip (for scripting)
- [ ] Document backup location in uninstall output
- [ ] Add integration test

**Files:**
- `src/cli/uninstall.rs`
- `src/backup/snapshot.rs` (NEW)

---

### Story 3.5: Implement Cleanup Logic

**As a** user completing uninstall
**I want** all zprof files removed cleanly
**So that** my system is back to pre-zprof state

**Acceptance Criteria:**
- [ ] Remove entire `.zsh-profiles/` directory
- [ ] Remove zprof-generated `.zshenv` from HOME
- [ ] Remove any zprof-generated symlinks
- [ ] Preserve restored configuration files
- [ ] Handle errors gracefully (permission denied, files in use)
- [ ] Show progress during cleanup:
  ```
  Cleaning up...
  ✓ Removed profiles
  ✓ Removed backups
  ✓ Removed zprof configuration
  ✓ Removed shell integration
  ```
- [ ] Offer to keep final backup (default: keep)
- [ ] Add `--keep-backups` flag to preserve `.zsh-profiles/backups/` only
- [ ] Add unit tests for cleanup logic

**Files:**
- `src/cli/uninstall.rs`
- `src/cleanup/mod.rs` (NEW)

---

### Story 3.6: Add Uninstall Confirmation Screen

**As a** user about to uninstall
**I want** to see a summary and confirm my choice
**So that** I don't accidentally remove my shell config

**Acceptance Criteria:**
- [ ] Show detailed summary before proceeding:
  ```
  Uninstall Summary

  Restoration:
    • Restore pre-zprof backup from Jan 15, 2025
    • 4 files will be restored to HOME
    • History file will be restored (15,234 entries)

  Cleanup:
    • Remove 3 profiles (work, personal, minimal)
    • Remove ~/.zsh-profiles/ directory
    • Remove zprof shell integration

  Safety:
    • Final backup will be saved to:
      ~/.zsh-profiles/backups/final-snapshot-20250120-143022.tar.gz

  This operation cannot be undone (except via the safety backup).

  Continue with uninstall? [y/N]
  ```
- [ ] Default to "No" (user must explicitly type 'y')
- [ ] Add `--yes` flag to skip confirmation
- [ ] Show different summary based on restoration choice
- [ ] Include file counts, sizes, timestamps
- [ ] Highlight destructive operations in red

**Files:**
- `src/tui/uninstall_confirm.rs` (NEW)
- `src/cli/uninstall.rs`

---

### Story 3.7: Handle Edge Cases and Validation

**As a** developer
**I want** robust error handling for uninstall
**So that** users don't end up in broken states

**Acceptance Criteria:**
- [ ] Validate preconditions:
  - zprof is actually installed
  - User has write permissions to HOME
  - No active zprof shells (warn if detected)
- [ ] Handle missing pre-zprof backup gracefully:
  - Show warning if "Restore original" selected but no backup
  - Offer to continue with "Promote profile" or "Clean removal"
- [ ] Handle file conflicts during restoration:
  - If `.zshrc` already exists in HOME, prompt to overwrite
  - Create `.zshrc.zprofbackup` if user declines
- [ ] Handle partial restoration failures:
  - Roll back to pre-uninstall state
  - Leave final backup intact for manual recovery
- [ ] Validate profile selection for "Promote profile" option
- [ ] Add comprehensive error messages
- [ ] Add integration tests for edge cases

**Files:**
- `src/cli/uninstall.rs`
- `src/backup/restore.rs` (NEW)
- `tests/uninstall_test.rs` (NEW)

---

### Story 3.8: Update Documentation

**As a** user
**I want** clear documentation about uninstall
**So that** I understand my options and can recover if needed

**Acceptance Criteria:**
- [ ] Update `docs/user-guide/commands.md` with uninstall command:
  - Full syntax and flags
  - Restoration options explained
  - Safety backup details
  - Recovery procedures
- [ ] Create `docs/user-guide/uninstalling.md`:
  - Step-by-step uninstall guide
  - What happens during each option
  - How to manually restore from backup
  - Troubleshooting section
- [ ] Update `docs/user-guide/installation.md` to mention safe removal
- [ ] Update `docs/user-guide/faq.md`:
  - "Can I remove zprof and go back to my old setup?"
  - "What happens to my history?"
  - "Where are my backups stored?"
- [ ] Add examples for common scenarios

**Files:**
- `docs/user-guide/commands.md`
- `docs/user-guide/uninstalling.md` (NEW)
- `docs/user-guide/installation.md`
- `docs/user-guide/faq.md`

---

## Technical Design

### Backup Directory Structure

```
~/.zsh-profiles/
└── backups/
    ├── pre-zprof/                    # Original config before zprof
    │   ├── backup-manifest.toml      # Metadata about backup
    │   ├── .zshrc
    │   ├── .zshenv
    │   ├── .zsh_history
    │   └── .oh-my-zsh/               # If framework detected
    ├── final-snapshot-20250120.tar.gz # Safety backup before uninstall
    └── final-snapshot-20250118.tar.gz # Previous safety backups
```

### Backup Manifest Format

```toml
# backup-manifest.toml

[metadata]
created_at = "2025-01-15T14:30:22Z"
zsh_version = "5.9"
os = "Darwin 23.1.0"

[detected_framework]
name = "oh-my-zsh"
path = "/Users/anna/.oh-my-zsh"

[files]
[[files.backed_up]]
path = ".zshrc"
size = 1234
checksum = "sha256:abcdef..."
permissions = "0644"

[[files.backed_up]]
path = ".zsh_history"
size = 51234
lines = 1523
checksum = "sha256:123456..."
```

### Uninstall Flow

```
zprof uninstall
      ↓
Select restoration option
      ↓
   ┌──┴──────────┬───────────────┐
   ↓             ↓               ↓
Restore       Promote         Clean
Original      Profile        Removal
   ↓             ↓               ↓
   └──────┬──────┴───────────────┘
          ↓
    Show Summary
          ↓
    Confirm (y/N)
          ↓
  Create Safety Backup
          ↓
   Restore Config
          ↓
      Cleanup
          ↓
  Show Completion Message
```

### Restoration Logic

```rust
// src/backup/restore.rs

pub enum RestoreOption {
    PreZprof,
    PromoteProfile(String),
    NoRestore,
}

pub struct RestorationPlan {
    pub option: RestoreOption,
    pub files_to_restore: Vec<PathBuf>,
    pub files_to_remove: Vec<PathBuf>,
    pub backup_location: PathBuf,
}

impl RestorationPlan {
    pub fn execute(&self) -> Result<()> {
        // Copy files from backup/profile to HOME
        // Remove zprof-specific files
        // Verify restoration
        Ok(())
    }
}
```

## Dependencies

- Epic 6 (Init Cleanup) - Both modify init behavior

## Risks & Mitigations

**Risk:** User loses shell config during failed uninstall
**Mitigation:** Safety backup created before any destructive operations, rollback on errors

**Risk:** Restored config doesn't work (incompatibilities)
**Mitigation:** Clear documentation, safety backup remains available for manual recovery

**Risk:** History loss during restoration
**Mitigation:** Explicit history handling, show line counts in summary, final backup includes everything

**Risk:** Active shell sessions break after uninstall
**Mitigation:** Detect active zprof shells, warn user to close them first

## Testing Strategy

- Unit tests for backup creation and validation
- Integration tests for full uninstall flows (all 3 restoration options)
- Edge case tests (missing backups, file conflicts, permission errors)
- Snapshot tests for TUI output and confirmation screens
- Manual testing of actual shell restoration on fresh systems

## Success Criteria

- [ ] Users can fully restore their pre-zprof configuration
- [ ] Users can promote any profile to root config
- [ ] All zprof files are cleanly removed
- [ ] History is preserved during restoration
- [ ] Safety backups created before destructive operations
- [ ] Clear documentation and error messages
- [ ] All integration tests passing
- [ ] No way to lose user data through normal uninstall flow

## Out of Scope

- Automatic detection of active shells (v0.3.0)
- Cloud backup of safety snapshots (v0.4.0)
- Scheduled automatic backups (v0.4.0)
- Restoration of partially corrupted backups (manual recovery only)
