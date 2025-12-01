# Story 4.8: Integrate Font Installation into Create Workflow

Status: ready-for-dev

## Story

As a user creating a profile with Starship,
I want automatic font installation offered,
so that my prompt works immediately.

## Acceptance Criteria

1. Modify `src/cli/create.rs` and `src/cli/create_wizard.rs`
2. Add `nerd_font` field to `Manifest` (`src/core/manifest.rs`)
3. Integration hook after prompt engine selection
4. Check if Nerd Font required and not detected
5. Show font selection TUI if needed
6. Download and install selected font
7. Display terminal configuration instructions
8. Handle skip gracefully with warning
9. Store font choice in manifest
10. Add integration test for full workflow

## Dev Agent Context

### Story Requirements from Epic

This story integrates the entire font installation workflow (Stories 4.1-4.7) into the profile creation process. It ensures users creating profiles with Nerd Font-dependent prompts get automatic font setup.

**Key User Flow (Full Integration):**
```
1. User runs: zprof create my-profile
2. CLI prompts for framework selection → User selects "oh-my-zsh"
3. CLI prompts for prompt engine → User selects "Starship"
4. [NEW] Check: Does Starship require Nerd Font? → YES
5. [NEW] Check: Is any Nerd Font installed? → NO
6. [NEW] Show font selection TUI (Story 4.4)
7. [NEW] User selects "FiraCode Nerd Font"
8. [NEW] Download font from GitHub (Story 4.5)
9. [NEW] Install font to ~/Library/Fonts/ (Story 4.6)
10. [NEW] Show terminal config instructions (Story 4.7)
11. [NEW] Wait for user acknowledgment
12. [NEW] Store font="FiraCode Nerd Font" in manifest
13. Continue with plugins, theme, etc.
14. Profile creation complete
```

**Skip Flow:**
```
1-6. [Same as above]
7. User selects "Skip font installation"
8. Show warning: "⚠ Your prompt may display boxes (□) or question marks (?)"
9. Store nerd_font_skipped=true in manifest
10. Continue with profile creation
```

**Expected Output:**
```
Creating profile: my-profile

Select a framework: oh-my-zsh
Select a prompt engine: Starship

Nerd Font Required
------------------
Starship uses icons and symbols that require a Nerd Font to display correctly.
We can automatically download and install a font for you.

[Font selection TUI displays]

Downloading FiraCode Nerd Font...
[Progress bar]

Installing fonts to ~/Library/Fonts/...
✓ Copied 12 font files
✓ Updated font cache
✓ Installation complete

Configure iTerm2 to use this font:
1. Open iTerm2 → Preferences → Profiles
[... full instructions ...]

Press Enter to continue...

[Continue with rest of profile creation]
```

### Architecture Compliance

**Modified Files:**
- `src/cli/create.rs` - Add `handle_font_installation()` hook
- `src/cli/create_wizard.rs` - Call hook after prompt engine selection (if using wizard)
- `src/core/manifest.rs` - Add `nerd_font: Option<String>` and `nerd_font_skipped: Option<bool>`

**Integration Point Location:**
```rust
// In create wizard, after prompt engine selection:

let selected_engine = prompt_engine_select::run()?;

// NEW: Font installation hook
if let Some(font_name) = handle_font_installation(&selected_engine)? {
    // Font installed, store in manifest
    wizard_state.nerd_font = Some(font_name);
} else {
    // Skipped or not required
    wizard_state.nerd_font = None;
}

// Continue with plugins, theme, etc.
```

**Manifest Schema Update:**
```toml
[profile]
name = "my-profile"
framework = "oh-my-zsh"
prompt_mode = "prompt_engine"
prompt_engine = "starship"
nerd_font = "FiraCode Nerd Font"        # NEW FIELD (optional)
nerd_font_skipped = false                # NEW FIELD (tracks if user skipped)
created = "2025-12-01T..."
modified = "2025-12-01T..."
```

**Error Handling Strategy:**
```rust
// If font installation fails, don't block profile creation
// Show error, offer skip option, continue

match handle_font_installation(&engine) {
    Ok(Some(font)) => {
        // Success
        manifest.nerd_font = Some(font);
    }
    Ok(None) => {
        // Skipped or not required
        manifest.nerd_font = None;
    }
    Err(e) => {
        // Installation error - don't fail entire profile creation
        eprintln!("⚠ Font installation failed: {}", e);
        eprintln!("  You can install fonts later with: zprof font install");
        manifest.nerd_font_skipped = Some(true);
    }
}
```

