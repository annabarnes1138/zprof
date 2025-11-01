# Story 1.1b: Framework Detection and Import During Init

Status: ready-for-dev

## Dev Agent Record

### Context Reference
- [Story Context XML](1-1b-framework-detection-and-import-during-init.context.xml) - Generated 2025-11-01

## Story

As a developer with an existing zsh framework,
I want zprof to detect and import my current setup during initialization,
So that I can immediately start using profile switching without manual migration.

## Acceptance Criteria

1. During `zprof init`, system detects existing zsh framework (oh-my-zsh, prezto, zimfw, zinit, zap) installations
2. If framework detected, prompts user: "Existing [framework] detected with [N] plugins and '[theme]' theme. Import as a profile? (y/n)"
3. On "y", prompts for profile name with default: "default"
4. System imports detected framework configuration into new profile directory
5. Backs up existing `~/.zshenv` to `~/.zsh-profiles/cache/backups/.zshenv.backup.TIMESTAMP` (if file exists)
6. Creates or updates `~/.zshenv` to set `ZDOTDIR` pointing to imported profile directory
7. User's `~/.zshrc` remains completely untouched (framework init becomes unreachable due to ZDOTDIR precedence)
8. TOML manifest is generated from imported framework configuration
9. Imported profile is set as active profile in global `config.toml`
10. Success message displays import details: framework type, plugin count, theme, and profile name
11. If user chooses "n" (skip import), init completes without import and user can create profiles manually later

## Tasks / Subtasks

