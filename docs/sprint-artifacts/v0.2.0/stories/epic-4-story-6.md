# Story 4.6: Implement Platform-Specific Font Installation

Status: ready-for-dev

## Story

As a user on macOS or Linux,
I want fonts automatically installed to the correct location,
so that they're available system-wide.

## Acceptance Criteria

1. Create `src/fonts/installer.rs`
2. Implement macOS installation (copy to `~/Library/Fonts/`, run `fc-cache`)
3. Implement Linux installation (copy to `~/.local/share/fonts/`, run `fc-cache -fv`)
4. Show installation progress with clear status messages
5. Handle permission errors gracefully
6. Offer manual installation instructions if auto-install fails
7. Return installation result (success, partial, failed)
8. Verify files copied successfully
9. Add unit tests with mock filesystem
10. Add platform-specific integration tests

## Dev Agent Context

### Story Requirements from Epic

This story implements platform-specific font installation after download (Story 4.5). It must handle macOS and Linux differences, refresh font caches, verify installation, and provide clear error messages with fallback to manual instructions.

**Key User Flow:**
1. User has downloaded font via Story 4.5 → `DownloadResult` with `font_files: Vec<PathBuf>`
2. Call `install_font(&download_result)` with downloaded font files
3. Detect platform (macOS, Linux, or Unsupported)
4. Copy font files to platform-specific directory
5. Refresh font cache (fc-cache on Linux, optional on macOS)
6. Verify installation succeeded
7. Show success message with file count and location
8. Return `InstallationResult` with details

**Expected Output Format:**
```
Installing FiraCode Nerd Font...
✓ Copied 12 font files to ~/Library/Fonts/
✓ Updated font cache
✓ Installation complete
```

**Platform-Specific Directories:**
- **macOS**: `~/Library/Fonts/` (user-level, no sudo required)
- **Linux**: `~/.local/share/fonts/` (user-level, XDG standard)
- **Windows**: Not supported in v0.2.0 (show manual instructions link)

### Architecture Compliance

**Module Location:** `src/fonts/installer.rs` (NEW)
- Part of `src/fonts/` module structure
- Follows platform detection pattern from `src/frameworks/installer.rs`
- Uses `dirs` crate for home directory paths (already in dependencies)
- Integrates with `std::process::Command` for `fc-cache` execution
- No sudo/elevation required (user-level installation only)

**Public API Design:**
```rust
// Main installation function
pub fn install_font(download_result: &DownloadResult) -> Result<InstallationResult>

// Platform detection
pub enum Platform {
    MacOS,
    Linux,
    Unsupported,
}

impl Platform {
    pub fn detect() -> Self
    pub fn font_dir(&self) -> Result<PathBuf>
}

// Installation result with detailed status
pub struct InstallationResult {
    pub success: bool,
    pub files_installed: usize,
    pub install_path: PathBuf,
    pub errors: Vec<String>,
    pub manual_instructions: Option<String>,
}
```

**Error Handling Strategy:**
```rust
use anyhow::{bail, Context, Result};

// Error types to handle:
// - Platform unsupported (Windows in v0.2.0)
// - Permission denied (font directory not writable)
// - Disk full (copy fails)
// - fc-cache not found (warn but continue)
// - fc-cache fails (warn but continue)
// - Partial installation (some files copied, others failed)
```

**NFR from Tech Spec:**
- Installation time: < 5 seconds for 12 font files
- File permissions: 0644 (rw-r--r--)
- Font cache refresh: graceful degradation if fc-cache missing
- Verification: Check file existence after copy
- Non-destructive: Never overwrite existing fonts, never require sudo

### Library and Framework Requirements

**Dependencies (already in Cargo.toml):**
```toml
anyhow = "1.0"           # Error handling
dirs = "5.0"             # Home directory paths
log = "0.4"              # Logging
```

**Required Imports:**
```rust
use anyhow::{bail, Context, Result};
use std::fs::{self, File};
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::os::unix::fs::PermissionsExt;  // For Unix-like systems

use crate::fonts::download::DownloadResult;
```

