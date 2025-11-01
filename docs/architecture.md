# Decision Architecture

## Executive Summary

zprof is a Rust CLI tool that enables instant, risk-free zsh configuration management through isolated profiles. The architecture leverages Rust's safety guarantees and performance to meet the sub-500ms switching requirement while ensuring non-destructive operations. Built on the rust-starter template with Clap for CLI commands and Ratatui for interactive TUI wizards, zprof uses TOML manifests for declarative profile configuration and tar.gz archives for portability. The system follows a modular architecture with clear boundaries between CLI commands, core business logic, framework-specific adapters, TUI components, and shell integration, ensuring AI agents can implement stories consistently without conflicts.

## Project Initialization

**First Implementation Story:** Initialize Rust project using the rust-starter template

```bash
# Install cargo-generate (one-time setup)
cargo install cargo-generate

# Initialize zprof project
cargo generate rusty-ferris-club/rust-starter --name zprof
# When prompted, select: "1. CLI with subcommands (based on clap)"
```

This establishes the base architecture with these decisions:

- Rust 1.74.0+ as the language and toolchain
- Clap 4.5.51 for CLI argument parsing with derive macros
- Workspace structure with subcommand organization
- insta for snapshot testing
- xtask build system for task automation
- GitHub Actions CI/CD for multi-platform releases

## Decision Summary

| Category             | Decision              | Version         | Affects Epics | Rationale                                                             | Provided by Starter |
| -------------------- | --------------------- | --------------- | ------------- | --------------------------------------------------------------------- | ------------------- |
| Language             | Rust                  | 1.74.0+         | All           | Performance, safety, single binary distribution                       | ✓                   |
| CLI Framework        | Clap (derive API)     | 4.5.51          | All           | Industry standard, type-safe argument parsing                         | ✓                   |
| TUI Framework        | Ratatui + Crossterm   | 0.29.0 / 0.29.0 | Epic 1        | Best-in-class Rust TUI, keyboard navigation, cross-platform           | -                   |
| Configuration Format | TOML (not YAML)       | 0.9             | Epic 2        | Rust/Go developer familiarity, no indentation errors, explicit typing | -                   |
| TOML Parsing         | serde + toml          | 1.0 / 0.9       | Epic 2        | Standard serde integration, well-maintained                           | -                   |
| Error Handling       | anyhow                | 2.0             | All           | Application-level ergonomic errors with context                       | -                   |
| Archive Format       | tar + flate2 (tar.gz) | 0.4 / 1.0       | Epic 2        | Standard format, cross-platform, compression                          | -                   |
| Git Operations       | git2                  | 0.20            | Epic 2        | Mature libgit2 bindings, handles auth                                 | -                   |
| Logging              | env_logger            | latest          | All           | Lightweight, RUST_LOG env var support                                 | -                   |
| Progress Indicators  | indicatif             | 0.18            | Epic 1, 2     | CLI progress bars for long operations                                 | -                   |
| Date/Time            | chrono                | latest          | All           | ISO 8601 timestamps for metadata                                      | -                   |
| Testing              | insta (snapshots)     | latest          | All           | Snapshot testing for CLI output validation                            | ✓                   |
| Build Tasks          | xtask                 | n/a             | All           | Rust-native task runner                                               | ✓                   |
| CI/CD                | GitHub Actions        | n/a             | All           | Multi-platform releases (Linux, macOS, Windows)                       | ✓                   |

## Project Structure

