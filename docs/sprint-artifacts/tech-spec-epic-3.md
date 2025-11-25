# Epic Technical Specification: Complete Uninstall System

Date: 2025-11-24
Author: Anna
Epic ID: 3
Status: Draft

---

## Overview

This technical specification details the implementation of a comprehensive uninstall system for zprof that allows users to safely remove the tool while providing flexible options for restoring their original shell configuration, promoting a profile to root, or performing a clean removal. The epic addresses a critical adoption barrier: users hesitate to try zprof because they worry about getting "locked in" or breaking their shell setup.

The uninstall system consists of three major components: (1) enhanced init-time backup creation to capture pre-zprof state, (2) a flexible uninstall command with multiple restoration strategies, and (3) safety mechanisms including final snapshots and validation to prevent data loss.

## Objectives and Scope

**In Scope:**
- Automatic backup of pre-zprof shell configurations during `zprof init`
- Moving root configs to backup location after initial backup
- Interactive uninstall command with three restoration options:
  - Restore original pre-zprof backup
  - Promote a profile to become root configuration
  - Clean removal without restoration
- Safety backup creation (tarball snapshot) before any destructive operations
- Complete cleanup of zprof files and directories
- Confirmation screens with detailed summaries
- Comprehensive error handling and edge case coverage
- Documentation for uninstall procedures and recovery

**Out of Scope:**
- Automatic detection of active shell sessions (deferred to v0.3.0)
- Cloud backup of safety snapshots (deferred to v0.4.0)
- Scheduled automatic backups (deferred to v0.4.0)
- Restoration of partially corrupted backups (manual recovery only)

## System Architecture Alignment

**Architectural Integration:**

This epic integrates with the following components from the existing architecture:

1. **Init System (`src/cli/init.rs`)**: Enhanced to create pre-zprof backups and move root configs
2. **Core Filesystem (`src/core/filesystem.rs`)**: Leverages existing safe file operations for backup/restore
3. **Core Config (`src/core/config.rs`)**: Reads global config to identify profile structure
4. **Shell Integration (`src/shell/`)**: Removes zprof-generated `.zshenv` and ZDOTDIR references

**New Modules:**
- `src/cli/uninstall.rs` - Uninstall command implementation
- `src/backup/pre_zprof.rs` - Pre-zprof backup creation and manifest
- `src/backup/restore.rs` - Restoration logic for all three options
- `src/backup/snapshot.rs` - Safety tarball creation
- `src/cleanup/mod.rs` - File removal coordination

**Architectural Principles Maintained:**
- **Non-destructive (until explicit uninstall)**: Safety backups created before any removal
- **Safe**: Validation, rollback on errors, final backup as recovery net
- **Modular**: Clear separation between backup, restore, and cleanup concerns

**Constraints:**
- Must preserve all existing functionality when NOT uninstalling
- Backup system must work across macOS and Linux filesystems
- No external dependencies for backup/restore (pure Rust using std::fs and tar crate)
- Final snapshot must complete in <10 seconds for typical profile sizes

## Detailed Design

### Services and Modules

| Module | Responsibility | Key Functions | Owner |
|--------|---------------|---------------|-------|
| `src/cli/uninstall.rs` | Orchestrate uninstall workflow, present TUI options, coordinate backup/restore/cleanup | `execute()`, `select_restoration_option()`, `confirm_uninstall()` | Story 3.3 |
| `src/backup/pre_zprof.rs` | Create and validate pre-zprof backups during init | `create_backup()`, `BackupManifest::save()`, `validate_backup()` | Story 3.1 |
| `src/backup/restore.rs` | Restore configurations from backup or profile | `restore_pre_zprof()`, `promote_profile()`, `RestorationPlan::execute()` | Story 3.3, 3.7 |
| `src/backup/snapshot.rs` | Create safety tarball before uninstall | `create_final_snapshot()`, `archive_profiles()` | Story 3.4 |
| `src/backup/mod.rs` | Backup module coordination | Re-exports, common types | Story 3.1 |
| `src/cleanup/mod.rs` | Remove zprof files and directories | `remove_profiles()`, `remove_zshenv()`, `cleanup_all()` | Story 3.5 |
| `src/tui/uninstall_select.rs` | TUI for restoration option selection | `show_restoration_menu()`, `show_profile_selector()` | Story 3.3 |
| `src/tui/uninstall_confirm.rs` | TUI for uninstall confirmation summary | `show_confirmation()`, `format_summary()` | Story 3.6 |
| `src/core/backup_manifest.rs` | TOML manifest for backup metadata | `BackupManifest` struct, serialization | Story 3.1 |

**Module Interaction Flow:**
1. `init.rs` → `pre_zprof::create_backup()` during initialization
2. `uninstall.rs` → `uninstall_select.rs` → user choice
3. `uninstall.rs` → `snapshot::create_final_snapshot()` → safety backup
4. `uninstall.rs` → `restore.rs` → restore configuration
5. `uninstall.rs` → `cleanup::cleanup_all()` → remove files

### Data Models and Contracts