**Platform Detection Pattern (from src/frameworks/installer.rs):**
```rust
impl Platform {
    pub fn detect() -> Self {
        #[cfg(target_os = "macos")]
        return Platform::MacOS;

        #[cfg(target_os = "linux")]
        return Platform::Linux;

        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        return Platform::Unsupported;
    }
}
```

**Font Cache Commands:**
- **macOS**: `fc-cache -f` (optional, mainly for XQuartz compatibility)
  - May not be installed by default
  - Graceful degradation if missing
- **Linux**: `fc-cache -fv` (recommended, required for most distros)
  - Usually pre-installed on desktop Linux
  - Warn if missing but installation still works
  - `-f` forces refresh, `-v` verbose output

### File Structure Requirements

**New File:** `src/fonts/installer.rs`

**Module Export:** Add to `src/fonts/mod.rs`:
```rust
pub mod installer;
pub use installer::{install_font, InstallationResult, Platform};
```

**Test File:** `tests/font_install_test.rs` (NEW)

**Naming Conventions:**
- Function names: snake_case (`install_font`, `copy_font_files`, `refresh_font_cache`)
- Struct names: PascalCase (`InstallationResult`, `Platform`)
- Enum variants: PascalCase (`MacOS`, `Linux`, `Unsupported`)
- Constants: SCREAMING_SNAKE_CASE (`FONT_FILE_PERMISSIONS`, `FONT_CACHE_CMD`)

### Testing Requirements

**Unit Tests** (in `src/fonts/installer.rs` under `#[cfg(test)]`):
1. Test platform detection (conditional compilation)
2. Test font directory resolution (macOS vs Linux)
3. Test file permission validation
4. Test error messages for unsupported platform
5. Test partial installation handling (some files succeed, others fail)
6. Test manual instructions generation

**Integration Tests** (`tests/font_install_test.rs`):
1. Test installation to temporary directory
   - Create temp dir mimicking `~/Library/Fonts/`
   - Copy font files
   - Verify file count and permissions
2. Test font cache refresh (mock fc-cache if needed)
3. Test permission denied scenario
4. Test disk full scenario (mock with limited quota)
5. Platform-specific tests (use `#[cfg(target_os = "...")]`)

**Test Pattern for Mock Filesystem:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_install_font_success() {
        // Create temp directory as mock font directory
        let temp_dir = TempDir::new().unwrap();
        let font_dir = temp_dir.path().join("fonts");
        fs::create_dir_all(&font_dir).unwrap();

        // Create mock font files
        let mock_fonts = create_mock_font_files(&temp_dir);

        // Mock DownloadResult
        let download_result = DownloadResult {
            temp_dir: temp_dir.path().to_path_buf(),
            font_files: mock_fonts,
            total_size: 1_000_000,
        };

        // Install
        let result = install_font_to_dir(&download_result, &font_dir).unwrap();

        assert!(result.success);
        assert_eq!(result.files_installed, mock_fonts.len());
        assert!(result.errors.is_empty());
    }

    #[test]
    fn test_platform_unsupported_error() {
        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        {
            let platform = Platform::detect();
            assert!(matches!(platform, Platform::Unsupported));

            let result = platform.font_dir();
            assert!(result.is_err());
        }
    }
}
```

### Previous Story Intelligence (Story 4.5)

**What Story 4.5 Completed:**
- Font download system (`src/fonts/download.rs`)
- `download_font()` returns `DownloadResult` with extracted font files
- ZIP extraction to temp directory
- Progress bars for download
- File size validation (1MB-50MB)
- Retry logic with exponential backoff

**Integration Point with Story 4.5:**
```rust
use crate::fonts::download::{download_font, DownloadResult};
use crate::fonts::installer::{install_font, InstallationResult};

// After download:
let download_result = download_font(selected_font)?;

// Story 4.6: Install the downloaded fonts
let install_result = install_font(&download_result)?;

