# Story 1.1: Initialize zprof Directory Structure

Status: done

## Story

As a developer,
I want to initialize zprof's directory structure in my home directory,
so that I have a clean foundation for managing multiple zsh profiles.

## Acceptance Criteria

1. `zprof init` command creates `~/.zsh-profiles/` directory structure with `profiles/`, `shared/`, and `cache/` subdirectories
2. Shared command history file `.zsh_history` is created in `shared/` directory
3. Global configuration file `config.toml` is created with sensible defaults
4. Command outputs success message confirming initialization
5. Running `zprof init` when already initialized warns user but does not corrupt existing data

## Tasks / Subtasks

- [x] Implement `init` CLI command (AC: #1, #4, #5)
  - [x] Create `cli/init.rs` with Clap Args struct for init command
  - [x] Implement execute function following CLI command pattern from architecture
  - [x] Add init subcommand to main CLI structure in `main.rs`
- [x] Implement directory structure creation logic (AC: #1)
  - [x] Create `core/filesystem.rs` module for safe file operations
  - [x] Implement function to create `~/.zsh-profiles/` base directory
  - [x] Implement function to create `profiles/`, `shared/`, and `cache/` subdirectories
  - [x] Use Rust's `std::fs::create_dir_all` for atomic directory creation
- [x] Initialize shared history file (AC: #2)
  - [x] Create empty `.zsh_history` file in `shared/` directory
  - [x] Set appropriate file permissions (user read/write only: 0600)
- [x] Generate default config.toml (AC: #3)
  - [x] Create `core/config.rs` module for config management
  - [x] Define Config struct with serde Deserialize/Serialize derives
  - [x] Implement function to generate default config.toml with placeholder values
  - [x] Write config.toml to `~/.zsh-profiles/` directory
- [x] Handle re-initialization gracefully (AC: #5)
  - [x] Check if `~/.zsh-profiles/` already exists before creating
  - [x] If exists, display warning message and exit without modifying
  - [x] Ensure no data corruption on repeated `zprof init` calls
- [x] Add user-friendly output messages (AC: #4)
  - [x] Success message with ✓ symbol showing directory path
  - [x] Confirmation of created subdirectories
  - [x] Warning message with → symbol for re-initialization attempts
- [x] Write integration tests (AC: All)
  - [x] Test fresh initialization creates all directories and files
  - [x] Test config.toml has correct structure and defaults
  - [x] Test .zsh_history file is created with correct permissions
  - [x] Test re-initialization shows warning without corruption
  - [x] Add snapshot test for success output message
  - [x] Add snapshot test for re-init warning message

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `cli/init.rs`, `core/filesystem.rs`, `core/config.rs`
- All modules must follow patterns defined in architecture.md Pattern 1 (CLI Command Structure)
- Error handling via anyhow::Result with context (Pattern 2)
- Safe file operations following Pattern 3 (Check -> Backup -> Operate -> Verify -> Cleanup)

**Directory Layout:**
```
~/.zsh-profiles/
├── config.toml            # zprof's own configuration
├── profiles/              # User's zsh profiles (empty initially)
├── shared/                # Cross-profile shared data
│   └── .zsh_history       # Shared command history (FR018)
└── cache/                 # Temporary downloads, backups
    ├── backups/           # Automatic backups (for future stories)
    └── downloads/         # Framework/plugin downloads (for future stories)
```

**config.toml Default Structure:**
```toml
# No active_profile initially (set when first profile created)
# default_framework is optional
```

**Error Handling:**
- Use `anyhow::Context` for all file operations
- Provide actionable error messages (what failed, why, how to fix)
- Never show raw Rust errors to users

**Testing Strategy:**
- Integration tests in `tests/init_test.rs`
- Use insta for snapshot testing CLI output
- Test both success and edge cases (re-init, permission errors)

### Project Structure Notes

**File Locations:**
- `src/cli/init.rs` - CLI command implementation
- `src/core/filesystem.rs` - Safe file operation utilities
- `src/core/config.rs` - Config struct and TOML generation
- `tests/init_test.rs` - Integration tests
- `tests/snapshots/` - insta snapshot files

**Dependencies to Add (if not present):**
```toml
[dependencies]
clap = { version = "4.5.51", features = ["derive"] }
anyhow = "2.0"
serde = { version = "1.0", features = ["derive"] }
toml = "0.9"
```

### References

- [Source: docs/epics.md#Story-1.1]
- [Source: docs/PRD.md#FR001-FR002-FR003]
- [Source: docs/architecture.md#Pattern-1-CLI-Command-Structure]
- [Source: docs/architecture.md#Pattern-2-Error-Handling]
- [Source: docs/architecture.md#Pattern-3-Safe-File-Operations]
- [Source: docs/architecture.md#User-Data-Directory-Structure]
- [Source: docs/architecture.md#NFR002-non-destructive-operations]

## Dev Agent Record

### Context Reference

- docs/stories/1-1-initialize-zprof-directory-structure.context.xml

### Agent Model Used

claude-sonnet-4-5-20250929

### Debug Log References

Implementation Plan:

1. Initialize Rust project with cargo init
2. Add required dependencies: clap, anyhow, serde, toml, dirs
3. Create core modules (config.rs, filesystem.rs) following architecture patterns
4. Implement CLI command structure in cli/init.rs
5. Wire everything into main.rs
6. Write comprehensive integration tests with insta snapshots
7. Validate all acceptance criteria through tests

### Completion Notes List

✓ Implemented complete `zprof init` command following Pattern 1 (CLI Command Structure)
✓ Created directory structure with all required subdirectories (profiles/, shared/, cache/, cache/backups/, cache/downloads/)
✓ Generated shared history file with correct 0600 permissions (AC#2)
✓ Created config.toml with optional fields using serde serialization (AC#3)
✓ Implemented graceful re-initialization handling with warning messages (AC#5)
✓ Added user-friendly output with ✓ and → symbols (AC#4)
✓ All 12 tests passing (6 unit tests + 6 integration tests)
✓ Used anyhow::Result with context throughout for proper error handling (Pattern 2)
✓ Followed Safe File Operations pattern (Pattern 3)

Technical decisions:

- Used serde with skip_serializing_if for optional config fields
- Added dirs crate for cross-platform home directory detection
- Normalized paths in snapshot tests for stable test output
- Set up cargo-insta for snapshot testing

### File List

- Cargo.toml
- src/main.rs
- src/cli/mod.rs
- src/cli/init.rs
- src/core/mod.rs
- src/core/config.rs
- src/core/filesystem.rs
- tests/init_test.rs
- tests/snapshots/init_test__init_success_output.snap
- tests/snapshots/init_test__reinit_warning_output.snap

## Change Log

- 2025-10-31: Story drafted by SM agent (Bob)
- 2025-10-31: Story implemented and completed by Dev agent (Amelia) - All acceptance criteria met, 12 tests passing
