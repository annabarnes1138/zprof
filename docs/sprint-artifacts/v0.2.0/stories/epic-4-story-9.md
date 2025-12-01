# Story 4.9: Add Font Management Command

Status: ready-for-dev

## Story

As a user,
I want to install or change fonts later,
so that I'm not locked into my initial choice.

## Acceptance Criteria

1. Create `src/cli/font.rs` module
2. Implement `zprof font list` - Show installed Nerd Fonts
3. Implement `zprof font install` - Install a new Nerd Font
4. Implement `zprof font info <name>` - Show terminal config for a font
5. Register in CLI router (`src/cli/mod.rs`, `src/main.rs`)
6. List shows installed fonts + which profiles use them
7. Install launches font selection TUI (reuse Story 4.4)
8. Info shows terminal configuration instructions
9. Add integration tests for font commands

## Dev Agent Context

### Story Requirements from Epic

This story creates standalone CLI commands for font management, allowing users to install, inspect, and configure fonts outside the profile creation workflow. This is useful for users who skipped font installation initially or want to try different fonts.

**Key User Flows:**

**Font List:**
```
$ zprof font list

Installed Nerd Fonts:

✓ FiraCode Nerd Font
  ~/Library/Fonts/FiraCodeNerdFont-Regular.ttf (+ 11 more files)
  Used by: work, personal

✓ JetBrainsMono Nerd Font
  ~/Library/Fonts/JetBrainsMonoNerdFont-Regular.ttf (+ 7 more files)
  (Not used by any profile)

No Nerd Fonts installed.
→ Install with: zprof font install
```

**Font Install:**
```
$ zprof font install

Checking for existing Nerd Fonts...
Found: FiraCode Nerd Font

[Font selection TUI displays]

User selects: JetBrainsMono Nerd Font

Downloading JetBrainsMono Nerd Font...
[Progress bar]

Installing fonts to ~/Library/Fonts/...
✓ Copied 8 font files
✓ Updated font cache
✓ Installation complete

Configure iTerm2 to use this font:
[... full instructions ...]

Press Enter to continue...
```

**Font Info:**
```
$ zprof font info FiraCode

FiraCode Nerd Font
------------------
Description: Programming ligatures, clean and modern
File Format: TrueType (.ttf)
Recommended for: Starship, Oh-My-Posh

Installation Status: ✓ Installed
  Location: ~/Library/Fonts/
  Files: 12

Terminal Configuration:
[... terminal-specific instructions ...]

Download URL: https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/FiraCode.zip
```

### Architecture Compliance

**New Module:** `src/cli/font.rs`
- Follows existing CLI command pattern (like `create.rs`, `delete.rs`)
- Subcommand structure using clap derive API
- Integrates with all font modules (detector, installer, terminal_config, nerd_fonts)

**CLI Router Integration:**
```rust
// src/main.rs (or src/cli/mod.rs)

#[derive(Parser)]
enum Commands {
    Create(create::CreateArgs),
    Use(use_cmd::UseArgs),
    List(list::ListArgs),
    Delete(delete::DeleteArgs),
    Font(font::FontArgs),           // NEW COMMAND
}

match &cli.command {
    Commands::Create(args) => create::execute(args)?,
    Commands::Font(args) => font::execute(args)?,  // NEW HANDLER
    // ... other commands
}
```

**Public API Design:**
```rust
// src/cli/font.rs

use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct FontArgs {
    #[command(subcommand)]
    pub command: FontCommand,
}

#[derive(Debug, Subcommand)]
pub enum FontCommand {
    /// List installed Nerd Fonts
    List,

    /// Install a new Nerd Font
    Install,

    /// Show information and configuration for a specific font
    Info {
        /// Font name (e.g., "FiraCode", "JetBrainsMono")
        font_name: String,
    },
}

pub fn execute(args: &FontArgs) -> Result<()> {
    match &args.command {
        FontCommand::List => list_fonts(),
        FontCommand::Install => install_font_interactive(),
        FontCommand::Info { font_name } => show_font_info(font_name),
    }
}
```

### Library and Framework Requirements

**Dependencies (all already in Cargo.toml):**
- `clap` - CLI parsing with subcommands
- `anyhow` - Error handling
- All font modules from Stories 4.1-4.7

