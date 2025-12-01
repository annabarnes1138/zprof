# Story 4.7: Generate Terminal Configuration Instructions

Status: ready-for-dev

## Story

As a user who installed a Nerd Font,
I want clear instructions to configure my terminal,
so that the font actually displays correctly.

## Acceptance Criteria

1. Create `src/fonts/terminal_config.rs`
2. Detect user's terminal emulator from `$TERM_PROGRAM` env var
3. Support major terminals (iTerm2, Terminal.app, VS Code, Alacritty, Kitty, GNOME Terminal, Konsole)
4. Generate terminal-specific step-by-step instructions
5. Show exact font name to use in settings
6. Include troubleshooting tips
7. Display instructions after installation with formatting
8. Add unit tests for terminal detection and instruction generation

## Dev Agent Context

### Story Requirements from Epic

This story generates terminal-specific configuration instructions after successful font installation (Story 4.6). It must detect the user's terminal, provide clear step-by-step instructions, and include troubleshooting guidance.

**Key User Flow:**
1. Font installation completes successfully (Story 4.6)
2. Detect terminal via `$TERM_PROGRAM` environment variable
3. Generate terminal-specific instructions for the installed font
4. Display formatted instructions to user
5. Wait for user acknowledgment before continuing
6. Store instructions in manifest for later reference

**Expected Output Format:**
```
✓ FiraCode Nerd Font installed successfully!

Configure your terminal to use this font:

iTerm2:
1. Open iTerm2 → Preferences → Profiles
2. Select your profile → Text tab
3. Change Font to "FiraCode Nerd Font Mono"
4. Restart your terminal

VS Code:
1. Open Settings (Cmd+,)
2. Search for "terminal font"
3. Set Terminal › Integrated: Font Family to "FiraCode Nerd Font Mono"
4. Reload window (Cmd+R)

Troubleshooting:
• Make sure to restart your terminal after installation
• Use the exact font name: "FiraCode Nerd Font Mono" (with quotes)
• If font doesn't appear, run: fc-cache -fv

Press Enter to continue...
```

**Supported Terminals:**
- **iTerm2** (macOS) - `$TERM_PROGRAM=iTerm.app`
- **Terminal.app** (macOS) - `$TERM_PROGRAM=Apple_Terminal`
- **VS Code** - `$TERM_PROGRAM=vscode`
- **Alacritty** - Manual detection via process name or config file
- **Kitty** - Manual detection via process name or config file
- **GNOME Terminal** (Linux) - `$TERM_PROGRAM=gnome-terminal`
- **Konsole** (Linux KDE) - `$KONSOLE_PROFILE_NAME` set
- **Unknown** - Generic fallback instructions

### Architecture Compliance

**Module Location:** `src/fonts/terminal_config.rs` (NEW)
- Part of `src/fonts/` module structure
- Pure data/formatting module (no side effects)
- No external dependencies beyond stdlib
- Integrates with installer module for post-installation display

**Public API Design:**
```rust
// Terminal detection
pub enum Terminal {
    ITerm2,
    AppleTerminal,
    VSCode,
    Alacritty,
    Kitty,
    GnomeTerminal,
    Konsole,
    Unknown,
}

impl Terminal {
    pub fn detect() -> Self
    pub fn name(&self) -> &str
}

// Instruction generation
pub fn generate_instructions(font: &NerdFont, terminal: &Terminal) -> String

// Display with user interaction
pub fn display_and_wait(font: &NerdFont, terminal: &Terminal)
```

**Error Handling Strategy:**
```rust
use anyhow::{Context, Result};

// Minimal errors - this is mostly data formatting
// - Missing environment variables (graceful degradation to Unknown)
// - Invalid font name (should never happen with registry)
```

**NFR from Tech Spec:**
- Clear, actionable instructions (tested with users)
- Support ALL major terminals (80% coverage of developer terminals)
- Exact font names (avoid user confusion)
- Troubleshooting section for common issues

### Library and Framework Requirements

**Dependencies (already in Cargo.toml):**
```toml
anyhow = "1.0"           # Error handling
```

**Required Imports:**
```rust
use std::env;
use std::io::{self, Write};

use crate::fonts::nerd_fonts::NerdFont;
```

**Environment Variables for Detection:**
```rust
// Primary detection
let term_program = env::var("TERM_PROGRAM").ok();

// Secondary detection
let konsole_profile = env::var("KONSOLE_PROFILE_NAME").ok();
let alacritty_socket = env::var("ALACRITTY_SOCKET").ok();

// Process-based detection (fallback)
// Check parent process name for Alacritty, Kitty
```

