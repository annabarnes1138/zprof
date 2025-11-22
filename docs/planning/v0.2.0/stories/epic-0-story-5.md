# Story 0.5: Ensure CLI Compatibility

**Epic:** Epic 0 - GUI Foundation
**Priority:** P0 (Critical)
**Status:** ready-for-dev

## Dev Agent Record

**Context Reference:**
- [epic-0-story-5.context.xml](epic-0-story-5.context.xml)

## User Story

**As a** developer
**I want** all existing CLI commands to work without regression
**So that** users can choose between GUI and CLI

## Acceptance Criteria

- [ ] Verify all CLI commands still work correctly:
  - `zprof init` - Initializes zprof
  - `zprof create <name>` - Creates profile
  - `zprof list` - Lists profiles
  - `zprof use <name>` - Switches profiles
  - `zprof delete <name>` - Deletes profile
  - `zprof show <name>` - Shows profile details
  - `zprof current` - Shows active profile
  - All other existing commands
- [ ] Add CLI integration tests (if not already comprehensive):
  - Test each command in isolation
  - Test with GUI dependencies present
  - Test with GUI running in background
  - Test with GUI closed
- [ ] Ensure no dependency conflicts:
  - CLI binary size doesn't bloat from GUI deps
  - CLI startup time remains fast (<100ms)
  - GUI dependencies are optional at compile time
- [ ] Add feature flags in `Cargo.toml`:
  - `gui` feature (default enabled)
  - CLI compiles without GUI if feature disabled
  - `cargo build --no-default-features` works for CLI-only
- [ ] Add `zprof gui` command:
  - Launch GUI application from CLI
  - `zprof gui --help` shows GUI-specific options
  - `zprof gui --version` shows version info
  - Command available: `zprof gui` (no args needed)
- [ ] Update help text:
  - Mention GUI availability in `zprof --help`
  - Add "GUI" section to command list:
    ```
    GUI Commands:
      gui         Launch the graphical interface
    ```
  - Document keyboard shortcuts in help
- [ ] Add E2E integration test suite:
  - **CLI creates profile → GUI displays it**
    - Run `zprof create test-profile`
    - Launch GUI via IPC
    - Call `list_profiles()` command
    - Verify "test-profile" appears in list
  - **GUI creates profile → CLI can use it**
    - Call `create_profile()` via IPC
    - Run `zprof list` command
    - Verify profile appears in CLI output
  - **CLI activates profile → GUI shows active badge**
    - Run `zprof use test-profile`
    - Call `get_active_profile()` via IPC
    - Verify returns "test-profile"
    - Call `list_profiles()` via IPC
    - Verify "test-profile" has `active: true`
  - **GUI deletes profile → CLI doesn't see it**
    - Call `delete_profile("test-profile")` via IPC
    - Run `zprof list` command
    - Verify profile not in output
- [ ] Document build process:
  - How to build GUI version: `cargo tauri build`
  - How to build CLI-only version: `cargo build --no-default-features`
  - Platform-specific notes:
    - macOS: Produces `.app` bundle + DMG
    - Linux: Produces `.deb`, `.appimage`
- [ ] Update README.md with dual interface documentation:
  - Explain CLI and GUI are complementary
  - Show how to launch GUI: `zprof gui`
  - Note that CLI remains fully functional
  - Link to GUI-specific documentation

## Technical Details

### Feature Flag Configuration

```toml
# Cargo.toml (root)

[features]
default = ["gui"]
gui = ["tauri-build"]

[dependencies]
# CLI dependencies (always included)
clap = { version = "4.5", features = ["derive"] }
anyhow = "2.0"
# ... other CLI deps

# GUI dependencies (optional)
[target.'cfg(feature = "gui")'.dependencies]
tauri-build = { version = "2.0", optional = true }

[build-dependencies]
# Only include tauri-build if GUI feature enabled
tauri-build = { version = "2.0", optional = true }
```

### GUI Launch Command

```rust
// src/cli/gui.rs

use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct GuiArgs {
    // Future: Add GUI-specific options (e.g., --profile <name> to open specific profile)
}

#[cfg(feature = "gui")]
pub fn execute(_args: GuiArgs) -> Result<()> {
    // Launch Tauri application
    println!("Launching zprof GUI...");

    // Tauri handles the GUI process
    // This will block until GUI is closed
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}

#[cfg(not(feature = "gui"))]
pub fn execute(_args: GuiArgs) -> Result<()> {
    eprintln!("Error: GUI not available in this build");
    eprintln!("This zprof binary was compiled without GUI support.");
    eprintln!("To use the GUI, install the full version or build with `cargo build`");
    std::process::exit(1);
}
```

### CLI/GUI Interop Test

