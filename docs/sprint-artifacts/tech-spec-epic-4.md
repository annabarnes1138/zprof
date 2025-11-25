# Epic Technical Specification: Nerd Font Auto-Installation

Date: 2025-11-24
Author: Anna
Epic ID: 4
Status: Draft

---

## Overview

This epic implements automatic Nerd Font detection, download, installation, and configuration guidance for zprof users creating profiles with modern prompt engines (Starship, Powerlevel10k, Oh-My-Posh). Modern prompt engines rely heavily on Nerd Fonts for glyphs and icons, but users often don't realize this requirement until after installation when their prompts display broken characters. This epic addresses that pain point by integrating font management directly into the profile creation workflow, providing a seamless first-run experience with visual prompts that "just work."

The implementation follows zprof's core architectural principles: non-destructive operations, safe file handling with backups, manifest-based configuration, and dual CLI/GUI interface support. The font installation system will be modular, extensible, and platform-aware (macOS and Linux).

## Objectives and Scope

**In Scope:**
- Nerd Font registry with 5-6 curated, tested fonts (FiraCode, JetBrainsMono, Meslo, Hack, CascadiaCode, UbuntuMono)
- Automatic detection of font requirements based on selected prompt engine
- Platform-specific font installation detection (macOS, Linux)
- TUI-based font selection interface with preview characters and recommendations
- HTTP download from nerdfonts.com GitHub releases with progress indication
- Platform-specific installation to user font directories (`~/Library/Fonts/`, `~/.local/share/fonts/`)
- Terminal configuration instruction generation (iTerm2, Terminal.app, VS Code, Alacritty, Kitty, GNOME Terminal, Konsole)
- Integration into `zprof create` workflow with graceful skip option
- Font management CLI commands (`zprof font list`, `install`, `info`)
- Manifest storage of font selection
- Documentation and troubleshooting guides

**Out of Scope:**
- Windows support (deferred to v0.3.0)
- Custom font uploads (system fonts only)
- Live font preview in terminal (technical limitation)
- Automatic terminal configuration (requires terminal-specific APIs)
- Font version management (always latest stable)
- Uninstall functionality (v0.3.0)

## System Architecture Alignment

This epic introduces a new top-level module `src/fonts/` that follows zprof's established architectural patterns:

**Module Structure:**
- `src/fonts/mod.rs` - Public module interface
- `src/fonts/nerd_fonts.rs` - Font registry (data model)
- `src/fonts/detector.rs` - Platform-specific font detection
- `src/fonts/download.rs` - HTTP download with progress
- `src/fonts/installer.rs` - Platform-specific installation
- `src/fonts/terminal_config.rs` - Terminal instruction generator

**Integration Points:**
- **CLI Layer** (`src/cli/create.rs`, `src/cli/font.rs`): Orchestrates font workflow, invokes core logic
- **Core Layer** (`src/core/manifest.rs`): Add `nerd_font` field to profile manifest TOML
- **Prompts Module** (`src/prompts/engine.rs`): Add `requires_nerd_font: bool` to `PromptEngine` registry
- **TUI Layer** (`src/tui/font_select.rs`): New font selection interface (follows existing TUI patterns)
- **GUI Layer** (future): Tauri commands will expose font operations via IPC (same pattern as profile management)

**Architectural Constraints:**
- No modification of business logic - font installation is a pure addition
- Follows "safe file operations" pattern: check → backup → operate → verify
- Platform detection follows existing pattern in `frameworks/installer.rs`
- HTTP operations use `reqwest` (new dependency, widely adopted in Rust ecosystem)
- Archive extraction uses `zip` crate (new dependency)
- All font operations are optional and non-blocking - users can skip without breaking profile creation

## Detailed Design

### Services and Modules

