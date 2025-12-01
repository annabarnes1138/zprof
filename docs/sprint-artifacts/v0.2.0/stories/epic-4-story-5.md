# Story 4.5: Implement Font Download

Status: ready-for-dev

## Story

As a user who selected a font,
I want it downloaded automatically,
so that I don't have to manually find and download it.

## Acceptance Criteria

1. Create `src/fonts/download.rs`
2. Download font from nerdfonts.com GitHub releases
3. Show progress bar during download with size/speed/ETA
4. Download to temp directory first
5. Verify download (size sanity check 1MB-50MB)
6. Extract ZIP archive and identify font files
7. Handle download errors gracefully (network timeout, invalid URL, incomplete download)
8. Add retry logic (2 retries with exponential backoff)
9. Clean up temp files on success or failure
10. Add unit tests with mock HTTP client
11. Add integration test with real download (marked as slow test)

## Dev Agent Context

### Story Requirements from Epic

This story implements HTTP download of Nerd Font ZIP archives from GitHub releases with progress indication, error handling, and extraction. It's called after the user selects a font in the TUI (Story 4.4).

**Key User Flow:**
1. User selects font from TUI → Returns `FontChoice::Font(&NerdFont)`
2. Call `download_font(font)` with selected font
3. Show progress bar: "Downloading FiraCode Nerd Font... 67% (12.3 MB / 18.4 MB)"
4. Download to temp directory
5. Verify file size is reasonable (1MB-50MB)
6. Extract ZIP to find all `.ttf`/`.otf` files
7. Return path to extracted font files
8. Cleanup happens automatically (temp files deleted)

**Expected Progress Bar Format:**
```
Downloading FiraCode Nerd Font...
████████████████████████░░░░░░░░ 67% (12.3 MB / 18.4 MB)
```

**Download URLs Pattern:**
```
https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/{FontName}.zip

Examples:
https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/FiraCode.zip
https://github.com/ryanoasis/nerd-fonts/releases/download/v3.1.1/JetBrainsMono.zip
```

### Architecture Compliance

**Module Location:** `src/fonts/download.rs` (NEW)
- Part of `src/fonts/` module structure
- Handles HTTP download and ZIP extraction
- Uses `reqwest` for HTTP (NEW DEPENDENCY)
- Uses `zip` crate for extraction (NEW DEPENDENCY)
- Integrates with `indicatif` for progress bars (already in deps)

**Public API Design:**
```rust
// Main download function
pub fn download_font(font: &NerdFont) -> Result<DownloadResult>

// Download result with extracted file paths
pub struct DownloadResult {
    pub temp_dir: PathBuf,          // Temp directory (caller must cleanup or use RAII)
    pub font_files: Vec<PathBuf>,   // Extracted .ttf/.otf files
    pub total_size: u64,            // Total bytes downloaded
}

// Progress callback for custom reporting (optional)
pub fn download_font_with_progress<F>(
    font: &NerdFont,
    progress_callback: F
) -> Result<DownloadResult>
where
    F: Fn(u64, u64) // (downloaded_bytes, total_bytes)
```

**Error Handling Strategy:**
```rust
use anyhow::{bail, Context, Result};

// Error types to handle:
// - Network errors (connection failed, timeout)
// - HTTP errors (404, 500, etc.)
// - File I/O errors (temp dir creation, write permissions)
// - ZIP extraction errors (corrupt archive, invalid format)
// - Verification errors (file too small/large, no fonts found)
```

**NFR from Tech Spec:**
- Download timeout: 30 seconds for HTTP requests
- Retry logic: 2 retries with exponential backoff (1s, 2s delays)
- File size validation: Reject if < 1MB or > 50MB
- Progress updates: Minimum 2 Hz (twice per second)
- Temp cleanup: Always delete on error or success

### Library and Framework Requirements

**NEW Dependencies to Add:**

Add to `Cargo.toml`:
```toml
[dependencies]
reqwest = { version = "0.12", features = ["blocking", "stream"] }
zip = "2.1"
bytes = "1.5"
```