**Required Imports:**
```rust
use anyhow::{bail, Context, Result};
use clap::{Args, Subcommand};

use crate::core::config::Config;
use crate::core::filesystem::get_zprof_dir;
use crate::core::manifest::Manifest;
use crate::fonts::detector::{detect_nerd_fonts, DetectionResult, list_installed_fonts};
use crate::fonts::download::download_font;
use crate::fonts::installer::install_font;
use crate::fonts::terminal_config::{Terminal, generate_instructions};
use crate::fonts::nerd_fonts::{get_all_fonts, get_font_by_id, NerdFont};
use crate::tui::font_select::{select_font, FontChoice};
```

### File Structure Requirements

**New File:** `src/cli/font.rs`

**Module Export:** Add to `src/cli/mod.rs`:
```rust
pub mod font;
```

**CLI Registration:** Update `src/main.rs` or `src/cli/mod.rs`:
```rust
// Add to Commands enum
Font(font::FontArgs),

// Add to match statement
Commands::Font(args) => font::execute(args)?,
```

### Testing Requirements

**Integration Tests** (`tests/font_commands_test.rs` - NEW):
```rust
#[test]
fn test_font_list_no_fonts() {
    // Setup: Clean font directory (mock or tempdir)
    // Run: zprof font list
    // Verify: Output shows "No Nerd Fonts installed"
    // Verify: Exit code 0
}

#[test]
fn test_font_list_with_fonts() {
    // Setup: Install mock fonts to temp directory
    // Run: zprof font list
    // Verify: Output shows font names
    // Verify: Shows file paths
    // Verify: Shows profile usage (if applicable)
}

#[test]
fn test_font_install_flow() {
    // Run: zprof font install (simulated TUI interaction)
    // Mock: User selects FiraCode
    // Verify: Download called
    // Verify: Installation called
    // Verify: Success message shown
}

#[test]
fn test_font_info_existing() {
    // Run: zprof font info FiraCode
    // Verify: Font details shown
    // Verify: Installation status shown
    // Verify: Terminal config shown
}

#[test]
fn test_font_info_unknown() {
    // Run: zprof font info NonExistent
    // Verify: Error message shown
    // Verify: Exit code non-zero
}
```

### Previous Story Intelligence (Stories 4.1-4.8)

**What Previous Stories Completed:**
All font infrastructure is complete:
- Font registry (Story 4.1)
- Font detection (Story 4.3)
- Font selection TUI (Story 4.4)
- Font download (Story 4.5)
- Font installation (Story 4.6)
- Terminal config (Story 4.7)
- Create workflow integration (Story 4.8)

**Reuse Opportunities:**
```rust
// List uses detector module:
let installed = list_installed_fonts()?;

// Install reuses entire workflow from Story 4.8:
let font = select_font()?;
let download_result = download_font(font)?;
let install_result = install_font(&download_result)?;

// Info uses registry + terminal_config:
let font = get_font_by_id(name)?;
let instructions = generate_instructions(font, &Terminal::detect());
```

### Implementation Guidance

**Font List Command:**
```rust
fn list_fonts() -> Result<()> {
    println!("Checking installed Nerd Fonts...\n");

    // Detect installed fonts
    let installed = match detect_nerd_fonts()? {
        DetectionResult::Installed(fonts) => fonts,
        DetectionResult::NotInstalled => {
            println!("No Nerd Fonts installed.");
            println!("→ Install with: zprof font install");
            return Ok(());
        }
    };

    println!("Installed Nerd Fonts:\n");

    // Load profiles to check usage
    let config = Config::load()?;
    let profiles = load_all_profiles(&config)?;

    for font in installed {
        println!("✓ {}", font.name);
        println!("  {}", font.path.display());

        // Show which profiles use this font
        let using_profiles: Vec<&str> = profiles
            .iter()
            .filter(|(_, manifest)| {
                manifest.profile.nerd_font
                    .as_ref()
                    .map(|f| f.contains(&font.name))
                    .unwrap_or(false)
            })
            .map(|(name, _)| name.as_str())
            .collect();

        if !using_profiles.is_empty() {
            println!("  Used by: {}", using_profiles.join(", "));
        } else {
            println!("  (Not used by any profile)");
        }

        println!();
    }

    Ok(())
}
```