| Module | Responsibility | Inputs | Outputs | Owner |
|--------|---------------|--------|---------|-------|
| `src/fonts/mod.rs` | Public API, module exports | N/A | Re-exports from submodules | Core |
| `src/fonts/nerd_fonts.rs` | Font registry and data model | Query parameters (name, filter) | `NerdFont` structs, font lists | Core |
| `src/fonts/detector.rs` | Detect installed Nerd Fonts | Platform, font directories | List of detected fonts, bool checks | Core |
| `src/fonts/download.rs` | Download fonts from GitHub | Font URL, destination path | Downloaded file path or error | Core |
| `src/fonts/installer.rs` | Install fonts to system directories | Font files, platform | Installation result (success/partial/fail) | Core |
| `src/fonts/terminal_config.rs` | Generate terminal config instructions | Font name, detected terminal | Formatted instruction text | Core |
| `src/tui/font_select.rs` | TUI for font selection | Available fonts, recommendations | Selected font or None (skip) | TUI |
| `src/cli/font.rs` | Font management CLI commands | Command args (list/install/info) | CLI output | CLI |
| `src/cli/create.rs` (modified) | Integrate font workflow into create | Prompt engine selection | Profile with font configured | CLI |

### Data Models and Contracts

**NerdFont Struct** (`src/fonts/nerd_fonts.rs`):
```rust
pub struct NerdFont {
    pub id: &'static str,              // "firacode"
    pub name: &'static str,            // "FiraCode Nerd Font"
    pub display_name: &'static str,    // "FiraCode Nerd Font Mono"
    pub description: &'static str,     // "Programming ligatures, clean and modern"
    pub preview_chars: &'static str,   // "⚡ ⬢  →  ✓  "
    pub download_url: &'static str,    // GitHub release URL
    pub file_format: FontFormat,       // TrueType or OpenType
    pub recommended: bool,             // Mark as recommended
    pub recommended_for: Vec<PromptEngine>, // Specific engine recommendations
}

pub enum FontFormat {
    TrueType,  // .ttf files
    OpenType,  // .otf files
}
```

**PromptEngine Enhancement** (`src/prompts/engine.rs`):
```rust
pub struct PromptEngine {
    pub name: &'static str,
    pub description: &'static str,
    pub requires_nerd_font: bool,  // NEW FIELD
    // ... existing fields
}
```

**Profile Manifest Enhancement** (`src/core/manifest.rs`):
```toml
[prompt]
mode = "engine"
engine = "starship"
nerd_font = "FiraCode Nerd Font"  # NEW FIELD (optional)
nerd_font_skipped = false          # NEW FIELD (tracks if user skipped)
```

**Platform Enum** (`src/fonts/installer.rs`):
```rust
pub enum Platform {
    MacOS,
    Linux,
    Unsupported,
}

pub struct InstallationResult {
    pub success: bool,
    pub files_installed: usize,
    pub install_path: PathBuf,
    pub errors: Vec<String>,
}
```

**Terminal Detection** (`src/fonts/terminal_config.rs`):
```rust
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
```

### APIs and Interfaces

**Public Font API** (`src/fonts/mod.rs`):
```rust
// Font registry queries
pub fn get_all_fonts() -> &'static [NerdFont];
pub fn get_recommended_fonts() -> Vec<&'static NerdFont>;
pub fn get_font_by_id(id: &str) -> Option<&'static NerdFont>;
pub fn get_fonts_for_engine(engine: &PromptEngine) -> Vec<&'static NerdFont>;

// Detection
pub fn has_nerd_font_installed() -> Result<bool>;
pub fn list_installed_fonts() -> Result<Vec<String>>;

// Download
pub fn download_font(font: &NerdFont, progress: impl ProgressCallback) -> Result<PathBuf>;

// Installation
pub fn install_font(font_path: &Path) -> Result<InstallationResult>;

// Terminal configuration
pub fn detect_terminal() -> Terminal;
pub fn generate_instructions(font: &NerdFont, terminal: &Terminal) -> String;
```

**CLI Commands** (`src/cli/font.rs`):
```rust
// zprof font list
pub fn list_fonts() -> Result<()>;

// zprof font install
pub fn install_font_interactive() -> Result<()>;

// zprof font info <name>
pub fn show_font_info(name: &str) -> Result<()>;
```