if install_result.success {
    println!("✓ Font installed successfully!");
} else {
    eprintln!("⚠ Partial installation. {} files installed, {} errors",
              install_result.files_installed,
              install_result.errors.len());

    if let Some(manual) = install_result.manual_instructions {
        println!("\n{}", manual);
    }
}

// Cleanup download temp files
fs::remove_dir_all(&download_result.temp_dir)?;
```

**Key Learnings:**
- Progress feedback is critical for user experience
- Error messages must be actionable (not just "failed")
- Cleanup is important (temp files, failed installations)
- Testing with real filesystem operations requires tempfile crate

### Git Intelligence (Recent Commits)

**Recent Work Pattern:**
```
880cc58 Apply code review fixes to font detection (Story 4.3)
afd2dde Implement Nerd Font detection system (Story 4.3)
02afd3c Implement Nerd Font registry (Story 4.1)
```

**Code Patterns Observed:**
1. **Platform detection** (from `src/frameworks/installer.rs`):
   ```rust
   #[cfg(target_os = "macos")]
   fn platform_specific() { }

   #[cfg(target_os = "linux")]
   fn platform_specific() { }
   ```

2. **Progress indication:**
   ```rust
   println!("Installing fonts...");
   // ... do work ...
   println!("✓ Installation complete");
   ```

3. **Comprehensive error messages:**
   ```rust
   bail!(
       "✗ Error: Could not install font to {}\n  → Check write permissions\n  → Try: sudo chown -R $USER {}",
       path.display(),
       path.display()
   );
   ```

### Latest Tech Information

**Rust File Operations (std::fs):**
- **Copy files**: `fs::copy(src, dest)` copies single file
- **Create directories**: `fs::create_dir_all(path)` creates parent dirs
- **Set permissions**: Unix-specific, use `PermissionsExt` trait
  ```rust
  #[cfg(unix)]
  use std::os::unix::fs::PermissionsExt;

  let mut perms = fs::metadata(&file)?.permissions();
  perms.set_mode(0o644);  // rw-r--r--
  fs::set_permissions(&file, perms)?;
  ```

**Font Cache Refresh (fc-cache):**
- Part of fontconfig package (Linux standard)
- May not be present on minimal macOS installs
- Command: `fc-cache -fv`
  - `-f`: Force refresh (ignore timestamps)
  - `-v`: Verbose output
- Exit codes:
  - `0`: Success
  - `1`: General error
  - `127`: Command not found

**Best Practices:**
1. **Check before overwrite**: Don't overwrite existing fonts silently
2. **Atomic operations**: Use temp file + rename for critical files
3. **Verify permissions**: Check directory is writable before copying
4. **Graceful degradation**: fc-cache failure should warn, not error
5. **User-friendly paths**: Use `~` in output, not full `/Users/...`

### Implementation Guidance

**Constants:**
```rust
const FONT_FILE_PERMISSIONS: u32 = 0o644;  // rw-r--r--

#[cfg(target_os = "macos")]
const FONT_CACHE_CMD: &str = "fc-cache";
#[cfg(target_os = "macos")]
const FONT_CACHE_ARGS: &[&str] = &["-f"];

#[cfg(target_os = "linux")]
const FONT_CACHE_CMD: &str = "fc-cache";
#[cfg(target_os = "linux")]
const FONT_CACHE_ARGS: &[&str] = &["-fv"];
```

**Platform Enum Implementation:**
```rust
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Platform {
    MacOS,
    Linux,
    Unsupported,
}

impl Platform {
    /// Detect the current platform at compile time
    pub fn detect() -> Self {
        #[cfg(target_os = "macos")]
        return Platform::MacOS;

        #[cfg(target_os = "linux")]
        return Platform::Linux;

        #[cfg(not(any(target_os = "macos", target_os = "linux")))]
        return Platform::Unsupported;
    }