### File Structure Requirements

**New File:** `src/fonts/terminal_config.rs`

**Module Export:** Add to `src/fonts/mod.rs`:
```rust
pub mod terminal_config;
pub use terminal_config::{Terminal, generate_instructions, display_and_wait};
```

**Naming Conventions:**
- Function names: snake_case (`detect_terminal`, `generate_instructions`, `display_and_wait`)
- Enum names: PascalCase (`Terminal`)
- Enum variants: PascalCase (`ITerm2`, `AppleTerminal`, `GnomeTerminal`)

### Testing Requirements

**Unit Tests** (in `src/fonts/terminal_config.rs` under `#[cfg(test)]`):
1. Test terminal detection from environment variables
   - `$TERM_PROGRAM=iTerm.app` → `Terminal::ITerm2`
   - `$TERM_PROGRAM=Apple_Terminal` → `Terminal::AppleTerminal`
   - `$TERM_PROGRAM=vscode` → `Terminal::VSCode`
   - No env vars → `Terminal::Unknown`
2. Test instruction generation for each terminal
   - Verify font name appears correctly
   - Verify step count is reasonable (3-5 steps)
   - Verify includes restart instruction
3. Test troubleshooting section always present
4. Test font name formatting (includes "Mono" suffix for display_name)

**Test Pattern:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_iterm2() {
        std::env::set_var("TERM_PROGRAM", "iTerm.app");
        let terminal = Terminal::detect();
        assert_eq!(terminal, Terminal::ITerm2);
        std::env::remove_var("TERM_PROGRAM");
    }

    #[test]
    fn test_generate_instructions_contains_font_name() {
        let font = &NERD_FONTS[0]; // FiraCode
        let terminal = Terminal::ITerm2;
        let instructions = generate_instructions(font, &terminal);

        assert!(instructions.contains(font.display_name));
        assert!(instructions.contains("iTerm2"));
        assert!(instructions.contains("Restart"));
    }

    #[test]
    fn test_all_terminals_have_instructions() {
        let font = &NERD_FONTS[0];
        for terminal in [
            Terminal::ITerm2,
            Terminal::AppleTerminal,
            Terminal::VSCode,
            Terminal::Alacritty,
            Terminal::Kitty,
            Terminal::GnomeTerminal,
            Terminal::Konsole,
            Terminal::Unknown,
        ] {
            let instructions = generate_instructions(font, &terminal);
            assert!(!instructions.is_empty());
            assert!(instructions.contains(font.display_name));
        }
    }
}
```

### Previous Story Intelligence (Story 4.6)

**What Story 4.6 Completed:**
- Font installation system (`src/fonts/installer.rs`)
- `install_font()` returns `InstallationResult` with success/failure
- Platform-specific installation (macOS, Linux)
- Font cache refresh
- Verification and error handling

**Integration Point with Story 4.6:**
```rust
use crate::fonts::installer::{install_font, InstallationResult};
use crate::fonts::terminal_config::{Terminal, display_and_wait};

// After installation:
let install_result = install_font(&download_result)?;