```rust
// tests/cli_gui_interop_test.rs

use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_cli_creates_gui_sees() {
    let temp_dir = TempDir::new().unwrap();
    let home = temp_dir.path();

    // Set HOME for test
    std::env::set_var("HOME", home);

    // Initialize zprof
    let output = Command::new("cargo")
        .args(&["run", "--", "init"])
        .output()
        .expect("Failed to run zprof init");
    assert!(output.status.success());

    // Create profile via CLI
    let output = Command::new("cargo")
        .args(&["run", "--", "create", "cli-test"])
        .output()
        .expect("Failed to run zprof create");
    assert!(output.status.success());

    // Verify via GUI IPC command
    let profiles = crate::commands::list_profiles().unwrap();
    assert_eq!(profiles.len(), 1);
    assert_eq!(profiles[0].name, "cli-test");
}

#[test]
fn test_gui_creates_cli_sees() {
    let temp_dir = TempDir::new().unwrap();
    let home = temp_dir.path();
    std::env::set_var("HOME", home);

    // Initialize zprof
    Command::new("cargo")
        .args(&["run", "--", "init"])
        .output()
        .expect("Failed to run zprof init");

    // Create profile via GUI IPC
    let config = crate::types::ProfileConfig {
        name: "gui-test".to_string(),
        framework: "oh-my-zsh".to_string(),
        prompt_mode: "framework_theme".to_string(),
        prompt_engine: None,
        framework_theme: Some("robbyrussell".to_string()),
        plugins: vec![],
        env_vars: std::collections::HashMap::new(),
    };

    crate::commands::create_profile(config).unwrap();

    // Verify via CLI
    let output = Command::new("cargo")
        .args(&["run", "--", "list"])
        .output()
        .expect("Failed to run zprof list");
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("gui-test"));
}

#[test]
fn test_activate_sync() {
    let temp_dir = TempDir::new().unwrap();
    let home = temp_dir.path();
    std::env::set_var("HOME", home);

    // Setup: init + create 2 profiles
    Command::new("cargo")
        .args(&["run", "--", "init"])
        .output()
        .unwrap();

    Command::new("cargo")
        .args(&["run", "--", "create", "profile1"])
        .output()
        .unwrap();

    Command::new("cargo")
        .args(&["run", "--", "create", "profile2"])
        .output()
        .unwrap();

    // Activate via CLI
    let output = Command::new("cargo")
        .args(&["run", "--", "use", "profile2"])
        .output()
        .expect("Failed to run zprof use");
    assert!(output.status.success());

    // Verify via GUI IPC
    let active = crate::commands::get_active_profile().unwrap();
    assert_eq!(active, Some("profile2".to_string()));

    let profiles = crate::commands::list_profiles().unwrap();
    let profile2 = profiles.iter().find(|p| p.name == "profile2").unwrap();
    assert!(profile2.active);
}
```

### Updated Help Text

```rust
// src/main.rs

#[derive(Parser)]
#[command(
    name = "zprof",
    about = "Manage multiple zsh configurations with ease",
    long_about = "zprof allows you to manage multiple isolated zsh profiles.
Switch between configurations instantly, experiment safely, and share profiles.

Available via CLI (this tool) or GUI (run 'zprof gui' to launch graphical interface)."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize zprof in your system
    Init(InitArgs),

    /// Create a new profile
    Create(CreateArgs),

    /// Switch to a different profile
    Use(UseArgs),

    /// List all profiles
    List(ListArgs),

    /// Show active profile
    Current,

    /// Delete a profile
    Delete(DeleteArgs),

    // ... other commands

    #[cfg(feature = "gui")]
    /// Launch the graphical user interface
    Gui(GuiArgs),
}
```

## Files Created/Modified

**New Files:**
- `src/cli/gui.rs`
- `tests/cli_gui_interop_test.rs`

**Modified Files:**
- `Cargo.toml` (add feature flags)
- `src/main.rs` (add `gui` command, update help)
- `src/cli/mod.rs` (export `gui` module)
- `README.md` (document GUI availability)
- `docs/user-guide/quick-start.md` (mention GUI option)

## Dependencies

- **Blocks:** All other Epic 0 stories (this is validation)
- **Requires:** Existing CLI commands, GUI IPC layer

## Testing

**Automated Tests:**
- Run `cargo test` - All existing tests pass
- Run `cargo test cli_gui_interop` - New interop tests pass
- Run `cargo build --no-default-features` - CLI-only build succeeds
- Run `cargo build` - Full build with GUI succeeds

**Manual Verification:**

1. **CLI Regression:**
   ```bash
   cargo run -- init
   cargo run -- create test1
   cargo run -- list
   cargo run -- use test1
   cargo run -- current
   cargo run -- delete test1
   ```
   All should work as before

2. **GUI Launch:**
   ```bash
   cargo run -- gui
   ```
   GUI window should open

3. **CLI-only Build:**
   ```bash
   cargo build --no-default-features --release
   ./target/release/zprof --help
   ./target/release/zprof gui
   # Should show error: "GUI not available in this build"
   ```

4. **Binary Size:**
   - CLI-only: `ls -lh target/release/zprof` (should be ~5-10MB)
   - Full with GUI: Size increases but acceptable (<50MB)

5. **Startup Performance:**
   ```bash
   time cargo run -- --version
   # Should be <100ms (cold), <50ms (warm)
   ```

**Success Criteria:**
- [ ] All existing CLI tests pass
- [ ] All new interop tests pass
- [ ] CLI-only build works
- [ ] Full build works
- [ ] GUI launches from CLI
- [ ] No performance regression
- [ ] Documentation updated

## Notes

- Feature flags allow optional GUI compilation
- CLI remains the primary interface for automation/scripting
- GUI is an enhancement, not a replacement
- Both interfaces share 100% of business logic (zero duplication)
- Integration tests ensure CLI and GUI stay in sync

## References

- Architecture Doc: [docs/developer/architecture.md](../../../developer/architecture.md) (Section: Dual Interface Architecture)
- Epic 0: [docs/planning/v0.2.0/epic-0-gui-foundation.md](../epic-0-gui-foundation.md) (Story 0.5)
- Cargo Features: https://doc.rust-lang.org/cargo/reference/features.html