**Font Install Command:**
```rust
fn install_font_interactive() -> Result<()> {
    println!("Checking for existing Nerd Fonts...\n");

    // Show already installed fonts
    if let DetectionResult::Installed(fonts) = detect_nerd_fonts()? {
        println!("Already installed:");
        for font in &fonts {
            println!("  ✓ {}", font.name);
        }
        println!();
    }

    // Show font selection TUI
    let font_choice = select_font()?;

    match font_choice {
        FontChoice::Font(font) => {
            // Reuse workflow from Story 4.8
            println!("\nDownloading {}...", font.name);
            let download_result = download_font(font)?;

            let install_result = install_font(&download_result)?;

            // Cleanup
            let _ = std::fs::remove_dir_all(&download_result.temp_dir);

            if !install_result.success {
                bail!("Font installation failed");
            }

            println!("\n✓ Font installed successfully!\n");

            // Show terminal config
            let terminal = Terminal::detect();
            let instructions = generate_instructions(font, &terminal);
            println!("{}", instructions);

            print!("\nPress Enter to continue...");
            std::io::stdout().flush()?;
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            Ok(())
        }

        FontChoice::Skip => {
            println!("Installation cancelled.");
            Ok(())
        }
    }
}
```

**Font Info Command:**
```rust
fn show_font_info(font_name: &str) -> Result<()> {
    // Look up font in registry (case-insensitive)
    let font = get_font_by_id(&font_name.to_lowercase())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Font '{}' not found in registry.\n\nAvailable fonts:\n{}",
                font_name,
                get_all_fonts()
                    .iter()
                    .map(|f| format!("  - {}", f.name))
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        })?;

    // Display font information
    println!("{}", font.name);
    println!("{}", "-".repeat(font.name.len()));
    println!("Description: {}", font.description);
    println!("File Format: {}", match font.file_format {
        FontFormat::TrueType => "TrueType (.ttf)",
        FontFormat::OpenType => "OpenType (.otf)",
    });

    if font.recommended {
        println!("Recommended: Yes");
    }

    if !font.recommended_for.is_empty() {
        let engines: Vec<&str> = font.recommended_for
            .iter()
            .map(|e| e.name())
            .collect();
        println!("Recommended for: {}", engines.join(", "));
    }

    println!();

    // Check installation status
    match detect_nerd_fonts()? {
        DetectionResult::Installed(fonts) => {
            if fonts.iter().any(|f| f.name.contains(font.name)) {
                println!("Installation Status: ✓ Installed");

                // Find exact match to show details
                if let Some(installed) = fonts.iter().find(|f| f.name.contains(font.name)) {
                    println!("  Location: {}", installed.path.parent().unwrap().display());
                    println!("  Files: {}", installed.file_count);
                }
            } else {
                println!("Installation Status: ✗ Not installed");
            }
        }
        DetectionResult::NotInstalled => {
            println!("Installation Status: ✗ Not installed");
        }
    }

    println!();

    // Show terminal configuration
    println!("Terminal Configuration:");
    let terminal = Terminal::detect();
    let instructions = generate_instructions(font, &terminal);
    println!("{}", instructions);

    println!();
    println!("Download URL: {}", font.download_url);

    Ok(())
}
```

**Helper: Load All Profiles:**
```rust
fn load_all_profiles(config: &Config) -> Result<Vec<(String, Manifest)>> {
    let profiles_dir = get_zprof_dir()?.join("profiles");

    if !profiles_dir.exists() {
        return Ok(Vec::new());
    }

    let mut profiles = Vec::new();

    for entry in std::fs::read_dir(&profiles_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let profile_name = path.file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string());

        let manifest_path = path.join("profile.toml");
        if !manifest_path.exists() {
            continue;
        }

        if let (Some(name), Ok(manifest)) = (profile_name, Manifest::load(&manifest_path)) {
            profiles.push((name, manifest));
        }
    }

    Ok(profiles)
}
```

### Critical Reminders