**Integration Hook** (`src/cli/create.rs`):
```rust
// Called after prompt engine selection
fn handle_font_installation(engine: &PromptEngine) -> Result<Option<String>> {
    if !engine.requires_nerd_font {
        return Ok(None);
    }

    if fonts::has_nerd_font_installed()? {
        return Ok(None);
    }

    // Show font selection TUI
    let selected = tui::font_select::run()?;

    if let Some(font) = selected {
        fonts::download_font(&font, progress_callback)?;
        fonts::install_font(&font)?;

        let terminal = fonts::detect_terminal();
        let instructions = fonts::generate_instructions(&font, &terminal);
        println!("{}", instructions);

        // Wait for acknowledgment
        wait_for_user();

        Ok(Some(font.name.to_string()))
    } else {
        // User skipped
        eprintln!("⚠️  Warning: Prompt may not display correctly without Nerd Fonts");
        Ok(None)
    }
}
```

### Workflows and Sequencing

**Profile Creation with Font Installation**:
```
1. User runs: zprof create my-profile
2. CLI prompts for framework selection
3. CLI prompts for prompt engine selection → User selects "Starship"
4. Check: Does Starship require Nerd Font? → YES
5. Check: Is any Nerd Font installed? → NO
6. Display font selection TUI
   - Show recommended fonts with preview chars
   - Highlight fonts recommended for Starship
   - Offer "Skip" option
7. User selects "FiraCode Nerd Font"
8. Download font from GitHub releases
   - Show progress bar (size/speed/ETA)
   - Verify download integrity
   - Extract ZIP archive to temp directory
9. Install font files
   - Detect platform (macOS/Linux)
   - Copy .ttf files to ~/Library/Fonts/ (macOS)
   - Run fc-cache -fv (if available)
   - Verify installation
10. Generate terminal configuration instructions
    - Detect terminal via $TERM_PROGRAM
    - Generate step-by-step instructions
    - Display to user with formatting
11. Wait for user acknowledgment
12. Store font selection in manifest
    - Update profile.toml: nerd_font = "FiraCode Nerd Font"
13. Continue with profile creation (plugins, theme, etc.)
14. Profile creation complete
```

**Font Installation Flow (Skip Path)**:
```
1-6. [Same as above]
7. User selects "Skip font installation"
8. Display warning message:
   "⚠️  Your prompt may display boxes (□) or question marks (?) instead of icons.
    Run 'zprof font install' later to add Nerd Fonts."
9. Store skip flag in manifest: nerd_font_skipped = true
10. Continue with profile creation
```

**Standalone Font Installation**:
```
1. User runs: zprof font install
2. Check: Is any Nerd Font already installed? → Display info if yes
3. Display font selection TUI
4. User selects font
5. Download → Install (same as profile creation flow)
6. Display terminal config instructions
7. Done
```

**Font Info Lookup**:
```
1. User runs: zprof font info FiraCode
2. Load font from registry
3. Display:
   - Font name and description
   - Download URL
   - Recommended for: [Starship, Oh-My-Posh]
   - Installation path (if installed)
   - Terminal configuration instructions
```

## Non-Functional Requirements

### Performance

- **Font download time**: Target < 30 seconds for typical font (15-20 MB) on broadband connection (5 Mbps+)
- **Installation time**: < 5 seconds for font file copying and cache refresh
- **Detection time**: < 100ms for checking existing font installation (file system scan)
- **TUI responsiveness**: Font selection interface renders in < 200ms
- **Progress feedback**: Download progress updates at minimum 2 Hz (twice per second) for smooth user experience
- **Profile creation overhead**: Font workflow adds maximum 60 seconds to profile creation (only when font installation needed)

**Performance Constraints from Architecture:**
- No blocking of profile creation - all font operations are optional
- Font detection caches results during session to avoid repeated file system scans
- Download uses streaming to avoid memory overhead for large files

### Security

- **Download integrity**: Verify downloaded file size matches expected range (reject files < 1MB or > 50MB as potentially corrupt)
- **URL validation**: Only allow downloads from `github.com/ryanoasis/nerd-fonts/releases/*` (hardcoded in registry)
- **Path traversal prevention**: Validate all file paths to prevent writing outside font directories
  - Reject paths containing `..` or absolute paths in archives
  - Use `std::fs::canonicalize()` to resolve symlinks before operations
