//! Nerd Font detection for macOS and Linux systems
//!
//! Scans standard font directories to detect installed Nerd Fonts,
//! caches results for performance, and provides a simple API for checking
//! if fonts are already installed before prompting the user to download them.

use std::path::{Path, PathBuf};
use std::sync::OnceLock;

/// Detection result indicating whether Nerd Fonts are installed
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DetectionResult {
    /// Nerd Fonts are installed (includes found font paths)
    Installed { fonts: Vec<PathBuf> },
    /// No Nerd Fonts detected
    NotInstalled,
}

impl DetectionResult {
    /// Check if Nerd Fonts are installed
    pub fn is_installed(&self) -> bool {
        matches!(self, DetectionResult::Installed { .. })
    }

    /// Get the list of detected font paths (empty if not installed)
    pub fn font_paths(&self) -> Vec<PathBuf> {
        match self {
            DetectionResult::Installed { fonts } => fonts.clone(),
            DetectionResult::NotInstalled => vec![],
        }
    }

    /// Get the count of detected fonts
    pub fn count(&self) -> usize {
        match self {
            DetectionResult::Installed { fonts } => fonts.len(),
            DetectionResult::NotInstalled => 0,
        }
    }
}

/// Cached detection result (thread-safe singleton)
static DETECTION_CACHE: OnceLock<DetectionResult> = OnceLock::new();

/// Detect if Nerd Fonts are installed (with caching)
///
/// Searches standard font directories for Nerd Font files matching
/// the pattern `*Nerd*.{ttf,otf}`. Results are cached for the lifetime
/// of the process.
///
/// # Platform Support
///
/// - **macOS**: Searches `~/Library/Fonts` and `/Library/Fonts`
/// - **Linux**: Searches `~/.local/share/fonts` and `/usr/share/fonts`
///
/// # Returns
///
/// `DetectionResult` indicating if fonts are installed and which ones
///
/// # Examples
///
/// ```no_run
/// use zprof::fonts::detector::detect_nerd_fonts;
///
/// let result = detect_nerd_fonts();
/// if result.is_installed() {
///     println!("Found {} Nerd Fonts", result.count());
/// } else {
///     println!("No Nerd Fonts detected");
/// }
/// ```
pub fn detect_nerd_fonts() -> DetectionResult {
    DETECTION_CACHE
        .get_or_init(|| detect_nerd_fonts_uncached())
        .clone()
}

/// Detect Nerd Fonts without caching (for testing)
///
/// This function bypasses the cache and is only used in tests.
#[cfg(test)]
pub(crate) fn detect_nerd_fonts_no_cache() -> DetectionResult {
    detect_nerd_fonts_uncached()
}

/// Detect Nerd Fonts without using the cache
fn detect_nerd_fonts_uncached() -> DetectionResult {
    let search_dirs = get_font_directories();
    let mut found_fonts = Vec::new();

    for dir in search_dirs {
        if let Ok(entries) = std::fs::read_dir(&dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();

                // Recursively search subdirectories
                if path.is_dir() {
                    found_fonts.extend(scan_directory_recursive(&path));
                } else if is_nerd_font_file(&path) {
                    found_fonts.push(path);
                }
            }
        }
    }

    if found_fonts.is_empty() {
        DetectionResult::NotInstalled
    } else {
        DetectionResult::Installed {
            fonts: found_fonts,
        }
    }
}

/// Get standard font directories based on the current platform
fn get_font_directories() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    #[cfg(target_os = "macos")]
    {
        // macOS font directories
        if let Some(home) = dirs::home_dir() {
            dirs.push(home.join("Library/Fonts"));
        }
        dirs.push(PathBuf::from("/Library/Fonts"));
    }

    #[cfg(target_os = "linux")]
    {
        // Linux font directories
        if let Some(home) = dirs::home_dir() {
            dirs.push(home.join(".local/share/fonts"));
        }
        dirs.push(PathBuf::from("/usr/share/fonts"));
    }

    // Filter to only existing directories
    dirs.into_iter().filter(|d| d.exists()).collect()
}

/// Recursively scan a directory for Nerd Font files
fn scan_directory_recursive(dir: &Path) -> Vec<PathBuf> {
    let mut fonts = Vec::new();

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();

            if path.is_dir() {
                // Recursively scan subdirectories
                fonts.extend(scan_directory_recursive(&path));
            } else if is_nerd_font_file(&path) {
                fonts.push(path);
            }
        }
    }

    fonts
}