```text
zprof/
├── Cargo.toml              # Workspace root with all dependencies
├── Cargo.lock
├── README.md
├── LICENSE
├── .gitignore
├── .github/
│   └── workflows/          # CI/CD for multi-platform releases
│       ├── ci.yml          # Continuous integration
│       └── release.yml     # Automated releases
├── src/
│   ├── main.rs            # Entry point, CLI setup with Clap
│   ├── lib.rs             # Library root for testing
│   ├── cli/               # Clap command definitions (one file per command)
│   │   ├── mod.rs
│   │   ├── init.rs        # `zprof init` - Story 1.1
│   │   ├── list.rs        # `zprof list` - Story 1.2
│   │   ├── create.rs      # `zprof create` - Stories 1.5, 1.6-1.8
│   │   ├── use_cmd.rs     # `zprof use` - Story 1.9 (use is Rust keyword)
│   │   ├── delete.rs      # `zprof delete` - Story 1.10
│   │   ├── current.rs     # `zprof current` - Story 1.3
│   │   ├── export.rs      # `zprof export` - Story 2.4
│   │   ├── import.rs      # `zprof import` - Stories 2.5, 2.6
│   │   └── edit.rs        # `zprof edit` - Story 2.3
│   ├── core/              # Core business logic
│   │   ├── mod.rs
│   │   ├── config.rs      # Manage ~/.zsh-profiles/config.toml
│   │   ├── profile.rs     # Profile CRUD operations
│   │   ├── manifest.rs    # Parse/generate profile.toml - Story 2.1, 2.2
│   │   └── filesystem.rs  # Safe file operations with backups (NFR002)
│   ├── frameworks/        # Framework detection & installation
│   │   ├── mod.rs         # Framework trait and common logic
│   │   ├── detector.rs    # Detect existing frameworks - Story 1.4
│   │   ├── oh_my_zsh.rs   # oh-my-zsh specific logic
│   │   ├── zimfw.rs       # zimfw specific logic
│   │   ├── prezto.rs      # prezto specific logic
│   │   ├── zinit.rs       # zinit specific logic
│   │   └── zap.rs         # zap specific logic
│   ├── tui/               # Terminal UI for interactive wizards
│   │   ├── mod.rs
│   │   ├── wizard.rs      # Main wizard orchestration
│   │   ├── framework_select.rs  # Framework selection screen - Story 1.6
│   │   ├── plugin_browser.rs    # Plugin multi-select - Story 1.7
│   │   └── theme_select.rs      # Theme selection - Story 1.8
│   ├── archive/           # Export/Import operations
│   │   ├── mod.rs
│   │   ├── export.rs      # Create .zprof tar.gz archives - Story 2.4
│   │   ├── import.rs      # Extract .zprof archives - Story 2.5
│   │   └── github.rs      # GitHub repository import - Story 2.6
│   ├── shell/             # Shell integration
│   │   ├── mod.rs
│   │   ├── zdotdir.rs     # ZDOTDIR manipulation for profile switching
│   │   └── generator.rs   # Generate .zshrc/.zshenv from manifest - Story 2.2
│   └── error.rs           # Error types and context helpers (anyhow)
├── tests/                 # Integration tests
│   ├── init_test.rs
│   ├── profile_lifecycle_test.rs
│   ├── import_export_test.rs
│   └── snapshots/         # insta snapshot files for CLI output
├── xtask/                 # Build tasks (from rust-starter)
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── docs/
    ├── architecture.md    # This document
    └── DEVELOPMENT.md     # Development guide for contributors
```

### User Data Directory Structure

```text
~/.zsh-profiles/
├── config.toml            # zprof's own configuration
│                          # - active_profile: string
│                          # - default_framework: string (optional)
├── profiles/              # User's zsh profiles
│   ├── work/
│   │   ├── profile.toml   # Manifest (framework, plugins, theme, env)
│   │   ├── .zshrc         # Generated from profile.toml
│   │   ├── .zshenv        # Generated from profile.toml
│   │   └── .oh-my-zsh/    # Framework installation (if oh-my-zsh)
│   └── experimental/
│       ├── profile.toml
│       ├── .zshrc
│       ├── .zshenv
│       └── .zimfw/        # Framework installation (if zimfw)
├── shared/                # Cross-profile shared data
│   └── .zsh_history       # Shared command history (FR018)
└── cache/                 # Temporary downloads, backups
    ├── backups/           # Automatic backups before modifications
    └── downloads/         # Framework/plugin downloads
```

## Epic to Architecture Mapping

### Epic 1: Core Profile Management & TUI Wizard