**DO:**
- ✅ Create `src/cli/font.rs` with subcommands
- ✅ Register in CLI router (`main.rs` and `cli/mod.rs`)
- ✅ Reuse existing font modules (don't duplicate logic)
- ✅ Show profile usage in `font list`
- ✅ Handle font not found in `font info`
- ✅ Use same TUI for `font install` as create workflow
- ✅ Show terminal config in `font info`
- ✅ Write integration tests for all commands
- ✅ Follow existing CLI command patterns
- ✅ Handle no fonts installed gracefully

**DON'T:**
- ❌ Don't duplicate download/install logic (reuse)
- ❌ Don't skip profile usage detection in list
- ❌ Don't hardcode font names (use registry)
- ❌ Don't forget to register in CLI router
- ❌ Don't skip error handling (font not found, etc.)
- ❌ Don't implement font uninstall (out of scope for v0.2.0)
- ❌ Don't modify existing CLI commands
- ❌ Don't forget help text for subcommands

### Acceptance Criteria Expanded

1. **Create `src/cli/font.rs` module**
   - New file with `FontArgs` and `FontCommand` structs
   - `execute()` function dispatches to subcommands
   - Export via `src/cli/mod.rs`

2. **Implement `zprof font list`**
   - Detect installed fonts using `detect_nerd_fonts()`
   - Show font name, location, file count
   - Load all profiles and check which use each font
   - Display "Used by: profile1, profile2" or "(Not used)"
   - Handle no fonts installed: show helpful message

3. **Implement `zprof font install`**
   - Show already installed fonts (if any)
   - Launch font selection TUI (reuse `font_select::select_font()`)
   - Download selected font (reuse `download_font()`)
   - Install font (reuse `install_font()`)
   - Show terminal configuration instructions
   - Wait for user acknowledgment

4. **Implement `zprof font info <name>`**
   - Look up font in registry by ID (case-insensitive)
   - Show font metadata (name, description, format, recommended)
   - Check and show installation status
   - Generate and show terminal configuration
   - Show download URL
   - Error if font not in registry

5. **Register in CLI router**
   - Add `Font(font::FontArgs)` to Commands enum in `main.rs`
   - Add `Commands::Font(args) => font::execute(args)?` to match
   - Add `pub mod font;` to `src/cli/mod.rs`

6. **List shows installed fonts + profile usage**
   - Iterate over detected fonts
   - For each font, load all profiles
   - Check if `profile.nerd_font` contains font name
   - Display profile names that use font

7. **Install launches TUI**
   - Call `font_select::select_font()?`
   - Same TUI as create workflow (Story 4.4)
   - Handle Skip gracefully

8. **Info shows terminal configuration**
   - Detect terminal with `Terminal::detect()`
   - Generate instructions with `generate_instructions()`
   - Display formatted instructions

9. **Integration tests**
   - Test `font list` with/without fonts
   - Test `font install` workflow
   - Test `font info` with valid/invalid names
   - File: `tests/font_commands_test.rs`

## Tasks / Subtasks

- [ ] Create `src/cli/font.rs` (AC: 1)
  - [ ] Module-level documentation
  - [ ] Define `FontArgs` struct with clap derive
  - [ ] Define `FontCommand` enum with subcommands
  - [ ] Implement `execute()` dispatcher
  - [ ] Import all required dependencies
- [ ] Implement `list_fonts()` (AC: 2, 6)
  - [ ] Call `detect_nerd_fonts()`
  - [ ] Handle no fonts installed
  - [ ] Load all profiles from config
  - [ ] Check profile usage for each font
  - [ ] Display font details + usage
- [ ] Implement `install_font_interactive()` (AC: 3, 7)
  - [ ] Show already installed fonts
  - [ ] Launch font selection TUI
  - [ ] Download selected font
  - [ ] Install font
  - [ ] Cleanup temp files
  - [ ] Show terminal config
  - [ ] Wait for acknowledgment
- [ ] Implement `show_font_info()` (AC: 4, 8)
  - [ ] Look up font in registry
  - [ ] Display font metadata
  - [ ] Check installation status
  - [ ] Generate terminal configuration
  - [ ] Show download URL
  - [ ] Error handling for unknown fonts
- [ ] Implement `load_all_profiles()` helper (AC: 6)
  - [ ] Read profiles directory
  - [ ] Load each profile.toml
  - [ ] Return Vec of (name, manifest) tuples
- [ ] Register CLI commands (AC: 5)
  - [ ] Add `pub mod font;` to `src/cli/mod.rs`
  - [ ] Add `Font` variant to Commands enum
  - [ ] Add match arm in CLI dispatcher
  - [ ] Update help text
- [ ] Write integration tests (AC: 9)
  - [ ] Create `tests/font_commands_test.rs`
  - [ ] Test `font list` (empty and with fonts)
  - [ ] Test `font install` flow
  - [ ] Test `font info` (valid and invalid)
- [ ] Documentation (AC: all)
  - [ ] Module-level docs
  - [ ] Function docs for each command
  - [ ] Help text for clap (auto-generated from docs)

## Dev Agent Record

### Context Reference

Epic: [epic-4-nerd-fonts.md](../epic-4-nerd-fonts.md)
Tech Spec: [tech-spec-epic-4.md](../../tech-spec-epic-4.md)
Previous Story: [epic-4-story-8.md](epic-4-story-8.md) (Create Workflow Integration)
CLI Pattern Reference: [src/cli/create.rs](../../../src/cli/create.rs)
Font Modules: All completed in Stories 4.1-4.7

### Agent Model Used

<!-- Will be filled during implementation -->

### Debug Log References

<!-- Will be filled during implementation -->

### Completion Notes List

<!-- Will be filled during implementation -->

### File List

<!-- Will be filled during implementation -->