- **File permissions**: Font files written with permissions `0644` (rw-r--r--)
- **No arbitrary code execution**: Font files are data only (`.ttf`, `.otf`), no scripts or executables
- **Temporary file cleanup**: Delete downloaded archives and extracted files from temp directory after installation (or on error)
- **User directory isolation**: Only write to user-owned directories (`~/Library/Fonts/`, `~/.local/share/fonts/`), never system directories
- **Input validation**: Sanitize font names before using in file paths (alphanumeric and hyphens only)

**Security Constraints from Architecture:**
- Follows zprof's "safe file operations" pattern: check → backup → operate → verify
- No elevation of privileges required (user-level font installation only)
- Uses Rust's memory safety guarantees (no buffer overflows)

### Reliability/Availability

- **Download retry logic**: Retry failed downloads up to 2 times with exponential backoff (1s, 2s delays)
- **Network timeout**: 30-second timeout for HTTP requests (both connection and read)
- **Partial download recovery**: Delete incomplete files on failure to prevent corruption
- **Graceful degradation**: If font installation fails, allow profile creation to continue with warning
- **Platform compatibility**: Detect unsupported platforms (Windows) and provide clear error message with manual instructions link
- **Font cache refresh**: Handle missing `fc-cache` gracefully (not critical on macOS, warn on Linux)
- **Installation verification**: Verify font files exist in destination after copy operation
- **Error recovery**: All errors include actionable messages (e.g., "Check internet connection", "Ensure ~/Library/Fonts is writable")

**Availability Constraints:**
- Font installation depends on GitHub availability (no SLA control)
- Fallback: If GitHub unreachable, offer manual download link in error message
- No persistent background processes - all operations are synchronous during profile creation

### Observability

**Required Logging Signals:**
- **Download events**: Start, progress (every 10%), completion/failure with duration and size
- **Installation events**: Platform detected, files copied (count), cache refresh result
- **Detection events**: Fonts found (count and names), search paths checked
- **Error events**: Download failures (HTTP status, network error), installation failures (permission denied, disk full)

**Log Format** (structured logging with `log` crate):
```rust
info!("Downloading font: {} from {}", font.name, font.download_url);
debug!("Download progress: {}% ({}/{} bytes)", percent, downloaded, total);
info!("Font installed: {} files to {}", file_count, install_path.display());
error!("Font installation failed: {}", error);
```

**Metrics** (future enhancement):
- Font installation success rate (per platform)
- Average download time by font
- Most selected fonts
- Skip rate (users who decline font installation)

**User-Facing Feedback:**
- Progress bars for downloads (using `indicatif` crate, consistent with existing TUI patterns)
- Success/failure messages with clear next steps
- Terminal configuration instructions displayed with formatting
- Warning messages for skipped installations

**Debugging Support:**
- Verbose mode (`--verbose` flag) logs HTTP headers, file paths, cache commands
- Installation result includes detailed error list for partial failures
- Detection result includes searched paths for troubleshooting

## Dependencies and Integrations

**New Rust Dependencies** (add to `Cargo.toml`):

| Crate | Version | Purpose | Features |
|-------|---------|---------|----------|
| `reqwest` | `0.12` | HTTP client for font downloads | `blocking`, `stream` |
| `zip` | `2.1` | Extract ZIP archives from GitHub releases | Default features |
| `bytes` | `1.5` | Efficient byte buffer handling for downloads | Default features |

**Existing Dependencies** (already in project):
- `anyhow` - Error handling
- `dirs` - User directory paths (`~/Library/Fonts/`, etc.)
- `indicatif` - Progress bars for downloads
- `log` - Logging
- `serde` / `toml` - Manifest serialization

**External System Dependencies**:

| Dependency | Platform | Required? | Purpose | Fallback |
|------------|----------|-----------|---------|----------|
| `fc-cache` | Linux | Recommended | Refresh font cache | Warn if missing, installation still works |
| `fc-list` | Linux | Optional | Verify font installation | Skip verification if missing |
| GitHub API | All | Required | Download fonts | Show manual download link if unavailable |
| Internet connection | All | Required | Download fonts | Graceful error with retry option |