### Library and Framework Requirements

**Dependencies (all already in Cargo.toml):**
- No new dependencies needed
- Uses modules from Stories 4.1-4.7

**Required Imports:**
```rust
// In src/cli/create.rs or src/cli/create_wizard.rs

use crate::fonts::detector::detect_nerd_fonts;
use crate::fonts::download::download_font;
use crate::fonts::installer::install_font;
use crate::fonts::terminal_config::{Terminal, display_and_wait};
use crate::fonts::nerd_fonts::get_all_fonts;
use crate::prompts::engine::PromptEngine;
use crate::tui::font_select::{select_font, FontChoice};
```

### File Structure Requirements

**Modified Files:**
1. `src/cli/create.rs` - Add integration hook function
2. `src/cli/create_wizard.rs` - Call hook at appropriate point
3. `src/core/manifest.rs` - Update schema with font fields

**No New Files Required** - This story integrates existing modules

### Testing Requirements

**Integration Test** (`tests/create_with_font_test.rs` - NEW):
```rust
#[test]
fn test_create_profile_with_font_requirement() {
    // Setup: Create temp profile directory
    // Run: zprof create test-profile (simulated)
    // Select: Starship (requires font)
    // Verify: Font detection runs
    // Verify: TUI shows if no font installed
    // Mock: User selects FiraCode
    // Verify: Download called
    // Verify: Installation called
    // Verify: Manifest has nerd_font="FiraCode Nerd Font"
}

#[test]
fn test_create_profile_with_font_skip() {
    // Same as above but user selects Skip
    // Verify: Warning shown
    // Verify: Manifest has nerd_font_skipped=true
    // Verify: Profile creation continues successfully
}

#[test]
fn test_create_profile_no_font_required() {
    // Select Pure prompt (doesn't require fonts)
    // Verify: Font workflow NOT triggered
    // Verify: No font fields in manifest
}
```

### Previous Story Intelligence (Stories 4.1-4.7)

**What Previous Stories Completed:**
- **Story 4.1**: Font registry (`src/fonts/nerd_fonts.rs`) - 6 curated fonts
- **Story 4.2**: Prompt engine metadata - `requires_nerd_font: bool` field
- **Story 4.3**: Font detection (`src/fonts/detector.rs`) - Checks if Nerd Fonts installed
- **Story 4.4**: Font selection TUI (`src/tui/font_select.rs`) - User picks font
- **Story 4.5**: Font download (`src/fonts/download.rs`) - Downloads from GitHub
- **Story 4.6**: Font installation (`src/fonts/installer.rs`) - Installs to system
- **Story 4.7**: Terminal config (`src/fonts/terminal_config.rs`) - Shows instructions

**Integration Points:**
```rust
// 1. Check requirement (Story 4.2)
if !engine.requires_nerd_font() {
    return Ok(None); // No font needed
}

// 2. Check detection (Story 4.3)
if detect_nerd_fonts()?.is_installed() {
    return Ok(None); // Already have font
}

// 3. Show TUI (Story 4.4)
let font_choice = select_font()?;
match font_choice {
    FontChoice::Font(font) => {
        // 4. Download (Story 4.5)
        let download_result = download_font(font)?;

        // 5. Install (Story 4.6)
        let install_result = install_font(&download_result)?;

        if install_result.success {
            // 6. Show config (Story 4.7)
            let terminal = Terminal::detect();
            display_and_wait(font, &terminal);

            // 7. Return font name for manifest
            Ok(Some(font.name.to_string()))
        } else {
            bail!("Font installation failed");
        }
    }
    FontChoice::Skip => {
        eprintln!("⚠ Warning: Your prompt may not display correctly without Nerd Fonts");
        Ok(None)
    }
}
```

### Git Intelligence (Recent Commits)

**Code Patterns from Create Workflow:**
```rust
// Pattern from src/cli/create.rs:

pub fn execute(args: CreateArgs) -> Result<()> {
    // 1. Validation
    validate_profile_name(&args.name)?;

    // 2. Detection (existing framework)
    let detected = detect_existing_framework();

    // 3. Wizard flow
    let wizard_state = if args.preset.is_some() {
        create_from_preset()?
    } else {
        run_wizard()?
    };

    // 4. Installation
    let profile_path = create_profile_directory(&args.name)?;
    install_profile(&wizard_state, &profile_path)?;

    // 5. Manifest generation
    let manifest = generate_manifest(&wizard_state)?;
    save_manifest(&profile_path, &manifest)?;

    // 6. Success message
    println!("✓ Profile '{}' created successfully", args.name);

    Ok(())
}
```