**Existing Dependencies (already in Cargo.toml):**
```toml
indicatif = "0.18"     # Progress bars
anyhow = "1.0"         # Error handling
dirs = "5.0"           # Temp directory paths
log = "0.4"            # Logging
```

**Required Imports:**
```rust
use anyhow::{bail, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::Duration;
use zip::ZipArchive;

use crate::fonts::nerd_fonts::NerdFont;
```

**Reqwest Features:**
- `blocking`: Synchronous HTTP client (simpler than async for this use case)
- `stream`: Stream response body for progress tracking
- Timeout configuration via `ClientBuilder`

**Zip Crate API:**
```rust
// Open ZIP archive from file
let file = File::open(zip_path)?;
let mut archive = ZipArchive::new(file)?;

// Iterate over files
for i in 0..archive.len() {
    let mut file = archive.by_index(i)?;
    let name = file.name();

    // Extract .ttf/.otf files only
    if name.ends_with(".ttf") || name.ends_with(".otf") {
        let outpath = dest_dir.join(name);
        let mut outfile = File::create(&outpath)?;
        io::copy(&mut file, &mut outfile)?;
    }
}
```

### File Structure Requirements

**New File:** `src/fonts/download.rs`

**Module Export:** Add to `src/fonts/mod.rs`:
```rust
pub mod download;
pub use download::{download_font, DownloadResult};
```

**Test File:** `tests/font_download_test.rs` (NEW)

**Naming Conventions:**
- Function names: snake_case (`download_font`, `extract_zip`, `verify_download`)
- Struct names: PascalCase (`DownloadResult`, `RetryConfig`)
- Constants: SCREAMING_SNAKE_CASE (`MAX_FILE_SIZE`, `MIN_FILE_SIZE`, `DOWNLOAD_TIMEOUT`)

### Testing Requirements

**Unit Tests** (in `src/fonts/download.rs` under `#[cfg(test)]`):
1. Test file size validation (reject < 1MB, > 50MB)
2. Test ZIP extraction with mock ZIP file
3. Test font file filtering (only .ttf/.otf extracted)
4. Test retry logic with mock failures
5. Test timeout handling
6. Test cleanup on error paths

**Integration Tests** (`tests/font_download_test.rs`):
1. Test real download from GitHub (marked with `#[ignore]` or `#[test] #[ignore]`)
   - Slow test, only run manually or in CI nightly
   - Download actual FiraCode.zip
   - Verify size and extraction
2. Test with invalid URL (404 error)
3. Test with network timeout (mock slow server)
4. Test with corrupt ZIP file

**Test Pattern for Mock HTTP:**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_verify_download_size_too_small() {
        let result = verify_download_size(500_000); // 500 KB
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too small"));
    }

    #[test]
    fn test_verify_download_size_too_large() {
        let result = verify_download_size(60_000_000); // 60 MB
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too large"));
    }

    #[test]
    fn test_extract_fonts_filters_non_font_files() {
        // Create mock ZIP with .ttf and .txt files
        // Verify only .ttf extracted
    }
}
```

**Integration Test Pattern:**
```rust
// tests/font_download_test.rs
use zprof::fonts::{download_font, nerd_fonts::get_font_by_id};

