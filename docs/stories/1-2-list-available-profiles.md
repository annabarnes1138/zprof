# Story 1.2: List Available Profiles

Status: ready-for-dev

## Story

As a developer,
I want to see all my available profiles with a visual indicator for the active one,
so that I know what profiles exist and which one I'm currently using.

## Acceptance Criteria

1. `zprof list` command displays all profiles in `~/.zsh-profiles/profiles/`
2. Active profile is visually indicated (e.g., with `*` or arrow)
3. Each profile shows its name and framework type
4. Output is human-readable and formatted clearly
5. Command handles empty profile directory gracefully with helpful message

## Tasks / Subtasks

- [ ] Implement `list` CLI command (AC: #1, #2, #4)
  - [ ] Create `cli/list.rs` with Clap Args struct for list command
  - [ ] Implement execute function following CLI command pattern from architecture
  - [ ] Add list subcommand to main CLI structure in `main.rs`
- [ ] Implement profile discovery logic (AC: #1, #3)
  - [ ] Create or extend `core/profile.rs` module for profile operations
  - [ ] Implement function to scan `~/.zsh-profiles/profiles/` directory
  - [ ] Read each profile's `profile.toml` manifest to extract metadata (name, framework)
  - [ ] Return list of ProfileInfo structs with name and framework type
- [ ] Implement active profile detection (AC: #2)
  - [ ] Read `~/.zsh-profiles/config.toml` to get active_profile value
  - [ ] Compare active_profile with discovered profiles
  - [ ] Mark active profile for visual indication
- [ ] Format and display output (AC: #2, #4)
  - [ ] Display each profile with name and framework type
  - [ ] Use `→` symbol to indicate active profile (following architecture error message format)
  - [ ] Ensure output is clean and human-readable
  - [ ] Sort profiles alphabetically for consistent display
- [ ] Handle edge cases gracefully (AC: #5)
  - [ ] Detect when `~/.zsh-profiles/profiles/` is empty
  - [ ] Display helpful message: "No profiles found. Create your first profile with 'zprof create <name>'"
  - [ ] Handle missing or malformed `profile.toml` files with warnings
  - [ ] Handle case where no active profile is set (all profiles unmarked)
- [ ] Add user-friendly error handling (AC: All)
  - [ ] Use anyhow::Context for all file operations following Pattern 2
  - [ ] Provide actionable error messages for permission issues
  - [ ] Handle missing `~/.zsh-profiles/` directory with suggestion to run `zprof init`
- [ ] Write integration tests (AC: All)
  - [ ] Test listing multiple profiles with correct formatting
  - [ ] Test active profile indicator appears correctly
  - [ ] Test empty profile directory shows helpful message
  - [ ] Test profiles sorted alphabetically
  - [ ] Test missing profile.toml handled gracefully
  - [ ] Add snapshot test for list output with multiple profiles
  - [ ] Add snapshot test for empty profile directory message

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `cli/list.rs`, `core/profile.rs`
- All modules must follow patterns defined in architecture.md Pattern 1 (CLI Command Structure)
- Error handling via anyhow::Result with context (Pattern 2)
- No backup operations needed (read-only command)

**TOML Manifest Reading:**
According to architecture.md Pattern 4, profile.toml schema:
```toml
[profile]
name = "work"
framework = "oh-my-zsh"  # oh-my-zsh | zimfw | prezto | zinit | zap
theme = "robbyrussell"
created = "2025-10-31T14:30:00Z"
modified = "2025-10-31T14:30:00Z"
```

config.toml schema:
```toml
active_profile = "work"
default_framework = "oh-my-zsh"  # optional
```

**Output Format Example:**
```
Available profiles:

→ work        (oh-my-zsh)
  experimental (zimfw)
  minimal      (zinit)
```

If empty:
```
No profiles found. Create your first profile with 'zprof create <name>'
```

**Error Handling:**
- Use `anyhow::Context` for all file operations
- Provide actionable error messages (what failed, why, how to fix)
- Never show raw Rust errors to users
- If `~/.zsh-profiles/` doesn't exist, suggest running `zprof init`

**Testing Strategy:**
- Integration tests in `tests/list_test.rs`
- Use insta for snapshot testing CLI output
- Test both success cases and edge cases (empty, malformed TOML)

**Performance Target (NFR001):**
- Expected execution time: < 50ms (directory scan + TOML read)
- This is a lightweight, read-only operation

### Project Structure Notes

**File Locations:**
- `src/cli/list.rs` - CLI command implementation
- `src/core/profile.rs` - Profile discovery and metadata extraction
- `src/core/config.rs` - Config reading (may already exist from Story 1.1)
- `tests/list_test.rs` - Integration tests
- `tests/snapshots/` - insta snapshot files

**Data Structures to Define:**
```rust
// In core/profile.rs
pub struct ProfileInfo {
    pub name: String,
    pub framework: String,
    pub is_active: bool,
}
```

**Dependencies (should already be in Cargo.toml from Story 1.1):**
```toml
[dependencies]
clap = { version = "4.5.51", features = ["derive"] }
anyhow = "2.0"
serde = { version = "1.0", features = ["derive"] }
toml = "0.9"
```

### References

- [Source: docs/epics.md#Story-1.2]
- [Source: docs/PRD.md#FR002-list-profiles]
- [Source: docs/architecture.md#Pattern-1-CLI-Command-Structure]
- [Source: docs/architecture.md#Pattern-2-Error-Handling]
- [Source: docs/architecture.md#Pattern-4-TOML-Manifest-Schema]
- [Source: docs/architecture.md#Error-Message-Format]
- [Source: docs/architecture.md#Epic-1-Story-1.2-Mapping]

## Dev Agent Record

### Context Reference

- docs/stories/1-2-list-available-profiles.context.xml

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