**Integration Points**:

1. **`src/prompts/engine.rs`** (Prompt Engine Registry)
   - Add `requires_nerd_font: bool` field to each engine
   - Mark Starship, Powerlevel10k, Oh-My-Posh, Spaceship as requiring fonts
   - Mark Pure as not requiring fonts

2. **`src/core/manifest.rs`** (Profile Manifest)
   - Add `nerd_font: Option<String>` to `[prompt]` section
   - Add `nerd_font_skipped: Option<bool>` to `[prompt]` section
   - Update serialization/deserialization

3. **`src/cli/create.rs`** (Profile Creation)
   - Add `handle_font_installation()` call after prompt engine selection
   - Pass selected font to manifest builder

4. **`src/cli/mod.rs`** (CLI Router)
   - Register new `font` subcommand
   - Route to `src/cli/font.rs`

5. **`src/frameworks/installer.rs`** (Platform Detection Pattern)
   - Reference existing pattern for platform-specific operations
   - Reuse platform detection logic if exposed

**External API Contract** (nerdfonts.com):

Download URLs follow this pattern:
```
https://github.com/ryanoasis/nerd-fonts/releases/download/v{version}/{FontName}.zip
```

Example:
```
https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/FiraCode.zip
```

**No API calls** - direct HTTP download of static assets. Version `v3.1.1` is current stable (as of 2025-11).

## Acceptance Criteria (Authoritative)

1. **Font Registry**: `src/fonts/nerd_fonts.rs` defines a static registry of 5-6 Nerd Fonts with complete metadata (name, description, download URL, preview chars, format, recommendations)

2. **Prompt Engine Metadata**: All prompt engines in `src/prompts/engine.rs` have `requires_nerd_font` boolean field accurately set (Starship=true, Powerlevel10k=true, Oh-My-Posh=true, Spaceship=true, Pure=false)

3. **Font Detection**: `fonts::has_nerd_font_installed()` correctly detects existing Nerd Font installations on macOS (`~/Library/Fonts/`, `/Library/Fonts/`) and Linux (`~/.local/share/fonts/`, `/usr/share/fonts/`)

4. **Font Selection TUI**: When Nerd Font required but not installed, a TUI displays available fonts with preview characters, highlights recommended fonts, and allows skipping

5. **Font Download**: Selected font downloads from GitHub with visible progress bar, HTTP retry on failure (2 attempts), and file size validation (1MB < size < 50MB)

6. **ZIP Extraction**: Downloaded ZIP archive extracts to temporary directory with all `.ttf`/`.otf` files identified

7. **Font Installation**: Font files copy to platform-specific directory (`~/Library/Fonts/` on macOS, `~/.local/share/fonts/` on Linux) with correct permissions (0644)

8. **Font Cache Refresh**: On Linux, `fc-cache -fv` runs after installation (graceful degradation if command not found); on macOS, optional `fc-cache` for XQuartz compatibility

9. **Terminal Detection**: `$TERM_PROGRAM` environment variable used to detect terminal (iTerm2, Apple_Terminal, vscode, etc.) with fallback to "Unknown"

10. **Configuration Instructions**: Terminal-specific instructions displayed after successful installation, covering iTerm2, Terminal.app, VS Code, Alacritty, Kitty, GNOME Terminal, Konsole, and generic fallback

11. **Manifest Storage**: Profile manifest stores selected font in `[prompt]` section with `nerd_font = "FontName"` or `nerd_font_skipped = true` if user skipped

12. **Create Workflow Integration**: `zprof create` workflow prompts for font installation after prompt engine selection, only when engine requires fonts and none detected, without blocking profile creation on skip/failure

13. **Font List Command**: `zprof font list` displays installed Nerd Fonts with file paths and usage by profiles

14. **Font Install Command**: `zprof font install` launches interactive font selection and installation flow (same as create workflow)

15. **Font Info Command**: `zprof font info <name>` displays font details including download URL, recommendations, installation status, and terminal config instructions