#[test]
#[ignore] // Slow test - requires network
fn test_download_real_font() {
    let font = get_font_by_id("firacode").expect("Font should exist");
    let result = download_font(font).expect("Download should succeed");

    assert!(!result.font_files.is_empty(), "Should extract at least one font file");
    assert!(result.total_size > 1_000_000, "Download should be > 1MB");

    // Cleanup
    std::fs::remove_dir_all(&result.temp_dir).ok();
}
```

### Previous Story Intelligence (Story 4.4)

**What Story 4.4 Completed:**
- Font selection TUI (`src/tui/font_select.rs`)
- `FontChoice` enum with `Font(&'static NerdFont)` and `Skip` variants
- Returns selected `NerdFont` reference to caller
- User can skip installation

**Integration Point with Story 4.4:**
```rust
use crate::tui::font_select::{select_font, FontChoice};
use crate::fonts::download::download_font;

// After TUI selection:
let choice = select_font()?;
match choice {
    FontChoice::Font(font) => {
        // Story 4.5: Download the selected font
        let download_result = download_font(font)?;
        // ... proceed to installation (Story 4.6)
    }
    FontChoice::Skip => {
        // User skipped, no download needed
    }
}
```

**Key Learnings:**
- Progress bars should be user-friendly with clear messages
- All operations must have graceful error handling
- Temp files must be cleaned up (use RAII pattern or explicit cleanup)
- Tests must cover edge cases and error paths

### Git Intelligence (Recent Commits)

**Recent Work Pattern:**
```
880cc58 Apply code review fixes to font detection (Story 4.3)
afd2dde Implement Nerd Font detection system (Story 4.3)
02afd3c Implement Nerd Font registry (Story 4.1)
```

**Code Patterns Observed:**
1. **Progress bar pattern** (from `src/frameworks/installer.rs`):
   ```rust
   let pb = ProgressBar::new(total_steps as u64);
   pb.set_style(
       ProgressStyle::default_bar()
           .template("[{bar:40.cyan/blue}] {pos}/{len} {msg}")
           .unwrap()
           .progress_chars("##-"),
   );
   pb.set_message("Downloading...");
   pb.inc(1);
   pb.finish_with_message("Done!");
   ```

2. **Logging pattern:**
   ```rust
   log::info!("Downloading font: {} from {}", font.name, font.download_url);
   log::debug!("Download progress: {}% ({}/{} bytes)", percent, downloaded, total);
   log::error!("Download failed: {}", error);
   ```

3. **Temp directory pattern:**
   ```rust
   let temp_dir = std::env::temp_dir().join(format!("zprof-font-{}", font.id));
   std::fs::create_dir_all(&temp_dir)?;
   ```

### Latest Tech Information

**Reqwest v0.12 (Current Stable):**
- **Blocking client** recommended for CLI applications (simpler than async)
- **Streaming downloads:**
  ```rust
  let mut response = client.get(url).send()?;
  let total_size = response.content_length().unwrap_or(0);

  let mut downloaded: u64 = 0;
  let mut buffer = [0; 8192];

  while let Ok(n) = response.read(&mut buffer) {
      if n == 0 { break; }
      file.write_all(&buffer[..n])?;
      downloaded += n as u64;
      progress_callback(downloaded, total_size);
  }
  ```

- **Timeout configuration:**
  ```rust
  let client = Client::builder()
      .timeout(Duration::from_secs(30))
      .connect_timeout(Duration::from_secs(10))
      .build()?;
  ```

- **Retry with exponential backoff:**
  ```rust
  let mut attempt = 0;
  let max_retries = 2;

  loop {
      match download_attempt(url) {
          Ok(result) => return Ok(result),
          Err(e) if attempt < max_retries => {
              attempt += 1;
              let delay = Duration::from_secs(2_u64.pow(attempt - 1));
              log::warn!("Download failed (attempt {}/{}), retrying in {:?}: {}",
                         attempt, max_retries + 1, delay, e);
              std::thread::sleep(delay);
          }
          Err(e) => return Err(e),
      }
  }
  ```

**Zip Crate v2.1:**
- Standard library for ZIP extraction in Rust
- Supports both reading and writing ZIP archives
- File-by-file extraction with progress tracking
- Handles nested directories automatically

**Security Best Practices:**
- Validate ZIP paths to prevent path traversal (`..` in filenames)
- Only extract known file types (`.ttf`, `.otf`)
- Verify file sizes before extraction
- Use temp directory with unique name (avoid conflicts)

### Implementation Guidance

**Constants:**
```rust
const MIN_FILE_SIZE: u64 = 1_000_000;      // 1 MB
const MAX_FILE_SIZE: u64 = 50_000_000;     // 50 MB
const DOWNLOAD_TIMEOUT_SECS: u64 = 30;
const MAX_RETRIES: u32 = 2;
const PROGRESS_UPDATE_HZ: u64 = 2;         // Updates per second
```

**Main Download Function:**
```rust
pub fn download_font(font: &NerdFont) -> Result<DownloadResult> {
    log::info!("Downloading font: {} from {}", font.name, font.download_url);

    // Create temp directory
    let temp_dir = std::env::temp_dir().join(format!("zprof-font-{}", font.id));
    fs::create_dir_all(&temp_dir)
        .context("Failed to create temporary directory")?;

    // Download with retry
    let zip_path = temp_dir.join(format!("{}.zip", font.id));
    download_with_retry(font.download_url, &zip_path)?;

    // Verify download
    let file_size = fs::metadata(&zip_path)?.len();
    verify_download_size(file_size)?;

    // Extract fonts
    let font_files = extract_fonts(&zip_path, &temp_dir)?;

    if font_files.is_empty() {
        bail!("No font files found in downloaded archive");
    }

    log::info!("Successfully downloaded {} font files ({} bytes)",
               font_files.len(), file_size);

    Ok(DownloadResult {
        temp_dir,
        font_files,
        total_size: file_size,
    })
}
```

**Download with Progress:**
```rust
fn download_with_retry(url: &str, dest: &Path) -> Result<()> {
    let mut attempt = 0;

    loop {
        match download_file(url, dest) {
            Ok(_) => return Ok(()),
            Err(e) if attempt < MAX_RETRIES => {
                attempt += 1;
                let delay = Duration::from_secs(2_u64.pow(attempt - 1));
                log::warn!("Download failed (attempt {}/{}), retrying in {:?}: {}",
                          attempt, MAX_RETRIES + 1, delay, e);
                std::thread::sleep(delay);
            }
            Err(e) => return Err(e).context("Download failed after retries"),
        }
    }
}

fn download_file(url: &str, dest: &Path) -> Result<()> {
    let client = Client::builder()
        .timeout(Duration::from_secs(DOWNLOAD_TIMEOUT_SECS))
        .build()?;

    let mut response = client.get(url).send()
        .context("Failed to initiate download")?;

    if !response.status().is_success() {
        bail!("Download failed with status: {}", response.status());
    }

    let total_size = response.content_length().unwrap_or(0);
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n[{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("█▓▒░ "),
    );
    pb.set_message(format!("Downloading..."));

    let mut file = File::create(dest)
        .context("Failed to create output file")?;
    let mut downloaded: u64 = 0;

    use std::io::Read;
    let mut buffer = [0; 8192];
    loop {
        let n = response.read(&mut buffer)
            .context("Failed to read from stream")?;
        if n == 0 {
            break;
        }
        file.write_all(&buffer[..n])
            .context("Failed to write to file")?;
        downloaded += n as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");
    Ok(())
}
```

**Verification:**
```rust
fn verify_download_size(size: u64) -> Result<()> {
    if size < MIN_FILE_SIZE {
        bail!("Downloaded file is too small ({} bytes, minimum {}). Download may be corrupt.",
              size, MIN_FILE_SIZE);
    }
    if size > MAX_FILE_SIZE {
        bail!("Downloaded file is too large ({} bytes, maximum {}). Unexpected file size.",
              size, MAX_FILE_SIZE);
    }
    Ok(())
}
```

**ZIP Extraction:**
```rust
fn extract_fonts(zip_path: &Path, dest_dir: &Path) -> Result<Vec<PathBuf>> {
    let file = File::open(zip_path)
        .context("Failed to open ZIP file")?;
    let mut archive = ZipArchive::new(file)
        .context("Failed to read ZIP archive")?;

    let mut font_files = Vec::new();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)
            .context(format!("Failed to read ZIP entry {}", i))?;

        let name = file.name();

        // Security: Prevent path traversal
        if name.contains("..") {
            log::warn!("Skipping suspicious path in ZIP: {}", name);
            continue;
        }

        // Only extract font files
        if name.ends_with(".ttf") || name.ends_with(".otf") {
            let filename = Path::new(name)
                .file_name()
                .context("Invalid filename in ZIP")?;

            let outpath = dest_dir.join(filename);
            let mut outfile = File::create(&outpath)
                .context(format!("Failed to create output file: {:?}", outpath))?;

            io::copy(&mut file, &mut outfile)
                .context("Failed to extract file")?;

            log::debug!("Extracted font: {}", outpath.display());
            font_files.push(outpath);
        }
    }

    Ok(font_files)
}
```

### Critical Reminders

**DO:**
- ✅ Add `reqwest`, `zip`, `bytes` to Cargo.toml dependencies
- ✅ Use blocking HTTP client (simpler than async for CLI)
- ✅ Show progress bar with bytes downloaded and ETA
- ✅ Implement retry logic with exponential backoff (1s, 2s delays)
- ✅ Validate file size (1MB-50MB range)
- ✅ Extract only `.ttf` and `.otf` files from ZIP
- ✅ Prevent path traversal (reject `..` in ZIP paths)
- ✅ Clean up temp files (document that caller is responsible for `DownloadResult.temp_dir`)
- ✅ Handle all error cases with clear messages
- ✅ Log download events (start, progress, completion, errors)
- ✅ Write comprehensive tests (unit + integration)
- ✅ Mark integration test with `#[ignore]` for real downloads
- ✅ Use existing progress bar style from `frameworks/installer.rs`

**DON'T:**
- ❌ Don't use async/await (blocking client is simpler for CLI)
- ❌ Don't download to profile directory (use temp directory)
- ❌ Don't extract all ZIP files (only .ttf/.otf)
- ❌ Don't skip size validation (prevents corrupt downloads)
- ❌ Don't skip retry logic (network failures are common)
- ❌ Don't leak temp files (document cleanup responsibility)
- ❌ Don't implement font installation here (that's Story 4.6)
- ❌ Don't hardcode timeouts (use constants for configurability)
- ❌ Don't ignore ZIP security (path traversal vulnerability)

