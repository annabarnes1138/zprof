# Epic Technical Specification: Init Cleanup and Enhancement

Date: 2025-11-24
Author: Anna
Epic ID: 6
Status: Draft

---

## Overview

Epic 6 enhances the `zprof init` command to create comprehensive pre-zprof backups and cleanly move root shell configuration files out of the user's HOME directory. This addresses user confusion about which shell configuration is active (root configs vs profile configs) while preserving the ability to fully restore the original setup. The enhancement provides safe backup mechanisms, clear communication about changes, framework detection, and idempotent re-initialization support.

This epic builds on zprof's core principle of **non-destructive operations** by ensuring users can safely try zprof without fear of losing their existing shell configurations or framework installations.

## Objectives and Scope

**In Scope:**
- Detect all existing shell configuration files (`.zshrc`, `.zshenv`, `.zprofile`, `.zlogin`, `.zlogout`, `.zsh_history`)
- Detect existing framework installations (oh-my-zsh, zimfw, prezto, zinit, zap)
- Create comprehensive backup to `.zsh-profiles/backups/pre-zprof/` with manifest
- Move root config files to backup location (excluding `.zsh_history`)
- Create minimal `.zshenv` for zprof integration
- Verify backup integrity with checksums
- Handle re-initialization safely without destroying existing backups
- Dry-run mode to preview changes
- Integration with Epic 3's uninstall system for restoration

**Out of Scope:**
- Windows support (future)
- Cloud backup of pre-zprof config (v0.4.0)
- Automatic framework migration (manual restore only)
- Backup encryption (relies on user's filesystem security)

## System Architecture Alignment

This epic extends the **Core** (`src/core/`) and **CLI** (`src/cli/`) modules within zprof's architecture:

**New Modules:**
- `src/backup/shell_config.rs` - Shell configuration detection
- `src/backup/pre_zprof.rs` - Pre-zprof backup creation and restoration
- `src/backup/verify.rs` - Backup verification
- `src/core/backup_manifest.rs` - Backup manifest data model

**Modified Modules:**
- `src/cli/init.rs` - Enhanced init workflow with backup/cleanup
- `src/backup/mod.rs` - Module root for backup functionality

**Architectural Alignment:**
- Follows **Safe File Operations** pattern (Check → Backup → Operate → Verify)
- Integrates with existing backup system from Epic 3 (Uninstall)
- Uses TOML for backup manifests (consistent with profile manifests)
- Leverages `anyhow` for error context
- Applies path validation and security patterns from core modules
- Maintains CLI command structure pattern with thin orchestration layer

## Detailed Design

### Services and Modules

| Module | Responsibility | Inputs | Outputs | Owner |
|--------|---------------|--------|---------|-------|
| `src/backup/shell_config.rs` | Detect existing shell configuration and frameworks | HOME directory path | `ShellConfigInfo` struct | TBD |
| `src/backup/pre_zprof.rs` | Create and manage pre-zprof backups | `ShellConfigInfo`, backup path | Backup directory, manifest | TBD |
| `src/backup/verify.rs` | Verify backup integrity | Backup manifest, backup directory | `VerificationReport` | TBD |
| `src/core/backup_manifest.rs` | Backup manifest data model | File list, metadata | TOML manifest | TBD |
| `src/cli/init.rs` (enhanced) | Orchestrate init workflow with backup | Command args | User output, filesystem changes | TBD |

**Module Dependencies:**
- `shell_config.rs` → depends on `dirs`, `std::fs`
- `pre_zprof.rs` → depends on `shell_config`, `backup_manifest`, `tar`, `flate2`
- `verify.rs` → depends on `backup_manifest`, checksum libraries
- `init.rs` → depends on all backup modules + existing core modules

### Data Models and Contracts

#### ShellConfigInfo

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
    pub target: Option<PathBuf>,  // If symlink, resolved target
}

pub struct HistoryFile {
    pub path: PathBuf,
    pub size: u64,
    pub line_count: usize,
}

pub struct FrameworkInfo {
    pub name: String,           // "oh-my-zsh", "zimfw", etc.
    pub install_path: PathBuf,  // ~/.oh-my-zsh
    pub size: u64,
}
```

#### BackupManifest

```rust
// src/core/backup_manifest.rs