16. **Skip Behavior**: If user skips font installation, profile creation continues with warning message and manifest flag `nerd_font_skipped = true`

17. **Error Handling**: All font operations (download, extraction, installation) have clear error messages with actionable suggestions (check internet, verify permissions, etc.)

18. **Platform Support**: Font installation works on macOS and Linux, with clear error message on unsupported platforms (Windows) directing to manual installation docs

19. **Non-Destructive**: Font installation never modifies existing fonts, never requires sudo/root, only writes to user directories

20. **Test Coverage**: Unit tests for font registry queries, detection logic (with mocks), platform detection; integration tests for download (marked slow), installation (with tempdir), and TUI interaction

## Traceability Mapping

| AC # | Spec Section | Component/API | Test Strategy |
|------|-------------|---------------|---------------|
| 1 | Data Models → NerdFont Struct | `src/fonts/nerd_fonts.rs::NERD_FONTS` | Unit test: verify registry has 5-6 fonts with all required fields |
| 2 | Data Models → PromptEngine Enhancement | `src/prompts/engine.rs::PromptEngine::requires_nerd_font` | Unit test: verify Starship/P10k/OMP return true, Pure returns false |
| 3 | APIs → Detection | `src/fonts/detector.rs::has_nerd_font_installed()` | Integration test: mock filesystem with/without fonts in standard paths |
| 4 | Workflows → Profile Creation (steps 6-7) | `src/tui/font_select.rs` | Snapshot test: TUI output with recommended fonts highlighted |
| 5 | APIs → Download | `src/fonts/download.rs::download_font()` | Integration test (slow): download real font, verify progress callback, test retry |
| 6 | APIs → Download | `src/fonts/download.rs::extract_zip()` | Unit test: extract test ZIP, verify .ttf files found |
| 7 | APIs → Installation | `src/fonts/installer.rs::install_font()` | Integration test: install to tempdir, verify files copied with 0644 perms |
| 8 | APIs → Installation | `src/fonts/installer.rs::refresh_font_cache()` | Unit test: verify fc-cache command constructed, graceful failure if missing |
| 9 | APIs → Terminal Configuration | `src/fonts/terminal_config.rs::detect_terminal()` | Unit test: mock $TERM_PROGRAM values, verify enum mapping |
| 10 | APIs → Terminal Configuration | `src/fonts/terminal_config.rs::generate_instructions()` | Snapshot test: instructions for each terminal type match expected format |
| 11 | Data Models → Manifest Enhancement | `src/core/manifest.rs::Manifest::prompt.nerd_font` | Unit test: serialize/deserialize TOML with font field |
| 12 | Workflows → Profile Creation (full flow) | `src/cli/create.rs::handle_font_installation()` | Integration test: create profile with/without font requirement, verify prompting |
| 13 | APIs → CLI Commands | `src/cli/font.rs::list_fonts()` | Integration test: install fonts to tempdir, verify list output |
| 14 | APIs → CLI Commands | `src/cli/font.rs::install_font_interactive()` | Integration test: invoke command, verify TUI launched |
| 15 | APIs → CLI Commands | `src/cli/font.rs::show_font_info()` | Snapshot test: output matches expected format with all details |
| 16 | Workflows → Skip Path (steps 7-10) | `src/cli/create.rs::handle_font_installation()` | Integration test: select skip, verify warning shown and manifest flag set |
| 17 | NFR → Reliability/Error Recovery | All `src/fonts/*.rs` error paths | Unit tests: network failure, permission denied, disk full, malformed ZIP |
| 18 | NFR → Platform Support | `src/fonts/installer.rs::Platform::detect()` | Unit test: conditional compilation for macOS/Linux, runtime check for Windows |
| 19 | System Architecture → Safety Constraints | All `src/fonts/*.rs` file operations | Code review: verify no use of system dirs, no sudo calls, backup pattern |
| 20 | Overview → Testing Strategy | All test files | CI: run unit tests, run integration tests, check coverage report |

## Risks, Assumptions, Open Questions

### Risks

