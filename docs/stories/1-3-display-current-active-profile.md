# Story 1.3: Display Current Active Profile

Status: ready-for-dev

## Story

As a developer,
I want to quickly check which profile is currently active,
so that I can confirm my shell environment context.

## Acceptance Criteria

1. `zprof current` command displays the currently active profile name
2. Output includes profile metadata (framework, creation date)
3. If no profile is active, displays clear message
4. Command executes in under 100ms for quick reference

## Tasks / Subtasks

- [ ] Implement `current` CLI command (AC: #1, #2, #3)
  - [ ] Create `cli/current.rs` with Clap Args struct for current command
  - [ ] Implement execute function following CLI command pattern from architecture
  - [ ] Add current subcommand to main CLI structure in `main.rs`
- [ ] Implement active profile retrieval (AC: #1)
  - [ ] Extend `core/config.rs` module to read `~/.zsh-profiles/config.toml`
  - [ ] Implement function to get active_profile value from config
  - [ ] Handle missing config file gracefully
- [ ] Load and display profile metadata (AC: #2)
  - [ ] Use `core/profile.rs` to load profile's `profile.toml` manifest
  - [ ] Extract profile name, framework, and creation date from manifest
  - [ ] Format metadata for display
- [ ] Handle no active profile case (AC: #3)
  - [ ] Detect when `active_profile` field is missing or empty in config.toml
  - [ ] Display clear message: "No active profile. Use 'zprof use <name>' to activate a profile."
  - [ ] Exit with success status (not an error condition)
- [ ] Format and display output (AC: #2)
  - [ ] Display profile name prominently
  - [ ] Show framework type
  - [ ] Show creation date in human-readable format
  - [ ] Use consistent formatting with other commands (following architecture)
- [ ] Add user-friendly error handling (AC: All)
  - [ ] Use anyhow::Context for all file operations following Pattern 2
  - [ ] Handle missing `~/.zsh-profiles/` directory with suggestion to run `zprof init`
  - [ ] Handle malformed `config.toml` with clear error message
  - [ ] Handle case where active profile no longer exists (deleted)
- [ ] Write integration tests (AC: All)
  - [ ] Test displaying current active profile with all metadata
  - [ ] Test no active profile message displays correctly
  - [ ] Test missing config.toml handled gracefully
  - [ ] Test deleted active profile handled gracefully
  - [ ] Test performance meets < 100ms requirement
  - [ ] Add snapshot test for current profile output
  - [ ] Add snapshot test for no active profile message

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `cli/current.rs`, `core/config.rs`
- All modules must follow patterns defined in architecture.md Pattern 1 (CLI Command Structure)
- Error handling via anyhow::Result with context (Pattern 2)
- No backup operations needed (read-only command)

**TOML Schema Reference:**
According to architecture.md Pattern 4:

config.toml:
```toml
active_profile = "work"
default_framework = "oh-my-zsh"  # optional
```

profile.toml:
```toml
[profile]
name = "work"
framework = "oh-my-zsh"
theme = "robbyrussell"
created = "2025-10-31T14:30:00Z"
modified = "2025-10-31T14:30:00Z"
```

**Output Format Example:**
```
Current profile: work

Framework: oh-my-zsh
Created: Oct 31, 2025
```

If no active profile:
```
No active profile. Use 'zprof use <name>' to activate a profile.
```

If active profile was deleted:
```
✗ Error: Active profile 'work' not found (may have been deleted)
  Suggestion: Run 'zprof list' to see available profiles, then 'zprof use <name>' to activate one
```

**Error Handling:**
- Use `anyhow::Context` for all file operations
- Provide actionable error messages (what failed, why, how to fix)
- Never show raw Rust errors to users
- If `~/.zsh-profiles/` doesn't exist, suggest running `zprof init`

**Testing Strategy:**
- Integration tests in `tests/current_test.rs`
- Use insta for snapshot testing CLI output
- Test both success cases and edge cases (no active, deleted profile)

**Performance Target (NFR001, AC: #4):**
- Expected execution time: < 10ms (read two TOML files)
- This is the fastest command in zprof - just config read + profile read

### Project Structure Notes

**File Locations:**
- `src/cli/current.rs` - CLI command implementation
- `src/core/config.rs` - Config reading (may already exist from Story 1.1)
- `src/core/profile.rs` - Profile metadata loading (will be extended from Story 1.2)
- `tests/current_test.rs` - Integration tests
- `tests/snapshots/` - insta snapshot files

**Data Structures to Use:**
```rust
// In core/config.rs
pub struct Config {
    pub active_profile: Option<String>,
    pub default_framework: Option<String>,
}

// In core/profile.rs (may already exist from Story 1.2)
pub struct ProfileMetadata {
    pub name: String,
    pub framework: String,
    pub theme: String,
    pub created: String,  // ISO 8601 timestamp
    pub modified: String, // ISO 8601 timestamp
}
```

**Dependencies (should already be in Cargo.toml):**
```toml
[dependencies]
clap = { version = "4.5.51", features = ["derive"] }
anyhow = "2.0"
serde = { version = "1.0", features = ["derive"] }
toml = "0.9"
chrono = "latest"  # For parsing ISO 8601 dates to human-readable format
```

**Date Formatting:**
Use chrono to parse ISO 8601 timestamps and format as "Oct 31, 2025" for display.

### References

- [Source: docs/epics.md#Story-1.3]
- [Source: docs/PRD.md#FR004-display-current-profile]
- [Source: docs/architecture.md#Pattern-1-CLI-Command-Structure]
- [Source: docs/architecture.md#Pattern-2-Error-Handling]
- [Source: docs/architecture.md#Pattern-4-TOML-Manifest-Schema]
- [Source: docs/architecture.md#Error-Message-Format]
- [Source: docs/architecture.md#Epic-1-Story-1.3-Mapping]
- [Source: docs/architecture.md#Performance-Considerations-NFR001]

## Dev Agent Record

### Context Reference

- docs/stories/1-3-display-current-active-profile.context.xml

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