if install_result.success {
    println!("✓ Font installed successfully!");

    // Story 4.7: Show terminal configuration instructions
    let terminal = Terminal::detect();
    display_and_wait(selected_font, &terminal);
}
```

**Key Learnings:**
- Clear, actionable output is critical for user experience
- Platform-specific behavior handled gracefully
- Verification steps prevent user confusion

### Git Intelligence (Recent Commits)

**Code Patterns Observed:**
1. **User-friendly output:**
   ```rust
   println!("✓ Installation complete");
   println!("⚠ Warning: Font may not display immediately");
   ```

2. **Formatted instructions:**
   ```rust
   println!("\nNext steps:");
   println!("  1. First step");
   println!("  2. Second step");
   ```

### Latest Tech Information

**Terminal Detection Methods:**
1. **Environment variables** (primary):
   - `$TERM_PROGRAM`: Set by iTerm2, Terminal.app, VS Code
   - `$KONSOLE_PROFILE_NAME`: Set by Konsole
   - `$ALACRITTY_SOCKET`: Set by Alacritty
2. **Process name detection** (fallback):
   - Read `/proc/self/stat` on Linux
   - Use `ps` command on macOS
3. **Config file detection** (last resort):
   - Check for `~/.config/alacritty/alacritty.yml`
   - Check for `~/.config/kitty/kitty.conf`

**Font Name Conventions:**
- **Registry name**: "FiraCode Nerd Font" (used in code)
- **Display name**: "FiraCode Nerd Font Mono" (used in terminal settings)
- **Variants**: Regular, Bold, Italic (all use same family name)
- **Spacing**: Must be exact (some terminals case-sensitive)

### Implementation Guidance

**Terminal Enum:**
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Terminal {
    ITerm2,
    AppleTerminal,
    VSCode,
    Alacritty,
    Kitty,
    GnomeTerminal,
    Konsole,
    Unknown,
}

impl Terminal {
    /// Detect terminal from environment variables
    pub fn detect() -> Self {
        // Check TERM_PROGRAM first (most reliable)
        if let Ok(term_program) = env::var("TERM_PROGRAM") {
            return match term_program.as_str() {
                "iTerm.app" => Terminal::ITerm2,
                "Apple_Terminal" => Terminal::AppleTerminal,
                "vscode" => Terminal::VSCode,
                _ => Terminal::Unknown,
            };
        }

        // Check for KONSOLE_PROFILE_NAME
        if env::var("KONSOLE_PROFILE_NAME").is_ok() {
            return Terminal::Konsole;
        }

        // Check for GNOME Terminal (less reliable)
        if let Ok(vte_version) = env::var("VTE_VERSION") {
            if !vte_version.is_empty() {
                return Terminal::GnomeTerminal;
            }
        }

        // Check for Alacritty
        if env::var("ALACRITTY_SOCKET").is_ok() {
            return Terminal::Alacritty;
        }

        // Fallback to Unknown
        Terminal::Unknown
    }

    /// Get display name of terminal
    pub fn name(&self) -> &str {
        match self {
            Terminal::ITerm2 => "iTerm2",
            Terminal::AppleTerminal => "Terminal.app",
            Terminal::VSCode => "VS Code",
            Terminal::Alacritty => "Alacritty",
            Terminal::Kitty => "Kitty",
            Terminal::GnomeTerminal => "GNOME Terminal",
            Terminal::Konsole => "Konsole",
            Terminal::Unknown => "your terminal",
        }
    }
}
```