    /// Get the user-level font directory for this platform
    pub fn font_dir(&self) -> Result<PathBuf> {
        match self {
            Platform::MacOS => {
                let home = dirs::home_dir()
                    .context("Could not determine home directory")?;
                Ok(home.join("Library/Fonts"))
            }
            Platform::Linux => {
                let home = dirs::home_dir()
                    .context("Could not determine home directory")?;
                Ok(home.join(".local/share/fonts"))
            }
            Platform::Unsupported => {
                bail!(
                    "Automatic font installation is not supported on this platform.\n\
                     Please install Nerd Fonts manually:\n\
                     → Visit https://www.nerdfonts.com/font-downloads\n\
                     → Download your preferred font\n\
                     → Follow your OS installation instructions"
                );
            }
        }
    }

    /// Get the display name of the font directory (with tilde)
    pub fn font_dir_display(&self) -> &str {
        match self {
            Platform::MacOS => "~/Library/Fonts",
            Platform::Linux => "~/.local/share/fonts",
            Platform::Unsupported => "<unsupported>",
        }
    }
}
```

**InstallationResult Struct:**
```rust
#[derive(Debug, Clone)]
pub struct InstallationResult {
    pub success: bool,
    pub files_installed: usize,
    pub install_path: PathBuf,
    pub errors: Vec<String>,
    pub manual_instructions: Option<String>,
}

impl InstallationResult {
    fn success(files_installed: usize, install_path: PathBuf) -> Self {
        Self {
            success: true,
            files_installed,
            install_path,
            errors: Vec::new(),
            manual_instructions: None,
        }
    }

    fn partial(files_installed: usize, install_path: PathBuf, errors: Vec<String>) -> Self {
        Self {
            success: files_installed > 0,
            files_installed,
            install_path,
            errors,
            manual_instructions: Some(generate_manual_instructions()),
        }
    }

    fn failed(error: String) -> Self {
        Self {
            success: false,
            files_installed: 0,
            install_path: PathBuf::new(),
            errors: vec![error],
            manual_instructions: Some(generate_manual_instructions()),
        }
    }
}
```

**Main Installation Function:**
```rust
/// Install downloaded fonts to the system font directory
///
/// This function performs the following steps:
/// 1. Detect platform (macOS, Linux, or Unsupported)
/// 2. Get platform-specific font directory
/// 3. Create directory if doesn't exist
/// 4. Copy font files with correct permissions
/// 5. Refresh font cache (fc-cache)
/// 6. Verify installation
///
/// # Errors
/// - Platform unsupported (Windows in v0.2.0)
/// - Permission denied writing to font directory
/// - Disk full or I/O errors during copy
///
/// # Examples
/// ```no_run
/// let download_result = download_font(font)?;
/// let install_result = install_font(&download_result)?;
///
/// if install_result.success {
///     println!("✓ Installed {} fonts", install_result.files_installed);
/// }
/// ```
pub fn install_font(download_result: &DownloadResult) -> Result<InstallationResult> {
    let platform = Platform::detect();

    log::info!("Installing fonts on platform: {:?}", platform);

    // Get font directory (this will error on unsupported platforms)
    let font_dir = platform.font_dir()?;

    // Create directory if doesn't exist
    if !font_dir.exists() {
        fs::create_dir_all(&font_dir)
            .context(format!("Failed to create font directory: {}", font_dir.display()))?;
        log::debug!("Created font directory: {}", font_dir.display());
    }

    // Verify directory is writable
    verify_writable(&font_dir)?;

    println!("Installing fonts to {}...", platform.font_dir_display());

    // Copy font files
    let (files_installed, errors) = copy_font_files(&download_result.font_files, &font_dir)?;

    if files_installed == 0 {
        return Ok(InstallationResult::failed(
            "Failed to install any font files".to_string()
        ));
    }

    // Show progress
    println!("✓ Copied {} font files to {}", files_installed, platform.font_dir_display());

    // Refresh font cache
    match refresh_font_cache(&platform) {
        Ok(_) => println!("✓ Updated font cache"),
        Err(e) => {
            log::warn!("Font cache refresh failed (non-critical): {}", e);
            println!("⚠ Could not refresh font cache (fonts may not be immediately available)");
        }
    }

    println!("✓ Installation complete");

    if !errors.is_empty() {
        Ok(InstallationResult::partial(files_installed, font_dir, errors))
    } else {
        Ok(InstallationResult::success(files_installed, font_dir))
    }
}
```

**Copy Font Files:**
```rust
fn copy_font_files(font_files: &[PathBuf], dest_dir: &Path) -> Result<(usize, Vec<String>)> {
    let mut installed = 0;
    let mut errors = Vec::new();

    for font_file in font_files {
        let filename = font_file
            .file_name()
            .context("Invalid font filename")?;
        let dest_path = dest_dir.join(filename);

        // Check if file already exists
        if dest_path.exists() {
            log::debug!("Font file already exists, skipping: {}", dest_path.display());
            continue;
        }

        // Copy file
        match fs::copy(font_file, &dest_path) {
            Ok(_) => {
                log::debug!("Copied font: {}", dest_path.display());

                // Set permissions (Unix only)
                #[cfg(unix)]
                if let Err(e) = set_font_permissions(&dest_path) {
                    log::warn!("Failed to set permissions on {}: {}", dest_path.display(), e);
                }

                installed += 1;
            }
            Err(e) => {
                let error_msg = format!("Failed to copy {}: {}", filename.to_string_lossy(), e);
                log::error!("{}", error_msg);
                errors.push(error_msg);
            }
        }
    }

    Ok((installed, errors))
}