#[derive(Serialize, Deserialize)]
pub struct BackupManifest {
    pub metadata: BackupMetadata,
    pub detected_framework: Option<FrameworkBackup>,
    pub files: Vec<BackedUpFile>,
}

#[derive(Serialize, Deserialize)]
pub struct BackupMetadata {
    pub created_at: DateTime<Utc>,
    pub zsh_version: String,
    pub os: String,
    pub zprof_version: String,
}

#[derive(Serialize, Deserialize)]
pub struct FrameworkBackup {
    pub name: String,
    pub path: String,
    pub backed_up_as: String,  // "framework-backup.tar.gz"
    pub size: u64,
}

#[derive(Serialize, Deserialize)]
pub struct BackedUpFile {
    pub path: String,
    pub size: u64,
    pub permissions: String,
    pub checksum: String,
    pub lines: Option<usize>,  // For text files like .zsh_history
}
```

#### VerificationReport

```rust
// src/backup/verify.rs

pub struct VerificationReport {
    pub all_files_present: bool,
    pub checksums_valid: bool,
    pub issues: Vec<VerificationIssue>,
}

pub struct VerificationIssue {
    pub file_path: String,
    pub issue_type: IssueType,
    pub message: String,
}

pub enum IssueType {
    Missing,
    SizeMismatch,
    ChecksumMismatch,
    PermissionMismatch,
}
```

### APIs and Interfaces

#### Public Functions

**Shell Config Detection:**
```rust
// src/backup/shell_config.rs

pub fn detect() -> Result<ShellConfigInfo>
pub fn find_zsh_configs(home: &Path) -> Result<Vec<ConfigFile>>
pub fn find_history(home: &Path) -> Result<Option<HistoryFile>>
pub fn detect_framework(home: &Path) -> Result<Option<FrameworkInfo>>
```

**Backup Creation:**
```rust
// src/backup/pre_zprof.rs

pub fn create_backup(config_info: &ShellConfigInfo) -> Result<PathBuf>
pub fn move_configs_to_backup(config_info: &ShellConfigInfo, backup_path: &Path) -> Result<()>
pub fn create_zprof_zshenv(home: &Path) -> Result<()>
pub fn backup_exists() -> Result<bool>
```

**Backup Verification:**
```rust
// src/backup/verify.rs

pub fn verify_backup(manifest: &BackupManifest, backup_path: &Path) -> Result<VerificationReport>
pub fn calculate_checksum(file_path: &Path) -> Result<String>
```

**Init Command Interface:**
```rust
// src/cli/init.rs

#[derive(Debug, Args)]
pub struct InitArgs {
    #[arg(long)]
    pub force: bool,        // Re-create backup even if exists

    #[arg(long)]
    pub dry_run: bool,      // Preview changes without applying

    #[arg(long)]
    pub skip_verification: bool,  // Skip backup verification (faster)
}

pub fn execute(args: InitArgs) -> Result<()>
```

### Workflows and Sequencing

#### Init Workflow (Normal Mode)

```
User: zprof init
    ↓
┌─────────────────────────────────────────┐
│ 1. Check if already initialized         │
│    - Look for ~/.zsh-profiles/          │
│    - Look for pre-zprof backup          │
└──────────────┬──────────────────────────┘
               ↓
    ┌──────────┴──────────┐
    │ Already initialized? │
    └──────────┬──────────┘
         Yes ↓      ↓ No
    ┌────────┴──────────────────────┐
    │ Show re-init dialog            │
    │ - Display backup date          │
    │ - Ask: Continue/View/Cancel    │
    └────────┬──────────────────────┘
             ↓ Continue
┌─────────────────────────────────────────┐
│ 2. Detect shell configuration           │
│    - Find all .zsh* files               │
│    - Detect framework (oh-my-zsh, etc)  │
│    - Count history lines                │
│    - Calculate total size               │
└──────────────┬──────────────────────────┘
               ↓
┌─────────────────────────────────────────┐
│ 3. Show detection summary               │
│    ✓ Found 5 config files               │
│    ✓ Detected oh-my-zsh framework       │
│    ✓ 1,523 history entries              │
│    ✓ Total size: ~47 MB                 │
└──────────────┬──────────────────────────┘
               ↓