### Acceptance Criteria Expanded

1. **Create `src/fonts/download.rs`**
   - New file in `src/fonts/` directory
   - Export via `src/fonts/mod.rs`: `pub mod download;`
   - Public API: `download_font()`, `DownloadResult` struct

2. **Download font from GitHub releases**
   - Use `reqwest::blocking::Client`
   - URL from `NerdFont.download_url` field
   - 30-second timeout
   - Streaming download (not load entire file into memory)

3. **Show progress bar**
   - Use `indicatif::ProgressBar`
   - Display: bytes downloaded / total bytes
   - Show ETA (estimated time remaining)
   - Format: "[████████░░░░] 12.3 MB / 18.4 MB (10s)"
   - Update at least 2 times per second

4. **Download to temp directory**
   - Use `std::env::temp_dir()` + unique subdirectory
   - Format: `/tmp/zprof-font-{font.id}/`
   - Create directory if doesn't exist
   - Return temp dir path in `DownloadResult`

5. **Verify download**
   - Check file size >= 1 MB
   - Check file size <= 50 MB
   - Clear error message if out of range

6. **Extract ZIP archive**
   - Use `zip::ZipArchive`
   - Extract only `.ttf` and `.otf` files
   - Reject paths containing `..` (security)
   - Return list of extracted file paths