#[cfg(unix)]
fn set_font_permissions(path: &Path) -> Result<()> {
    use std::os::unix::fs::PermissionsExt;

    let mut perms = fs::metadata(path)?.permissions();
    perms.set_mode(FONT_FILE_PERMISSIONS);
    fs::set_permissions(path, perms)?;

    Ok(())
}
```

**Verify Directory Writable:**
```rust
fn verify_writable(dir: &Path) -> Result<()> {
    // Try to create a temp file in the directory
    let test_file = dir.join(".zprof-install-test");

    match File::create(&test_file) {
        Ok(_) => {
            // Clean up test file
            let _ = fs::remove_file(&test_file);
            Ok(())
        }
        Err(e) => {
            bail!(
                "Font directory is not writable: {}\n  → Error: {}\n  → Try: chmod u+w {}",
                dir.display(),
                e,
                dir.display()
            );
        }
    }
}
```

**Font Cache Refresh:**
```rust
fn refresh_font_cache(platform: &Platform) -> Result<()> {
    let cmd = FONT_CACHE_CMD;
    let args = match platform {
        Platform::MacOS => &["-f"][..],
        Platform::Linux => &["-fv"][..],
        Platform::Unsupported => return Ok(()), // No-op
    };

    log::debug!("Running font cache refresh: {} {}", cmd, args.join(" "));

    match Command::new(cmd).args(args).output() {
        Ok(output) if output.status.success() => {
            log::debug!("Font cache refreshed successfully");
            if !output.stdout.is_empty() {
                log::debug!("fc-cache output: {}", String::from_utf8_lossy(&output.stdout));
            }
            Ok(())
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            bail!("fc-cache failed with status {}: {}", output.status, stderr);
        }
        Err(e) if e.kind() == io::ErrorKind::NotFound => {
            // fc-cache not installed (common on macOS)
            log::debug!("fc-cache not found (this is normal on some systems)");
            Ok(()) // Graceful degradation
        }
        Err(e) => {
            bail!("Failed to execute fc-cache: {}", e);
        }
    }
}
```

**Manual Instructions:**
```rust
fn generate_manual_instructions() -> String {
    let platform = Platform::detect();

    format!(
        "\nManual Installation Instructions:\n\
         \n\
         1. Download the font from https://www.nerdfonts.com/font-downloads\n\
         2. Extract the ZIP archive\n\
         3. Copy .ttf/.otf files to: {}\n\
         4. {}\n\
         5. Restart your terminal\n",
        platform.font_dir_display(),
        match platform {
            Platform::MacOS => "Fonts are ready (no cache refresh needed)",
            Platform::Linux => "Run: fc-cache -fv",
            Platform::Unsupported => "Follow your OS instructions to refresh font cache",
        }
    )
}
```

### Critical Reminders

**DO:**
- ✅ Follow platform detection pattern from `src/frameworks/installer.rs`
- ✅ Use `dirs::home_dir()` for home directory resolution
- ✅ Create font directory if it doesn't exist
- ✅ Verify directory is writable before copying
- ✅ Set file permissions to 0644 on Unix systems
- ✅ Gracefully handle fc-cache missing (warn, don't error)
- ✅ Skip existing font files (don't overwrite)
- ✅ Return detailed `InstallationResult` with errors
- ✅ Provide manual instructions on failure
- ✅ Log all operations for debugging
- ✅ Show user-friendly progress messages (with ✓ checkmarks)
- ✅ Use tilde (~) in displayed paths
- ✅ Write comprehensive tests with tempfile

**DON'T:**
- ❌ Don't require sudo/elevation (user-level installation only)
- ❌ Don't overwrite existing fonts without warning
- ❌ Don't fail hard if fc-cache missing (graceful degradation)
- ❌ Don't install to system directories (/usr/share/fonts)
- ❌ Don't assume font directory exists (create if needed)
- ❌ Don't skip permission checks
- ❌ Don't leave temp files behind
- ❌ Don't implement terminal configuration here (that's Story 4.7)
- ❌ Don't hardcode paths (use dirs crate)
- ❌ Don't forget platform-specific testing

### Acceptance Criteria Expanded

1. **Create `src/fonts/installer.rs`**
   - New file in `src/fonts/` directory
   - Export via `src/fonts/mod.rs`: `pub mod installer;`
   - Public API: `install_font()`, `Platform`, `InstallationResult`

2. **Implement macOS installation**
   - Detect platform with `#[cfg(target_os = "macos")]`
   - Font directory: `~/Library/Fonts/`
   - Copy .ttf/.otf files to directory
   - Optional: Run `fc-cache -f` (for XQuartz)
   - Verify files exist after copy

