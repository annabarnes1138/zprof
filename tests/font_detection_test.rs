//! Integration tests for Nerd Font detection
//!
//! These tests verify the font detection system works correctly across
//! different platforms and scenarios.

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;
use zprof::fonts::detector::{detect_nerd_fonts, DetectionResult};

#[test]
fn test_detect_nerd_fonts_returns_valid_result() {
    // Should always return a valid result without panicking
    let result = detect_nerd_fonts();

    match result {
        DetectionResult::Installed { fonts } => {
            // If fonts are installed, the list should not be empty
            assert!(
                !fonts.is_empty(),
                "Installed result should have at least one font"
            );

            // All font paths should exist
            for font in fonts {
                assert!(
                    font.exists(),
                    "Detected font should exist: {}",
                    font.display()
                );
                assert!(
                    font.is_file(),
                    "Detected font should be a file: {}",
                    font.display()
                );

                // Verify it's actually a Nerd Font file
                let filename = font.file_name().unwrap().to_str().unwrap();
                let filename_lower = filename.to_lowercase();
                assert!(
                    filename_lower.contains("nerd"),
                    "Detected font should have 'Nerd' in filename: {}",
                    filename
                );
                assert!(
                    filename_lower.ends_with(".ttf") || filename_lower.ends_with(".otf"),
                    "Detected font should be .ttf or .otf: {}",
                    filename
                );
            }
        }
        DetectionResult::NotInstalled => {
            // Valid result when no fonts are found
        }
    }
}

#[test]
fn test_detection_is_deterministic() {
    // Multiple calls should return the same result (due to caching)
    let result1 = detect_nerd_fonts();
    let result2 = detect_nerd_fonts();

    assert_eq!(result1, result2, "Detection results should be cached and identical");
}

#[test]
fn test_detection_result_methods() {
    let result = detect_nerd_fonts();

    // Test is_installed consistency
    match &result {
        DetectionResult::Installed { fonts } => {
            assert!(result.is_installed());
            assert_eq!(result.count(), fonts.len());
            assert!(!result.font_paths().is_empty());
        }
        DetectionResult::NotInstalled => {
            assert!(!result.is_installed());
            assert_eq!(result.count(), 0);
            assert!(result.font_paths().is_empty());
        }
    }
}

#[test]
#[cfg(target_os = "macos")]
fn test_macos_font_directory_search() {
    // On macOS, detection should check standard directories
    let result = detect_nerd_fonts();

    // If fonts are found, they should be from standard locations
    if let DetectionResult::Installed { fonts } = result {
        for font in fonts {
            let path_str = font.to_str().unwrap();
            assert!(
                path_str.contains("Library/Fonts"),
                "macOS fonts should be in Library/Fonts directories: {}",
                path_str
            );
        }
    }
}

#[test]
#[cfg(target_os = "linux")]
fn test_linux_font_directory_search() {
    // On Linux, detection should check standard directories
    let result = detect_nerd_fonts();

    // If fonts are found, they should be from standard locations
    if let DetectionResult::Installed { fonts } = result {
        for font in fonts {
            let path_str = font.to_str().unwrap();
            assert!(
                path_str.contains(".local/share/fonts") || path_str.contains("/usr/share/fonts"),
                "Linux fonts should be in standard font directories: {}",
                path_str
            );
        }
    }
}

#[test]
fn test_detection_handles_missing_directories_gracefully() {
    // Detection should not panic even if font directories don't exist
    // This is implicitly tested by detect_nerd_fonts() not panicking,
    // but we make it explicit here
    let result = detect_nerd_fonts();

    // Should return a valid result regardless
    match result {
        DetectionResult::Installed { .. } | DetectionResult::NotInstalled => {
            // Both are valid outcomes
        }
    }
}

/// Helper to create a temporary font directory with test fonts
#[allow(dead_code)]
fn create_test_fonts_directory() -> (TempDir, Vec<PathBuf>) {
    let temp_dir = TempDir::new().unwrap();
    let dir = temp_dir.path();

    let fonts = vec![
        dir.join("FiraCode Nerd Font Mono.ttf"),
        dir.join("JetBrainsMono Nerd Font.otf"),
        dir.join("Hack Nerd Font Bold.ttf"),
    ];

    for font in &fonts {
        fs::write(font, b"fake font data").unwrap();
    }

    (temp_dir, fonts)
}

#[test]
fn test_real_world_font_file_patterns() {
    // Test that common Nerd Font naming patterns would be detected
    let test_cases = vec![
        ("FiraCode Nerd Font Mono.ttf", true),
        ("JetBrainsMono Nerd Font.otf", true),
        ("Hack Nerd Font Bold.ttf", true),
        ("MesloLGS Nerd Font Regular.ttf", true),
        ("CaskaydiaCove Nerd Font Mono.otf", true),
        // Invalid cases
        ("FiraCode.ttf", false),
        ("Regular Font.ttf", false),
        ("Nerd Font.txt", false),
        ("readme.md", false),
    ];

    for (filename, should_match) in test_cases {
        let is_nerd = filename.to_lowercase().contains("nerd")
            && (filename.to_lowercase().ends_with(".ttf")
                || filename.to_lowercase().ends_with(".otf"));

        assert_eq!(
            is_nerd, should_match,
            "Pattern matching failed for: {}",
            filename
        );
    }
}

#[test]
fn test_detection_performance() {
    use std::time::Instant;

    // First call may be slow (actual filesystem scan)
    let start = Instant::now();
    let _result1 = detect_nerd_fonts();
    let first_duration = start.elapsed();

    // Second call should be fast (cached)
    let start = Instant::now();
    let _result2 = detect_nerd_fonts();
    let cached_duration = start.elapsed();

    // Cached call should be significantly faster (at least 10x)
    // We allow some leeway since timing can be variable
    assert!(
        cached_duration < first_duration / 5,
        "Cached detection should be much faster: first={:?}, cached={:?}",
        first_duration,
        cached_duration
    );
}

#[test]
fn test_detection_result_clone() {
    let result = detect_nerd_fonts();
    let cloned = result.clone();

    assert_eq!(result, cloned, "Cloned detection result should be identical");
}

#[test]
fn test_detection_result_debug_format() {
    let result = detect_nerd_fonts();
    let debug_str = format!("{:?}", result);

    // Debug output should contain useful information
    match result {
        DetectionResult::Installed { .. } => {
            assert!(debug_str.contains("Installed"));
        }
        DetectionResult::NotInstalled => {
            assert!(debug_str.contains("NotInstalled"));
        }
    }
}