- [ ] Integrate framework detection during init (AC: #1, #2)
  - [ ] Call `frameworks::detector::detect_existing_framework()` from Story 1.4
  - [ ] Display detected framework details to user (type, plugins, theme)
  - [ ] Handle case when no framework detected (continue with basic init)

- [ ] Implement interactive import prompt (AC: #2, #3, #11)
  - [ ] Use dialoguer::Confirm for "Import as a profile? (y/n)" prompt
  - [ ] Use dialoguer::Input for profile name with default "default"
  - [ ] Handle 'y', 'n', and invalid inputs with clear feedback
  - [ ] On 'n', display message that user can create profiles later with `zprof create`

- [ ] Import framework configuration to profile (AC: #4, #8)
  - [ ] Create profile directory at `~/.zsh-profiles/profiles/<name>/`
  - [ ] Use `core/filesystem.rs` safe file operations following Pattern 3
  - [ ] Copy framework installation directory (e.g., ~/.oh-my-zsh → profile/.oh-my-zsh)
  - [ ] Copy .zshrc to profile/.zshrc
  - [ ] Copy .zshenv (if exists and doesn't conflict with zprof's ZDOTDIR export) to profile/.zshenv
  - [ ] Copy any framework-specific config files (.zimrc, .zpreztorc, etc.)
  - [ ] Generate profile.toml manifest using `core/manifest.rs` from Story 1.5

- [ ] Manage ~/.zshenv for profile switching (AC: #5, #6, #7)
  - [ ] Implement backup of existing `~/.zshenv` with timestamp
  - [ ] Create/update `~/.zshenv` to export ZDOTDIR following Pattern 5
  - [ ] Add comment in `~/.zshenv`: "# Managed by zprof - DO NOT EDIT MANUALLY"
  - [ ] Include backup path reference in comment
  - [ ] Verify user's `~/.zshrc` is NOT modified (NFR002 compliance)

- [ ] Update global config with imported profile (AC: #9)
  - [ ] Load `~/.zsh-profiles/config.toml` using `core/config.rs`
  - [ ] Set `active_profile` field to imported profile name
  - [ ] Add profile to tracked profiles list
  - [ ] Save updated config.toml

- [ ] Display success message (AC: #10)
  - [ ] Show confirmation: "✓ Imported [framework] as profile '[name]' (now active)"
  - [ ] Display framework details: type, plugin count, theme
  - [ ] Show profile location path
  - [ ] Show `.zshenv` backup location
  - [ ] Inform user: "Open a new terminal to use this profile"
  - [ ] Follow error message format from architecture.md consistency rules

- [ ] Handle edge cases and errors (AC: All)
  - [ ] Handle permission errors during `.zshenv` backup/modification with context using Pattern 2
  - [ ] Handle case where `.zshenv` already has ZDOTDIR set (warn user, ask to overwrite)
  - [ ] Gracefully handle partial framework detection (some config missing)
  - [ ] Handle profile name conflicts (unlikely during init, but validate)
  - [ ] Log operations with env_logger for debugging

- [ ] Write comprehensive tests (AC: All)
  - [ ] Integration test: init with oh-my-zsh detected → import → verify profile created
  - [ ] Integration test: init with framework detected, user chooses 'n' → no import
  - [ ] Integration test: verify `.zshenv` created/updated with ZDOTDIR
  - [ ] Integration test: verify `.zshenv` backup created with timestamp
  - [ ] Integration test: verify user's `.zshrc` untouched (NFR002 critical)
  - [ ] Integration test: verify imported profile set as active in config.toml
  - [ ] Unit test: ZDOTDIR path generation is correct
  - [ ] Unit test: backup filename generation with timestamp
  - [ ] Snapshot test: CLI output for successful import
  - [ ] Snapshot test: CLI output when user declines import

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `cli/init.rs` (enhancement), `shell/zdotdir.rs` (new), `frameworks/detector.rs` (from Story 1.4)
- Secondary: `core/manifest.rs` (from Story 1.5), `core/filesystem.rs`, `core/config.rs`
- Follow Pattern 1 (CLI Command Structure) for command enhancement
- Follow Pattern 2 (Error Handling) with anyhow::Result and context
- Follow Pattern 3 (Safe File Operations) for all file operations (NFR002 compliance)
- **Follow Pattern 5 (Shell Integration via .zshenv)** for managing user's shell configuration

### Pattern 5: Shell Integration via .zshenv

**Key architectural decision:** zprof manages `~/.zshenv` to control profile loading, NOT `~/.zshrc`

**Why .zshenv?**
- zsh sources `~/.zshenv` before `~/.zshrc` in startup order
- Setting `ZDOTDIR` in `.zshenv` causes zsh to source `$ZDOTDIR/.zshrc` instead of `~/.zshrc`
- User's original `~/.zshrc` remains pristine and untouched (strong NFR002 compliance)
- Framework initialization in `~/.zshrc` becomes unreachable but harmless

**Implementation pattern:**
```rust
// shell/zdotdir.rs
pub fn set_active_profile(profile_path: &Path) -> Result<()> {
    // 1. Backup existing ~/.zshenv if exists
    let zshenv_path = home_dir()?.join(".zshenv");
    if zshenv_path.exists() {
        backup_zshenv(&zshenv_path)?;
    }

    // 2. Create/update ~/.zshenv with ZDOTDIR export
    let zdotdir_line = format!("export ZDOTDIR={}", profile_path.display());
    let content = format!(
        "# Managed by zprof - DO NOT EDIT MANUALLY\n\
         # Original .zshenv backed up to: {}\n\
         {}\n",
        backup_path.display(),
        zdotdir_line
    );

    fs::write(&zshenv_path, content)
        .context("Failed to write .zshenv")?;

    // 3. Verify original .zshrc untouched (NFR002)
    // (no operation on .zshrc needed)

    Ok(())
}
```

**zsh startup order with ZDOTDIR:**
```
1. /etc/zshenv
2. ~/.zshenv           ← zprof sets ZDOTDIR here
3. $ZDOTDIR/.zprofile
4. $ZDOTDIR/.zshrc     ← profile's .zshrc loads
5. $ZDOTDIR/.zlogin

(~/.zshrc is NEVER sourced when ZDOTDIR is set)
```

### Integration with Story 1.4

Story 1.4 (Framework Detection) provides the detection mechanism. Expected interface:

```rust
// Available from frameworks/detector.rs (Story 1.4)
pub fn detect_existing_framework() -> Option<FrameworkInfo>;

pub struct FrameworkInfo {
    pub framework_type: FrameworkType,
    pub plugins: Vec<String>,
    pub theme: String,
    pub config_path: PathBuf,
    pub install_path: PathBuf,
}
```

### Integration with Story 1.5

Story 1.5 provides the manifest generation and framework file copying logic. This story reuses:
- `core/manifest.rs` - TOML manifest generation from FrameworkInfo
- Framework file copying patterns from Story 1.5's implementation

### Relationship to Story 1.1a

**Story 1.1a (DONE):** Creates directory structure, shared history, config.toml
**Story 1.1b (THIS STORY):** Detects framework, imports configuration, enables profile switching

These are sequential enhancements to the `zprof init` command:
1. Story 1.1a runs first (directory setup)
2. If Story 1.1b detects framework, it prompts for import
3. If import accepted, Story 1.1b creates first profile and enables switching
4. If import declined, user can manually create profiles later

**Implementation approach:**
- Enhance existing `cli/init.rs` from Story 1.1a
- Add framework detection check after directory creation succeeds
- Add conditional import flow if framework detected

### User Flow Examples

**Scenario 1: User with oh-my-zsh**
```bash
$ zprof init
✓ Created directory structure at ~/.zsh-profiles/
✓ Initialized shared history
✓ Created configuration file

Existing oh-my-zsh detected with 5 plugins and 'robbyrussell' theme.
Import as a profile? (y/n): y
Profile name [default]: work

Importing framework configuration...
✓ Copied oh-my-zsh installation
✓ Backed up ~/.zshenv to ~/.zsh-profiles/cache/backups/.zshenv.backup.20251101-143022
✓ Updated ~/.zshenv to enable profile switching
✓ Generated profile manifest

✓ Imported oh-my-zsh as profile 'work' (now active)
  Framework: oh-my-zsh
  Plugins: 5 (git, docker, kubectl, node, rust)
  Theme: robbyrussell
  Location: ~/.zsh-profiles/profiles/work

Open a new terminal to use this profile.
Your original ~/.zshrc remains untouched as a backup.
```

**Scenario 2: User declines import**
```bash
$ zprof init
✓ Created directory structure at ~/.zsh-profiles/
✓ Initialized shared history
✓ Created configuration file

Existing oh-my-zsh detected with 5 plugins and 'robbyrussell' theme.
Import as a profile? (y/n): n

Skipping import. You can create profiles later with:
  zprof create <name>  - Import current setup
  zprof wizard        - Interactive profile creation
```

**Scenario 3: No framework detected**
```bash
$ zprof init
✓ Created directory structure at ~/.zsh-profiles/
✓ Initialized shared history
✓ Created configuration file

No existing framework detected.
Create your first profile with:
  zprof wizard  - Interactive profile creation
```

### Dependencies to Add

```toml
[dependencies]
dialoguer = "0.11"  # Interactive prompts (already added in Story 1.5)
chrono = "0.4"      # Timestamps for backups (already added in Story 1.5)
```

### Error Handling Examples

```rust
// .zshenv already has ZDOTDIR set
if zshenv_content.contains("ZDOTDIR=") {
    let should_overwrite = dialoguer::Confirm::new()
        .with_prompt("~/.zshenv already sets ZDOTDIR. Overwrite for zprof?")
        .default(false)
        .interact()?;

    if !should_overwrite {
        bail!("Cannot enable profile switching - ~/.zshenv already manages ZDOTDIR");
    }
}

// Permission error during .zshenv modification
set_active_profile(&profile_path)
    .context("Failed to update ~/.zshenv. Check file permissions.")?;

// Framework detection partial/incomplete
if framework_info.plugins.is_empty() {
    warn!("No plugins detected - importing framework structure only");
}
```

### Testing Strategy

**Critical NFR002 Tests:**
```rust
#[test]
fn test_init_import_preserves_zshrc() {
    // Setup: Create fake ~/.zshrc with oh-my-zsh init
    let zshrc_content = "source ~/.oh-my-zsh/oh-my-zsh.sh\n";
    fs::write(home.join(".zshrc"), zshrc_content)?;

    // Execute: Init with import
    run_init_with_import()?;

    // Verify: .zshrc untouched
    let after = fs::read_to_string(home.join(".zshrc"))?;
    assert_eq!(zshrc_content, after, ".zshrc was modified!");
}

#[test]
fn test_zshenv_backup_created() {
    // Setup: Create existing .zshenv
    fs::write(home.join(".zshenv"), "export PATH=/custom:$PATH\n")?;

    // Execute: Init with import
    let timestamp = run_init_with_import()?;

    // Verify: Backup exists with timestamp
    let backup_path = zprof_dir
        .join("cache/backups")
        .join(format!(".zshenv.backup.{}", timestamp));
    assert!(backup_path.exists(), "Backup not created");
}
```

### Project Structure Notes

**New Module Created:**
- `src/shell/zdotdir.rs` - Manages `~/.zshenv` and ZDOTDIR setting

**Modified Files:**
- `src/cli/init.rs` - Enhanced with framework detection and import flow
- `src/shell/mod.rs` - Export zdotdir module

**Integration Points:**
- Uses `frameworks::detector` from Story 1.4 to detect existing frameworks
- Uses `core/manifest` from Story 1.5 to generate profile.toml
- Reuses file copying patterns from Story 1.5 implementation
- Builds on directory structure from Story 1.1a

### References

- [Source: docs/epics.md#Story-1.1-enhanced]
- [Source: docs/PRD.md#FR006-import-during-init]
- [Source: docs/architecture.md#Pattern-5-Shell-Integration]
- [Source: docs/architecture.md#NFR002-non-destructive-operations]
- [Source: docs/sprint-change-proposal-2025-11-01.md]

## Change Log

- 2025-11-01: Story created by Architect agent (Winston) during correct-course workflow
- Status: todo (awaiting implementation)

## Prerequisites

- Story 1.1a: Initialize zprof Directory Structure (done)
- Story 1.4: Framework Detection for Smart Profile Creation (done)
- Story 1.5: Profile Creation with Import Current Setup (review - provides manifest generation)