3. **Implement Linux installation**
   - Detect platform with `#[cfg(target_os = "linux")]`
   - Font directory: `~/.local/share/fonts/`
   - Create directory if doesn't exist
   - Copy .ttf/.otf files to directory
   - Run `fc-cache -fv` to refresh cache
   - Graceful degradation if fc-cache missing

4. **Show installation progress**
   - Print: "Installing fonts to ~/Library/Fonts/..."
   - Print: "✓ Copied 12 font files to ~/Library/Fonts/"
   - Print: "✓ Updated font cache"
   - Print: "✓ Installation complete"
   - Use tilde (~) in paths for readability

5. **Handle permission errors**
   - Check directory is writable before copying
   - Clear error message if permission denied
   - Suggest fix: `chmod u+w <directory>`
   - Provide manual installation instructions

6. **Offer manual installation instructions**
   - Generate instructions in `InstallationResult.manual_instructions`
   - Include download URL, extraction steps, directory path
   - Platform-specific cache refresh command
   - Show on any installation failure or partial success

7. **Return installation result**
   - `success: bool` - true if all files installed
   - `files_installed: usize` - count of successfully installed files
   - `install_path: PathBuf` - destination directory
   - `errors: Vec<String>` - list of errors encountered
   - `manual_instructions: Option<String>` - fallback instructions

8. **Verify files copied successfully**
   - Check destination file exists after copy
   - Count successful vs failed files
   - Report partial success if some files installed

9. **Unit tests**
   - Test platform detection (MacOS, Linux, Unsupported)
   - Test font directory resolution for each platform
   - Test permission validation
   - Test manual instructions generation
   - All in `#[cfg(test)] mod tests` section