| Story                                                   | Primary Modules                            | Secondary Modules                        |
| ------------------------------------------------------- | ------------------------------------------ | ---------------------------------------- |
| 1.1 - Initialize zprof directory structure              | `cli/init.rs`, `core/filesystem.rs`        | `core/config.rs`                         |
| 1.2 - List available profiles                           | `cli/list.rs`, `core/profile.rs`           | -                                        |
| 1.3 - Display current active profile                    | `cli/current.rs`, `core/config.rs`         | -                                        |
| 1.4 - Framework detection for smart profile creation    | `frameworks/detector.rs`                   | All `frameworks/*.rs`                    |
| 1.5 - Profile creation with import current setup        | `cli/create.rs`, `frameworks/detector.rs`  | `core/manifest.rs`, `core/filesystem.rs` |
| 1.6 - TUI wizard framework selection                    | `tui/framework_select.rs`, `tui/wizard.rs` | `frameworks/mod.rs`                      |
| 1.7 - TUI wizard plugin browser                         | `tui/plugin_browser.rs`, `tui/wizard.rs`   | -                                        |
| 1.8 - TUI wizard theme selection and profile generation | `tui/theme_select.rs`, `tui/wizard.rs`     | `core/manifest.rs`, `shell/generator.rs` |
| 1.9 - Switch active profile                             | `cli/use_cmd.rs`, `shell/zdotdir.rs`       | `core/config.rs`                         |
| 1.10 - Delete profile                                   | `cli/delete.rs`, `core/profile.rs`         | `core/filesystem.rs`                     |

### Epic 2: TOML Manifests & Export/Import

| Story                                          | Primary Modules                      | Secondary Modules    |
| ---------------------------------------------- | ------------------------------------ | -------------------- |
| 2.1 - Parse and validate TOML manifests        | `core/manifest.rs`                   | -                    |
| 2.2 - Generate shell configuration from TOML   | `shell/generator.rs`                 | `core/manifest.rs`   |
| 2.3 - Manual TOML editing with live validation | `cli/edit.rs`, `core/manifest.rs`    | `shell/generator.rs` |
| 2.4 - Export profile to archive                | `cli/export.rs`, `archive/export.rs` | -                    |
| 2.5 - Import profile from local archive        | `cli/import.rs`, `archive/import.rs` | `core/manifest.rs`   |
| 2.6 - Import profile from GitHub repository    | `cli/import.rs`, `archive/github.rs` | `archive/import.rs`  |

## Technology Stack Details

### Core Technologies

**Rust 1.74.0+**

- Modern language features (async/await, pattern matching)
- Memory safety guarantees (prevents dotfile corruption - NFR002)
- Zero-cost abstractions for sub-500ms performance (NFR001)
- Single binary compilation for easy distribution

**Clap 4.5.51 (Derive API)**

- Type-safe CLI argument parsing
- Automatic help generation
- Subcommand structure: `zprof <verb> <noun>`
- Shell completion generation (bash, zsh, fish)

**Ratatui 0.29.0 + Crossterm 0.29.0**

- TUI framework for interactive wizards
- Multi-select lists (plugin browser)
- Single-select lists (framework/theme selection)
- Keyboard-only navigation (no mouse required)
- Works on all platforms (Linux, macOS, Windows)

**TOML (not YAML)**

- Configuration format: `profile.toml`, `config.toml`
- Parsed with `toml 0.9` + `serde 1.0`
- No indentation sensitivity (user error prevention)
- Explicit typing for better validation
- Familiar to Rust and Go developers

### Integration Technologies

**tar 0.4 + flate2 1.0**

- Archive format for `.zprof` files
- Cross-platform tar.gz compression
- Portable profile sharing

**git2 0.20**

- Rust bindings to libgit2
- GitHub repository cloning for `zprof import github:user/repo`
- Handles authentication via git credential helpers

**anyhow 2.0**

- Application-level error handling
- Rich error context with `.context()`
- User-friendly error messages

**env_logger**

- Lightweight logging for troubleshooting
- `RUST_LOG=debug zprof ...` for verbose output
- Minimal overhead in production

**indicatif 0.18**

- Progress bars for long operations
- Framework installation progress
- Archive extraction feedback

**chrono**

- ISO 8601 timestamps for profile metadata
- Creation dates, last modified times

### Testing & Build

**insta (Snapshot Testing)**

