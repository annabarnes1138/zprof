# Epic 6: Init Cleanup and Enhancement

**Priority:** P0 (Must Have)
**Estimated Effort:** 2 days
**Owner:** TBD

## Overview

Enhance the `zprof init` process to create comprehensive pre-zprof backups and cleanly move root configuration files out of the way, eliminating confusion about which shell config is active while preserving the ability to fully restore the original setup.

## Problem Statement

Current init behavior:
- Doesn't backup user's existing shell configuration
- Leaves `.zshrc` and other config files in HOME root
- Users get confused about which config is active (root vs profile)
- Can't easily restore original setup without zprof
- No detection or handling of existing framework installations

This creates:
- Fear of trying zprof (what if something breaks?)
- Confusion when both root `.zshrc` and profile configs exist
- Difficulty troubleshooting (which file is actually sourced?)
- No clean way to "undo" zprof installation

## Goals

1. **Safe backup**: Preserve user's original shell configuration before any modifications
2. **Clean state**: Remove root config files after backing up (except zprof integration)
3. **Clear communication**: Users understand what's happening and can restore anytime
4. **Framework detection**: Detect and preserve existing framework installations
5. **Idempotent operation**: Re-running init is safe and doesn't break existing backups

## User Stories

### Story 6.1: Detect Existing Shell Configuration

**As a** user running `zprof init`
**I want** my existing config detected
**So that** zprof can properly back it up

**Acceptance Criteria:**
- [ ] Create `src/backup/shell_config.rs`
- [ ] Detect all shell config files in HOME:
  - `.zshrc` (primary config)
  - `.zshenv` (environment variables)
  - `.zprofile` (login shell config)
  - `.zlogin` (login shell hook)
  - `.zlogout` (logout hook)
  - Any other `.zsh*` files
  - `.zsh_history` (command history)
- [ ] Detect existing framework installations:
  - `.oh-my-zsh/`
  - `.zimfw/`
  - `.zprezto/`
  - `.zinit/`
  - `.zap/`
  - Other common framework directories
- [ ] Parse `.zshrc` to identify framework in use
- [ ] Return `ShellConfigInfo` struct with:
  - List of config files found
  - Detected framework (if any)
  - History file location and size
  - Total size of configuration
- [ ] Handle symlinks (resolve and backup targets)
- [ ] Add unit tests with mock filesystem

**Files:**
- `src/backup/shell_config.rs` (NEW)
- `src/backup/mod.rs`

---

### Story 6.2: Create Comprehensive Pre-zprof Backup

**As a** user running `zprof init`
**I want** all my shell config backed up automatically
**So that** I can restore it if needed

**Acceptance Criteria:**
- [ ] Extend backup logic from Epic 3, Story 3.1
- [ ] Create `.zsh-profiles/backups/pre-zprof/` directory
- [ ] Backup all detected config files with original permissions
- [ ] Backup framework directories (if detected):
  - Create tarball of framework directory
  - Store as `framework-backup.tar.gz`
  - Include in backup manifest
- [ ] Create `backup-manifest.toml`:
  ```toml
  [metadata]
  created_at = "2025-01-20T14:30:22Z"
  zsh_version = "5.9"
  os = "Darwin 23.1.0"
  zprof_version = "0.2.0"

  [detected_framework]
  name = "oh-my-zsh"
  path = "/Users/anna/.oh-my-zsh"
  backed_up_as = "framework-backup.tar.gz"

  [[files]]
  path = ".zshrc"
  size = 1234
  permissions = "0644"
  checksum = "sha256:abcdef..."

  [[files]]
  path = ".zsh_history"
  size = 51234
  lines = 1523
  checksum = "sha256:123456..."
  ```