**BackupManifest (`src/core/backup_manifest.rs`)**

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct BackupManifest {
    pub metadata: BackupMetadata,
    pub detected_framework: Option<DetectedFramework>,
    pub files: Vec<BackedUpFile>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackupMetadata {
    pub created_at: DateTime<Utc>,
    pub zsh_version: String,
    pub os: String,
    pub zprof_version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectedFramework {
    pub name: String,
    pub path: PathBuf,
    pub config_files: Vec<PathBuf>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BackedUpFile {
    pub path: PathBuf,          // Relative to HOME
    pub size: u64,
    pub checksum: String,       // SHA256 hex
    pub permissions: u32,       // Unix permissions
    pub is_symlink: bool,
    pub symlink_target: Option<PathBuf>,
}
```

**RestorationPlan (`src/backup/restore.rs`)**

```rust
#[derive(Debug)]
pub enum RestoreOption {
    PreZprof,
    PromoteProfile(String),
    NoRestore,
}

#[derive(Debug)]
pub struct RestorationPlan {
    pub option: RestoreOption,
    pub files_to_restore: Vec<FileOperation>,
    pub files_to_remove: Vec<PathBuf>,
    pub backup_source: PathBuf,
    pub history_handling: HistoryHandling,
}

#[derive(Debug)]
pub struct FileOperation {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub operation: OperationType,
}

#[derive(Debug)]
pub enum OperationType {
    Copy,
    Move,
    Symlink,
}

#[derive(Debug)]
pub enum HistoryHandling {
    Restore,
    Merge,
    Skip,
}
```

**UninstallSummary (for confirmation screen)**

```rust
#[derive(Debug)]
pub struct UninstallSummary {
    pub restoration: RestorationSummary,
    pub cleanup: CleanupSummary,
    pub safety: SafetySummary,
}

#[derive(Debug)]
pub struct RestorationSummary {
    pub option: RestoreOption,
    pub file_count: usize,
    pub history_entries: Option<usize>,
    pub source_date: Option<DateTime<Utc>>,
}

#[derive(Debug)]
pub struct CleanupSummary {
    pub profile_count: usize,
    pub total_size: u64,
    pub directories: Vec<PathBuf>,
}

#[derive(Debug)]
pub struct SafetySummary {
    pub backup_path: PathBuf,
    pub backup_size: u64,
}
```

### APIs and Interfaces

**CLI Command Interface**

```rust
// src/cli/uninstall.rs
#[derive(Debug, Args)]
pub struct UninstallArgs {
    /// Skip confirmation prompts (non-interactive)
    #[arg(long, short = 'y')]
    pub yes: bool,

    /// Skip creating safety backup
    #[arg(long)]
    pub no_backup: bool,

    /// Keep backups directory when removing profiles
    #[arg(long)]
    pub keep_backups: bool,

    /// Specify restoration option directly (skip TUI)
    #[arg(long, value_enum)]
    pub restore: Option<RestoreOptionCli>,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum RestoreOptionCli {
    Original,
    Promote,
    Clean,
}

pub fn execute(args: UninstallArgs) -> Result<()>
```

**Backup Module Public API**

```rust
// src/backup/pre_zprof.rs
pub fn create_backup(home_dir: &Path, backup_dir: &Path) -> Result<BackupManifest>
pub fn validate_backup(backup_dir: &Path) -> Result<BackupManifest>
pub fn backup_exists(backup_dir: &Path) -> bool

// src/backup/restore.rs
pub fn restore_pre_zprof(backup_dir: &Path, home_dir: &Path) -> Result<()>
pub fn promote_profile(profile_name: &str, home_dir: &Path) -> Result<()>
pub fn create_restoration_plan(option: RestoreOption) -> Result<RestorationPlan>

// src/backup/snapshot.rs
pub fn create_final_snapshot(profiles_dir: &Path, output_path: &Path) -> Result<u64>
```

**Cleanup Module Public API**

```rust
// src/cleanup/mod.rs
pub fn remove_profiles_dir(profiles_dir: &Path, keep_backups: bool) -> Result<()>
pub fn remove_zprof_zshenv(home_dir: &Path) -> Result<()>
pub fn cleanup_all(config: &CleanupConfig) -> Result<CleanupReport>

pub struct CleanupConfig {
    pub profiles_dir: PathBuf,
    pub home_dir: PathBuf,
    pub keep_backups: bool,
}

pub struct CleanupReport {
    pub removed_files: Vec<PathBuf>,
    pub removed_dirs: Vec<PathBuf>,
    pub errors: Vec<CleanupError>,
}
```

### Workflows and Sequencing

**Init Enhancement Sequence (Story 3.1-3.2)**

```
User runs: zprof init
         ↓
1. Check if pre-zprof backup exists
   ├─ YES → Skip backup creation
   └─ NO  → Continue
         ↓
2. Create backup directory: ~/.zsh-profiles/backups/pre-zprof/
         ↓
3. Scan HOME for zsh config files:
   - .zshrc, .zshenv, .zprofile, .zlogin, .zlogout
   - .zsh_history
   - Framework directories (.oh-my-zsh, etc.)
         ↓
4. For each file:
   - Copy to backup directory
   - Calculate SHA256 checksum
   - Record metadata (size, permissions, symlink status)
         ↓
5. Detect existing framework (if any)
   - Scan for .oh-my-zsh, .zimfw, etc.
   - Record framework info in manifest
         ↓
6. Create backup-manifest.toml
         ↓
7. Move (not copy) backed-up files from HOME to backup dir
   - Leave only zprof-generated .zshenv
         ↓
8. Display confirmation message
   "✓ Backed up your existing shell config to ~/.zsh-profiles/backups/pre-zprof/"
         ↓
9. Continue with normal init process
```

**Uninstall Workflow Sequence (Stories 3.3-3.6)**

```
User runs: zprof uninstall
         ↓
1. Validate zprof is installed
   - Check ~/.zsh-profiles/ exists
   - Check config.toml exists
         ↓
2. Load configuration
   - Read active profile
   - Count profiles
   - Get backup locations
         ↓
3. Show restoration option TUI (unless --restore specified)
   ┌──────────────────────────────────────────────┐
   │ What would you like to do?                   │
   │                                              │
   │ > Restore original (pre-zprof backup)        │
   │   Promote profile to root                    │
   │   Clean removal (no restoration)             │
   │   Cancel                                     │
   └──────────────────────────────────────────────┘
         ↓
4. If "Promote profile" selected:
   - Show profile selection TUI
   - User selects profile
         ↓
5. Create restoration plan
   - Identify files to restore
   - Identify files to remove
   - Calculate sizes, counts
         ↓
6. Show confirmation summary (unless --yes)
   ┌──────────────────────────────────────────────┐
   │ Uninstall Summary                            │
   │                                              │
   │ Restoration:                                 │
   │   • Restore pre-zprof backup (Jan 15, 2025)  │
   │   • 4 files will be restored to HOME         │
   │   • History: 15,234 entries                  │
   │                                              │
   │ Cleanup:                                     │
   │   • Remove 3 profiles (work, personal, min)  │
   │   • Remove ~/.zsh-profiles/ (2.3 MB)         │
   │   • Remove zprof shell integration           │
   │                                              │
   │ Safety:                                      │
   │   • Final backup: final-snapshot-[date].tgz  │
   │                                              │
   │ Continue? [y/N]                              │
   └──────────────────────────────────────────────┘
         ↓
7. Create safety backup (unless --no-backup)
   - Archive entire ~/.zsh-profiles/
   - Save to ~/.zsh-profiles/backups/final-snapshot-{timestamp}.tar.gz
   - Display: "✓ Safety backup created (2.3 MB)"
         ↓
8. Execute restoration
   ├─ Restore Pre-Zprof:
   │  - Copy files from backups/pre-zprof/ to HOME
   │  - Restore .zsh_history
   │  - Verify checksums
   │
   ├─ Promote Profile:
   │  - Copy profile configs to HOME
   │  - Copy profile history to .zsh_history
   │  - Merge shared history if enabled
   │
   └─ Clean Removal:
      - Skip restoration
         ↓
9. Execute cleanup
   - Remove ~/.zsh-profiles/profiles/
   - Remove ~/.zsh-profiles/shared/
   - Remove ~/.zsh-profiles/config.toml
   - Remove zprof-generated ~/.zshenv
   - Keep backups/ if --keep-backups
   - Otherwise remove entire ~/.zsh-profiles/
         ↓
10. Display completion message
    "✅ zprof uninstalled successfully

     Your shell config has been restored to pre-zprof state.
     Safety backup available at: [path]

     Restart your shell to complete: exec zsh"
```

**Error Handling Flow**

```
At any step, if error occurs:
         ↓
1. Capture error with context
         ↓
2. Check if state is revertible
   ├─ Before cleanup → Rollback restoration
   └─ During cleanup → Document partial state
         ↓
3. Display error message with recovery instructions
   "Error: Failed to restore .zshrc

    Your data is safe in the final backup:
    ~/.zsh-profiles/backups/final-snapshot-[date].tar.gz

    To manually recover:
    1. Extract the backup
    2. Copy files to HOME as needed

    Would you like to retry? [y/n]"
         ↓
4. If retry selected → Jump to failed step
5. If abort → Exit with non-zero status
```

## Non-Functional Requirements

### Performance

| Metric | Target | Rationale |
|--------|--------|-----------|
| Backup creation during init | < 2 seconds | Most users have < 10 config files, should feel instant |
| Safety snapshot creation | < 10 seconds | Typical profiles total 2-5 MB, tar compression is fast |
| Restoration execution | < 3 seconds | Copying files from backup to HOME, minimal processing |
| Total uninstall time | < 20 seconds | End-to-end including TUI interactions, backup, restore, cleanup |
| Checksum validation | SHA256, all files | Balance security and speed, standard for integrity |

**Performance Constraints:**
- Must work efficiently on slow filesystems (NFS mounts, network drives)
- No operations should block shell startup (all run before zsh initialization)
- Progress feedback required for any operation > 1 second

### Security

**Data Integrity:**
- All backup files verified with SHA256 checksums before and after copy
- Manifest includes checksums for validation during restoration
- Symlink targets verified to prevent following malicious links outside HOME

**File Permissions:**
- Preserve original Unix permissions during backup and restore (chmod mode)
- Backup directory permissions: 700 (owner read/write/execute only)
- Manifest file permissions: 600 (owner read/write only)
- Safety tarball permissions: 600 (owner read/write only)

**Path Validation:**
- All file operations validate paths are within expected directories
- Prevent path traversal attacks via ".." in filenames
- Symlink resolution checks to prevent escaping HOME directory

**Sensitive Data Handling:**
- History files may contain sensitive commands (API keys, passwords)
- Backups stored in user's HOME directory only (no tmp files)
- No logging of file contents or history entries
- No transmission of backup data (all local operations)

**Authentication/Authorization:**
- No authentication required (local user operations only)
- All operations run with current user's permissions
- Cannot modify files outside user's ownership

### Reliability/Availability

**Error Recovery:**
- Rollback mechanism for failed restorations (restore pre-uninstall state from safety backup)
- Atomic file operations where possible (write to temp, then move)
- Partial cleanup recovery: document state, provide manual recovery steps
- Idempotency: re-running backup creation is safe (skips if exists)

**Data Durability:**
- Safety backup created BEFORE any destructive operations
- Pre-zprof backup never deleted during uninstall (unless entire dir removed)
- Final snapshot preserved even after successful uninstall
- Backup manifest includes all metadata needed for recovery

**Graceful Degradation:**
- If framework detection fails during backup: proceed without framework info
- If checksum validation fails during restore: warn user but allow continuation with flag
- If history file missing: continue without history restoration
- If cleanup partially fails: report succeeded/failed items, allow retry

**Availability:**
- No network dependencies (all local filesystem operations)
- No external service dependencies
- Works offline
- No database required

**Failure Modes:**
| Failure | Detection | Recovery |
|---------|-----------|----------|
| Insufficient disk space | Check before backup/snapshot creation | Abort with clear message, suggest cleanup |
| File permission denied | Try operation, catch error | Display affected file, suggest manual fix |
| Concurrent modification | Checksum mismatch | Abort restoration, alert user, preserve safety backup |
| Corrupted backup manifest | TOML parse error | Abort, suggest manual inspection of backup directory |
| Missing pre-zprof backup | Check directory existence | Disable "Restore original" option, offer alternatives |

### Observability

**Logging:**
- Log level: INFO for normal operations, DEBUG for troubleshooting
- Log file location: `~/.zsh-profiles/logs/uninstall-{timestamp}.log`
- Log retention: Keep last 5 uninstall logs
- Log contents:
  - Timestamp for each operation
  - Files backed up/restored/removed with sizes
  - Checksum validations
  - Error messages with full context
  - User choices (restoration option selected)

**Progress Feedback:**
- Spinner for operations < 3 seconds
- Progress bar for operations > 3 seconds (snapshot creation, large restorations)
- Step-by-step indicators:
  ```
  Creating safety backup...
  ✓ Archived profiles (1.2 MB)
  ✓ Archived shared history (0.5 MB)
  ✓ Archived backups (0.6 MB)
  ✓ Safety backup created: final-snapshot-20250124.tar.gz (2.3 MB)
  ```

**Metrics Tracked:**
- Backup creation time (init)
- Safety snapshot size and creation time
- Number of files restored/removed
- Total uninstall duration
- Error rate (failures per operation type)

**User-Facing Output:**
- Clear success/failure messages
- File counts and sizes in human-readable format (MB, not bytes)
- Actionable error messages with suggestions
- Recovery instructions if uninstall fails mid-process

**Debugging Support:**
- `--verbose` flag for detailed output to stdout
- Manifest files human-readable (TOML format)
- Safety backups easily extractable (standard tar.gz)
- Log files include full error stack traces

## Dependencies and Integrations

### External Dependencies

All dependencies already present in `Cargo.toml`, no new crates required:

| Crate | Version | Purpose | Usage in Epic |
|-------|---------|---------|---------------|
| `std::fs` | stdlib | File operations | Backup/restore file copying, directory creation/removal |
| `std::path` | stdlib | Path manipulation | Building paths to backups, profiles, HOME |
| `anyhow` | 1.0 | Error handling | Result types, error context in all modules |
| `chrono` | 0.4 | Date/time | Timestamps in manifest, snapshot filenames |
| `serde` | 1.0 | Serialization | Manifest TOML serialization/deserialization |
| `toml` | 0.9 | TOML parsing | Read/write `backup-manifest.toml` |
| `tar` | 0.4 | Tarball creation | Create final safety snapshots |
| `flate2` | 1.0 | Gzip compression | Compress tar archives (tar.gz) |
| `dialoguer` | 0.11 | Interactive prompts | Confirmation dialogs, option selection |
| `ratatui` | 0.29.0 | TUI framework | Restoration option menu, profile selector |
| `crossterm` | 0.29.0 | Terminal control | TUI rendering, key events |
| `indicatif` | 0.18 | Progress bars | Safety backup creation progress |
| `sha2` | (via std) | Checksums | SHA256 for file integrity validation |
| `clap` | 4.5.51 | CLI parsing | Parse `UninstallArgs` |

**No new dependencies required** - all functionality can be implemented with existing crates.

### Internal Module Dependencies

**Direct Dependencies (this epic modifies/uses):**
- `src/cli/init.rs` - Enhanced to call pre-zprof backup creation
- `src/core/config.rs` - Read global config to find profiles directory
- `src/core/filesystem.rs` - Reuse safe file operations (copy_with_backup, etc.)
- `src/frameworks/detector.rs` - Detect existing frameworks during backup

**New Modules (created by this epic):**
- `src/backup/mod.rs` - New module root
- `src/backup/pre_zprof.rs` - Pre-zprof backup logic
- `src/backup/restore.rs` - Restoration logic
- `src/backup/snapshot.rs` - Safety tarball creation
- `src/cleanup/mod.rs` - Cleanup coordination
- `src/core/backup_manifest.rs` - Manifest data model
- `src/cli/uninstall.rs` - Uninstall command
- `src/tui/uninstall_select.rs` - Restoration option TUI
- `src/tui/uninstall_confirm.rs` - Confirmation summary TUI

**Module Dependency Graph:**

```
src/cli/uninstall.rs
    ├─> src/backup/snapshot.rs
    │       └─> tar, flate2
    ├─> src/backup/restore.rs
    │       ├─> src/backup/pre_zprof.rs
    │       ├─> src/core/filesystem.rs
    │       └─> src/core/config.rs
    ├─> src/cleanup/mod.rs
    │       └─> src/core/filesystem.rs
    ├─> src/tui/uninstall_select.rs
    │       └─> ratatui, crossterm
    └─> src/tui/uninstall_confirm.rs
            └─> dialoguer

src/cli/init.rs
    └─> src/backup/pre_zprof.rs
            ├─> src/core/backup_manifest.rs
            │       └─> serde, toml, chrono
            ├─> src/frameworks/detector.rs
            └─> src/core/filesystem.rs
```

### Integration Points

**Epic 6 (Init Cleanup) Integration:**
- Both epics modify `src/cli/init.rs`
- Epic 3 adds pre-zprof backup creation
- Epic 6 refactors init structure and validation
- **Coordination:** Epic 3 backup logic should be modular function called from refactored init
- **Resolution:** Ensure Epic 6 stories preserve backup creation calls

**Existing CLI Command Integration:**
- `src/cli/mod.rs` - Add `uninstall` subcommand registration
- `src/main.rs` - Wire up UninstallArgs to execute function

**Existing Filesystem Integration:**
- Leverage `src/core/filesystem.rs::copy_with_backup()` for safe file operations
- Extend if needed with checksumming functionality

**Framework Detection Integration:**
- Use `src/frameworks/detector.rs::detect_framework()` during backup
- No modifications to detector needed, read-only usage

### System Integration

**Shell Environment:**
- Reads: `$HOME` environment variable
- Reads: `~/.zsh-profiles/config.toml` (global config)
- Reads: `~/.zsh-profiles/profiles/*/profile.toml` (profile manifests)
- Modifies: `~/.zshenv` (removes zprof-generated version during uninstall)
- Modifies: Shell config files in HOME (restores from backup)

**Filesystem Layout:**

```
$HOME/
├── .zsh-profiles/
│   ├── config.toml                          [read, delete on uninstall]
│   ├── profiles/                            [read, delete on uninstall]
│   │   ├── work/
│   │   ├── personal/
│   │   └── minimal/
│   ├── shared/                              [delete on uninstall]
│   │   └── .zsh_history
│   ├── backups/
│   │   ├── pre-zprof/                       [create on init, read on uninstall]
│   │   │   ├── backup-manifest.toml
│   │   │   ├── .zshrc
│   │   │   ├── .zshenv
│   │   │   └── .zsh_history
│   │   └── final-snapshot-YYYYMMDD.tar.gz   [create on uninstall]
│   └── logs/
│       └── uninstall-{timestamp}.log        [create on uninstall]
├── .zshrc                                   [restore on uninstall]
├── .zshenv                                  [restore/remove on uninstall]
└── .zsh_history                             [restore on uninstall]
```

**No External Services:**
- No network calls
- No database connections
- No cloud storage
- Entirely local filesystem operations

## Acceptance Criteria (Authoritative)

Epic-level acceptance criteria extracted and consolidated from all 8 user stories:

### Epic Success Criteria

1. **Pre-zprof Backup Created During Init**
   - Pre-zprof backup directory created at `~/.zsh-profiles/backups/pre-zprof/` during first init
   - All root shell config files backed up (.zshrc, .zshenv, .zprofile, .zlogin, .zlogout, .zsh_history)
   - Backup manifest created with timestamps, checksums, and framework detection
   - Skip backup if already exists (idempotent)
   - Confirmation message displayed showing backup location

2. **Root Configs Moved After Backup**
   - Backed-up files moved (not copied) from HOME to backup directory
   - Only zprof-generated .zshenv remains in HOME root
   - Symlinks resolved and backed up correctly
   - File permissions preserved
   - Clear output message documenting the move

3. **Uninstall Command Available**
   - `zprof uninstall` command registered and executable
   - Three restoration options available: Restore Original, Promote Profile, Clean Removal
   - Interactive TUI displays restoration options with descriptions
   - Non-interactive mode via `--restore` and `--yes` flags
   - Cancel option returns without changes

4. **Restore Original Option Works**
   - Files restored from `backups/pre-zprof/` to HOME
   - History file restored with correct entry count
   - Checksums validated during restoration
   - Error if pre-zprof backup missing (with clear message)
   - Original shell config fully functional after restoration

5. **Promote Profile Option Works**
   - Profile selection TUI shows all available profiles
   - Selected profile's configs copied to HOME root
   - Profile history merged into .zsh_history
   - Profile becomes functional as root config after exec zsh

6. **Clean Removal Option Works**
   - No restoration performed
   - HOME directory left clean (no configs restored)
   - User can manually set up shell after removal

7. **Safety Backup Created**
   - Final snapshot tarball created before destructive operations
   - Tarball includes all profiles, history, backups, config
   - Displayed file path and size to user
   - Tarball extractable with standard tar command
   - Abort uninstall if backup creation fails
   - `--no-backup` flag available for scripting

8. **Complete Cleanup Performed**
   - Entire `~/.zsh-profiles/` directory removed
   - Zprof-generated `~/.zshenv` removed
   - Restored configuration preserved
   - Progress messages shown during cleanup
   - Errors handled gracefully with clear messages
   - `--keep-backups` flag preserves backup directory only

9. **Confirmation Screen Shown**
   - Detailed summary displayed before uninstall proceeds
   - Summary includes: restoration option, file counts, sizes, dates
   - Destructive operations highlighted
   - User must explicitly confirm (default: No)
   - `--yes` flag bypasses confirmation for automation

10. **Edge Cases Handled**
    - Works when no pre-zprof backup exists (disables Restore option)
    - Handles file conflicts during restoration (prompt or backup)
    - Validates user has write permissions
    - Handles symlinks correctly
    - Partial failures rolled back or documented for manual recovery
    - Cannot delete active profile (must deactivate first)
    - Warnings for active shell sessions (if detectable)

11. **Documentation Complete**
    - `docs/user-guide/commands.md` updated with uninstall command
    - `docs/user-guide/uninstalling.md` created with step-by-step guide
    - FAQ updated with uninstall questions
    - Recovery procedures documented
    - Installation guide mentions safe removal

12. **Testing Complete**
    - Unit tests for backup creation, restoration, cleanup
    - Integration tests for all three restoration workflows
    - Edge case tests (missing backups, permissions, conflicts)
    - Snapshot tests for TUI output
    - All tests passing, no regressions

## Traceability Mapping

Map acceptance criteria → technical design → components → test strategy:

| AC # | Spec Section | Component(s) | API(s) | Test Approach |
|------|-------------|--------------|--------|---------------|
| 1 | Init Enhancement Sequence | `backup/pre_zprof.rs`, `core/backup_manifest.rs` | `create_backup()`, `BackupManifest::save()` | Unit: backup creation, manifest serialization<br>Integration: full init flow |
| 2 | Init Enhancement Sequence | `backup/pre_zprof.rs`, `cli/init.rs` | `create_backup()` | Integration: verify files moved, not copied<br>Snapshot: output messages |
| 3 | Uninstall Workflow, CLI Interface | `cli/uninstall.rs`, `tui/uninstall_select.rs` | `execute()`, `select_restoration_option()` | Integration: command execution<br>Snapshot: TUI rendering |
| 4 | Restoration, Error Handling | `backup/restore.rs`, `backup/pre_zprof.rs` | `restore_pre_zprof()`, `validate_backup()` | Unit: file copying, checksum validation<br>Integration: end-to-end restore |
| 5 | Restoration, Promote Profile | `backup/restore.rs`, `core/profile.rs` | `promote_profile()`, `create_restoration_plan()` | Integration: profile promotion flow<br>Manual: verify shell works post-promotion |
| 6 | Restoration, Clean Removal | `backup/restore.rs`, `cleanup/mod.rs` | `RestorationPlan::execute()`, `cleanup_all()` | Integration: clean removal, verify HOME state |
| 7 | Uninstall Workflow (Step 7) | `backup/snapshot.rs` | `create_final_snapshot()` | Unit: tarball creation<br>Integration: verify archive contents |
| 8 | Uninstall Workflow (Step 9), Cleanup | `cleanup/mod.rs` | `cleanup_all()`, `remove_profiles_dir()`, `remove_zprof_zshenv()` | Unit: directory removal<br>Integration: verify clean state post-uninstall |
| 9 | Uninstall Workflow (Step 6), Confirmation | `tui/uninstall_confirm.rs` | `show_confirmation()`, `format_summary()` | Snapshot: confirmation screen<br>Integration: user confirmation flow |
| 10 | Error Handling Flow, Edge Cases | `backup/restore.rs`, `cli/uninstall.rs` | All error paths | Edge case tests: missing backups, permissions, conflicts<br>Unit: validation logic |
| 11 | Documentation (out of code scope) | N/A | N/A | Manual review of documentation completeness |
| 12 | All sections | All modules | All APIs | Test coverage: cargo tarpaulin >80%<br>CI: all tests pass on macOS/Linux |

**Component Coverage Matrix:**

| Component | Stories | Primary Tests | Secondary Tests |
|-----------|---------|---------------|-----------------|
| `backup/pre_zprof.rs` | 3.1, 3.2 | backup_creation_test.rs | init_test.rs (integration) |
| `backup/restore.rs` | 3.3, 3.7 | restore_test.rs | uninstall_test.rs (integration) |
| `backup/snapshot.rs` | 3.4 | snapshot_test.rs | uninstall_test.rs |
| `cleanup/mod.rs` | 3.5 | cleanup_test.rs | uninstall_test.rs |
| `cli/uninstall.rs` | 3.3, 3.6, 3.7 | uninstall_test.rs (integration) | - |
| `tui/uninstall_select.rs` | 3.3 | tui_snapshot_tests.rs | - |
| `tui/uninstall_confirm.rs` | 3.6 | tui_snapshot_tests.rs | - |
| `core/backup_manifest.rs` | 3.1 | manifest_test.rs | - |

**Epic → Story → Test Plan:**

- **Story 3.1**: Pre-zprof backup → Unit tests (backup logic) + Integration (init flow)
- **Story 3.2**: Move configs → Integration tests (verify move semantics)
- **Story 3.3**: Uninstall command → Integration tests (all 3 restoration options)
- **Story 3.4**: Safety backup → Unit tests (tar creation) + Integration (uninstall flow)
- **Story 3.5**: Cleanup logic → Unit tests (removal) + Integration (complete uninstall)
- **Story 3.6**: Confirmation screen → Snapshot tests (TUI output)
- **Story 3.7**: Edge cases → Edge case test suite (15+ scenarios)
- **Story 3.8**: Documentation → Manual review checklist

## Risks, Assumptions, Open Questions

### Risks

| Risk | Severity | Probability | Mitigation | Owner |
|------|----------|-------------|------------|-------|
| **User loses shell config during failed uninstall** | CRITICAL | LOW | Safety backup created BEFORE destructive operations; rollback mechanism; extensive testing | Story 3.4, 3.7 |
| **Restored config doesn't work (incompatibilities)** | HIGH | MEDIUM | Clear documentation of restoration process; safety backup remains for manual recovery; warn about potential framework version mismatches | Story 3.7, 3.8 |
| **History loss during restoration** | HIGH | LOW | Explicit history handling in plan; show line counts in summary; validate history files exist before restoration | Story 3.3, 3.7 |
| **Active shell sessions break after uninstall** | MEDIUM | HIGH | Warning message to close active shells; document limitation; future: detect active sessions (v0.3.0) | Story 3.7, 3.8 |
| **Concurrent zprof operations corrupt state** | MEDIUM | LOW | Uninstall requires exclusive access; no file locking implemented (assumption: single user usage) | Assumption 3 |
| **Disk space exhaustion during backup** | MEDIUM | LOW | Check available space before snapshot creation; typical profiles are small (< 10 MB) | Story 3.4 |
| **Symlink following security issue** | LOW | LOW | Validate symlink targets stay within HOME; don't follow links outside safe directories | Story 3.7 |
| **Epic 6 merge conflicts in init.rs** | MEDIUM | MEDIUM | Coordinate with Epic 6 implementation; modular backup function easy to integrate; both epics tested together | Integration Point |

### Assumptions

| # | Assumption | Validation | Impact if Wrong |
|---|------------|------------|-----------------|
| 1 | Users have < 100 config files in HOME | Typical usage patterns | If wrong: longer backup times, but still works |
| 2 | Pre-zprof backup created during first init only | Init logic enforces this | If re-init overwrites: original backup lost (MUST prevent) |
| 3 | Single user runs zprof operations at a time | No file locking needed | If wrong: potential corruption during concurrent uninstall |
| 4 | Users read confirmation messages | Safety critical | If wrong: accidental data loss (MUST show confirmation) |
| 5 | Zsh config files are text-based TOML/shell scripts | Standard zsh patterns | If wrong: backup/restore may fail for binary configs (unlikely) |
| 6 | HOME environment variable is set correctly | Standard on all *nix systems | If wrong: cannot determine backup location (fail early) |
| 7 | Users have write permissions to HOME | Standard user setup | If wrong: backup/restore fails with clear error |
| 8 | Framework directories are self-contained | oh-my-zsh, zimfw patterns | If wrong: incomplete framework backup (acceptable, focus on configs) |

**Assumption Validation Plan:**
- Assumption 2: Add explicit check in init.rs to prevent overwriting existing pre-zprof backup
- Assumption 4: Require explicit confirmation (default: No) for destructive operations
- Assumption 6: Validate HOME env var exists and is directory before any operations
- Assumption 7: Check write permissions before backup/restore operations

### Open Questions

| Question | Status | Resolution | Owner |
|----------|--------|------------|-------|
| Q1: Should uninstall detect active zprof shell sessions? | OPEN | Deferred to v0.3.0 - warn users manually for now | Story 3.8 (docs) |
| Q2: What if user's .zsh_history is > 100 MB? | RESOLVED | No special handling, back up as-is; safety backup may be large but acceptable | Story 3.4 |
| Q3: Should we support partial uninstall (keep some profiles)? | OPEN | Out of scope for MVP; full uninstall only; future enhancement | Epic scope |
| Q4: How to handle framework binary files (oh-my-zsh .git, etc.)? | RESOLVED | Don't back up framework binaries, only config files; users can reinstall frameworks | Story 3.1 |
| Q5: Should promote-profile merge or replace history? | RESOLVED | Merge if shared history enabled, replace otherwise; show line count in summary | Story 3.3 |
| Q6: What if Epic 6 refactors init.rs significantly? | OPEN | Coordinate during sprint planning; Epic 3 backup function is modular and reusable | Integration |
| Q7: Should we validate restored configs with `zsh -n`? | RESOLVED | Yes for generated configs, no for user backups (may have intentional syntax for plugins) | Story 3.7 |

## Test Strategy Summary

### Test Pyramid

```
       /\
      /  \        E2E: 2 scenarios (full uninstall on fresh system)
     /____\
    /      \       Integration: 8 test suites
   /________\
  /          \     Unit: 25+ test cases
 /__________  \
```

**Target Coverage:** 80%+ code coverage via `cargo tarpaulin`

### Unit Tests

**Location:** `tests/unit/` and inline `#[cfg(test)] mod tests`

**Coverage:**
- Backup creation logic (manifest generation, file copying, checksumming)
- Restoration plan creation (option selection, file operation planning)
- Snapshot tarball creation and extraction
- Cleanup file removal logic
- Manifest TOML serialization/deserialization
- Path validation and sanitization
- Checksum calculation and validation
- Permission preservation logic

**Tools:** `cargo test`, `insta` for snapshot assertions

**Example Test Cases:**
```rust
#[test]
fn test_backup_manifest_serialization()
#[test]
fn test_create_backup_with_symlinks()
#[test]
fn test_restoration_plan_for_promote_profile()
#[test]
fn test_cleanup_removes_all_zprof_files()
#[test]
fn test_checksum_validation_detects_corruption()
```

### Integration Tests

**Location:** `tests/integration/`

**Test Suites:**
1. `init_backup_test.rs` - Full init flow with backup creation
2. `uninstall_restore_original_test.rs` - Restore pre-zprof backup flow
3. `uninstall_promote_profile_test.rs` - Promote profile to root flow
4. `uninstall_clean_removal_test.rs` - Clean removal flow
5. `uninstall_safety_backup_test.rs` - Safety snapshot creation and verification
6. `uninstall_edge_cases_test.rs` - 15+ edge case scenarios
7. `uninstall_permissions_test.rs` - Permission handling (read-only files, etc.)
8. `epic3_epic6_integration_test.rs` - Combined init refactor + backup tests

**Environment:** Uses `tempfile` crate for isolated test environments

**Pattern:**
```rust
#[test]
fn test_full_uninstall_restore_original() {
    // Setup: Create temp HOME, init zprof, create profiles
    // Execute: Run uninstall with "restore original" option
    // Assert: Original configs restored, zprof files removed, safety backup exists
}
```

### Edge Case Tests

**Critical Scenarios (Story 3.7):**
1. Uninstall when no pre-zprof backup exists → Disable "Restore original" option
2. File conflict during restoration (.zshrc exists) → Prompt or backup
3. Insufficient disk space → Abort with clear error
4. Permission denied during cleanup → Report error, partial cleanup
5. Corrupted backup manifest → Abort, suggest manual inspection
6. Symlink points outside HOME → Skip or warn
7. User cancels mid-operation → Rollback or safe state
8. History file missing → Continue without history restoration
9. Framework directory is massive (> 1 GB) → Don't backup framework binaries
10. Concurrent modification during restoration → Checksum mismatch, abort
11. Active profile cannot be deleted → Validation error with guidance
12. Empty profiles directory → Handle gracefully
13. .zshenv is read-only → Attempt to change permissions or warn
14. Pre-zprof backup partially corrupted → Use available files, warn about missing
15. User aborts confirmation → No changes made, exit cleanly

### Snapshot Tests

**Location:** `tests/snapshots/`

**Purpose:** Validate TUI output and user-facing messages

**Tools:** `insta` crate for snapshot management

**Scenarios:**
- Restoration option TUI rendering
- Profile selection TUI rendering
- Confirmation summary formatting (all 3 options)
- Success messages
- Error messages with recovery instructions
- Progress output

**Update snapshots:** `cargo insta review`

### Manual Testing Checklist

**Platforms:** macOS (Ventura+), Linux (Ubuntu 22.04+)

**Scenarios:**
1. Fresh install → init → uninstall with restore original
2. Existing oh-my-zsh user → init → uninstall with restore original
3. Multiple profiles → uninstall with promote profile
4. Clean removal → verify HOME is clean
5. Re-init after uninstall → verify works correctly
6. Uninstall with `--yes --restore=original` (non-interactive)
7. Uninstall with `--keep-backups`
8. Extract safety backup manually and inspect contents

### Regression Testing

**Scope:** Ensure existing functionality unaffected

**Critical Paths:**
- `zprof init` still works (with new backup creation)
- `zprof create` unaffected
- `zprof use` unaffected
- `zprof list` unaffected
- All existing tests still pass (188 tests currently)

### Performance Testing

**Metrics to Validate:**
- Init with backup: < 2 seconds (measure on test system)
- Safety snapshot creation: < 10 seconds for 5 MB profiles
- Restoration: < 3 seconds for typical config set
- Total uninstall: < 20 seconds end-to-end

**Tools:** `std::time::Instant` for timing, log results

### Acceptance Testing

**Definition of Done for Epic:**
- [ ] All 12 epic acceptance criteria verified (manual checklist)
- [ ] All unit tests passing (25+)
- [ ] All integration tests passing (8 suites)
- [ ] All edge case tests passing (15 scenarios)
- [ ] Snapshot tests passing (6 scenarios)
- [ ] Code coverage ≥ 80%
- [ ] Manual testing checklist completed on macOS and Linux
- [ ] No regressions in existing tests (188/188 passing)
- [ ] Performance targets met
- [ ] Documentation reviewed and complete
- [ ] Code review approved
- [ ] Clippy lints clean (`cargo clippy -- -D warnings`)

**Testing Tools:**
- `cargo test` - Run all tests
- `cargo tarpaulin` - Code coverage
- `cargo clippy` - Lints
- `cargo insta` - Snapshot tests
- `serial_test` crate - Tests modifying HOME (run serially)