┌─────────────────────────────────────────┐
│ 4. Create backup                        │
│    - Create backup directory            │
│    - Copy all config files              │
│    - Create framework tarball           │
│    - Generate manifest.toml             │
└──────────────┬──────────────────────────┘
               ↓
┌─────────────────────────────────────────┐
│ 5. Verify backup                        │
│    - Check all files present            │
│    - Verify checksums                   │
│    - Validate manifest                  │
└──────────────┬──────────────────────────┘
          ↓ Success    ↓ Failure
    ┌──────────────────────────┐
    │ Show error, abort init   │
    └──────────────────────────┘
               ↓
┌─────────────────────────────────────────┐
│ 6. Move root configs to backup          │
│    - Move .zshrc → backup/              │
│    - Move .zshenv → backup/             │
│    - Move framework dir → backup/       │
│    - Keep .zsh_history in place         │
└──────────────┬──────────────────────────┘
               ↓
┌─────────────────────────────────────────┐
│ 7. Create new .zshenv                   │
│    - Write zprof integration            │
│    - Set ZPROF_ROOT                     │
│    - Set ZDOTDIR loader                 │
└──────────────┬──────────────────────────┘
               ↓
┌─────────────────────────────────────────┐
│ 8. Show completion summary              │
│    ✅ Backup created (2.3 MB)           │
│    ✅ Root configs moved                │
│    ✅ zprof integration ready           │
│                                         │
│    Next: zprof create <profile-name>    │
└─────────────────────────────────────────┘
```

#### Dry-Run Workflow

```
User: zprof init --dry-run
    ↓
┌─────────────────────────────────────────┐
│ 1. Detect shell configuration           │
│    (same as normal mode)                │
└──────────────┬──────────────────────────┘
               ↓
