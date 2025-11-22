# Architecture Overview

This document describes the high-level architecture of zprof for contributors and developers.

## System Overview

zprof is a Rust CLI tool for managing multiple zsh configurations through isolated profiles. It provides instant profile switching, safe experimentation, and portable profile sharing.

**Key architectural principles:**
- **Non-destructive**: Never modifies original user configs
- **Safe**: Automatic backups, validation before operations
- **Fast**: < 500ms profile switching
- **Modular**: Clear separation between CLI, GUI, core logic, frameworks, and shell integration
- **Dual Interface**: GUI for visual workflows, CLI for automation and power users

## Technology Stack

| Component | Technology | Version | Purpose |
|-----------|------------|---------|---------|
| Language | Rust | 1.70+ | Performance, safety, single-binary distribution |
| CLI Framework | Clap | 4.5+ | Type-safe argument parsing |
| GUI Framework | Tauri | 2.0+ | Native desktop application with web UI |
| Frontend | Svelte | 4+ | Reactive UI components |
| Styling | Tailwind CSS + shadcn/ui | Latest | Design system and utilities |
| Config Format | TOML | 0.9 | Profile manifests |
| Archives | tar + flate2 | 0.4 / 1.0 | Export/import |
| Git Operations | git2 | 0.20 | GitHub imports |
| Error Handling | anyhow | 2.0 | Rich error context |
| Testing | insta | latest | Snapshot testing |
| IPC | Tauri Commands | 2.0+ | Frontend ↔ Backend communication |