### Latest Tech Information

**WizardState Enhancement:**
```rust
// In src/frameworks/installer.rs (WizardState)

#[derive(Debug, Clone)]
pub struct WizardState {
    pub profile_name: String,
    pub framework: FrameworkType,
    pub plugins: Vec<String>,
    pub theme: String,
    pub prompt_engine: Option<String>,
    pub nerd_font: Option<String>,           // NEW FIELD
    pub nerd_font_skipped: bool,             // NEW FIELD
}
```

**Manifest Field Addition:**
```rust
// In src/core/manifest.rs

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProfileSection {
    pub name: String,
    pub framework: String,
    #[serde(flatten)]
    pub prompt_mode: PromptMode,
    #[serde(default)]
    pub created: DateTime<Utc>,
    #[serde(default)]
    pub modified: DateTime<Utc>,

    // NEW FIELDS
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nerd_font: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nerd_font_skipped: Option<bool>,
}
```

### Implementation Guidance

**Font Installation Hook Function:**
```rust
// Add to src/cli/create.rs or src/cli/create_wizard.rs

/// Handle font installation after prompt engine selection
///
/// Checks if the selected prompt engine requires Nerd Fonts, and if so,
/// guides the user through font selection, download, and installation.
///
/// Returns:
/// - Ok(Some(font_name)) if font was installed
/// - Ok(None) if font not required, already installed, or user skipped
/// - Err(_) if installation failed and couldn't recover
fn handle_font_installation(engine: &PromptEngine) -> Result<Option<String>> {
    // Check if engine requires Nerd Font
    if !engine.requires_nerd_font {
        log::debug!("Prompt engine '{}' doesn't require Nerd Font, skipping", engine.name);
        return Ok(None);
    }

    // Check if Nerd Font already installed
    match detect_nerd_fonts()? {
        DetectionResult::Installed(_) => {
            log::info!("Nerd Font already installed, skipping installation");
            return Ok(None);
        }
        DetectionResult::NotInstalled => {
            log::info!("Nerd Font required but not installed");
        }
    }

    // Show informational message
    println!("\nNerd Font Required");
    println!("------------------");
    println!("{} uses icons and symbols that require a Nerd Font to display correctly.", engine.name);
    println!("We can automatically download and install a font for you.\n");

    // Show font selection TUI
    let font_choice = match font_select::select_font() {
        Ok(choice) => choice,
        Err(e) => {
            log::warn!("Font selection failed: {}", e);
            eprintln!("⚠ Font selection cancelled. You can install fonts later with: zprof font install");
            return Ok(None);
        }
    };

    match font_choice {
        FontChoice::Font(font) => {
            // Download font
            println!("\nDownloading {}...", font.name);
            let download_result = match download_font(font) {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("✗ Font download failed: {}", e);
                    eprintln!("  You can try again later with: zprof font install");
                    return Ok(None);
                }
            };

            // Install font
            let install_result = match install_font(&download_result) {
                Ok(result) => result,
                Err(e) => {
                    eprintln!("✗ Font installation failed: {}", e);
                    eprintln!("  You can try again later with: zprof font install");

                    // Cleanup download
                    let _ = fs::remove_dir_all(&download_result.temp_dir);

                    return Ok(None);
                }
            };

            // Cleanup download temp files
            if let Err(e) = fs::remove_dir_all(&download_result.temp_dir) {
                log::warn!("Failed to cleanup temp files: {}", e);
            }

            if !install_result.success {
                eprintln!("⚠ Font installation incomplete. {} of {} files installed.",
                          install_result.files_installed,
                          install_result.errors.len());

                if let Some(instructions) = install_result.manual_instructions {
                    println!("{}", instructions);
                }

                return Ok(None);
            }

            // Show terminal configuration instructions
            println!("\n✓ Font installed successfully!\n");
            let terminal = Terminal::detect();
            display_and_wait(font, &terminal);

            // Return font name for manifest
            Ok(Some(font.name.to_string()))
        }

        FontChoice::Skip => {
            eprintln!("\n⚠ Warning: Your prompt may display boxes (□) or question marks (?)");
            eprintln!("           instead of icons without a Nerd Font installed.");
            eprintln!("\n  To install fonts later, run: zprof font install\n");
            Ok(None)
        }
    }
}
```