/// Check if a file is a Nerd Font based on filename pattern
///
/// Matches files with "Nerd" in the name and .ttf or .otf extension.
/// Case-insensitive matching for robustness.
fn is_nerd_font_file(path: &Path) -> bool {
    if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
        let filename_lower = filename.to_lowercase();

        // Check for "nerd" in filename
        let has_nerd = filename_lower.contains("nerd");

        // Check for valid font extension
        let has_valid_ext = filename_lower.ends_with(".ttf") || filename_lower.ends_with(".otf");

        has_nerd && has_valid_ext
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    /// Helper to create a test font file
    fn create_test_font(dir: &Path, filename: &str) -> PathBuf {
        let path = dir.join(filename);
        fs::write(&path, b"fake font data").unwrap();
        path
    }

    #[test]
    fn test_is_nerd_font_file_valid() {
        // Valid Nerd Font files
        assert!(is_nerd_font_file(Path::new(
            "FiraCode Nerd Font Mono.ttf"
        )));
        assert!(is_nerd_font_file(Path::new(
            "JetBrainsMono Nerd Font.otf"
        )));
        assert!(is_nerd_font_file(Path::new(
            "Hack Nerd Font Bold.ttf"
        )));

        // Case insensitive
        assert!(is_nerd_font_file(Path::new(
            "firacode NERD font mono.TTF"
        )));
    }

    #[test]
    fn test_is_nerd_font_file_invalid() {
        // Missing "Nerd" in name
        assert!(!is_nerd_font_file(Path::new("FiraCode.ttf")));
        assert!(!is_nerd_font_file(Path::new("JetBrainsMono.otf")));

        // Wrong extension
        assert!(!is_nerd_font_file(Path::new("FiraCode Nerd Font.txt")));
        assert!(!is_nerd_font_file(Path::new("FiraCode Nerd Font.zip")));

        // Not a font file
        assert!(!is_nerd_font_file(Path::new("README.md")));
    }

    #[test]
    fn test_detection_result_is_installed() {
        let installed = DetectionResult::Installed {
            fonts: vec![PathBuf::from("/Library/Fonts/FiraCode Nerd Font.ttf")],
        };
        assert!(installed.is_installed());

        let not_installed = DetectionResult::NotInstalled;
        assert!(!not_installed.is_installed());
    }

    #[test]
    fn test_detection_result_count() {
        let result = DetectionResult::Installed {
            fonts: vec![
                PathBuf::from("/Library/Fonts/FiraCode Nerd Font.ttf"),
                PathBuf::from("/Library/Fonts/JetBrains Nerd Font.ttf"),
            ],
        };
        assert_eq!(result.count(), 2);

        let empty = DetectionResult::NotInstalled;
        assert_eq!(empty.count(), 0);
    }

    #[test]
    fn test_detection_result_font_paths() {
        let fonts = vec![
            PathBuf::from("/Library/Fonts/FiraCode Nerd Font.ttf"),
            PathBuf::from("/Library/Fonts/JetBrains Nerd Font.ttf"),
        ];

        let result = DetectionResult::Installed {
            fonts: fonts.clone(),
        };
        assert_eq!(result.font_paths(), fonts);

        let empty = DetectionResult::NotInstalled;
        assert_eq!(empty.font_paths(), Vec::<PathBuf>::new());
    }

    #[test]
    fn test_scan_directory_finds_fonts() {
        let temp_dir = TempDir::new().unwrap();
        let dir = temp_dir.path();

        // Create some test fonts
        create_test_font(dir, "FiraCode Nerd Font.ttf");
        create_test_font(dir, "JetBrains Nerd Font.otf");
        create_test_font(dir, "Regular Font.ttf"); // Not a Nerd Font

        let found = scan_directory_recursive(dir);

        assert_eq!(found.len(), 2);
        assert!(found
            .iter()
            .any(|p| p.file_name().unwrap() == "FiraCode Nerd Font.ttf"));
        assert!(found
            .iter()
            .any(|p| p.file_name().unwrap() == "JetBrains Nerd Font.otf"));
    }

    #[test]
    fn test_scan_directory_recursive_nested() {
        let temp_dir = TempDir::new().unwrap();
        let dir = temp_dir.path();

        // Create nested directory structure
        let subdir = dir.join("nerd-fonts");
        fs::create_dir(&subdir).unwrap();

        create_test_font(dir, "Root Nerd Font.ttf");
        create_test_font(&subdir, "Nested Nerd Font.ttf");

        let found = scan_directory_recursive(dir);

        assert_eq!(found.len(), 2);
        assert!(found
            .iter()
            .any(|p| p.file_name().unwrap() == "Root Nerd Font.ttf"));
        assert!(found
            .iter()
            .any(|p| p.file_name().unwrap() == "Nested Nerd Font.ttf"));
    }

    #[test]
    fn test_scan_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let found = scan_directory_recursive(temp_dir.path());
        assert_eq!(found.len(), 0);
    }

    #[test]
    fn test_get_font_directories_returns_valid_paths() {
        let dirs = get_font_directories();

        // Should return at least one directory on supported platforms
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            assert!(!dirs.is_empty(), "Should find at least one font directory");

            // All returned directories should exist
            for dir in &dirs {
                assert!(
                    dir.exists(),
                    "Font directory should exist: {}",
                    dir.display()
                );
            }
        }
    }

    #[test]
    #[cfg(target_os = "macos")]
    fn test_macos_font_directories() {
        let dirs = get_font_directories();

        // macOS should check /Library/Fonts at minimum
        assert!(
            dirs.iter().any(|d| d == &PathBuf::from("/Library/Fonts")),
            "macOS should include /Library/Fonts"
        );
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_linux_font_directories() {
        let dirs = get_font_directories();

        // Linux should check /usr/share/fonts at minimum
        assert!(
            dirs.iter()
                .any(|d| d == &PathBuf::from("/usr/share/fonts")),
            "Linux should include /usr/share/fonts"
        );
    }

    #[test]
    fn test_caching_returns_same_result() {
        let result1 = detect_nerd_fonts();
        let result2 = detect_nerd_fonts();

        // Should return identical results (cached)
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_detect_without_cache() {
        // Test the non-cached version directly
        let result = detect_nerd_fonts_no_cache();

        // Should return a valid result (either installed or not)
        match result {
            DetectionResult::Installed { fonts } => {
                assert!(!fonts.is_empty(), "If installed, should have at least one font");
            }
            DetectionResult::NotInstalled => {
                // This is also valid - no fonts found
            }
        }
    }
}