**Note:** TUI (Ratatui/Crossterm) was deprecated in v0.2.0 in favor of Tauri GUI (see [Technical Decision AD-003](technical-decisions.md#ad-003-gui-technology-selection-tauri))

## Project Structure

```
zprof/
├── src/                   # Rust CLI binary
│   ├── cli/               # Command implementations
│   │   ├── init.rs        # Initialize zprof
│   │   ├── create.rs      # Create profiles
│   │   ├── use_cmd.rs     # Switch profiles
│   │   ├── list.rs        # List profiles
│   │   ├── current.rs     # Show active profile
│   │   ├── delete.rs      # Delete profiles
│   │   ├── edit.rs        # Edit manifests
│   │   ├── export.rs      # Export archives
│   │   ├── import.rs      # Import archives
│   │   ├── gui.rs         # Launch GUI application (NEW)
│   │   └── rollback.rs    # Restore original config
│   │
│   ├── core/              # Core business logic
│   │   ├── config.rs      # Global config management
│   │   ├── profile.rs     # Profile CRUD operations
│   │   ├── manifest.rs    # TOML manifest parsing/generation
│   │   └── filesystem.rs  # Safe file operations
│   │
│   ├── frameworks/        # Framework support
│   │   ├── detector.rs    # Detect existing frameworks
│   │   ├── oh_my_zsh.rs   # oh-my-zsh support
│   │   ├── zimfw.rs       # zimfw support
│   │   ├── prezto.rs      # prezto support
│   │   ├── zinit.rs       # zinit support
│   │   ├── zap.rs         # zap support
│   │   ├── plugin.rs      # Plugin registry
│   │   └── theme.rs       # Theme registry
│   │
│   ├── prompts/           # Prompt engine support (NEW)
│   │   ├── mod.rs         # Prompt module root
│   │   └── engine.rs      # Prompt engine registry
│   │
│   ├── archive/           # Import/export
│   │   ├── export.rs      # Create .zprof archives
│   │   ├── import.rs      # Extract archives
│   │   └── github.rs      # GitHub imports
│   │
│   └── shell/             # Shell integration
│       ├── generator.rs   # Generate .zshrc/.zshenv
│       └── zdotdir.rs     # ZDOTDIR management
│
├── src-tauri/             # Tauri Rust backend (NEW)
│   ├── Cargo.toml
│   ├── tauri.conf.json    # Tauri app configuration
│   ├── build.rs
│   ├── icons/             # App icons (macOS .icns, Linux .png)
│   └── src/
│       ├── main.rs        # Tauri app entry point
│       ├── lib.rs         # Public library interface
│       ├── commands.rs    # IPC command handlers
│       ├── types.rs       # GUI-specific types
│       └── error.rs       # Error handling for IPC
│
├── src-ui/                # Svelte frontend (NEW)
│   ├── package.json
│   ├── vite.config.js
│   ├── tsconfig.json
│   ├── tailwind.config.js
│   ├── public/
│   │   └── favicon.ico
│   └── src/
│       ├── main.js
│       ├── App.svelte
│       ├── components/    # Reusable UI components
│       │   ├── ui/        # shadcn/ui components
│       │   │   ├── button.svelte
│       │   │   ├── card.svelte
│       │   │   └── ...
│       │   ├── Sidebar.svelte
│       │   ├── ProfileCard.svelte
│       │   └── ThemePreviewCard.svelte
│       ├── views/         # Main application views
│       │   ├── ProfileList.svelte
│       │   ├── CreateWizard.svelte
│       │   ├── Settings.svelte
│       │   └── About.svelte
│       ├── lib/
│       │   ├── api.ts     # Tauri IPC client wrapper
│       │   ├── types.ts   # TypeScript type definitions
│       │   ├── stores.ts  # Svelte stores (global state)
│       │   └── utils.ts   # Utility functions
│       └── styles/
│           └── globals.css # Tailwind + design tokens
│
└── tests/                 # Integration tests
    ├── init_test.rs
    ├── create_test.rs
    ├── use_test.rs
    ├── framework_detection_test.rs
    ├── gui_commands_test.rs (NEW)
    └── cli_gui_interop_test.rs (NEW)
```

## Architecture Overview

### Dual Interface Architecture

zprof provides two complementary interfaces sharing the same business logic:

```
┌────────────────────────────────────────────────────────┐
│                     User Interfaces                     │
├──────────────────────┬─────────────────────────────────┤
│   CLI (Terminal)     │     GUI (Tauri Desktop App)     │
│   - Automation       │     - Visual workflows          │
│   - Scripting        │     - Theme preview             │
│   - Power users      │     - Profile management        │
│   - SSH/Remote       │     - First-time users          │
└──────────┬───────────┴───────────┬─────────────────────┘
           │                       │
           │    ┌──────────────────┴──────────────┐
           │    │    Tauri IPC Layer              │
           │    │    (commands.rs)                │
           │    └──────────────────┬──────────────┘
           │                       │
           └───────────────────────┘
                       │
           ┌───────────▼──────────────────────────┐
           │   Core Business Logic (Rust)         │
           │   - Profile CRUD (core/profile.rs)   │
           │   - Framework Support (frameworks/)  │
           │   - Shell Generation (shell/)        │
           │   - Manifest Parsing (core/)         │
           └──────────────────────────────────────┘
                       │
           ┌───────────▼──────────────────────────┐
           │      Filesystem & Shell              │
           │   - ~/.zsh-profiles/                 │
           │   - ~/.zshenv (ZDOTDIR)              │
           └──────────────────────────────────────┘
```

**Key Principles:**
- **Shared Logic:** CLI and GUI use identical business logic (no duplication)
- **Feature Parity:** Both interfaces can perform all core operations
- **Independent:** CLI works without GUI, GUI is optional feature
- **Complementary:** GUI for discovery/visual tasks, CLI for automation

### GUI/IPC Communication Pattern

**Frontend (Svelte) ↔ Backend (Rust) via Tauri Commands:**

```typescript
// Frontend: src-ui/src/lib/api.ts
import { invoke } from '@tauri-apps/api/core';

export async function listProfiles(): Promise<ProfileInfo[]> {
  return await invoke('list_profiles');
}

export async function activateProfile(name: string): Promise<void> {
  return await invoke('activate_profile', { name });
}
```

```rust
// Backend: src-tauri/src/commands.rs
#[tauri::command]
pub fn list_profiles() -> Result<Vec<ProfileInfo>, String> {
    let profiles = core::profile::list_all()
        .map_err(|e| e.to_string())?;

    Ok(profiles.into_iter().map(|p| ProfileInfo::from(p)).collect())
}

#[tauri::command]
pub fn activate_profile(name: String) -> Result<(), String> {
    core::profile::activate(&name)
        .map_err(|e| e.to_string())
}
```

**Data Flow:**
1. User clicks "Activate" button in GUI
2. Svelte component calls `api.activateProfile(name)`
3. Tauri IPC invokes Rust `activate_profile` command
4. Command calls existing `core::profile::activate()`
5. Result serialized to JSON, returned to frontend
6. Frontend updates UI based on result

## Data Flow Patterns

### Profile Creation (CLI)

```
User runs: zprof create work
         ↓
CLI (create.rs) parses args
         ↓
Calls core::profile::create() (business logic)
         ↓
Manifest created (manifest.rs)
         ↓
Framework installed (frameworks/*.rs)
         ↓
Shell configs generated (generator.rs)
         ↓
Profile directory created (filesystem.rs)
```

### Profile Creation (GUI)

```
User opens GUI → ProfileList view
         ↓
Clicks "Create Profile" button
         ↓
CreateWizard sheet opens (Svelte component)
         ↓
User selects framework, plugins, theme (visual UI)
         ↓
User clicks "Create" → invoke('create_profile', config)
         ↓
Tauri IPC → commands::create_profile()
         ↓
Calls core::profile::create() (SAME business logic as CLI)
         ↓
Returns success/error to GUI
         ↓
GUI shows success message, navigates to profile list
```

**Note:** Both CLI and GUI paths converge at `core::profile::create()` - zero duplication

### Profile Switching

```
User runs: zprof use work
         ↓
CLI (use_cmd.rs) validates profile exists
         ↓
Config updated (config.rs): active_profile = "work"
         ↓
ZDOTDIR set (zdotdir.rs): ~/.zshenv points to profile
         ↓
User runs: exec zsh
         ↓
zsh reads ~/.zshenv → sources $ZDOTDIR/.zshrc
```

### Configuration Generation

```
profile.toml (manifest)
         ↓
manifest.rs parses TOML
         ↓
generator.rs generates .zshrc/.zshenv
         ↓
Framework-specific config (e.g., .zimrc for zimfw)
         ↓
Files written to profile directory
         ↓
Validated with `zsh -n`
```

## Key Design Patterns

### 1. CLI Command Structure

All CLI commands follow this pattern:

```rust
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct CommandArgs {
    // Command-specific arguments
}

pub fn execute(args: CommandArgs) -> Result<()> {
    // 1. Validate inputs
    // 2. Load configuration
    // 3. Perform operation
    // 4. Display output
    Ok(())
}
```

### 2. Framework Trait

All frameworks implement the `Framework` trait:

```rust
pub trait Framework {
    fn name(&self) -> &str;
    fn detect() -> Option<FrameworkInfo>;
    fn install(profile_path: &Path) -> Result<()>;
    fn get_plugins() -> Vec<Plugin>;
    fn get_themes() -> Vec<Theme>;
}
```

### 3. Safe File Operations

All file modifications follow this pattern:

```rust
// 1. Check: Verify source exists, destination is valid
// 2. Backup: Create backup if modifying existing file
// 3. Operate: Perform the file operation (copy, move, write)
// 4. Verify: Confirm operation succeeded
```

Example in `filesystem.rs`:
```rust
pub fn copy_with_backup(src: &Path, dest: &Path) -> Result<()> {
    // Check
    ensure!(src.exists(), "Source does not exist");

    // Backup
    if dest.exists() {
        create_backup(dest)?;
    }

    // Operate
    fs::copy(src, dest)?;

    // Verify
    ensure!(dest.exists(), "Copy verification failed");
    Ok(())
}
```

### 4. Manifest-Based Configuration

**Single source of truth**: `profile.toml`

Shell configs (`.zshrc`, `.zshenv`) are **generated artifacts**:
- Generated from manifest using `generator.rs`
- Include "DO NOT EDIT" warning header
- Regenerated when manifest changes
- Validated with `zsh -n` before writing

### 5. ZDOTDIR-Based Profile Switching

Profile activation uses zsh's `ZDOTDIR` mechanism:

```bash
# ~/.zshenv (managed by zprof)
export ZDOTDIR="$HOME/.zsh-profiles/profiles/work"
export HISTFILE="$HOME/.zsh-profiles/shared/.zsh_history"
```

When zsh starts, it sources `$ZDOTDIR/.zshrc` instead of `~/.zshrc`.

**Benefits:**
- Non-destructive (original `~/.zshrc` untouched)
- Instant switching (just update `~/.zshenv`)
- Native zsh feature (no hacks)

## Module Responsibilities

### CLI (`src/cli/`)

**Purpose**: Parse commands, orchestrate operations, display output

**Rules**:
- Thin layer, delegates to core/frameworks/tui
- Handles argument parsing (Clap)
- Displays user-friendly output
- No business logic

### Core (`src/core/`)

**Purpose**: Core business logic and data management

**Modules**:
- `config.rs`: Global config (`~/.zsh-profiles/config.toml`)
- `profile.rs`: Profile CRUD operations
- `manifest.rs`: TOML parsing/validation/generation
- `filesystem.rs`: Safe file operations with backups

**Rules**:
- Framework-agnostic
- All file operations go through `filesystem.rs`
- Validates all inputs before operations

### Frameworks (`src/frameworks/`)

**Purpose**: Framework-specific detection, installation, and config generation

**Modules**:
- `detector.rs`: Detects existing frameworks
- `{framework}.rs`: Framework-specific implementation
- `plugin.rs`: Plugin registry (600+ plugins)
- `theme.rs`: Theme registry

**Rules**:
- Each framework implements `Framework` trait
- Detection is read-only (never modifies files)
- Installation is idempotent

### GUI (`src-tauri/` + `src-ui/`)

**Purpose**: Native desktop application for visual workflows

**Architecture**:
- **Backend (`src-tauri/src/`)**: Tauri commands exposing business logic via IPC
- **Frontend (`src-ui/src/`)**: Svelte components for UI

**Modules**:
- `commands.rs`: Tauri command handlers (list_profiles, create_profile, etc.)
- `types.rs`: GUI-specific types for IPC serialization
- `error.rs`: Error conversion for JSON responses
- `api.ts` (frontend): TypeScript wrapper for Tauri invoke calls
- `stores.ts` (frontend): Global state management (active profile, theme, etc.)

**Rules**:
- **Backend**: Thin IPC layer, delegates to core/frameworks
- **Frontend**: Pure UI logic, no business logic
- **Communication**: All data via Tauri IPC (JSON serialization)
- **Error Handling**: Convert Rust errors to user-friendly messages
- **State Sync**: Frontend polls or subscribes to backend state changes

**Deprecated: TUI (`src/tui/`)** - Removed in v0.2.0, replaced by Tauri GUI

### Shell (`src/shell/`)

**Purpose**: Shell configuration generation and ZDOTDIR management

**Modules**:
- `generator.rs`: Generate `.zshrc`, `.zshenv` from manifests
- `zdotdir.rs`: Manage `~/.zshenv` for profile switching

**Rules**:
- Generates deterministic output (same manifest → same config)
- Validates generated configs with `zsh -n`
- Escapes shell values properly

### Archive (`src/archive/`)

**Purpose**: Import/export profiles as portable archives

**Modules**:
- `export.rs`: Create `.zprof` tar.gz archives
- `import.rs`: Extract and validate archives
- `github.rs`: Clone GitHub repos

**Rules**:
- Archives exclude framework binaries (too large)
- Metadata includes zprof version, export date
- Validates archives before extraction

## Data Model

### Global Config (`~/.zsh-profiles/config.toml`)

```toml
active_profile = "work"
default_framework = "oh-my-zsh"
```

### Profile Manifest (`profiles/<name>/profile.toml`)

```toml
[profile]
name = "work"
framework = "oh-my-zsh"
theme = "robbyrussell"
created = "2025-11-01T10:00:00Z"
modified = "2025-11-15T14:30:00Z"

[plugins]
enabled = ["git", "docker", "zsh-autosuggestions"]

[env]
EDITOR = "vim"
NODE_ENV = "development"
```

### Framework Info (Runtime)

```rust
pub struct FrameworkInfo {
    pub framework_type: FrameworkType,
    pub plugins: Vec<String>,
    pub theme: String,
    pub config_path: PathBuf,    // e.g., ~/.zshrc
    pub install_path: PathBuf,   // e.g., ~/.oh-my-zsh
}
```

## Extension Points

### Adding a New Framework

1. Create `src/frameworks/newframework.rs`
2. Implement `Framework` trait
3. Add to `FrameworkType` enum
4. Add detection in `detector.rs`
5. Add generator in `shell/generator.rs`
6. Update plugin/theme registries
7. Add tests

See [Adding Frameworks](adding-frameworks.md) for detailed guide.

### Adding a New Plugin

1. Add to `PLUGIN_REGISTRY` in `frameworks/plugin.rs`
2. Specify framework compatibility
3. Provide repo URL (for managers like zap)

### Adding a New Command

1. Create `src/cli/newcommand.rs`
2. Define `CommandArgs` struct
3. Implement `execute()` function
4. Register in `src/main.rs`
5. Add integration test

## Testing Strategy

### Unit Tests

- In-module tests (`#[cfg(test)] mod tests`)
- Test individual functions
- Mock filesystem operations

### Integration Tests

- In `tests/` directory
- Test complete command workflows
- Use `tempfile` for isolated environments
- `serial_test` for tests that modify HOME

### Snapshot Tests

- Using `insta` crate
- Validate CLI output
- Update snapshots with `cargo insta review`

**Example**:
```rust
#[test]
fn test_list_command_output() {
    let output = run_command("list");
    insta::assert_snapshot!(output);
}
```

## Performance Considerations

### Profile Switching Performance

**Target**: < 500ms from `zprof use` to completion

**Optimizations**:
- Minimal file I/O (only update `config.toml` and `~/.zshenv`)
- No validation unless `--validate` flag
- Lazy framework installation (only when creating profile)

### Shell Startup Performance

**Not controlled by zprof**, depends on:
- Framework choice (zinit > zimfw > oh-my-zsh)
- Number of plugins
- Theme complexity

zprof provides:
- Performance tips in docs
- Recommendations during wizard
- Future: startup time profiling

## Security Considerations

### Path Traversal Prevention

All paths validated to prevent escaping `~/.zsh-profiles/`:

```rust
pub fn validate_profile_name(name: &str) -> Result<()> {
    ensure!(!name.contains(".."), "Invalid profile name");
    ensure!(!name.contains("/"), "Invalid profile name");
    Ok(())
}
```

### Shell Injection Prevention

All values escaped before writing to shell configs:

```rust
pub fn escape_shell_value(value: &str) -> String {
    value.replace("\"", "\\\"")
         .replace("$", "\\$")
         .replace("`", "\\`")
}
```

### Backup Before Modification

Every destructive operation creates a backup:
- Profile deletion → backup to `cache/backups/`
- ZDOTDIR change → backup `~/.zshenv`
- Config regeneration → backup old `.zshrc`

## Error Handling

### Error Types

Using `anyhow` for application errors:

```rust
use anyhow::{Context, Result};

pub fn load_profile(name: &str) -> Result<Profile> {
    let path = get_profile_path(name)?;

    let content = fs::read_to_string(&path)
        .with_context(|| format!("Failed to read profile at {}", path.display()))?;

    // ...
}
```

### User-Friendly Messages

Errors include context for users:

```
Error: Failed to create profile 'work'

Caused by:
    0: Failed to install oh-my-zsh
    1: Failed to clone repository
    2: Network timeout

Suggestion: Check your internet connection and try again
```

## Future Architecture Considerations

### Plugin Version Management

Currently: Install latest version of plugins

Future: Support version pinning in manifest:

```toml
[plugins]
enabled = [
    { name = "git", version = "latest" },
    { name = "docker", version = "1.2.3" }
]
```

### Remote Profile Sync

Currently: Manual export/import

Future: Sync profiles to cloud storage:

```bash
zprof sync --remote s3://mybucket/zprof-profiles
```

### Performance Profiling

Future: Built-in startup time profiling:

```bash
zprof profile startup
# → Framework init: 200ms
# → Plugin loading: 300ms
# → Theme init: 50ms
```

## Development Workflow

See [Contributing Guide](contributing.md) for:
- Setting up development environment
- Running tests
- Code style guidelines
- Submitting pull requests

See [Testing Guide](testing.md) for:
- Test organization
- Writing integration tests
- Using snapshot tests
- Debugging test failures
