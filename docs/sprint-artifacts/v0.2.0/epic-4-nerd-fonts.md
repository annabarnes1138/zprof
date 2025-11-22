# Epic 4: Nerd Font Auto-Installation

**Priority:** P1 (Should Have)
**Estimated Effort:** 3 days
**Owner:** TBD

## Overview

Automatically detect when users select prompts that require Nerd Fonts (Starship, Powerlevel10k, etc.), download and install appropriate fonts, and provide clear terminal configuration instructions.

## Problem Statement

Modern prompt engines rely on Nerd Fonts for icons and symbols, but:
- Users don't know they need special fonts
- Installation is manual and OS-specific
- Terminal configuration is unclear
- Broken/missing glyphs frustrate users

This creates a poor first-run experience where prompts look broken until users manually research, download, and configure fonts.

## Goals

1. **Automatic detection**: Know when Nerd Fonts are required
2. **Easy installation**: One-click font download and install
3. **Clear guidance**: Show exactly how to configure terminals
4. **Graceful degradation**: Offer fallback options if auto-install fails
5. **User control**: Allow skipping or choosing specific fonts

## User Stories

### Story 4.1: Create Nerd Font Registry

**As a** developer
**I want** a registry of recommended Nerd Fonts
**So that** users have curated, tested options

**Acceptance Criteria:**
- [ ] Create `src/fonts/nerd_fonts.rs`
- [ ] Define `NerdFont` struct:
  - name, display_name
  - description, preview_chars
  - download_url (GitHub releases)
  - file_format (ttf, otf)
  - recommended (bool)