10. **Integration tests**
    - Create `tests/font_install_test.rs`
    - Test installation to temp directory
    - Test permission denied scenario
    - Test partial installation
    - Platform-specific tests with `#[cfg(target_os = "...")]`

## Tasks / Subtasks

- [ ] Create `src/fonts/installer.rs` (AC: 1)
  - [ ] Module-level documentation
  - [ ] Define constants (permissions, fc-cache commands)
  - [ ] Import all required dependencies
- [ ] Define `Platform` enum (AC: 2, 3)
  - [ ] Variants: MacOS, Linux, Unsupported
  - [ ] Implement `detect()` with conditional compilation
  - [ ] Implement `font_dir()` with platform-specific paths
  - [ ] Implement `font_dir_display()` with tilde notation
- [ ] Define `InstallationResult` struct (AC: 7)
  - [ ] Fields: success, files_installed, install_path, errors, manual_instructions
  - [ ] Helper constructors: success(), partial(), failed()
- [ ] Implement `install_font()` function (AC: 2, 3, 4, 5, 8)
  - [ ] Detect platform
  - [ ] Get font directory
  - [ ] Create directory if doesn't exist
  - [ ] Verify directory is writable
  - [ ] Copy font files
  - [ ] Refresh font cache
  - [ ] Show progress messages
  - [ ] Return installation result
- [ ] Implement `copy_font_files()` (AC: 2, 3, 8)
  - [ ] Iterate over font files
  - [ ] Skip existing files
  - [ ] Copy file to destination
  - [ ] Set permissions (Unix)
  - [ ] Track successes and errors
  - [ ] Return counts and error list
- [ ] Implement `verify_writable()` (AC: 5)
  - [ ] Create test file in directory
  - [ ] Clean up test file
  - [ ] Return clear error if permission denied
- [ ] Implement `refresh_font_cache()` (AC: 2, 3)
  - [ ] Platform-specific fc-cache command
  - [ ] Execute with `Command::new()`
  - [ ] Graceful degradation if not found
  - [ ] Log output for debugging
- [ ] Implement `set_font_permissions()` (AC: 2, 3)
  - [ ] Unix-specific with `#[cfg(unix)]`
  - [ ] Set mode to 0644
  - [ ] Handle errors gracefully
- [ ] Implement `generate_manual_instructions()` (AC: 6)
  - [ ] Platform-specific instructions
  - [ ] Include download URL
  - [ ] Include directory path
  - [ ] Include cache refresh command
- [ ] Write unit tests (AC: 9)
  - [ ] Test platform detection
  - [ ] Test font_dir() for each platform
  - [ ] Test manual instructions generation
  - [ ] Test InstallationResult constructors
- [ ] Write integration tests (AC: 10)
  - [ ] Create `tests/font_install_test.rs`
  - [ ] Test installation to temp directory
  - [ ] Test file permissions verification
  - [ ] Test partial installation handling
- [ ] Export module (AC: 1)
  - [ ] Add `pub mod installer;` to `src/fonts/mod.rs`
  - [ ] Re-export public API
- [ ] Documentation (AC: all)
  - [ ] Module-level docs with usage examples
  - [ ] Function docs with error cases
  - [ ] Struct docs with field descriptions

## Dev Agent Record

### Context Reference

Epic: [epic-4-nerd-fonts.md](../epic-4-nerd-fonts.md)
Tech Spec: [tech-spec-epic-4.md](../../tech-spec-epic-4.md)
Previous Story: [epic-4-story-5.md](epic-4-story-5.md) (Font Download - ready-for-dev)
Platform Pattern Reference: [src/frameworks/installer.rs](../../../src/frameworks/installer.rs)
Download Module: [src/fonts/download.rs](../../../src/fonts/download.rs)

### Agent Model Used

<!-- Will be filled during implementation -->

### Debug Log References

<!-- Will be filled during implementation -->

### Completion Notes List

<!-- Will be filled during implementation -->

### File List

<!-- Will be filled during implementation -->