- CLI output validation
- Snapshot tests for all commands
- Easy to update expected outputs

**xtask**

- Rust-native task runner
- Build, test, lint tasks
- No external dependencies (Make, npm scripts, etc.)

**GitHub Actions**

- Multi-platform CI (Linux, macOS, Windows)
- Automated releases with binaries
- Homebrew tap automation

## Implementation Patterns

These patterns ensure all AI agents write compatible, consistent code across all stories.

### Pattern 1: CLI Command Structure

**Every command in `src/cli/` follows this structure:**

```rust
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct CommandArgs {
    // Command-specific arguments
}

pub fn execute(args: CommandArgs) -> Result<()> {
    // 1. Validate inputs
    // 2. Load configuration if needed
    // 3. Perform operation
    // 4. Display user-friendly output
    // 5. Return Result
    Ok(())
}
```

### Pattern 2: Error Handling (Critical for NFR002)

**ALL fallible operations MUST use anyhow::Result with context:**

```rust
use anyhow::{Context, Result};

fn backup_file(path: &Path) -> Result<()> {
    std::fs::copy(path, backup_path)
        .context(format!("Failed to backup {:?}. Check file permissions.", path))?;
    Ok(())
}
```

**Never show raw Rust errors to users. Always provide:**

- What failed
- Why it might have failed
- How to fix it

### Pattern 3: Safe File Operations (NFR002 Compliance)

**ALL agents MUST follow this pattern for destructive operations:**

```rust
// Pattern: Check -> Backup -> Operate -> Verify -> Cleanup
fn modify_dotfile(path: &Path) -> Result<()> {
    // 1. Check
    ensure!(path.exists(), "File does not exist: {:?}", path);

    // 2. Backup
    let backup = backup_file(path)?;

    // 3. Operate
    match perform_modification(path) {
        Ok(_) => {
            // 4. Verify
            verify_file_valid(path)?;
            // 5. Cleanup backup
            fs::remove_file(backup)?;
            Ok(())
        }
        Err(e) => {
            // Restore from backup on failure
            fs::copy(backup, path)?;
            Err(e)
        }
    }
}
```

### Pattern 4: TOML Manifest Schema

**All profile.toml files follow this schema:**

```toml
[profile]
name = "work"
framework = "oh-my-zsh"  # oh-my-zsh | zimfw | prezto | zinit | zap
theme = "robbyrussell"
created = "2025-10-31T14:30:00Z"
modified = "2025-10-31T14:30:00Z"

[plugins]
enabled = [
    "git",
    "docker",
    "kubectl",
    "fzf"
]

[env]
EDITOR = "vim"
GOPATH = "$HOME/go"
```

