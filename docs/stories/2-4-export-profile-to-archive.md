# Story 2.4: Export Profile to Archive

Status: ready-for-dev

## Story

As a developer,
I want to export my profile to a portable .zprof archive,
so that I can share it with teammates or use it on other machines.

## Acceptance Criteria

1. `zprof export <profile-name>` creates .zprof archive file (tar.gz)
2. Archive contains profile.toml manifest and any custom configuration files
3. Archive includes metadata (export date, zprof version, framework version)
4. Archive excludes cache files and installed framework binaries (manifest describes installation)
5. Archive saved to current directory with filename `<profile-name>.zprof`
6. Success message displays archive path and size

## Tasks / Subtasks

- [ ] Create export command CLI interface (AC: #1)
  - [ ] Create `cli/export.rs` module
  - [ ] Define ExportArgs with profile_name and optional output path
  - [ ] Follow Pattern 1 (CLI Command Structure) from architecture
  - [ ] Add comprehensive error handling with anyhow::Context
  - [ ] Register command in main.rs subcommand list

- [ ] Create archive module structure (AC: All)
  - [ ] Create `archive/mod.rs` module
  - [ ] Create `archive/export.rs` submodule
  - [ ] Define ArchiveMetadata struct for metadata.json
  - [ ] Follow architecture patterns for module organization
  - [ ] Add logging for debugging

- [ ] Define archive metadata structure (AC: #3)
  - [ ] Create ArchiveMetadata struct with serde serialization
  - [ ] Fields: profile_name, framework, export_date (ISO 8601)
  - [ ] Fields: zprof_version (from Cargo.toml or env!("CARGO_PKG_VERSION"))
  - [ ] Fields: framework_version (optional, detected if available)
  - [ ] Fields: exported_by (user name from $USER or whoami)
  - [ ] Serialize to JSON for human-readability
  - [ ] Include in archive as `metadata.json`

- [ ] Implement profile file collection (AC: #2, #4)
  - [ ] Get profile directory path from profile_name
  - [ ] Verify profile exists
  - [ ] Collect files to include in archive:
    - [ ] profile.toml (required manifest)
    - [ ] .zshrc (generated file, for reference)
    - [ ] .zshenv (generated file, for reference)
    - [ ] Any custom configuration files user added
  - [ ] Exclude framework installations (.oh-my-zsh/, .zimfw/, .zprezto/, .zinit/, .zap/)
  - [ ] Exclude cache and temporary files (*.tmp, *.cache, *.log)
  - [ ] Exclude hidden editor files (.swp, .swo, *~)
  - [ ] Log which files are included vs excluded

- [ ] Implement tar.gz archive creation (AC: #1, #5)
  - [ ] Use tar 0.4 + flate2 1.0 crates per architecture
  - [ ] Create tar::Builder with gzip compression
  - [ ] Add metadata.json to archive root
  - [ ] Add profile files with relative paths (strip profile dir prefix)
  - [ ] Set file permissions in tar (preserve original permissions)
  - [ ] Generate output filename: `<profile-name>.zprof`
  - [ ] Save to current working directory (or user-specified path)
  - [ ] Close and flush tar archive

- [ ] Validate archive integrity (AC: #2, #3, #4)
  - [ ] After creating archive, verify it's readable
  - [ ] Check archive contains at least profile.toml and metadata.json
  - [ ] Verify archive size is reasonable (not empty, not suspiciously large)
  - [ ] Log archive statistics (file count, total size)

- [ ] Display export success message (AC: #6)
  - [ ] Show archive path (absolute or relative to cwd)
  - [ ] Show archive file size in human-readable format (KB, MB)
  - [ ] Show file count in archive
  - [ ] Show profile details (name, framework)
  - [ ] Use consistent success format (✓ symbol per architecture)
  - [ ] Provide next steps (how to import on another machine)

- [ ] Handle edge cases and errors (AC: All)
  - [ ] Profile doesn't exist: clear error with suggestion
  - [ ] Profile directory empty: warning but create archive anyway
  - [ ] Output file already exists: prompt to overwrite or rename
  - [ ] Insufficient disk space: clear error before starting archive
  - [ ] Permission denied on output directory: helpful error
  - [ ] Tar archive creation fails: cleanup partial file
  - [ ] Empty profile (no files): warning but allow export
  - [ ] Custom files too large: warn if archive > 10MB

- [ ] Write comprehensive tests (AC: All)
  - [ ] Unit test metadata generation
  - [ ] Unit test file collection (include/exclude logic)
  - [ ] Integration test export creates valid tar.gz
  - [ ] Integration test archive contains expected files
  - [ ] Integration test archive excludes framework binaries
  - [ ] Integration test metadata.json is valid JSON
  - [ ] Integration test output filename format
  - [ ] Test archive can be extracted with standard tar
  - [ ] Test exported archive size is reasonable
  - [ ] Manual test export → extract → inspect contents

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `cli/export.rs`, `archive/export.rs`
- Secondary: `core/manifest.rs` (read manifest), `core/profile.rs` (profile operations)
- Follow Pattern 1 (CLI Command Structure)
- Follow Pattern 2 (Error Handling)
- Uses tar 0.4 + flate2 1.0 per architecture decision
- Implements shareable profile ecosystem per Epic 2 goals

**Export Module Pattern:**

```rust
// archive/export.rs
use anyhow::{Context, Result, ensure, bail};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::Write;
use tar::Builder;
use flate2::write::GzEncoder;
use flate2::Compression;
use serde::{Serialize, Deserialize};
use chrono::Utc;
use crate::core::manifest;

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchiveMetadata {
    pub profile_name: String,
    pub framework: String,
    pub export_date: String,  // ISO 8601
    pub zprof_version: String,
    pub framework_version: Option<String>,
    pub exported_by: String,
}

pub fn export_profile(profile_name: &str, output_path: Option<PathBuf>) -> Result<PathBuf> {
    log::info!("Exporting profile: {}", profile_name);

    // 1. Get profile directory
    let profile_dir = get_profile_dir(profile_name)?;
    ensure!(
        profile_dir.exists(),
        "Profile '{}' not found. Run 'zprof list' to see available profiles.",
        profile_name
    );

    // 2. Load manifest for metadata
    let manifest = manifest::load_and_validate(profile_name)
        .context("Cannot export profile with invalid manifest")?;

    // 3. Collect files to include
    let files_to_include = collect_files(&profile_dir)?;
    log::info!("Collected {} files for export", files_to_include.len());

    // 4. Create metadata
    let metadata = create_metadata(&manifest)?;

    // 5. Determine output path
    let archive_path = output_path.unwrap_or_else(|| {
        let cwd = std::env::current_dir().unwrap_or_default();
        cwd.join(format!("{}.zprof", profile_name))
    });

    // 6. Check if output file exists
    if archive_path.exists() {
        bail!(
            "Archive already exists: {}\nUse --force to overwrite or specify a different path.",
            archive_path.display()
        );
    }

    // 7. Create tar.gz archive
    create_archive(&archive_path, &profile_dir, &files_to_include, &metadata)?;

    // 8. Validate archive
    validate_archive(&archive_path)?;

    log::info!("Export completed: {:?}", archive_path);
    Ok(archive_path)
}

fn get_profile_dir(profile_name: &str) -> Result<PathBuf> {
    let home = dirs::home_dir()
        .context("Could not find home directory")?;

    Ok(home
        .join(".zsh-profiles")
        .join("profiles")
        .join(profile_name))
}

fn collect_files(profile_dir: &Path) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();

    // Required file
    let manifest_path = profile_dir.join("profile.toml");
    ensure!(
        manifest_path.exists(),
        "profile.toml not found in profile directory"
    );
    files.push(manifest_path);

    // Optional generated files (for reference)
    let zshrc_path = profile_dir.join(".zshrc");
    if zshrc_path.exists() {
        files.push(zshrc_path);
    }

    let zshenv_path = profile_dir.join(".zshenv");
    if zshenv_path.exists() {
        files.push(zshenv_path);
    }

    // Walk directory for custom files
    for entry in fs::read_dir(profile_dir)? {
        let entry = entry?;
        let path = entry.path();

        // Skip directories (framework installations)
        if path.is_dir() {
            log::debug!("Excluding directory: {:?}", path);
            continue;
        }

        // Skip already-added files
        if files.contains(&path) {
            continue;
        }

        // Skip excluded patterns
        if should_exclude(&path) {
            log::debug!("Excluding file: {:?}", path);
            continue;
        }

        // Include custom file
        log::debug!("Including custom file: {:?}", path);
        files.push(path);
    }

    Ok(files)
}

fn should_exclude(path: &Path) -> bool {
    let filename = path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("");

    // Framework directories
    if filename.starts_with(".oh-my-zsh")
        || filename.starts_with(".zimfw")
        || filename.starts_with(".zprezto")
        || filename.starts_with(".zinit")
        || filename.starts_with(".zap")
    {
        return true;
    }

    // Cache and temporary files
    if filename.ends_with(".tmp")
        || filename.ends_with(".cache")
        || filename.ends_with(".log")
        || filename.ends_with(".swp")
        || filename.ends_with(".swo")
        || filename.ends_with("~")
    {
        return true;
    }

    false
}

fn create_metadata(manifest: &manifest::ProfileManifest) -> Result<ArchiveMetadata> {
    let export_date = Utc::now().to_rfc3339();
    let zprof_version = env!("CARGO_PKG_VERSION").to_string();

    // Try to get username
    let exported_by = std::env::var("USER")
        .or_else(|_| std::env::var("USERNAME"))
        .unwrap_or_else(|_| "unknown".to_string());

    Ok(ArchiveMetadata {
        profile_name: manifest.profile.name.clone(),
        framework: manifest.profile.framework.clone(),
        export_date,
        zprof_version,
        framework_version: None,  // TODO: Detect framework version if installed
        exported_by,
    })
}

fn create_archive(
    archive_path: &Path,
    profile_dir: &Path,
    files: &[PathBuf],
    metadata: &ArchiveMetadata,
) -> Result<()> {
    // Create tar.gz file
    let tar_file = File::create(archive_path)
        .context(format!("Failed to create archive: {:?}", archive_path))?;

    let encoder = GzEncoder::new(tar_file, Compression::default());
    let mut tar = Builder::new(encoder);

    // Add metadata.json
    let metadata_json = serde_json::to_string_pretty(&metadata)
        .context("Failed to serialize metadata")?;

    let metadata_bytes = metadata_json.as_bytes();
    let mut header = tar::Header::new_gnu();
    header.set_size(metadata_bytes.len() as u64);
    header.set_mode(0o644);
    header.set_cksum();

    tar.append_data(&mut header, "metadata.json", metadata_bytes)
        .context("Failed to add metadata to archive")?;

    // Add profile files
    for file_path in files {
        // Get relative path within profile directory
        let relative_path = file_path.strip_prefix(profile_dir)
            .context(format!("File not in profile directory: {:?}", file_path))?;

        let mut file = File::open(file_path)
            .context(format!("Failed to open file: {:?}", file_path))?;

        tar.append_file(relative_path, &mut file)
            .context(format!("Failed to add file to archive: {:?}", file_path))?;

        log::debug!("Added to archive: {:?}", relative_path);
    }

    // Finalize archive
    tar.finish()
        .context("Failed to finalize tar archive")?;

    Ok(())
}

fn validate_archive(archive_path: &Path) -> Result<()> {
    // Check archive exists and has reasonable size
    let metadata = fs::metadata(archive_path)
        .context("Archive file not found after creation")?;

    let size = metadata.len();

    ensure!(size > 0, "Archive is empty");

    if size > 10 * 1024 * 1024 {  // 10 MB
        log::warn!("Archive is larger than expected: {} bytes", size);
    }

    log::info!("Archive size: {} bytes", size);
    Ok(())
}

pub fn format_file_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;

    if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} bytes", bytes)
    }
}
```

**CLI Command Pattern:**

```rust
// cli/export.rs
use anyhow::{Context, Result};
use clap::Args;
use std::path::PathBuf;
use crate::archive::export;

#[derive(Debug, Args)]
pub struct ExportArgs {
    /// Name of the profile to export
    pub profile_name: String,

    /// Output path for .zprof archive (default: ./<profile-name>.zprof)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Overwrite existing archive without prompting
    #[arg(short, long)]
    pub force: bool,
}

pub fn execute(args: ExportArgs) -> Result<()> {
    // Export profile to archive
    let archive_path = export::export_profile(&args.profile_name, args.output)
        .context("Failed to export profile")?;

    // Get archive metadata
    let metadata = std::fs::metadata(&archive_path)?;
    let size = export::format_file_size(metadata.len());

    // Count files in archive (read tar to count)
    let file_count = count_archive_files(&archive_path)?;

    // Display success message
    println!("✓ Profile exported successfully");
    println!();
    println!("  Profile: {}", args.profile_name);
    println!("  Archive: {}", archive_path.display());
    println!("  Size: {}", size);
    println!("  Files: {}", file_count);
    println!();
    println!("  → Share this archive with teammates or import on another machine:");
    println!("    zprof import {}", archive_path.display());

    Ok(())
}

fn count_archive_files(archive_path: &std::path::Path) -> Result<usize> {
    use std::fs::File;
    use flate2::read::GzDecoder;
    use tar::Archive;

    let tar_file = File::open(archive_path)?;
    let decoder = GzDecoder::new(tar_file);
    let mut archive = Archive::new(decoder);

    let count = archive.entries()?.count();
    Ok(count)
}
```

**Example User Flow:**

```bash
# Export current work profile
$ zprof export work
✓ Profile exported successfully

  Profile: work
  Archive: /Users/anna/work.zprof
  Size: 2.34 KB
  Files: 4

  → Share this archive with teammates or import on another machine:
    zprof import /Users/anna/work.zprof

# Verify archive contents with standard tar
$ tar -tzf work.zprof
metadata.json
profile.toml
.zshrc
.zshenv

# View metadata
$ tar -xOf work.zprof metadata.json
{
  "profile_name": "work",
  "framework": "oh-my-zsh",
  "export_date": "2025-10-31T16:45:00Z",
  "zprof_version": "0.1.0",
  "framework_version": null,
  "exported_by": "anna"
}
```

**Archive Structure:**

```
work.zprof (tar.gz)
├── metadata.json          # Export metadata
├── profile.toml           # Profile manifest (required)
├── .zshrc                 # Generated shell config (reference)
└── .zshenv                # Generated env config (reference)
```

**Security Considerations:**

- No sensitive data in archives (no passwords, tokens)
- Environment variables in profile.toml are user-managed
- Archives are portable and can be inspected with standard tools
- No executable code in archives (only configuration)

**Why Exclude Framework Binaries (AC #4):**

Framework installations (.oh-my-zsh/, .zimfw/, etc.) are excluded because:

1. **Size**: Framework dirs can be 10-100+ MB (oh-my-zsh is ~30MB)
2. **Reproducibility**: Manifest describes what to install, not binaries themselves
3. **Platform Independence**: Binaries may be platform-specific
4. **Version Control**: Import process installs latest framework version
5. **Simplicity**: Smaller archives are faster to share

Import workflow (Story 2.5) will:
- Read profile.toml manifest
- Install framework and plugins based on manifest
- Regenerate shell files from manifest

This aligns with "manifest as source of truth" principle (ADR-002).

### Project Structure Notes

**New Files Created:**
- `src/archive/mod.rs` - Archive operations module
- `src/archive/export.rs` - Profile export to .zprof archive
- `src/cli/export.rs` - CLI command for export

**Modified Files:**
- `src/main.rs` - Register `export` subcommand
- `src/cli/mod.rs` - Export export module
- `Cargo.toml` - Add dependencies: tar = "0.4", flate2 = "1.0", serde_json (if not already present)

**Dependencies Added:**
```toml
[dependencies]
tar = "0.4"
flate2 = "1.0"
serde_json = "1.0"  # For metadata.json serialization
```

### Learnings from Previous Stories

**From Story 2.1: Parse and Validate TOML Manifests (Status: drafted)**

Export relies on manifest validation:

- **Load Manifest**: Use `manifest::load_and_validate()` to get profile data
- **Metadata Source**: Extract profile name, framework from manifest
- **Validation Gate**: Only export profiles with valid manifests
- **Manifest Required**: profile.toml is required file in archive

**From Story 2.2: Generate Shell Configuration from TOML (Status: drafted)**

Export includes generated files:

- **Generated Files**: .zshrc and .zshenv included for reference
- **Not Required**: Import process can regenerate these files
- **Documentation**: Helps recipients understand what profile does

**From Story 2.3: Manual TOML Editing with Live Validation (Status: drafted)**

Export enables sharing manually-edited profiles:

- **Edit → Export**: Users can customize profiles and share them
- **Validation**: Ensures exported profiles are valid and usable
- **Workflow**: Create → Edit → Export → Share

**Workflow Context:**

This story enables the "sharing" part of the shareable profile ecosystem:

1. **Local Workflow** (Stories 1.x, 2.1-2.3):
   - Create profile
   - Customize profile
   - Validate configuration

2. **Sharing Workflow** (Stories 2.4-2.6):
   - Export profile to .zprof archive (Story 2.4)
   - Import from local archive (Story 2.5)
   - Import from GitHub repo (Story 2.6)

3. **Use Cases**:
   - Share team standardized profiles
   - Backup profiles for safekeeping
   - Migrate profiles between machines
   - Publish profiles to community

**Export vs Import Design:**

Export (this story):
- Creates portable archive from local profile
- Includes manifest + generated files
- Excludes framework binaries (small archive)
- Adds metadata for traceability

Import (Story 2.5):
- Extracts archive
- Validates manifest
- Installs framework per manifest
- Regenerates shell files

Together they enable full profile portability.

### References

- [Source: docs/epics.md#Story-2.4]
- [Source: docs/PRD.md#FR015-export-to-archive]
- [Source: docs/PRD.md#Epic-2-YAML-Manifests-Export-Import]
- [Source: docs/architecture.md#Archive-Format-tar-gz]
- [Source: docs/architecture.md#Epic-2-Story-2.4-Mapping]
- [Source: docs/stories/2-1-parse-and-validate-yaml-manifests.md]
- [Source: docs/stories/2-2-generate-shell-configuration-from-yaml.md]

## Dev Agent Record

### Context Reference

- [Story Context XML](/Users/anna/code/annabarnes1138/zprof/docs/stories/2-4-export-profile-to-archive.context.xml)

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