7. **Handle download errors**
   - Network connection failed → Retry
   - HTTP 404/500 → Clear error message
   - Timeout → Retry
   - Incomplete download → Retry
   - Context messages for all errors

8. **Retry logic**
   - Maximum 2 retries (3 total attempts)
   - Exponential backoff: 1s, 2s
   - Log each retry attempt
   - Final error if all attempts fail

9. **Clean up temp files**
   - Document that caller must cleanup `DownloadResult.temp_dir`
   - Consider RAII wrapper for auto-cleanup (optional enhancement)
   - Delete ZIP file after extraction (optional - saves space)

10. **Unit tests**
    - Test size validation (< 1MB, > 50MB)
    - Test ZIP extraction (mock ZIP file)
    - Test font file filtering
    - Test retry logic (mock failures)
    - All in `#[cfg(test)] mod tests` section

11. **Integration test**
    - Test real download in `tests/font_download_test.rs`
    - Mark with `#[ignore]` attribute
    - Download FiraCode.zip
    - Verify extraction
    - Cleanup after test

## Tasks / Subtasks

- [ ] Add dependencies to Cargo.toml (AC: 2)
  - [ ] Add `reqwest = { version = "0.12", features = ["blocking", "stream"] }`
  - [ ] Add `zip = "2.1"`
  - [ ] Add `bytes = "1.5"`