**config.toml (zprof's own config):**

```toml
active_profile = "work"
default_framework = "oh-my-zsh"  # optional
```

### Pattern 5: Shell Integration

**Profile switching mechanism:**

```rust
// Set ZDOTDIR to profile directory
env::set_var("ZDOTDIR", profile_path);

// Execute new shell (replaces current process)
std::process::Command::new("zsh")
    .exec(); // Never returns
```

### Pattern 6: Framework Trait

**All framework implementations in `src/frameworks/*.rs` implement:**

```rust
pub trait Framework {
    fn name(&self) -> &str;
    fn detect() -> Option<FrameworkInfo>;
    fn install(profile_path: &Path) -> Result<()>;
    fn get_plugins() -> Vec<Plugin>;
    fn get_themes() -> Vec<Theme>;
}
```

## Consistency Rules

### Naming Conventions

**Files and Modules:**

- Snake case: `profile_manager.rs`, `framework_select.rs`
- CLI commands: `use_cmd.rs` (when keyword conflicts)

**Rust Code:**

- Structs: PascalCase - `ProfileConfig`, `FrameworkInfo`
- Functions: snake_case - `load_profile()`, `generate_zshrc()`
- Constants: SCREAMING_SNAKE_CASE - `DEFAULT_PROFILE_DIR`

**User-Facing:**

- Commands: kebab-case - `zprof init`, `zprof use`
- Profile names: user's choice (alphanumeric + hyphens recommended)
- File extensions: `.toml`, `.zprof` (tar.gz)

### Error Message Format

**Success:**

```sh
✓ Profile 'work' created successfully
```

**Error:**

```sh
✗ Error: Profile 'work' already exists
  Suggestion: Use 'zprof delete work' first or choose a different name
```

**Symbols:**

- ✓ Success
- ✗ Error
- - Active/selected item
- → Action/suggestion

### Code Organization Patterns

**Module Organization:**

- One CLI command = one file in `cli/`
- One framework = one file in `frameworks/`
- One TUI screen = one file in `tui/`

**Import Order:**

1. Standard library (`std::`)
2. External crates (`anyhow`, `clap`, etc.)
3. Internal crates (`crate::core::`, `crate::cli::`)

**Testing:**

- Unit tests in same file as code (`#[cfg(test)] mod tests`)
- Integration tests in `tests/` directory
- Snapshot tests for CLI output

### Logging Strategy

**Use env_logger for development/debugging:**

```rust
// Initialize once in main.rs
env_logger::init();

// Use throughout codebase
log::debug!("Loading profile: {}", name);
log::info!("Profile created successfully");
log::warn!("Framework not detected, using default");
log::error!("Failed to backup file: {}", path.display());
```

**Levels:**

- `error!` - Operation failures
- `warn!` - Recoverable issues
- `info!` - Important state changes
- `debug!` - Detailed troubleshooting
- `trace!` - Very verbose (rarely used)

**User control:**

```bash
RUST_LOG=debug zprof create test  # Verbose output
RUST_LOG=info zprof create test   # Normal output
zprof create test                 # Minimal output (default)
```

## Security Architecture

### Authentication & Authorization

**No user authentication required** - zprof is a single-user local tool.

**File System Permissions:**

- All operations respect existing file permissions
- Backups created with same permissions as originals
- Config directory: `~/.zsh-profiles/` (user-only access recommended)

### Data Protection

**Backup Strategy (NFR002 Compliance):**

- Automatic backups before ANY destructive operation
- Backups stored in `~/.zsh-profiles/cache/backups/`
- Backup naming: `<filename>.backup.<timestamp>`
- Restoration on failure

**Sensitive Data Handling:**

- No passwords or secrets stored by zprof
- Environment variables in `profile.toml` are user-managed
- GitHub authentication uses git credential helpers (no token storage)

**Import Security:**

- Validate `.zprof` archives before extraction
- Prevent path traversal attacks in archive extraction
- GitHub imports clone to temporary directory, validate before copying

## Performance Considerations

### NFR001: Sub-500ms Profile Switching

**Optimization strategies:**

1. **Pre-compiled binary** - Rust compiles to native code
2. **Minimal file operations** - Only update ZDOTDIR env var
3. **No network calls** - Switching is 100% local
4. **exec()** - Replace process instead of spawning child

**Expected performance:**

- `zprof use <profile>`: < 100ms (just env var + exec)
- `zprof list`: < 50ms (directory scan + TOML read)
- `zprof current`: < 10ms (read config.toml)

### Memory Efficiency (NFR003)

**Lightweight design:**

- No background daemon
- No persistent cache in memory
- CLI process exits after each command
- TUI only loads when needed (create wizard)

**Target:** < 50MB RAM usage during execution

## Deployment Architecture

### Distribution Method

**Homebrew (Primary):**

```bash
brew tap anna/zprof
brew install zprof
```

**Manual Installation:**

Download binary from GitHub releases:

- Linux: `zprof-linux-x86_64`
- macOS: `zprof-darwin-x86_64`, `zprof-darwin-arm64`
- Windows: `zprof-windows-x86_64.exe` (WSL recommended)

### Build Process

**GitHub Actions workflow:**

1. **CI Pipeline** (on PR/push):

   - `cargo test` - Run all tests
   - `cargo clippy` - Linting
   - `cargo fmt --check` - Format checking
   - Test on: Linux, macOS, Windows

2. **Release Pipeline** (on tag push):
   - Build for all platforms
   - Run tests
   - Create GitHub release
   - Upload binaries as artifacts
   - Update Homebrew formula

### Runtime Dependencies

**None!** - Single static binary.

**Optional:**

- git (for `zprof import github:user/repo`)
- zsh (obviously, for using profiles)

## Development Environment

### Prerequisites

**Required:**

- Rust 1.74.0+ (`rustup install stable`)
- cargo (bundled with Rust)

**Recommended:**

- rust-analyzer (LSP)
- clippy (`rustup component add clippy`)
- rustfmt (`rustup component add rustfmt`)

### Setup Commands

```bash
# Clone repository
git clone https://github.com/anna/zprof.git
cd zprof

# Generate project from template (first time only)
cargo install cargo-generate
cargo generate rusty-ferris-club/rust-starter --name zprof

# Build project
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run -- init

# Build release binary
cargo build --release

# Run clippy
cargo clippy

# Format code
cargo fmt
```

### Development Workflow

1. **Create feature branch** from main
2. **Implement story** following architecture patterns
3. **Write tests** (unit + integration + snapshots)
4. **Run checks**: `cargo test && cargo clippy && cargo fmt --check`
5. **Create PR** with CI passing
6. **Merge** after review

## Architecture Decision Records (ADRs)

### ADR-001: Use Rust instead of Go

**Status:** Accepted

**Context:** CLI tool needs sub-500ms performance and safe file operations.

**Decision:** Implement in Rust despite Go being more familiar to target users.

**Rationale:**

- Rust's ownership model prevents file corruption bugs (NFR002)
- Performance easily meets NFR001 requirements
- Memory safety eliminates entire class of bugs
- Single binary distribution (like Go)
- Excellent CLI/TUI ecosystem (clap, ratatui)

**Consequences:**

- Steeper learning curve for contributors
- Longer compile times than Go
- Strong safety guarantees worth the trade-off

### ADR-002: Use TOML instead of YAML for Manifests

**Status:** Accepted

**Context:** Need human-editable configuration format for profiles.

**Decision:** Use TOML for `profile.toml` and `config.toml`.

**Rationale:**

- Go/Rust developers already familiar with TOML
- No indentation sensitivity (prevents user errors)
- Explicit typing enables better validation
- serde_yaml is deprecated, serde_toml is maintained
- Cargo.toml precedent in Rust ecosystem

**Consequences:**

- Less compact than YAML for nested structures
- Need to educate users if expecting YAML
- Better error prevention outweighs verbosity

### ADR-003: Use Ratatui for TUI (not CLI-only)

**Status:** Accepted

**Context:** Framework/plugin/theme selection needs user-friendly interface.

**Decision:** Implement interactive TUI wizard with Ratatui for profile creation.

**Rationale:**

- Multi-select plugin browser impossible with pure CLI
- Better UX for browsing 50+ plugins
- Keyboard navigation familiar to terminal users
- Ratatui is mature, well-maintained
- Optional - only used for `create` command

**Consequences:**

- Increased complexity vs pure CLI
- Larger binary size
- Better user experience justifies added complexity

### ADR-004: exec() for Profile Switching (not subshell)

**Status:** Accepted

**Context:** Need instant profile switching without leaving current terminal.

**Decision:** Use `exec zsh` to replace current shell process.

**Rationale:**

- Instant switch (no nested shells)
- Clean process tree
- Native zsh behavior via ZDOTDIR
- Meets < 500ms requirement easily

**Consequences:**

- Current shell session ends (expected behavior)
- Can't return to previous profile without `zprof use` again
- Standard behavior for environment managers (nvm, pyenv, rbenv)

### ADR-005: No Background Daemon

**Status:** Accepted

**Context:** Could implement background process for auto-switching or monitoring.

**Decision:** zprof is a simple CLI tool with no daemon.

**Rationale:**

- Simpler architecture
- Lower memory footprint (NFR003)
- No persistent processes to manage
- Auto-switching deferred to Phase 2+

**Consequences:**

- No automatic profile switching based on directory
- No continuous monitoring features
- Keeps MVP focused and lightweight

---

**Generated by BMAD Decision Architecture Workflow v1.3.2**
**Date:** 2025-10-31
**For:** Anna
**Project:** zprof (Level 2 Greenfield)