- [ ] Add 5-6 popular Nerd Fonts:
  - FiraCode Nerd Font (recommended, programming ligatures)
  - JetBrainsMono Nerd Font (recommended, clean and modern)
  - Meslo Nerd Font (recommended for Powerlevel10k)
  - Hack Nerd Font (classic programming font)
  - CascadiaCode Nerd Font (Microsoft's modern font)
  - UbuntuMono Nerd Font (clean, widely compatible)
- [ ] Add download URLs from nerdfonts.com releases
- [ ] Create constant registry `NERD_FONTS`
- [ ] Add helper methods to get by name or recommended
- [ ] Add unit tests

**Files:**
- `src/fonts/nerd_fonts.rs` (NEW)
- `src/fonts/mod.rs` (NEW)

---

### Story 4.2: Detect Font Requirements from Prompt Engine

**As a** user selecting a prompt engine
**I want** to know if it requires Nerd Fonts
**So that** I can install them if needed

**Acceptance Criteria:**
- [ ] Add `requires_nerd_font: bool` to `PromptEngine` enum/registry
- [ ] Mark engines that require Nerd Fonts:
  - Starship: true
  - Powerlevel10k: true
  - Oh-My-Posh: true
  - Pure: false
  - Spaceship: true
- [ ] Create function `check_nerd_font_required(engine: &PromptEngine) -> bool`
- [ ] Add to prompt engine selection TUI:
  ```
  ┌────────────────────────────────────┐
  │ ⚡ Starship                         │
  │ Fast, minimal, highly customizable │
  │                                     │
  │ Requires: Nerd Font                │
  └────────────────────────────────────┘
  ```
- [ ] Show icon: 🔤 for fonts required
- [ ] Update prompt engine registry in `src/prompts/engine.rs`
- [ ] Add unit tests

**Files:**
- `src/prompts/engine.rs`
- `src/tui/prompt_engine_select.rs`

---

### Story 4.3: Check for Existing Nerd Font Installation

**As a** user
**I want** zprof to detect if I already have Nerd Fonts
**So that** I'm not prompted to install if unnecessary

**Acceptance Criteria:**
- [ ] Create `src/fonts/detector.rs`
- [ ] Implement platform-specific detection:
  - **macOS**: Check `~/Library/Fonts/` and `/Library/Fonts/`
  - **Linux**: Check `~/.local/share/fonts/` and `/usr/share/fonts/`
- [ ] Search for files matching pattern `*Nerd*.{ttf,otf}`
- [ ] Return list of detected Nerd Fonts
- [ ] Create function `has_nerd_font_installed() -> bool`
- [ ] Add heuristic detection (check for common names)
- [ ] Cache detection result during session
- [ ] Add unit tests with mock filesystem
- [ ] Add integration test with real font directories

**Files:**
- `src/fonts/detector.rs` (NEW)
- `tests/font_detection_test.rs` (NEW)

---

### Story 4.4: Create Font Selection TUI

**As a** user who needs a Nerd Font
**I want** to choose which font to install
**So that** I can pick one that matches my preferences

**Acceptance Criteria:**
- [ ] Create `src/tui/font_select.rs`
- [ ] Show when Nerd Font is required but not detected
- [ ] Display font options as cards:
  ```
  Select a Nerd Font to install:

  > ✨ FiraCode Nerd Font (recommended)
    Programming ligatures, clean and modern
    Preview: ⚡ ⬢  →  ✓

    JetBrainsMono Nerd Font (recommended)
    Designed for developers, excellent readability
    Preview: ⚡ ⬢  →  ✓

    Meslo Nerd Font (recommended for Powerlevel10k)
    Optimized for Powerlevel10k prompt
    Preview: ⚡ ⬢  →  ✓

    Skip font installation (prompt may not display correctly)
  ```
- [ ] Highlight recommended fonts
- [ ] Show preview characters for each font
- [ ] Allow skipping installation
- [ ] Return selected `NerdFont` or `None`
- [ ] Keyboard navigation (↑↓, Enter, Esc)
- [ ] Add help text explaining why Nerd Fonts are needed

**Files:**
- `src/tui/font_select.rs` (NEW)

---

### Story 4.5: Implement Font Download

**As a** user who selected a font
**I want** it downloaded automatically
**So that** I don't have to manually find and download it

**Acceptance Criteria:**
- [ ] Create `src/fonts/download.rs`
- [ ] Download font from nerdfonts.com GitHub releases
- [ ] Show progress bar during download:
  ```
  Downloading FiraCode Nerd Font...
  ████████████████████████░░░░░░░░ 67% (12.3 MB / 18.4 MB)
  ```
- [ ] Download to temp directory first
- [ ] Verify download (checksum if available, or size sanity check)
- [ ] Extract if zip/tar.gz (most Nerd Fonts are zipped)
- [ ] Handle download errors gracefully:
  - Network timeout
  - Invalid URL
  - Incomplete download
- [ ] Add retry logic (2 retries with exponential backoff)
- [ ] Clean up temp files on success or failure
- [ ] Add unit tests with mock HTTP client
- [ ] Add integration test with real download (marked as slow test)

**Files:**
- `src/fonts/download.rs` (NEW)
- `tests/font_download_test.rs` (NEW)

**Dependencies:**
- Add `reqwest` crate for HTTP downloads
- Add `zip` or `tar` crate for extraction

---

### Story 4.6: Implement Platform-Specific Font Installation

**As a** user on macOS or Linux
**I want** fonts automatically installed to the correct location
**So that** they're available system-wide

**Acceptance Criteria:**
- [ ] Create `src/fonts/installer.rs`
- [ ] Implement macOS installation:
  - Copy `.ttf`/`.otf` files to `~/Library/Fonts/`
  - Run `fc-cache -f` if available (for XQuartz)
  - Verify files copied successfully
- [ ] Implement Linux installation:
  - Copy files to `~/.local/share/fonts/`
  - Create directory if doesn't exist
  - Run `fc-cache -fv` to refresh font cache
  - Verify with `fc-list` if available
- [ ] Show installation progress:
  ```
  Installing FiraCode Nerd Font...
  ✓ Copied 12 font files to ~/Library/Fonts/
  ✓ Updated font cache
  ✓ Installation complete
  ```
- [ ] Handle permission errors
- [ ] Offer manual installation instructions if auto-install fails
- [ ] Return installation result (success, partial, failed)
- [ ] Add unit tests with mock filesystem
- [ ] Add platform-specific integration tests

**Files:**
- `src/fonts/installer.rs` (NEW)
- `tests/font_install_test.rs` (NEW)

---

### Story 4.7: Generate Terminal Configuration Instructions

**As a** user who installed a Nerd Font
**I want** clear instructions to configure my terminal
**So that** the font actually displays correctly

**Acceptance Criteria:**
- [ ] Create `src/fonts/terminal_config.rs`
- [ ] Detect user's terminal emulator:
  - Check `$TERM_PROGRAM` env var
  - Common values: iTerm.app, Apple_Terminal, vscode, etc.
  - Fallback to generic instructions
- [ ] Generate terminal-specific instructions:
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

  macOS Terminal:
  1. Terminal → Preferences → Profiles
  2. Select your profile → Font tab
  3. Click "Change" and select "FiraCode Nerd Font Mono"
  4. Set as Default profile

  Generic:
  Set your terminal font to "FiraCode Nerd Font Mono"
  Consult your terminal's documentation for font settings.
  ```
- [ ] Support major terminals:
  - iTerm2 (macOS)
  - Terminal.app (macOS)
  - VS Code integrated terminal
  - Alacritty (config file example)
  - Kitty (config file example)
  - GNOME Terminal (Linux)
  - Konsole (Linux)
- [ ] Show exact font name to use
- [ ] Add troubleshooting tips
- [ ] Offer to open documentation URL
- [ ] Store instructions in manifest for later reference

**Files:**
- `src/fonts/terminal_config.rs` (NEW)
- `src/tui/terminal_instructions.rs` (NEW)

---

### Story 4.8: Integrate Font Installation into Create Workflow

**As a** user creating a profile with Starship
**I want** automatic font installation offered
**So that** my prompt works immediately

**Acceptance Criteria:**
- [ ] After prompt engine selection, check if Nerd Font required
- [ ] If required and not detected, show font selection TUI
- [ ] If user selects a font:
  - Download font
  - Install font
  - Show terminal configuration instructions
  - Wait for user acknowledgment before continuing
- [ ] If user skips:
  - Show warning about potential display issues
  - Add note to profile manifest
  - Continue with profile creation
- [ ] Store font choice in manifest:
  ```toml
  [prompt]
  mode = "engine"
  engine = "starship"
  nerd_font = "FiraCode Nerd Font"
  ```
- [ ] Update create workflow in `src/cli/create.rs`
- [ ] Add integration test for full flow with font installation
- [ ] Handle errors gracefully (fall back to skip)

**Files:**
- `src/cli/create.rs`
- `src/cli/create_wizard.rs`
- `src/core/manifest.rs` (add `nerd_font` field)

---

### Story 4.9: Add Font Management Command

**As a** user
**I want** to install or change fonts later
**So that** I'm not locked into my initial choice

**Acceptance Criteria:**
- [ ] Create `zprof font` command with subcommands:
  - `zprof font list` - Show installed Nerd Fonts
  - `zprof font install` - Install a new Nerd Font
  - `zprof font info <name>` - Show terminal config for a font
- [ ] `zprof font list` output:
  ```
  Installed Nerd Fonts:

  ✓ FiraCode Nerd Font
    ~/Library/Fonts/FiraCodeNerdFont-Regular.ttf
    Used by: work, personal

  ✓ JetBrainsMono Nerd Font
    ~/Library/Fonts/JetBrainsMonoNerdFont-Regular.ttf
    (Not used by any profile)
  ```
- [ ] `zprof font install` launches font selection TUI
- [ ] `zprof font info FiraCode` shows terminal configuration instructions
- [ ] Update CLI help text
- [ ] Add integration tests for font commands
- [ ] Update documentation

**Files:**
- `src/cli/font.rs` (NEW)
- `src/cli/mod.rs`
- `src/main.rs`

---

### Story 4.10: Update Documentation

**As a** user
**I want** documentation about Nerd Fonts
**So that** I understand why they're needed and how to manage them

**Acceptance Criteria:**
- [ ] Create `docs/user-guide/nerd-fonts.md`:
  - What are Nerd Fonts and why do prompts need them
  - List of supported fonts with previews
  - Automatic vs manual installation
  - Terminal configuration for all major terminals
  - Troubleshooting (fonts not showing, wrong glyphs)
- [ ] Update `docs/user-guide/quick-start.md` to mention font installation step
- [ ] Update `docs/user-guide/commands.md` with `zprof font` commands
- [ ] Update `docs/user-guide/troubleshooting.md`:
  - "Prompt shows boxes or question marks"
  - "Installed font but terminal doesn't show it"
- [ ] Add FAQ entries:
  - "Why does my prompt look broken?"
  - "Can I use my own Nerd Font?"
  - "How do I uninstall a font?"
- [ ] Add screenshots/examples of properly configured terminals

**Files:**
- `docs/user-guide/nerd-fonts.md` (NEW)
- `docs/user-guide/quick-start.md`
- `docs/user-guide/commands.md`
- `docs/user-guide/troubleshooting.md`
- `docs/user-guide/faq.md`

---

## Technical Design

### Font Data Structure

```rust
// src/fonts/nerd_fonts.rs

pub struct NerdFont {
    pub id: &'static str,
    pub name: &'static str,
    pub display_name: &'static str,
    pub description: &'static str,
    pub preview_chars: &'static str,
    pub download_url: &'static str,
    pub file_format: FontFormat,
    pub recommended: bool,
    pub recommended_for: Vec<PromptEngine>,
}

pub enum FontFormat {
    TrueType,  // .ttf
    OpenType,  // .otf
}

pub const NERD_FONTS: &[NerdFont] = &[
    NerdFont {
        id: "firacode",
        name: "FiraCode Nerd Font",
        display_name: "FiraCode Nerd Font Mono",
        description: "Programming ligatures, clean and modern",
        preview_chars: "⚡ ⬢  →  ✓  ",
        download_url: "https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/FiraCode.zip",
        file_format: FontFormat::TrueType,
        recommended: true,
        recommended_for: vec![PromptEngine::Starship, PromptEngine::OhMyPosh],
    },
    // More fonts...
];
```

### Font Installation Flow

```
Prompt Engine Selection
         ↓
    Requires Nerd Font?
         ↓
    Already Installed?
         ↓
      ┌──No──┐
      ↓      ↓
Font Selection
      ↓
  Download Font
      ↓
  Install Font
      ↓
Show Terminal Config
      ↓
Continue Profile Creation
```

### Platform Detection

```rust
// src/fonts/installer.rs

pub enum Platform {
    MacOS,
    Linux,
    Unsupported,
}

impl Platform {
    pub fn detect() -> Self {
        #[cfg(target_os = "macos")]
        return Platform::MacOS;

        #[cfg(target_os = "linux")]
        return Platform::Linux;

        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        return Platform::Unsupported;
    }

    pub fn font_dir(&self) -> Result<PathBuf> {
        match self {
            Platform::MacOS => {
                let home = dirs::home_dir()?;
                Ok(home.join("Library/Fonts"))
            }
            Platform::Linux => {
                let home = dirs::home_dir()?;
                Ok(home.join(".local/share/fonts"))
            }
            Platform::Unsupported => {
                bail!("Automatic font installation not supported on this platform")
            }
        }
    }
}
```

## Dependencies

- None (can be developed in parallel with other epics)

## Risks & Mitigations

**Risk:** Download failures due to network issues
**Mitigation:** Retry logic, clear error messages, offer manual download option

**Risk:** Font doesn't display correctly after installation
**Mitigation:** Terminal-specific instructions, troubleshooting guide, font cache refresh

**Risk:** Platform-specific installation differences
**Mitigation:** Test on macOS and major Linux distros, generic fallback instructions

**Risk:** Large download sizes
**Mitigation:** Download only selected font, show progress, allow cancellation

## Testing Strategy

- Unit tests for font registry and detection logic
- Integration tests with mock filesystem for installation
- Manual testing on macOS and Linux with various terminals
- Test download/install with real Nerd Font releases
- Snapshot tests for TUI output and instructions
- Test terminal detection with various `$TERM_PROGRAM` values

## Success Criteria

- [ ] Nerd Font automatically offered when selecting compatible prompt engine
- [ ] Fonts download and install successfully on macOS and Linux
- [ ] Terminal configuration instructions are clear and accurate
- [ ] Users can skip font installation without blocking profile creation
- [ ] Font management commands work (`list`, `install`, `info`)
- [ ] Documentation comprehensive and helpful
- [ ] All integration tests passing
- [ ] No broken prompts due to missing fonts (or clear warning if skipped)

## Out of Scope

- Windows support (v0.3.0)
- Custom font uploads (use system fonts only)
- Font preview in terminal before selection (technical limitation)
- Automatic terminal configuration (would require terminal-specific APIs)
- Font version management (always use latest stable from nerdfonts.com)