┌─────────────────────────────────────────┐
│ 2. Show preview (NO CHANGES MADE)       │
│                                         │
│    Backup:                              │
│    ✓ Would back up 5 config files       │
│      • .zshrc (1.2 KB)                  │
│      • .zshenv (234 bytes)              │
│      • .zsh_history (50.1 KB)           │
│    ✓ Would back up oh-my-zsh (45 MB)    │
│    ✓ Total: ~47 MB                      │
│                                         │
│    Cleanup:                             │
│    ✓ Would move configs to backup       │
│    ✓ Would create new .zshenv           │
│                                         │
│    Run without --dry-run to proceed     │
└─────────────────────────────────────────┘
```

## Non-Functional Requirements

### Performance

**Target Metrics:**
- Initial detection: < 200ms for typical HOME directory (< 100 files)
- Backup creation: < 5 seconds for standard config (< 50 MB)
- Framework tarball compression: Acceptable delay for large frameworks (oh-my-zsh ~45 MB → ~3-5 seconds)
- Checksum verification: < 1 second for typical config files
- Re-initialization check: < 50ms (simple directory existence check)

**Optimization Strategies:**
- Lazy framework detection (only parse .zshrc if framework directory exists)
- Parallel checksumming for multiple files (rayon crate if needed)
- Skip large binary files in checksum (focus on text configs)
- Stream file copies instead of loading into memory
- Cache directory listings during detection phase

**Performance Requirements from Epic 6:**
- Init command should complete within 10 seconds on typical systems
- Dry-run mode should be < 1 second (detection only, no I/O)
- No performance regression on existing init functionality

### Security

**Path Traversal Prevention:**
- Validate all paths to prevent escaping HOME directory
- Resolve symlinks before operations to prevent following malicious links outside HOME
- Use canonicalized paths for all file operations
- Apply existing `validate_profile_name()` pattern to backup paths

**Backup Integrity:**
- SHA-256 checksums for all backed-up files (stored in manifest)
- Verify checksums after backup creation
- Detect file tampering during restoration (Epic 3)
- Store checksums in manifest.toml for audit trail

**Permission Preservation:**
- Capture and restore original file permissions (mode bits)
- Preserve ownership where possible (same user)
- Handle read-only files correctly (don't fail backup)
- Warn if permission restoration fails during uninstall

**Sensitive Data Handling:**
- `.zsh_history` may contain sensitive commands (API keys, passwords)
- Framework configs may contain tokens/credentials
- Backup directory permissions: 0700 (user-only access)
- Warn users about sensitive data in backups (documentation)
- No cloud upload or external transmission (all local)

**Framework Detection Safety:**
- Read-only detection (never modify framework files)
- Handle malformed .zshrc gracefully (parse errors don't fail init)
- Limit .zshrc parsing depth (prevent infinite loops)
- Timeout on large file parsing (> 1 MB config files)

### Reliability/Availability

**Atomic Operations:**
- Backup creation is atomic: Complete success or rollback on failure
- Use temporary directory during backup, rename when complete
- If backup fails mid-process, clean up partial backup
- Don't move configs until backup is verified

**Failure Recovery:**
- If backup verification fails, abort and leave original configs untouched
- If config move fails, restore from just-created backup
- If .zshenv creation fails, restore original .zshenv
- Provide rollback function: `zprof init --rollback` (restore pre-init state)

**Idempotency:**
- Running `zprof init` multiple times is safe
- Re-init preserves existing backup (unless `--force`)
- Detect partial init state and recover automatically
- Clear error messages for inconsistent states

**Data Integrity:**
- Verify file sizes match after copy
- Verify checksums match after backup
- Detect corruption during verification
- Fail loudly if integrity check fails (better safe than sorry)

**Degradation Behavior:**
- If framework detection fails → Continue without framework backup (warn user)
- If history file unreadable → Continue without history backup (warn user)
- If verification fails → Abort init, provide manual recovery steps
- If disk space insufficient → Abort before any modifications

### Observability

**Logging:**
- Log all backup operations to `~/.zsh-profiles/logs/init.log`
- Include timestamps, file paths, sizes, checksums
- Log detection phase: found files, frameworks, sizes
- Log verification results: pass/fail for each file
- Rotate logs (keep last 10 init runs)

**User Feedback:**
- Progress indicators for long operations:
  - "Detecting shell configuration..."
  - "Creating backup... (2.3 MB)"
  - "Verifying backup integrity..."
  - "Moving root configs..."
- Success/failure messages for each phase
- Clear error messages with remediation steps
- Summary at end with backup location and size

**Metrics Collected:**
- Number of config files backed up
- Framework type and size (if detected)
- Backup duration (total and per-phase)
- Verification duration
- History line count
- Total backup size

**Error Reporting:**
- Structured error messages with context:
  ```
  Error: Failed to create backup

  Caused by:
      0: Failed to copy .zshrc
      1: Permission denied: /Users/anna/.zshrc

  Suggestion: Check file permissions and try again
  ```
- Include system info in error context (OS, zsh version, zprof version)
- Suggest next steps (check permissions, disk space, file integrity)

**Debug Mode:**
- `ZPROF_DEBUG=1 zprof init` → Verbose logging to stderr
- Show detected files, checksums, all operations
- Useful for troubleshooting init failures

## Dependencies and Integrations

### Rust Dependencies (Cargo.toml)

**Existing Dependencies:**
- `std::fs` - File system operations (copy, move, metadata)
- `std::path` - Path manipulation and validation
- `dirs = "5.0"` - Cross-platform HOME directory discovery
- `anyhow = "1.0"` - Error handling with context
- `serde = { version = "1.0", features = ["derive"] }` - Serialization for manifest
- `toml = "0.9"` - TOML parsing/generation for manifests
- `tar = "0.4"` - Framework tarball creation
- `flate2 = "1.0"` - Gzip compression for tarballs
- `chrono = { version = "0.4", features = ["serde"] }` - Timestamps in manifest

**New Dependencies (to be added):**
- `sha2 = "0.10"` - SHA-256 checksums for verification
- `walkdir = "2.4"` - Recursive directory traversal for framework detection
- (Optional) `rayon = "1.8"` - Parallel checksum computation if performance testing shows benefit

### Integration with Existing Modules

**Epic 3 (Uninstall System) Integration:**
- Shares backup manifest format (`BackupManifest` struct)
- Uninstall command reads `pre-zprof/backup-manifest.toml` to restore
- Restoration process reverses init cleanup:
  1. Move backed-up configs from `backups/pre-zprof/` → HOME
  2. Extract framework tarball if present
  3. Remove zprof-created `.zshenv`
  4. Validate restoration with checksums

**Core Filesystem Module:**
- Use existing `src/core/filesystem.rs::copy_with_backup()` pattern
- Extend with `move_with_verification()` for config cleanup
- Use `validate_path()` for security checks

**Framework Detection Module:**
- Leverage existing `src/frameworks/detector.rs`
- Reuse `FrameworkType` enum and detection logic
- Store detected framework in backup manifest for reference

**CLI Init Module:**
- Modify existing `src/cli/init.rs::execute()`
- Insert backup workflow before profile creation
- Maintain backward compatibility (existing init functionality unchanged)

### External System Dependencies

**Shell Commands:**
- `zsh --version` - Capture zsh version for manifest
- `uname -a` - Capture OS info for manifest
- No other external commands (pure Rust implementation)

**Filesystem Assumptions:**
- HOME directory is writable
- At least 100 MB free space for typical backup (framework + configs)
- Support for POSIX file permissions (chmod 0700)
- Symlink support (resolve and follow)

### Data Flow with Epic 3

```
Init (Epic 6)                          Uninstall (Epic 3)
     ↓                                       ↓