**Integration in Wizard:**
```rust
// In src/cli/create_wizard.rs (or create.rs)

// After prompt engine selection:
let selected_engine = prompt_mode_select::run(&framework)?;

// Store engine in wizard state
let mut wizard_state = WizardState {
    profile_name: name.clone(),
    framework,
    prompt_engine: Some(selected_engine.name.to_string()),
    plugins: Vec::new(),
    theme: String::new(),
    nerd_font: None,              // Will be filled by font hook
    nerd_font_skipped: false,     // Will be set if user skips
};

// NEW: Font installation hook
match handle_font_installation(&selected_engine) {
    Ok(Some(font_name)) => {
        wizard_state.nerd_font = Some(font_name);
        wizard_state.nerd_font_skipped = false;
    }
    Ok(None) => {
        wizard_state.nerd_font = None;
        // Check if user explicitly skipped
        // (vs. font not required or already installed)
        // This requires FontChoice to be returned, or internal state tracking
    }
    Err(e) => {
        // Don't fail profile creation on font error
        log::error!("Font installation hook failed: {}", e);
        eprintln!("⚠ Font setup encountered an error. Continuing without font installation.");
        wizard_state.nerd_font_skipped = true;
    }
}

// Continue with plugins, theme, etc.
```

**Manifest Generation Update:**
```rust
// In src/core/manifest.rs or wherever manifest is built

let profile_section = ProfileSection {
    name: wizard_state.profile_name.clone(),
    framework: wizard_state.framework.id().to_string(),
    prompt_mode: if let Some(engine) = &wizard_state.prompt_engine {
        PromptMode::PromptEngine {
            engine: engine.clone(),
        }
    } else {
        PromptMode::FrameworkTheme {
            theme: wizard_state.theme.clone(),
        }
    },
    created: Utc::now(),
    modified: Utc::now(),

    // NEW: Font fields
    nerd_font: wizard_state.nerd_font.clone(),
    nerd_font_skipped: if wizard_state.nerd_font.is_none() && wizard_state.nerd_font_skipped {
        Some(true)
    } else {
        None
    },
};
```

### Critical Reminders