- [ ] Create `src/fonts/download.rs` (AC: 1)
  - [ ] Module-level documentation
  - [ ] Define constants (MIN/MAX size, timeout, retries)
  - [ ] Define `DownloadResult` struct
  - [ ] Import all required dependencies
- [ ] Implement `download_font()` function (AC: 2, 3, 4, 5, 6)
  - [ ] Create temp directory
  - [ ] Call download with retry
  - [ ] Verify file size
  - [ ] Extract fonts from ZIP
  - [ ] Return `DownloadResult`
- [ ] Implement `download_with_retry()` (AC: 8)
  - [ ] Retry loop with max 2 retries
  - [ ] Exponential backoff (1s, 2s)
  - [ ] Logging for each attempt
- [ ] Implement `download_file()` (AC: 2, 3, 7)
  - [ ] Create HTTP client with timeout
  - [ ] Send GET request
  - [ ] Check HTTP status
  - [ ] Create progress bar
  - [ ] Stream download with progress updates
  - [ ] Write to file
- [ ] Implement `verify_download_size()` (AC: 5)
  - [ ] Check >= 1 MB
  - [ ] Check <= 50 MB
  - [ ] Clear error messages
- [ ] Implement `extract_fonts()` (AC: 6)
  - [ ] Open ZIP archive
  - [ ] Iterate over entries
  - [ ] Filter .ttf/.otf files
  - [ ] Prevent path traversal
  - [ ] Extract to temp directory
  - [ ] Return file paths
- [ ] Write unit tests (AC: 10)
  - [ ] Test size validation
  - [ ] Test ZIP extraction (mock)
  - [ ] Test file filtering
  - [ ] Test retry logic
- [ ] Write integration test (AC: 11)
  - [ ] Create `tests/font_download_test.rs`
  - [ ] Test real download with `#[ignore]`
  - [ ] Verify extraction
  - [ ] Cleanup temp files
- [ ] Export module (AC: 1)
  - [ ] Add `pub mod download;` to `src/fonts/mod.rs`
  - [ ] Re-export public API
- [ ] Documentation (AC: all)
  - [ ] Module-level docs
  - [ ] Function docs with error cases
  - [ ] Struct docs with cleanup notes

## Dev Agent Record

### Context Reference

Epic: [epic-4-nerd-fonts.md](../epic-4-nerd-fonts.md)
Tech Spec: [tech-spec-epic-4.md](../../tech-spec-epic-4.md)
Previous Story: [epic-4-story-4.md](epic-4-story-4.md) (Font Selection TUI - ready-for-dev)
Nerd Font Registry: [src/fonts/nerd_fonts.rs](../../../src/fonts/nerd_fonts.rs)
Progress Bar Pattern: [src/frameworks/installer.rs](../../../src/frameworks/installer.rs)

### Agent Model Used

<!-- Will be filled during implementation -->

### Debug Log References

<!-- Will be filled during implementation -->

### Completion Notes List

<!-- Will be filled during implementation -->

### File List

<!-- Will be filled during implementation -->