**Risk 1: GitHub Download Failures**
- **Impact**: Users unable to install fonts due to network issues or GitHub outages
- **Probability**: Medium (network issues common, GitHub outages rare)
- **Mitigation**:
  - Implement retry logic with exponential backoff (2 retries)
  - Provide manual download link in error message
  - Clear error messages distinguishing network vs GitHub issues
  - Consider future enhancement: cache popular fonts locally or offer CDN mirrors

**Risk 2: Font Cache Refresh Issues**
- **Impact**: Fonts installed but not immediately available in terminal
- **Probability**: Low on macOS, Medium on Linux (depends on distro)
- **Mitigation**:
  - Graceful degradation: warn if `fc-cache` missing but continue
  - Terminal instructions include manual cache refresh steps
  - Detect and handle permission errors gracefully
  - Test on major Linux distros (Ubuntu, Fedora, Arch)

**Risk 3: Terminal Detection Inaccuracy**
- **Impact**: Users shown incorrect terminal configuration instructions
- **Probability**: Medium (`$TERM_PROGRAM` not set by all terminals)
- **Mitigation**:
  - Provide generic fallback instructions
  - Include link to comprehensive documentation
  - Allow users to request instructions for specific terminal via `zprof font info`
  - Future: allow manual terminal selection in TUI

**Risk 4: Large Download Sizes**
- **Impact**: Slow downloads (15-20MB per font), user frustration
- **Probability**: High on slow connections
- **Mitigation**:
  - Show accurate progress bar with speed and ETA
  - Allow cancellation (Ctrl+C) during download
  - Only download selected font (not entire Nerd Fonts collection)
  - Document expected download sizes upfront in TUI

**Risk 5: Platform-Specific Installation Differences**
- **Impact**: Font installs on one platform but not another
- **Probability**: Medium (different filesystem layouts, permissions)
- **Mitigation**:
  - Extensive integration testing on macOS and major Linux distros
  - Platform detection with explicit error on unsupported platforms
  - Verify installation with file existence checks
  - Comprehensive troubleshooting documentation

### Assumptions

**Assumption 1**: Users have write permissions to `~/Library/Fonts/` (macOS) or `~/.local/share/fonts/` (Linux)
- **Validation**: Check permissions before installation, provide clear error if denied
- **Impact if false**: Installation fails, but we can guide user to fix permissions

**Assumption 2**: Nerd Fonts GitHub release URLs remain stable
- **Validation**: Hardcode version `v3.1.1` URLs, test downloads in CI
- **Impact if false**: Downloads break, require code update to new URL pattern
- **Monitoring**: Manual check for new Nerd Fonts releases quarterly

**Assumption 3**: ZIP extraction is consistent across Nerd Fonts releases
- **Validation**: Test extraction with all 6 fonts in registry
- **Impact if false**: Extraction fails, user sees clear error
- **Mitigation**: Validate ZIP structure during extraction

**Assumption 4**: Users understand they need to restart terminal after font installation
- **Validation**: Explicitly state in terminal configuration instructions
- **Impact if false**: Users frustrated by fonts not appearing immediately
- **Mitigation**: Bold text in instructions: "**Restart your terminal**"

**Assumption 5**: `dialoguer` or `ratatui` TUI patterns are sufficient for font selection
- **Validation**: Prototype TUI early in implementation (Story 4.4)
- **Impact if false**: Need to adjust TUI approach
- **Mitigation**: Follow existing `src/tui/` patterns from framework selection

### Open Questions

**Question 1**: Should we support font variants (Regular, Bold, Italic)?
- **Current approach**: Install all variants from ZIP (simpler, more complete)
- **Alternative**: Only install Regular variant (smaller download)
- **Decision needed by**: Story 4.5 (Download) implementation
- **Recommendation**: Install all variants - users expect fonts to work in all contexts

**Question 2**: How to handle multiple Nerd Fonts already installed?
- **Current approach**: If any Nerd Font detected, skip installation prompt
- **Alternative**: Still offer to install additional fonts
- **Decision needed by**: Story 4.3 (Detection) implementation
- **Recommendation**: Skip prompt but mention in output "Nerd Font detected: FiraCode" so users know

