# Story 1.5: Profile Creation with Import Current Setup

Status: ready-for-dev

## Story

As a developer with an existing zsh configuration,
I want to import my current setup as a zprof profile,
so that I can preserve my working configuration before experimenting.

## Acceptance Criteria

1. When framework detected, `zprof create <name>` prompts "Import current setup? (y/n)"
2. On "y", system copies current framework files to new profile directory
3. Profile includes detected framework, plugins, theme, and custom configurations
4. TOML manifest is generated from imported configuration
5. Original dotfiles remain untouched and functional
6. Success message confirms profile creation with imported details

## Tasks / Subtasks

- [ ] Implement `zprof create` CLI command (AC: #1)
  - [ ] Create `cli/create.rs` with CreateArgs struct using Clap derive API per Pattern 1
  - [ ] Accept profile name as required positional argument
  - [ ] Add `execute(args: CreateArgs) -> Result<()>` function following Pattern 1
  - [ ] Wire up command in main.rs CLI structure
- [ ] Integrate framework detection (AC: #1, #3)
  - [ ] Call `frameworks::detector::detect_existing_framework()` from Story 1.4
  - [ ] Handle case when framework is detected vs not detected
  - [ ] If detected, display framework info (type, plugins, theme) to user
  - [ ] If not detected, show message and defer to Story 1.6 (TUI wizard path)
- [ ] Implement interactive import prompt (AC: #1)
  - [ ] Display "Import current setup? (y/n)" prompt when framework detected
  - [ ] Read user input from stdin
  - [ ] Handle 'y', 'n', and invalid inputs with clear feedback
  - [ ] On 'n', show message that import is skipped (defer to TUI wizard in Story 1.6)
  - [ ] Support case-insensitive input (Y/y, N/n)
- [ ] Copy framework files to profile directory (AC: #2, #3, #5)
  - [ ] Create profile directory at `~/.zsh-profiles/profiles/<name>/`
  - [ ] Use `core/filesystem.rs` safe file operations following Pattern 3
  - [ ] Copy framework installation directory (e.g., ~/.oh-my-zsh → profile/.oh-my-zsh)
  - [ ] Copy .zshrc to profile/.zshrc (preserve original in home directory per NFR002)
  - [ ] Copy .zshenv if exists to profile/.zshenv
  - [ ] Copy any framework-specific config files (.zimrc, .zpreztorc, etc.)
  - [ ] Verify original dotfiles unchanged after copy (AC: #5)
- [ ] Generate TOML manifest from imported config (AC: #4)
  - [ ] Create `core/manifest.rs` with Manifest struct using serde
  - [ ] Define Manifest schema matching architecture.md Pattern 4
  - [ ] Extract framework info from FrameworkInfo struct
  - [ ] Write profile.toml with framework type, plugins, theme, env vars
  - [ ] Add creation timestamp using chrono
  - [ ] Save manifest to `~/.zsh-profiles/profiles/<name>/profile.toml`
- [ ] Update global config to track new profile (AC: #6)
  - [ ] Load `~/.zsh-profiles/config.toml` using `core/config.rs`
  - [ ] Add new profile to configuration if not already tracked
  - [ ] Save updated config.toml
- [ ] Display success message (AC: #6)
  - [ ] Show confirmation: "✓ Profile '<name>' created successfully"
  - [ ] Display imported framework details (type, plugin count, theme)
  - [ ] Show profile location path
  - [ ] Suggest next steps: "Use 'zprof use <name>' to switch to this profile"
  - [ ] Follow error message format from architecture.md consistency rules
- [ ] Handle edge cases and errors (AC: All)
  - [ ] Check if profile name already exists, show error with suggestion
  - [ ] Handle permission errors during file copy with context using Pattern 2
  - [ ] Handle invalid profile names (special chars, path traversal attempts)
  - [ ] Gracefully handle partial detection (some config missing)
  - [ ] Log operations with env_logger for debugging
- [ ] Write comprehensive tests (AC: All)
  - [ ] Unit test CreateArgs parsing with Clap
  - [ ] Unit test manifest generation from FrameworkInfo
  - [ ] Integration test full `zprof create work` flow with mock detected framework
  - [ ] Test import flow with 'y' response
  - [ ] Test skip import with 'n' response
  - [ ] Test profile name conflict handling
  - [ ] Test original dotfiles remain unchanged after import (NFR002)
  - [ ] Test invalid profile name rejection
  - [ ] Snapshot test CLI output with insta crate
  - [ ] Test creation completes in under 5 seconds for typical profile

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `cli/create.rs`, `core/manifest.rs`, `core/filesystem.rs`
- Secondary: `frameworks/detector.rs` (from Story 1.4), `core/config.rs`
- Follow Pattern 1 (CLI Command Structure) for command implementation
- Follow Pattern 2 (Error Handling) with anyhow::Result and context
- Follow Pattern 3 (Safe File Operations) for all file copies (NFR002 compliance)
- Follow Pattern 4 (TOML Manifest Schema) for profile.toml generation

**Key Patterns to Apply:**

**Pattern 1 - CLI Command Structure:**
```rust
// cli/create.rs
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct CreateArgs {
    /// Name of the profile to create
    #[arg(value_name = "NAME")]
    pub name: String,
}

pub fn execute(args: CreateArgs) -> Result<()> {
    // 1. Validate profile name
    // 2. Detect existing framework
    // 3. Prompt for import if framework found
    // 4. Copy files and generate manifest
    // 5. Display success message
    Ok(())
}
```

**Pattern 3 - Safe File Operations (Critical for NFR002):**
```rust
// Must preserve original dotfiles - use copy, NOT move
fn copy_framework_files(source: &Path, dest: &Path) -> Result<()> {
    // 1. Check source exists
    ensure!(source.exists(), "Source does not exist: {:?}", source);

    // 2. Create destination (no backup needed - creating new)
    fs::create_dir_all(dest)?;

    // 3. Copy (not move!) to preserve originals
    copy_dir_recursive(source, dest)
        .context("Failed to copy framework files")?;

    // 4. Verify source still exists (sanity check)
    ensure!(source.exists(), "Original files missing after copy!");

    Ok(())
}
```

**Pattern 4 - TOML Manifest Schema:**
```toml
[profile]
name = "work"
framework = "oh-my-zsh"
theme = "robbyrussell"
created = "2025-10-31T20:30:00Z"
modified = "2025-10-31T20:30:00Z"

[plugins]
enabled = [
    "git",
    "docker",
    "kubectl"
]

[env]
# Empty for now, can be manually added later
```

**User Flow:**
1. User runs: `zprof create work`
2. System detects oh-my-zsh installation
3. Prompt: "Detected oh-my-zsh with 3 plugins (git, docker, kubectl) and theme 'robbyrussell'. Import current setup? (y/n)"
4. User types: `y`
5. System copies ~/.oh-my-zsh → ~/.zsh-profiles/profiles/work/.oh-my-zsh
6. System copies ~/.zshrc → ~/.zsh-profiles/profiles/work/.zshrc
7. System generates ~/.zsh-profiles/profiles/work/profile.toml
8. Output: "✓ Profile 'work' created successfully"
9. Output: "  Framework: oh-my-zsh"
10. Output: "  Plugins: 3 (git, docker, kubectl)"
11. Output: "  Theme: robbyrussell"
12. Output: "  → Use 'zprof use work' to switch to this profile"

**Dependencies to Add:**
```toml
[dependencies]
chrono = "0.4"              # Timestamps for manifest
dialoguer = "0.11"          # Interactive prompts (y/n confirmation)
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"                # TOML parsing/serialization
```

**Error Handling Examples:**
```rust
// Profile name conflict
if profile_exists(&name) {
    bail!("✗ Error: Profile '{}' already exists\n  → Use 'zprof delete {}' first or choose a different name", name, name);
}

// Invalid profile name
if !is_valid_profile_name(&name) {
    bail!("✗ Error: Invalid profile name '{}'\n  → Use alphanumeric characters and hyphens only", name);
}

// Permission error
copy_framework_files(source, dest)
    .context(format!("Failed to copy framework files from {:?}. Check file permissions.", source))?;
```

### Project Structure Notes

**New Files Created:**
- `src/cli/create.rs` - Main command implementation
- `src/core/manifest.rs` - TOML manifest parsing/generation
- `src/core/filesystem.rs` - Safe file operations with backups
- `tests/create_test.rs` - Integration tests for create command

**Modified Files:**
- `src/main.rs` - Wire up `create` subcommand
- `src/cli/mod.rs` - Export CreateArgs and execute function
- `Cargo.toml` - Add dependencies (chrono, dialoguer, serde, toml)

**Profile Directory Structure Created:**
```
~/.zsh-profiles/profiles/work/
├── profile.toml        # Generated manifest
├── .zshrc              # Copied from home
├── .zshenv             # Copied if exists
└── .oh-my-zsh/         # Framework installation copy
    └── ... (all files)
```

**Integration Points:**
- Uses `frameworks::detector` from Story 1.4 to detect existing framework
- Creates foundation for Story 1.6 (TUI wizard) when no framework detected
- Generates profile.toml that will be used by Story 2.1 (manifest parsing)
- Sets up profile structure that Story 1.9 (switch profile) will activate

### Learnings from Previous Story

**From Story 1.4 (Status: drafted)**

Story 1.4 implements framework detection but hasn't been implemented yet. When implementing Story 1.5, ensure:

**Integration Requirements:**
- Use `frameworks::detector::detect_existing_framework()` function
- Expect `Option<FrameworkInfo>` return type
- FrameworkInfo contains: framework_type, plugins, theme, config_path, install_path
- Detection handles all 5 frameworks: oh-my-zsh, zimfw, prezto, zinit, zap
- Detection is fault-tolerant (returns None if issues, doesn't crash)

**Expected Interface from Story 1.4:**
```rust
// Available from frameworks/detector.rs
pub fn detect_existing_framework() -> Option<FrameworkInfo>;

// Available from frameworks/mod.rs
pub struct FrameworkInfo {
    pub framework_type: FrameworkType,
    pub plugins: Vec<String>,
    pub theme: String,
    pub config_path: PathBuf,
    pub install_path: PathBuf,
}
```

**Coordination Note:**
Since Story 1.4 is drafted but not implemented, Story 1.5 implementation should:
1. Define the expected FrameworkInfo struct if not yet present
2. Create stub/mock detection for testing purposes
3. Document the expected integration contract clearly
4. Be ready to integrate real detection once Story 1.4 is complete

### References

- [Source: docs/epics.md#Story-1.5]
- [Source: docs/PRD.md#FR006-import-current-setup]
- [Source: docs/architecture.md#Pattern-1-CLI-Command-Structure]
- [Source: docs/architecture.md#Pattern-2-Error-Handling]
- [Source: docs/architecture.md#Pattern-3-Safe-File-Operations]
- [Source: docs/architecture.md#Pattern-4-TOML-Manifest-Schema]
- [Source: docs/architecture.md#Epic-1-Story-1.5-Mapping]
- [Source: docs/architecture.md#NFR002-non-destructive-operations]

## Dev Agent Record

### Context Reference

- docs/stories/1-5-profile-creation-with-import-current-setup.context.xml

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