Create backup/                         Read manifest
  ├─ backup-manifest.toml       →      Parse file list
  ├─ .zshrc                     →      Restore to HOME
  ├─ .zshenv                    →      Restore to HOME
  └─ frameworks/                       Extract tarball
      └─ .oh-my-zsh.tar.gz      →      Extract to HOME
     ↓                                       ↓
Move configs to backup                 Remove zprof files
     ↓                                       ↓
Create zprof .zshenv                   Verify restoration
     ↓                                       ↓
Done (configs in backup)               Done (original state restored)
```

## Acceptance Criteria (Authoritative)

### AC-1: Shell Configuration Detection
**Given** a user runs `zprof init`
**When** the detection phase executes
**Then** all shell config files are detected (`.zshrc`, `.zshenv`, `.zprofile`, `.zlogin`, `.zlogout`, `.zsh_history`)
**And** existing frameworks are detected (oh-my-zsh, zimfw, prezto, zinit, zap)
**And** symlinks are resolved to their targets
**And** file sizes and permissions are captured
**And** history line count is calculated

### AC-2: Pre-zprof Backup Creation
**Given** shell configuration has been detected
**When** backup creation executes
**Then** a backup directory is created at `~/.zsh-profiles/backups/pre-zprof/`
**And** all detected config files are copied with original permissions
**And** framework directory is compressed to `framework-backup.tar.gz` (if present)
**And** a `backup-manifest.toml` is generated with:
- Metadata (timestamp, OS, zsh version, zprof version)
- List of backed-up files with sizes, permissions, checksums
- Framework info (if detected)
**And** backup directory permissions are set to 0700 (user-only)

### AC-3: Backup Verification
**Given** backup has been created
**When** verification executes
**Then** all files in manifest exist in backup directory
**And** file sizes match manifest
**And** SHA-256 checksums match manifest
**And** verification report shows success or detailed issues
**And** init aborts if verification fails (backup integrity critical)

### AC-4: Root Config Cleanup
**Given** backup has been verified successfully
**When** cleanup phase executes
**Then** all config files are moved from HOME to `backups/pre-zprof/`
**And** framework directories are moved to `backups/pre-zprof/frameworks/`
**And** `.zsh_history` remains in HOME (shared history)
**And** a new `.zshenv` is created with zprof integration code
**And** HOME directory contains only `.zshenv` and `.zsh_history` (zprof-managed)

### AC-5: Re-initialization Safety
**Given** zprof is already initialized
**When** user runs `zprof init` again
**Then** existing backup is detected and preserved
**And** user is shown backup date and size
**And** user can choose to continue (safe re-init) or cancel
**And** if continuing, no backup overwrite occurs (unless `--force` flag)
**And** directory structure is validated and recreated if needed

### AC-6: Dry-Run Mode
**Given** user runs `zprof init --dry-run`
**When** dry-run executes
**Then** detection phase runs normally
**And** a preview of changes is displayed (files, sizes, actions)
**And** NO files are created, moved, or modified
**And** user is instructed to run without `--dry-run` to proceed
**And** dry-run completes in < 1 second

### AC-7: Force Re-backup
**Given** user runs `zprof init --force`
**When** force re-backup executes
**Then** user is warned that existing backup will be replaced
**And** user must confirm (y/N)
**And** if confirmed, old backup is archived/renamed before creating new one
**And** new backup is created with current timestamp
**And** old backup is kept as `pre-zprof.backup-YYYY-MM-DD-HHMMSS`

### AC-8: Error Handling and Rollback
**Given** any phase of init fails
**When** an error occurs
**Then** partial changes are rolled back automatically
**And** original configs remain untouched (if backup not verified)
**And** clear error message is shown with cause and suggestion
**And** logs are written to `~/.zsh-profiles/logs/init.log`
**And** user can retry after fixing issue

### AC-9: Integration with Uninstall
**Given** init has completed successfully
**When** user runs `zprof uninstall` (Epic 3)
**Then** pre-zprof backup is detected
**And** all backed-up configs are restored to HOME
**And** framework tarball is extracted (if present)
**And** zprof `.zshenv` is removed
**And** original shell configuration is fully restored
**And** checksums are verified during restoration

### AC-10: Documentation Updates
**Given** Epic 6 implementation is complete
**When** documentation is reviewed
**Then** all user-facing docs reflect new init behavior
**And** backup process is explained clearly
**And** restoration process (uninstall) is documented
**And** FAQ covers common init questions
**And** CLI help text (`zprof init --help`) is accurate

## Traceability Mapping

| AC ID | Epic 6 Story | Spec Section | Components/APIs | Test Idea |
|-------|--------------|--------------|----------------|-----------|
| AC-1 | Story 6.1 | Data Models: `ShellConfigInfo` | `shell_config.rs::detect()` | Unit: Mock filesystem with various configs<br>Integration: Real HOME with oh-my-zsh |
| AC-2 | Story 6.2 | APIs: `create_backup()` | `pre_zprof.rs::create_backup()` | Unit: Verify manifest structure<br>Integration: Full backup with framework |
| AC-3 | Story 6.5 | APIs: `verify_backup()` | `verify.rs::verify_backup()` | Unit: Checksum mismatch detection<br>Integration: Corrupt file scenario |
| AC-4 | Story 6.3 | Workflows: Init Normal Mode | `pre_zprof.rs::move_configs_to_backup()`<br>`pre_zprof.rs::create_zprof_zshenv()` | Integration: Verify HOME state after cleanup<br>Manual: Check .zshenv content |
| AC-5 | Story 6.4 | Workflows: Re-init | `init.rs::execute()` re-init branch | Integration: Run init twice, verify backup preserved |
| AC-6 | Story 6.6 | Workflows: Dry-Run | `init.rs::execute()` with `dry_run=true` | Integration: Verify no filesystem changes<br>Snapshot: Output format |
| AC-7 | Story 6.4 | APIs: `create_backup()` with force | `init.rs::execute()` with `force=true` | Integration: Old backup archived correctly |
| AC-8 | All Stories | NFR: Reliability | Error handling in all modules | Unit: Simulate failures at each phase<br>Integration: Partial backup rollback |
| AC-9 | Story 6.2 | Integration: Epic 3 | `uninstall.rs` (Epic 3) reads manifest | Integration: Init → Uninstall → Verify restoration |
| AC-10 | Story 6.7 | Documentation | N/A | Manual: Review all docs, run CLI help |

## Risks, Assumptions, Open Questions

### Risks

**RISK-1: User loses shell configuration during backup failure**
- **Severity:** Critical
- **Likelihood:** Low
- **Mitigation:**
  - Atomic backup operations (all or nothing)
  - Verify backup before moving any files
  - Rollback mechanism on failure
  - Comprehensive error handling and logging
  - Dry-run mode for preview
- **Contingency:** Manual recovery instructions in docs

**RISK-2: Backup verification false positive (corrupted backup passes)**
- **Severity:** High
- **Likelihood:** Very Low
- **Mitigation:**
  - Use industry-standard SHA-256 checksums
  - Verify both size and checksum
  - Test verification logic extensively
  - Manual spot-check during testing
- **Contingency:** User can manually verify backup contents before continuing

**RISK-3: Framework tarball fails mid-compression**
- **Severity:** Medium
- **Likelihood:** Low (framework directories can be large)
- **Mitigation:**
  - Use temporary file, rename on success
  - Check disk space before compression
  - Progress indicator for large frameworks
  - Timeout for unreasonably large directories (> 500 MB)
- **Contingency:** Warn user, continue without framework backup

**RISK-4: Concurrent shell sessions break after config cleanup**
- **Severity:** Medium
- **Likelihood:** Medium (users often have multiple terminals)
- **Mitigation:**
  - Clear warning in output: "Restart all shell sessions"
  - Document this behavior in FAQ
  - Consider detecting active shells (ps aux | grep zsh)
- **Contingency:** User restarts shells manually

**RISK-5: Symlink handling creates dangling references**
- **Severity:** Medium
- **Likelihood:** Low
- **Mitigation:**
  - Resolve symlinks and backup targets
  - Detect and handle circular symlinks
  - Test with various symlink scenarios
  - Document symlink behavior
- **Contingency:** User manually fixes symlinks if restoration fails

**RISK-6: Epic 3 uninstall code doesn't exist yet**
- **Severity:** High
- **Likelihood:** High (dependency on Epic 3)
- **Mitigation:**
  - Design backup format to be Epic 3-compatible
  - Document restoration process for manual recovery
  - Implement basic uninstall stub for testing
  - Coordinate with Epic 3 implementation
- **Contingency:** Manual restoration instructions until Epic 3 ships

### Assumptions

**ASSUMPTION-1: HOME directory structure**
- Users have standard HOME directory with write permissions
- Config files are in predictable locations (`.zshrc` in HOME root)
- Framework installations are in standard locations (`.oh-my-zsh/`, etc.)

**ASSUMPTION-2: Disk space availability**
- Users have at least 100 MB free space for typical backups
- Large framework installations (oh-my-zsh ~45 MB) can be compressed
- Users are on modern filesystems (not FAT32 with 4GB limits)

**ASSUMPTION-3: POSIX compliance**
- Target systems support POSIX file permissions (chmod, mode bits)
- Symlinks are supported and can be resolved
- File metadata (size, mtime) is reliable

**ASSUMPTION-4: zsh as primary shell**
- Users are using zsh (zprof's target shell)
- zsh is installed and `zsh --version` works
- Configuration follows standard zsh conventions

**ASSUMPTION-5: Epic 3 compatibility**
- Epic 3 uninstall will use the manifest format defined here
- Epic 3 will implement tarball extraction for frameworks
- Epic 3 will validate checksums during restoration

**ASSUMPTION-6: Single-user systems**
- Backups are user-specific (not system-wide)
- Each user manages their own zprof installation
- No multi-user coordination needed

### Open Questions

**QUESTION-1: Should we backup .zsh_history or just reference it?**
- **Context:** History file can be large (50+ MB) and changes constantly
- **Options:**
  - Copy to backup (snapshot in time, slower)
  - Keep reference only (faster, but can't restore exact history)
  - Make it configurable with flag
- **Decision Needed By:** Story 6.2 implementation
- **Current Approach:** Copy to backup for safety, but keep original in place for shared history

**QUESTION-2: What's the maximum framework size we support?**
- **Context:** Some frameworks with plugins can be > 500 MB
- **Options:**
  - Hard limit (e.g., 500 MB, fail if larger)
  - Warning threshold (e.g., 200 MB, warn but proceed)
  - No limit (risk of filling disk/long compression time)
- **Decision Needed By:** Story 6.2 implementation
- **Current Approach:** Warn at 200 MB, timeout at 10 minutes compression

**QUESTION-3: How to handle modified .zshrc during backup?**
- **Context:** User might edit .zshrc while backup is in progress
- **Options:**
  - Lock file during backup (prevent edits)
  - Detect modification via mtime (abort if changed)
  - Accept risk (unlikely in practice)
- **Decision Needed By:** Story 6.2 implementation
- **Current Approach:** Quick operation (<5s) makes this unlikely; verify checksum catches it

**QUESTION-4: Should dry-run show checksums?**
- **Context:** Computing checksums takes time, defeats "fast preview" goal
- **Options:**
  - Show checksums (complete preview, slower)
  - Skip checksums (fast, but incomplete preview)
  - Make it optional with `--dry-run --verbose`
- **Decision Needed By:** Story 6.6 implementation
- **Current Approach:** Skip checksums in dry-run for speed

**QUESTION-5: Epic 3 delivery timeline?**
- **Context:** Epic 6 creates backups, Epic 3 restores them
- **Impact:** Can't fully test restoration without Epic 3
- **Options:**
  - Wait for Epic 3 before shipping Epic 6
  - Ship Epic 6 with manual restoration docs
  - Implement minimal uninstall stub in Epic 6
- **Decision Needed By:** Sprint planning
- **Current Approach:** TBD based on roadmap

## Test Strategy Summary

### Unit Tests

**Coverage Areas:**
- Shell config detection logic (`shell_config.rs`)
  - Various file combinations (.zshrc only, .zshrc + .zshenv, etc.)
  - Symlink resolution
  - Framework detection parsing
  - Edge cases: empty files, read-only files, missing permissions
- Backup manifest serialization (`backup_manifest.rs`)
  - TOML round-trip (serialize → deserialize)
  - All fields present and correct
- Checksum calculation (`verify.rs`)
  - Known file → expected SHA-256
  - Large files (streaming)
  - Binary vs text files
- Path validation (security)
  - Prevent directory traversal (../)
  - Absolute vs relative paths
  - Symlink escapes

**Test Infrastructure:**
- Mock filesystem with `tempfile` crate
- Predefined test fixtures (.zshrc, .zshenv examples)
- Snapshot tests with `insta` for manifest format

### Integration Tests

**Coverage Areas:**
- Full init workflow (first-time)
  - Create HOME with sample configs
  - Run init
  - Verify backup created correctly
  - Verify configs moved
  - Verify .zshenv created
- Re-initialization workflow
  - Init twice
  - Verify first backup preserved
  - Verify safe re-init behavior
- Dry-run workflow
  - Run with --dry-run
  - Verify NO filesystem changes
  - Verify output accuracy
- Framework backup
  - Create mock oh-my-zsh directory
  - Verify tarball creation
  - Verify tarball contents
- Verification failure scenarios
  - Corrupt file mid-backup
  - Missing file
  - Checksum mismatch
  - Verify rollback behavior

**Test Infrastructure:**
- `serial_test` for tests that modify HOME
- Temporary HOME directories per test
- Cleanup after each test (remove temp dirs)

### Manual Testing Checklist

**Pre-release Validation:**
- [ ] Test on fresh system (no existing .zshrc)
- [ ] Test with oh-my-zsh installed
- [ ] Test with zimfw installed
- [ ] Test with multiple frameworks (edge case)
- [ ] Test with symlinked .zshrc
- [ ] Test with large history file (> 50 MB)
- [ ] Test with read-only .zshrc (permission error)
- [ ] Test init → uninstall → verify restoration (requires Epic 3)
- [ ] Test --dry-run output accuracy
- [ ] Test --force re-backup
- [ ] Test disk space exhaustion (graceful failure)
- [ ] Test on macOS (primary target)
- [ ] Test on Linux (Ubuntu, Fedora)
- [ ] Verify CLI help text (`zprof init --help`)
- [ ] Review all documentation updates

### Performance Testing

**Benchmarks:**
- Detection phase: < 200ms for typical HOME
- Backup creation: < 5s for standard config (no framework)
- Backup with oh-my-zsh: < 10s (includes compression)
- Verification: < 1s for typical config
- Dry-run: < 1s

**Load Testing:**
- Large history file: 100 MB, 10K lines
- Large framework: 500 MB (artificial test case)
- Many config files: 50+ .zsh* files (edge case)

### Edge Case Testing

**Unusual Scenarios:**
- No .zshrc exists (fresh user)
- Empty .zshrc (0 bytes)
- .zshrc is a directory (invalid)
- .zshrc is a binary file (corrupted)
- Circular symlinks (.zshrc → .zshenv → .zshrc)
- Framework directory is a symlink
- HOME on read-only filesystem (should fail gracefully)
- Concurrent init processes (race condition)
- System crash during backup (incomplete backup detection)

### Regression Testing

**Ensure No Breakage:**
- Existing init functionality (without Epic 6 features) still works
- Profile creation after init works normally
- GUI init integration works (if applicable)
- All existing tests still pass

### Test Automation

**CI/CD Pipeline:**
- Run all unit tests on every commit
- Run integration tests on PR
- Run manual checklist subset on release candidate
- Snapshot test updates reviewed in PR
- Code coverage target: 80%+ for backup modules