- [ ] Skip backup if `pre-zprof/` already exists (don't overwrite)
- [ ] Show summary:
  ```
  Backing up your shell configuration...
  ✓ Found 5 config files
  ✓ Detected oh-my-zsh framework
  ✓ Backed up 1,523 history entries
  ✓ Backup saved to ~/.zsh-profiles/backups/pre-zprof/ (2.3 MB)

  Your original setup is preserved and can be restored anytime with 'zprof uninstall'.
  ```
- [ ] Add integration test

**Files:**
- `src/backup/pre_zprof.rs`
- `src/cli/init.rs`
- `src/core/backup_manifest.rs`

---

### Story 6.3: Move Root Configs to Backup Location

**As a** user completing `zprof init`
**I want** root config files moved out of HOME
**So that** only zprof integration remains active

**Acceptance Criteria:**
- [ ] After creating backup, move (not copy) config files:
  - Move `.zshrc` → `.zsh-profiles/backups/pre-zprof/.zshrc`
  - Move `.zshenv` → `.zsh-profiles/backups/pre-zprof/.zshenv`
  - Move other `.zsh*` files similarly
  - Keep `.zsh_history` in place (zprof uses it for shared history)
- [ ] Move framework directories:
  - Move `.oh-my-zsh/` → `.zsh-profiles/backups/pre-zprof/frameworks/.oh-my-zsh/`
  - Same for other frameworks
- [ ] Create new `.zshenv` with zprof integration:
  ```bash
  # zprof shell integration
  # This file was created by zprof v0.2.0
  # Your original .zshenv is backed up at ~/.zsh-profiles/backups/pre-zprof/

  export ZPROF_ROOT="$HOME/.zsh-profiles"

  # Load active profile if one is set
  if [[ -f "$ZPROF_ROOT/current" ]]; then
      export ZDOTDIR="$ZPROF_ROOT/profiles/$(cat $ZPROF_ROOT/current)"
  fi
  ```
- [ ] Show confirmation:
  ```
  Cleaning up root configuration...
  ✓ Moved 5 config files to backup
  ✓ Moved oh-my-zsh framework to backup
  ✓ Created new .zshenv for zprof integration

  Your HOME directory is now clean. All shell configs will come from profiles.
  ```
- [ ] Handle edge cases:
  - Files are read-only (preserve permissions in backup)
  - Files are symlinks (resolve and move targets)
  - Concurrent shell sessions (warn user to restart)
- [ ] Add integration test

**Files:**
- `src/backup/pre_zprof.rs`
- `src/cli/init.rs`

---

### Story 6.4: Handle Re-initialization

**As a** user running `zprof init` again
**I want** it to be safe and not destroy my backups
**So that** I can fix broken installations without risk

**Acceptance Criteria:**
- [ ] Detect if already initialized:
  - `.zsh-profiles/` directory exists
  - `backups/pre-zprof/` backup exists
- [ ] Show different message for re-init:
  ```
  zprof is already initialized.

  Existing backup found from Jan 15, 2025
  This backup will be preserved.

  What would you like to do?
  > Continue (safe re-initialization)
    View backup details
    Cancel
  ```
- [ ] If continuing:
  - Skip backup step (preserve existing)
  - Recreate directory structure if needed
  - Update `.zshenv` if missing or outdated
  - Don't move files again (already backed up)
- [ ] Show summary of existing installation:
  ```
  Current zprof installation:
  ✓ 3 profiles (work, personal, minimal)
  ✓ Active profile: work
  ✓ Pre-zprof backup: Jan 15, 2025 (2.3 MB)
  ✓ Shared history: 1,523 entries
  ```
- [ ] Add `--force` flag to re-backup (overwrites existing):
  ```
  zprof init --force

  ⚠ Warning: This will replace your existing pre-zprof backup.
  Continue? [y/N]
  ```
- [ ] Add integration test for re-init

**Files:**
- `src/cli/init.rs`

---

### Story 6.5: Add Backup Verification

**As a** user who backed up configs
**I want** verification that backup is complete
**So that** I trust I can restore later

**Acceptance Criteria:**
- [ ] After creating backup, verify:
  - All detected files were backed up
  - File sizes match
  - Checksums match (for critical files)
  - Backup directory is readable
- [ ] Create verification function:
  ```rust
  fn verify_backup(manifest: &BackupManifest) -> Result<VerificationReport> {
      // Check each file in manifest exists in backup
      // Verify sizes and checksums
      // Return detailed report
  }
  ```
- [ ] Show verification results:
  ```
  Verifying backup...
  ✓ All 5 files backed up successfully
  ✓ Checksums verified
  ✓ Backup is complete and valid
  ```
- [ ] If verification fails:
  ```
  ✗ Backup verification failed!

  Issues found:
  • .zshrc: checksum mismatch (file may have changed during backup)
  • .oh-my-zsh/: incomplete backup (only 45/52 MB)

  Recommendation: Run 'zprof init --force' to re-create backup.
  ```
- [ ] Add `--skip-verification` flag for faster init (development use)
- [ ] Add unit tests for verification logic

**Files:**
- `src/backup/verify.rs` (NEW)
- `src/backup/pre_zprof.rs`
- `src/cli/init.rs`

---

### Story 6.6: Add Init Dry-Run Mode

**As a** user considering zprof
**I want** to see what init will do without committing
**So that** I can understand the changes before proceeding

**Acceptance Criteria:**
- [ ] Add `--dry-run` flag to `zprof init`
- [ ] Show what would happen without making changes:
  ```
  zprof init --dry-run

  Dry Run - No changes will be made

  The following operations would be performed:

  Backup:
  ✓ Would back up 5 config files:
    • .zshrc (1.2 KB)
    • .zshenv (234 bytes)
    • .zprofile (567 bytes)
    • .zlogin (123 bytes)
    • .zsh_history (50.1 KB, 1,523 lines)

  ✓ Would back up oh-my-zsh framework (45 MB)

  ✓ Total backup size: ~47 MB
  ✓ Backup location: ~/.zsh-profiles/backups/pre-zprof/

  Cleanup:
  ✓ Would move config files to backup location
  ✓ Would move .oh-my-zsh/ to backup location
  ✓ Would create new .zshenv with zprof integration

  Result:
  • Your HOME directory would be clean
  • All configs would be safely backed up
  • You could restore anytime with 'zprof uninstall'

  Run 'zprof init' without --dry-run to proceed.
  ```
- [ ] Detect potential issues:
  - Not enough disk space for backup
  - Permission errors
  - Conflicting files/directories
- [ ] Add integration test

**Files:**
- `src/cli/init.rs`

---

### Story 6.7: Update Init Documentation

**As a** user
**I want** clear documentation about init behavior
**So that** I understand what happens to my config

**Acceptance Criteria:**
- [ ] Update `docs/user-guide/quick-start.md`:
  - Explain backup and cleanup process
  - Show what happens to root configs
  - Mention restore option
- [ ] Update `docs/user-guide/commands.md`:
  - Document `zprof init` flags:
    - `--force` (re-backup)
    - `--dry-run` (preview)
    - `--skip-verification` (faster init)
  - Add examples
- [ ] Create section in `docs/user-guide/installation.md`:
  ```markdown
  ## What Happens During Init

  When you run `zprof init`, the following occurs:

  1. **Backup**: All your existing shell config is backed up...
  2. **Cleanup**: Root config files are moved to backup...
  3. **Integration**: A new `.zshenv` is created...

  Your original setup is completely preserved and can be
  restored anytime using `zprof uninstall`.
  ```
- [ ] Update `docs/user-guide/faq.md`:
  - "What happens to my .zshrc?"
  - "Can I undo zprof init?"
  - "Is it safe to run init multiple times?"
- [ ] Add troubleshooting section for init issues

**Files:**
- `docs/user-guide/quick-start.md`
- `docs/user-guide/commands.md`
- `docs/user-guide/installation.md`
- `docs/user-guide/faq.md`

---

## Technical Design

### Shell Config Detection

```rust
// src/backup/shell_config.rs

pub struct ShellConfigInfo {
    pub config_files: Vec<ConfigFile>,
    pub history_file: Option<HistoryFile>,
    pub framework: Option<FrameworkInfo>,
    pub total_size: u64,
}

pub struct ConfigFile {
    pub path: PathBuf,
    pub size: u64,
    pub permissions: u32,
    pub is_symlink: bool,
    pub target: Option<PathBuf>,  // If symlink
}

pub struct HistoryFile {
    pub path: PathBuf,
    pub size: u64,
    pub line_count: usize,
}

impl ShellConfigInfo {
    pub fn detect() -> Result<Self> {
        let home = dirs::home_dir()?;

        // Find all .zsh* files
        let config_files = find_zsh_configs(&home)?;

        // Find history
        let history_file = find_history(&home)?;

        // Detect framework
        let framework = detect_framework(&home)?;

        Ok(ShellConfigInfo {
            config_files,
            history_file,
            framework,
            total_size: calculate_total_size(),
        })
    }
}
```

### Init Workflow

```
zprof init
    ↓
Detect existing installation
    ↓
 Already initialized?
    ↓
  ┌─Yes─┐
  ↓     ↓
Show re-init flow
  ↓
  └──────┐
         ↓
Detect shell config
         ↓
Show dry-run preview (if --dry-run)
         ↓
Create backup
         ↓
Verify backup
         ↓
Move root configs
         ↓
Create new .zshenv
         ↓
Show completion summary
```

### Directory Structure After Init

```
HOME/
├── .zshenv                          # zprof integration (NEW)
├── .zsh_history                     # Preserved in place
└── .zsh-profiles/
    ├── config.toml
    ├── current                      # Active profile symlink
    ├── profiles/
    │   └── (empty until first profile created)
    └── backups/
        └── pre-zprof/
            ├── backup-manifest.toml
            ├── .zshrc               # Original backed up
            ├── .zshenv              # Original backed up
            ├── .zprofile            # Original backed up
            ├── .zsh_history         # Copy for safety
            └── frameworks/
                └── .oh-my-zsh/      # Original framework
```

## Dependencies

- **Epic 3 (Complete Uninstall System)**: Shares backup logic and manifest format

## Risks & Mitigations

**Risk:** User loses shell config during init
**Mitigation:** Backup created first, verification before cleanup, dry-run mode

**Risk:** Backup fails mid-process
**Mitigation:** Atomic operations, rollback on error, clear error messages

**Risk:** Existing shells break after cleanup
**Mitigation:** Warning to restart shells, clear documentation

**Risk:** Framework in backup doesn't work when restored
**Mitigation:** Complete tarball backup, preserve permissions, test restoration

## Testing Strategy

- Unit tests for config detection logic
- Unit tests for backup verification
- Integration tests for full init flow
- Integration tests for re-init scenarios
- Edge case tests (symlinks, read-only files, missing permissions)
- Snapshot tests for output messages
- Manual testing on fresh systems with various frameworks

## Success Criteria

- [ ] All existing shell config is backed up before modifications
- [ ] Root config files are moved to backup location
- [ ] Only zprof `.zshenv` remains in HOME root
- [ ] Backup is verified as complete and valid
- [ ] Re-running init is safe and doesn't break backups
- [ ] Users can restore original setup via uninstall
- [ ] Dry-run mode shows accurate preview
- [ ] Clear documentation and error messages
- [ ] All integration tests passing
- [ ] No data loss in any scenario

## Out of Scope

- Windows support (future)
- Cloud backup of pre-zprof config (v0.4.0)
- Automatic framework migration (manual restore only)
- Backup encryption (all local, user's filesystem security)