**Question 3**: Should we track which profiles use which fonts?
- **Current approach**: Yes - manifest stores font name, `zprof font list` shows usage
- **Alternative**: Don't track usage (simpler)
- **Decision needed by**: Story 4.9 (Font Management Command) implementation
- **Recommendation**: Track usage - helps users understand dependencies when managing fonts

**Question 4**: Should font installation be available in preset wizard?
- **Current approach**: Integrated into standard create workflow
- **Consideration**: Preset wizard (Epic 2) may select prompt engine → should trigger font check
- **Decision needed by**: Epic 2 completion / integration point
- **Recommendation**: Yes, add same font check hook to preset-based creation path

**Question 5**: Should we validate font files after installation (e.g., check they're valid .ttf)?
- **Current approach**: Basic validation (file size, extension)
- **Alternative**: Deep validation using font parsing library
- **Decision needed by**: Story 4.6 (Installation) implementation
- **Recommendation**: Basic validation only - avoid adding heavy font parsing dependencies

## Test Strategy Summary

### Test Levels

**Unit Tests** (`#[cfg(test)] mod tests`):
- Font registry queries (`get_all_fonts`, `get_recommended_fonts`, `get_font_by_id`)
- Platform detection logic (macOS/Linux/Unsupported)
- Terminal detection from `$TERM_PROGRAM` values
- Font name validation and sanitization
- URL parsing and validation
- File permission helpers
- Error message formatting

**Integration Tests** (`tests/` directory):
- Font detection with mock filesystem (create temp dirs with/without fonts)
- Font installation to temporary directories (verify files copied, permissions set)
- ZIP extraction with real/mock ZIP files
- TUI interaction simulation (select font, skip)
- CLI command execution (`font list`, `font info`)
- Create workflow with font installation (end-to-end)
- Error scenarios (network failure, disk full, permission denied)

**Snapshot Tests** (`insta` crate):
- TUI output for font selection screen
- Terminal configuration instructions for each terminal type
- `zprof font list` output format
- `zprof font info` output format
- Error messages

**Manual Tests** (pre-release validation):
- Real font download from GitHub (verify progress bar, speed)
- Installation on macOS (Intel, Apple Silicon)
- Installation on Linux (Ubuntu 22.04, Fedora 39, Arch)
- Terminal configuration verification (iTerm2, Terminal.app, VS Code, Alacritty, Kitty)
- Profile creation with various prompt engines
- Font selection UX (keyboard navigation, preview chars display)

### Coverage Targets

- **Critical paths**: 100% (font download, installation, manifest storage)
- **Business logic**: 90% (detection, TUI, CLI commands)
- **Error handling**: 80% (all error branches exercised)
- **Overall**: 85%+ line coverage

### Test Data

**Mock Font ZIP Structure**:
```
FiraCodeNerdFont.zip
├── FiraCodeNerdFont-Regular.ttf
├── FiraCodeNerdFont-Bold.ttf
├── FiraCodeNerdFont-Italic.ttf
└── README.md (ignored)
```

**Mock Font Directories** (integration tests):
```
temp_home/
├── Library/Fonts/           # macOS
│   └── SomeNerdFont.ttf
└── .local/share/fonts/      # Linux
    └── AnotherNerdFont.ttf
```

### Continuous Integration

- Run unit tests on every commit
- Run integration tests (excluding slow downloads) on every commit
- Run slow tests (real downloads) nightly
- Run manual test checklist before release
- Clippy checks with `--deny warnings`
- Format checks with `rustfmt`
- Snapshot review required for any UI changes

### Performance Testing

- Download speed: Test with throttled connection (1 Mbps, 5 Mbps, 10 Mbps)
- Installation time: Measure on HDD vs SSD
- Detection time: Test with 0, 10, 100 fonts in system directories
- TUI responsiveness: Measure render time with 6 fonts in registry

### Accessibility Testing

- TUI keyboard navigation works without mouse
- Progress bars provide textual feedback (not just visual)
- Error messages clear for users unfamiliar with fonts
- Terminal instructions avoid jargon