**DO:**
- ✅ Hook into create workflow AFTER prompt engine selection
- ✅ Check `engine.requires_nerd_font` before showing font UI
- ✅ Check if font already installed (don't re-prompt)
- ✅ Handle all three cases: install, skip, error
- ✅ Add `nerd_font` field to Manifest schema
- ✅ Add `nerd_font_skipped` field to Manifest schema
- ✅ Update WizardState with font fields
- ✅ Cleanup temp files after installation
- ✅ Don't block profile creation on font errors
- ✅ Show clear warnings when user skips
- ✅ Write integration test for full flow
- ✅ Test skip path explicitly

**DON'T:**
- ❌ Don't fail profile creation if font install fails
- ❌ Don't show font UI if engine doesn't require fonts
- ❌ Don't show font UI if fonts already installed
- ❌ Don't skip cleanup of temp download files
- ❌ Don't forget to wait for user acknowledgment after instructions
- ❌ Don't modify preset workflow (separate concern, can add later)
- ❌ Don't change manifest format in breaking way (use Option fields)
- ❌ Don't skip error handling in hook function
- ❌ Don't test only happy path (test skip and error cases)

### Acceptance Criteria Expanded

1. **Modify `src/cli/create.rs` and `src/cli/create_wizard.rs`**
   - Add `handle_font_installation()` function to create.rs or create_wizard.rs
   - Call hook after prompt engine selection
   - Pass `PromptEngine` to hook
   - Store returned font name in wizard state

2. **Add `nerd_font` field to Manifest**
   - Update `ProfileSection` struct in `src/core/manifest.rs`
   - Add `nerd_font: Option<String>` field
   - Add `nerd_font_skipped: Option<bool>` field
   - Both fields serialize as TOML (skip if None)
   - Update serialization/deserialization

3. **Integration hook after prompt engine selection**
   - Hook runs after prompt mode/engine selection
   - Before plugins/theme selection
   - Returns `Result<Option<String>>` (font name or None)

4. **Check if Nerd Font required and not detected**
   - Check `engine.requires_nerd_font` boolean
   - Call `detect_nerd_fonts()`
   - Skip entire flow if not required or already installed

5. **Show font selection TUI if needed**
   - Call `font_select::select_font()`
   - Display informational message before TUI
   - Handle Esc/cancellation gracefully

6. **Download and install selected font**
   - Call `download_font(selected_font)`
   - Call `install_font(&download_result)`
   - Show progress during download
   - Cleanup temp files after installation

7. **Display terminal configuration instructions**
   - Call `Terminal::detect()`
   - Call `display_and_wait(font, terminal)`
   - Wait for user Enter before continuing

8. **Handle skip gracefully with warning**
   - Detect `FontChoice::Skip` from TUI
   - Show warning message about broken icons
   - Suggest `zprof font install` command
   - Continue profile creation normally

9. **Store font choice in manifest**
   - Set `wizard_state.nerd_font = Some(font_name)` if installed
   - Set `wizard_state.nerd_font = None` if skipped/not required
   - Set `wizard_state.nerd_font_skipped = true` if user explicitly skipped
   - Serialize to `profile.toml`

10. **Integration test for full workflow**
    - Test create with Starship (requires font)
    - Test create with Pure (doesn't require font)
    - Test create with skip
    - Test manifest has correct font fields
    - File: `tests/create_with_font_test.rs`

## Tasks / Subtasks

- [ ] Update `src/core/manifest.rs` (AC: 2)
  - [ ] Add `nerd_font: Option<String>` to ProfileSection
  - [ ] Add `nerd_font_skipped: Option<bool>` to ProfileSection
  - [ ] Update Deserialize implementation if needed
  - [ ] Test serialization/deserialization
- [ ] Update `WizardState` (AC: 1, 9)
  - [ ] Add `nerd_font: Option<String>` field
  - [ ] Add `nerd_font_skipped: bool` field
  - [ ] Update constructor/builder
- [ ] Implement `handle_font_installation()` (AC: 3, 4, 5, 6, 7, 8)
  - [ ] Check if font required
  - [ ] Check if font already installed
  - [ ] Show informational message
  - [ ] Call font selection TUI
  - [ ] Handle Font choice: download + install
  - [ ] Handle Skip choice: warning message
  - [ ] Cleanup temp files
  - [ ] Return font name or None
- [ ] Integrate hook into wizard (AC: 1, 3)
  - [ ] Find insertion point (after prompt engine selection)
  - [ ] Call `handle_font_installation(&engine)`
  - [ ] Store result in wizard_state
  - [ ] Handle errors without failing profile creation
- [ ] Update manifest generation (AC: 9)
  - [ ] Include `nerd_font` field from wizard_state
  - [ ] Include `nerd_font_skipped` field from wizard_state
  - [ ] Verify TOML serialization
- [ ] Write integration tests (AC: 10)
  - [ ] Create `tests/create_with_font_test.rs`
  - [ ] Test with font-requiring engine
  - [ ] Test with non-requiring engine
  - [ ] Test skip flow
  - [ ] Test manifest fields
- [ ] Test error handling (AC: 8)
  - [ ] Test download failure
  - [ ] Test install failure
  - [ ] Verify profile creation continues
  - [ ] Verify warning messages shown
- [ ] Documentation (AC: all)
  - [ ] Function docs for `handle_font_installation()`
  - [ ] Update WizardState docs
  - [ ] Update Manifest schema docs

## Dev Agent Record

### Context Reference

Epic: [epic-4-nerd-fonts.md](../epic-4-nerd-fonts.md)
Tech Spec: [tech-spec-epic-4.md](../../tech-spec-epic-4.md)
Previous Stories: Stories 4.1-4.7 (Font system complete)
Create Workflow: [src/cli/create.rs](../../../src/cli/create.rs)
Wizard: [src/cli/create_wizard.rs](../../../src/cli/create_wizard.rs)
Manifest: [src/core/manifest.rs](../../../src/core/manifest.rs)

### Agent Model Used

<!-- Will be filled during implementation -->

### Debug Log References

<!-- Will be filled during implementation -->

### Completion Notes List

<!-- Will be filled during implementation -->

### File List

<!-- Will be filled during implementation -->