**Instruction Generation:**
```rust
/// Generate terminal-specific configuration instructions
///
/// Creates step-by-step instructions for configuring the given font
/// in the detected terminal emulator. Includes troubleshooting tips.
///
/// # Examples
/// ```
/// let terminal = Terminal::detect();
/// let instructions = generate_instructions(font, &terminal);
/// println!("{}", instructions);
/// ```
pub fn generate_instructions(font: &NerdFont, terminal: &Terminal) -> String {
    let font_name = font.display_name;

    let mut output = String::new();
    output.push_str(&format!("\nConfigure {} to use this font:\n\n", terminal.name()));

    match terminal {
        Terminal::ITerm2 => {
            output.push_str("iTerm2:\n");
            output.push_str("1. Open iTerm2 → Preferences (Cmd+,)\n");
            output.push_str("2. Select Profiles tab → Select your profile\n");
            output.push_str("3. Go to Text tab\n");
            output.push_str(&format!("4. Change Font to \"{}\"\n", font_name));
            output.push_str("5. Close preferences and restart your terminal\n");
        }
        Terminal::AppleTerminal => {
            output.push_str("Terminal.app:\n");
            output.push_str("1. Open Terminal → Preferences (Cmd+,)\n");
            output.push_str("2. Select Profiles tab → Select your profile\n");
            output.push_str("3. Click the Font tab\n");
            output.push_str(&format!("4. Click \"Change\" and select \"{}\"\n", font_name));
            output.push_str("5. Set as Default profile if desired\n");
            output.push_str("6. Close preferences and restart your terminal\n");
        }
        Terminal::VSCode => {
            output.push_str("VS Code:\n");
            output.push_str("1. Open Settings (Cmd+, or Ctrl+,)\n");
            output.push_str("2. Search for \"terminal font\"\n");
            output.push_str(&format!("3. Set \"Terminal › Integrated: Font Family\" to \"{}\"\n", font_name));
            output.push_str("4. Reload window (Cmd+R or Ctrl+R)\n");
        }
        Terminal::Alacritty => {
            output.push_str("Alacritty:\n");
            output.push_str("1. Open your Alacritty config file:\n");
            output.push_str("   ~/.config/alacritty/alacritty.yml (or alacritty.toml)\n");
            output.push_str("2. Add or update the font configuration:\n");
            output.push_str("\n");
            output.push_str("   font:\n");
            output.push_str(&format!("     normal:\n       family: \"{}\"\n", font_name));
            output.push_str("\n");
            output.push_str("3. Save the file and restart Alacritty\n");
        }
        Terminal::Kitty => {
            output.push_str("Kitty:\n");
            output.push_str("1. Open your Kitty config file:\n");
            output.push_str("   ~/.config/kitty/kitty.conf\n");
            output.push_str("2. Add or update the font_family line:\n");
            output.push_str("\n");
            output.push_str(&format!("   font_family {}\n", font_name));
            output.push_str("\n");
            output.push_str("3. Save the file and restart Kitty\n");
        }
        Terminal::GnomeTerminal => {
            output.push_str("GNOME Terminal:\n");
            output.push_str("1. Open Terminal → Preferences\n");
            output.push_str("2. Select your profile\n");
            output.push_str("3. Uncheck \"Use the system fixed width font\"\n");
            output.push_str(&format!("4. Select \"{}\" from the font picker\n", font_name));
            output.push_str("5. Close preferences and restart your terminal\n");
        }
        Terminal::Konsole => {
            output.push_str("Konsole:\n");
            output.push_str("1. Open Settings → Edit Current Profile\n");
            output.push_str("2. Go to Appearance tab\n");
            output.push_str(&format!("3. Select \"{}\" from the font dropdown\n", font_name));
            output.push_str("4. Click OK and restart Konsole\n");
        }
        Terminal::Unknown => {
            output.push_str("Generic Instructions:\n");
            output.push_str("1. Open your terminal's preferences/settings\n");
            output.push_str("2. Look for font or appearance settings\n");
            output.push_str(&format!("3. Set the font to \"{}\"\n", font_name));
            output.push_str("4. Save and restart your terminal\n");
            output.push_str("\n");
            output.push_str("Consult your terminal's documentation for specific steps.\n");
        }
    }

    // Add troubleshooting section
    output.push_str("\nTroubleshooting:\n");
    output.push_str(&format!("• Make sure to use the exact font name: \"{}\"\n", font_name));
    output.push_str("• Restart your terminal completely after changing the font\n");
    output.push_str("• If font doesn't appear in the font picker, run: fc-cache -fv\n");
    output.push_str("• Some terminals require selecting the 'Mono' variant specifically\n");

    output
}
```

**Display and Wait:**
```rust
/// Display instructions and wait for user acknowledgment
///
/// Shows the configuration instructions and pauses until the user
/// presses Enter. This ensures the user has time to read the instructions
/// before the wizard continues.
pub fn display_and_wait(font: &NerdFont, terminal: &Terminal) {
    let instructions = generate_instructions(font, terminal);
    println!("{}", instructions);

    // Wait for user
    print!("\nPress Enter to continue...");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
}
```

### Critical Reminders

**DO:**
- ✅ Detect terminal from `$TERM_PROGRAM` environment variable
- ✅ Support all major terminals (iTerm2, Terminal.app, VS Code, Alacritty, Kitty, GNOME Terminal, Konsole)
- ✅ Provide clear, numbered step-by-step instructions
- ✅ Use exact font display name (e.g., "FiraCode Nerd Font Mono")
- ✅ Include "restart terminal" step in all instructions
- ✅ Add troubleshooting section with common issues
- ✅ Format output for readability (spacing, indentation)
- ✅ Wait for user acknowledgment before continuing
- ✅ Write tests for all terminals
- ✅ Handle Unknown terminal gracefully with generic instructions

**DON'T:**
- ❌ Don't hardcode font names (use `font.display_name`)
- ❌ Don't skip troubleshooting section
- ❌ Don't assume terminal is detected correctly (provide Unknown fallback)
- ❌ Don't forget to mention "restart terminal"
- ❌ Don't write instructions that assume GUI access
- ❌ Don't implement actual terminal configuration (just instructions)
- ❌ Don't forget config file paths for Alacritty/Kitty
- ❌ Don't skip testing with actual terminals

### Acceptance Criteria Expanded

1. **Create `src/fonts/terminal_config.rs`**
   - New file in `src/fonts/` directory
   - Export via `src/fonts/mod.rs`: `pub mod terminal_config;`
   - Public API: `Terminal`, `detect()`, `generate_instructions()`, `display_and_wait()`

2. **Detect terminal from `$TERM_PROGRAM`**
   - Check `$TERM_PROGRAM` environment variable
   - Map known values to `Terminal` enum
   - Fallback to `Terminal::Unknown` if not detected

3. **Support major terminals**
   - iTerm2: Preferences → Profiles → Text → Font
   - Terminal.app: Preferences → Profiles → Font → Change
   - VS Code: Settings → Terminal Font Family
   - Alacritty: Config file `~/.config/alacritty/alacritty.yml`
   - Kitty: Config file `~/.config/kitty/kitty.conf`
   - GNOME Terminal: Preferences → Profile → Font
   - Konsole: Settings → Edit Current Profile → Appearance
   - Unknown: Generic instructions

4. **Generate terminal-specific instructions**
   - Numbered steps (3-6 steps per terminal)
   - Exact paths and menu items
   - Font name from `font.display_name`
   - "Restart terminal" step included

5. **Show exact font name**
   - Use `font.display_name` (e.g., "FiraCode Nerd Font Mono")
   - Include in instructions with quotes for clarity
   - Mention in troubleshooting section

6. **Include troubleshooting tips**
   - Exact font name reminder
   - Restart terminal reminder
   - fc-cache command if font doesn't appear
   - Mono variant selection note

7. **Display instructions with formatting**
   - Clear section headers
   - Indentation for steps
   - Blank lines for readability
   - Troubleshooting section at end

8. **Unit tests**
   - Test terminal detection from env vars
   - Test instruction generation for all terminals
   - Test font name appears in instructions
   - Test troubleshooting section always present

## Tasks / Subtasks

- [ ] Create `src/fonts/terminal_config.rs` (AC: 1)
  - [ ] Module-level documentation
  - [ ] Import required dependencies
- [ ] Define `Terminal` enum (AC: 2, 3)
  - [ ] Variants for all supported terminals
  - [ ] Implement `detect()` method
  - [ ] Implement `name()` method
- [ ] Implement `detect()` method (AC: 2)
  - [ ] Check `$TERM_PROGRAM` environment variable
  - [ ] Check `$KONSOLE_PROFILE_NAME` for Konsole
  - [ ] Check `$VTE_VERSION` for GNOME Terminal
  - [ ] Check `$ALACRITTY_SOCKET` for Alacritty
  - [ ] Fallback to Unknown
- [ ] Implement `generate_instructions()` (AC: 3, 4, 5, 6, 7)
  - [ ] iTerm2 instructions
  - [ ] Terminal.app instructions
  - [ ] VS Code instructions
  - [ ] Alacritty instructions (with config file)
  - [ ] Kitty instructions (with config file)
  - [ ] GNOME Terminal instructions
  - [ ] Konsole instructions
  - [ ] Unknown/generic instructions
  - [ ] Troubleshooting section
- [ ] Implement `display_and_wait()` (AC: 7)
  - [ ] Generate instructions
  - [ ] Display to stdout
  - [ ] Prompt "Press Enter to continue..."
  - [ ] Wait for user input
- [ ] Write unit tests (AC: 8)
  - [ ] Test detection from TERM_PROGRAM
  - [ ] Test detection fallbacks
  - [ ] Test instruction generation for each terminal
  - [ ] Test font name appears in output
  - [ ] Test troubleshooting section present
- [ ] Export module (AC: 1)
  - [ ] Add `pub mod terminal_config;` to `src/fonts/mod.rs`
  - [ ] Re-export public API
- [ ] Documentation (AC: all)
  - [ ] Module-level docs with examples
  - [ ] Function docs with usage
  - [ ] Terminal enum documentation

## Dev Agent Record

### Context Reference

Epic: [epic-4-nerd-fonts.md](../epic-4-nerd-fonts.md)
Tech Spec: [tech-spec-epic-4.md](../../tech-spec-epic-4.md)
Previous Story: [epic-4-story-6.md](epic-4-story-6.md) (Font Installation - ready-for-dev)
Nerd Font Registry: [src/fonts/nerd_fonts.rs](../../../src/fonts/nerd_fonts.rs)

### Agent Model Used

<!-- Will be filled during implementation -->

### Debug Log References

<!-- Will be filled during implementation -->

### Completion Notes List

<!-- Will be filled during implementation -->

### File List

<!-- Will be filled during implementation -->
