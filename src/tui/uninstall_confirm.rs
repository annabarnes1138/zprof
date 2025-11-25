//! Uninstall confirmation screen with detailed summary
//!
//! Displays comprehensive uninstall summary including:
//! - Restoration plan details
//! - Cleanup operations
//! - Safety backup information
//! - File counts and sizes
//!
//! Requires explicit user confirmation (default: No) before proceeding.

use anyhow::Result;
use chrono::{DateTime, Utc};
use dialoguer::Confirm;

use crate::backup::SafetySummary;
use crate::cleanup::CleanupSummary;

/// Complete summary of the uninstall operation
#[derive(Debug)]
pub struct UninstallSummary {
    pub restoration: RestorationSummary,
    pub cleanup: CleanupSummary,
    pub safety: SafetySummary,
}

/// Summary of restoration plan
#[derive(Debug)]
pub struct RestorationSummary {
    pub option: RestoreOption,
    pub file_count: usize,
    pub history_entries: Option<usize>,
    pub source_date: Option<DateTime<Utc>>,
}

/// Restoration option types
#[derive(Debug, Clone)]
pub enum RestoreOption {
    PreZprof,
    PromoteProfile(String),
    NoRestore,
}

/// Show confirmation dialog with detailed summary
///
/// Displays:
/// - Restoration plan (option, file count, history entries, source date)
/// - Cleanup plan (profile count, total size, directories)
/// - Safety backup (path, size)
/// - Destructive operation warnings
///
/// # Arguments
/// * `summary` - Complete uninstall summary to display
///
/// # Returns
/// * `Ok(true)` - User confirmed, proceed with uninstall
/// * `Ok(false)` - User declined, cancel uninstall
///
/// # Defaults
/// The confirmation defaults to "No" - user must explicitly type 'y' or 'yes' to proceed.
pub fn show_confirmation(summary: &UninstallSummary) -> Result<bool> {
    // Display formatted summary
    let formatted = format_summary(summary);
    println!("\n{}", formatted);

    // Show destructive operation warning
    println!("╭─────────────────────────────────────────────────────────────╮");
    println!("│  ⚠️  This action cannot be undone!                          │");
    println!("╰─────────────────────────────────────────────────────────────╯");
    println!();

    // Prompt for confirmation with default: No
    let confirmed = Confirm::new()
        .with_prompt("Continue with uninstall?")
        .default(false)
        .interact()?;

    Ok(confirmed)
}

/// Format complete uninstall summary for display
///
/// Creates a comprehensive text summary with:
/// - Header box
/// - Restoration details
/// - Cleanup details
/// - Safety backup details
///
/// All sizes are human-readable (KB, MB, GB)
/// All timestamps are human-readable (e.g., "Jan 15, 2025")
fn format_summary(summary: &UninstallSummary) -> String {
    let mut output = String::new();

    // Header
    output.push_str("╭─────────────────────────────────────────────────────────────╮\n");
    output.push_str("│                    Uninstall Summary                        │\n");
    output.push_str("╰─────────────────────────────────────────────────────────────╯\n");
    output.push('\n');

    // Restoration section
    output.push_str("  Restoration:\n");
    match &summary.restoration.option {
        RestoreOption::PreZprof => {
            output.push_str("    • Restore pre-zprof backup to HOME\n");
            if let Some(date) = &summary.restoration.source_date {
                output.push_str(&format!("    • Backup created: {}\n", format_timestamp(date)));
            }
            output.push_str(&format!("    • {} files will be restored\n", summary.restoration.file_count));
            if let Some(entries) = summary.restoration.history_entries {
                output.push_str(&format!("    • History: {} entries\n", format_number(entries)));
            }
        }
        RestoreOption::PromoteProfile(profile_name) => {
            output.push_str(&format!("    • Promote profile '{}' to HOME\n", profile_name));
            output.push_str(&format!("    • {} configuration files will be copied\n", summary.restoration.file_count));
            if let Some(entries) = summary.restoration.history_entries {
                output.push_str(&format!("    • History: {} entries\n", format_number(entries)));
            }
        }
        RestoreOption::NoRestore => {
            output.push_str("    • No restoration (clean removal)\n");
            output.push_str("    • HOME directory will be left without shell configs\n");
        }
    }
    output.push('\n');

    // Cleanup section with highlighting for destructive operations
    output.push_str("  Cleanup:\n");
    if summary.cleanup.profile_count > 0 {
        output.push_str(&format!("    • Remove {} profile(s)\n", summary.cleanup.profile_count));
    }
    output.push_str(&format!("    • Remove ~/.zsh-profiles/ ({})\n", format_size(summary.cleanup.total_size)));
    output.push_str("    • Remove zprof shell integration\n");
    output.push('\n');

    // Safety backup section
    output.push_str("  Safety:\n");
    output.push_str(&format!("    • Final backup: {}\n",
        summary.safety.backup_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("final-snapshot.tar.gz")
    ));
    output.push_str(&format!("    • Backup size: {}\n", format_size(summary.safety.backup_size)));
    output.push_str(&format!("    • Location: {}\n", summary.safety.backup_path.display()));
    output.push('\n');

    output
}

/// Format file size in human-readable format (B, KB, MB, GB)
fn format_size(bytes: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if bytes >= GB {
        format!("{:.2} GB", bytes as f64 / GB as f64)
    } else if bytes >= MB {
        format!("{:.2} MB", bytes as f64 / MB as f64)
    } else if bytes >= KB {
        format!("{:.2} KB", bytes as f64 / KB as f64)
    } else {
        format!("{} B", bytes)
    }
}

/// Format timestamp in human-readable format
///
/// # Examples
/// - "Jan 15, 2025 14:30"
/// - "Dec 1, 2024 09:15"
fn format_timestamp(dt: &DateTime<Utc>) -> String {
    dt.format("%b %d, %Y %H:%M").to_string()
}

/// Format number with thousands separators
fn format_number(n: usize) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();

    for (i, ch) in chars.iter().enumerate() {
        if i > 0 && (chars.len() - i).is_multiple_of(3) {
            result.push(',');
        }
        result.push(*ch);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeZone;
    use std::path::PathBuf;

    #[test]
    fn test_format_size_bytes() {
        assert_eq!(format_size(0), "0 B");
        assert_eq!(format_size(512), "512 B");
        assert_eq!(format_size(1023), "1023 B");
    }

    #[test]
    fn test_format_size_kilobytes() {
        assert_eq!(format_size(1024), "1.00 KB");
        assert_eq!(format_size(1536), "1.50 KB");
        assert_eq!(format_size(10240), "10.00 KB");
    }

    #[test]
    fn test_format_size_megabytes() {
        assert_eq!(format_size(1_048_576), "1.00 MB");
        assert_eq!(format_size(2_097_152), "2.00 MB");
        assert_eq!(format_size(5_242_880), "5.00 MB");
    }

    #[test]
    fn test_format_size_gigabytes() {
        assert_eq!(format_size(1_073_741_824), "1.00 GB");
        assert_eq!(format_size(2_147_483_648), "2.00 GB");
    }

    #[test]
    fn test_format_timestamp() {
        let dt = Utc.with_ymd_and_hms(2025, 1, 15, 14, 30, 0).unwrap();
        let formatted = format_timestamp(&dt);
        assert!(formatted.contains("Jan 15, 2025"));
        assert!(formatted.contains("14:30"));
    }

    #[test]
    fn test_format_number() {
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(123), "123");
        assert_eq!(format_number(1000), "1,000");
        assert_eq!(format_number(1234), "1,234");
        assert_eq!(format_number(15234), "15,234");
        assert_eq!(format_number(1234567), "1,234,567");
    }

    #[test]
    fn test_format_summary_pre_zprof() {
        let summary = UninstallSummary {
            restoration: RestorationSummary {
                option: RestoreOption::PreZprof,
                file_count: 4,
                history_entries: Some(15234),
                source_date: Some(Utc.with_ymd_and_hms(2025, 1, 15, 10, 0, 0).unwrap()),
            },
            cleanup: CleanupSummary {
                profile_count: 3,
                total_size: 2_411_724,
                directories: vec![],
            },
            safety: SafetySummary {
                backup_path: PathBuf::from("/home/user/.zsh-profiles/backups/final-snapshot-20250124.tar.gz"),
                backup_size: 2_411_724,
            },
        };

        let formatted = format_summary(&summary);

        // Check header
        assert!(formatted.contains("Uninstall Summary"));

        // Check restoration section
        assert!(formatted.contains("Restore pre-zprof backup to HOME"));
        assert!(formatted.contains("Jan 15, 2025"));
        assert!(formatted.contains("4 files will be restored"));
        assert!(formatted.contains("15,234 entries"));

        // Check cleanup section
        assert!(formatted.contains("Remove 3 profile(s)"));
        assert!(formatted.contains("2.30 MB"));

        // Check safety section
        assert!(formatted.contains("final-snapshot-20250124.tar.gz"));
    }

    #[test]
    fn test_format_summary_promote_profile() {
        let summary = UninstallSummary {
            restoration: RestorationSummary {
                option: RestoreOption::PromoteProfile("work".to_string()),
                file_count: 3,
                history_entries: Some(8500),
                source_date: None,
            },
            cleanup: CleanupSummary {
                profile_count: 2,
                total_size: 1_048_576,
                directories: vec![],
            },
            safety: SafetySummary {
                backup_path: PathBuf::from("/home/user/.zsh-profiles/backups/final-snapshot.tar.gz"),
                backup_size: 1_048_576,
            },
        };

        let formatted = format_summary(&summary);

        assert!(formatted.contains("Promote profile 'work' to HOME"));
        assert!(formatted.contains("3 configuration files will be copied"));
        assert!(formatted.contains("8,500 entries"));
        assert!(formatted.contains("Remove 2 profile(s)"));
    }

    #[test]
    fn test_format_summary_clean_removal() {
        let summary = UninstallSummary {
            restoration: RestorationSummary {
                option: RestoreOption::NoRestore,
                file_count: 0,
                history_entries: None,
                source_date: None,
            },
            cleanup: CleanupSummary {
                profile_count: 0,
                total_size: 512,
                directories: vec![],
            },
            safety: SafetySummary {
                backup_path: PathBuf::from("/home/user/.zsh-profiles/backups/final-snapshot.tar.gz"),
                backup_size: 512,
            },
        };

        let formatted = format_summary(&summary);

        assert!(formatted.contains("No restoration (clean removal)"));
        assert!(formatted.contains("HOME directory will be left without shell configs"));
        assert!(formatted.contains("512 B"));
    }

    #[test]
    fn test_restore_option_variants() {
        let opt1 = RestoreOption::PreZprof;
        let opt2 = RestoreOption::PromoteProfile("test".to_string());
        let opt3 = RestoreOption::NoRestore;

        // Should compile and not panic
        let _ = format!("{:?}", opt1);
        let _ = format!("{:?}", opt2);
        let _ = format!("{:?}", opt3);
    }

    #[test]
    fn test_uninstall_summary_creation() {
        let summary = UninstallSummary {
            restoration: RestorationSummary {
                option: RestoreOption::PreZprof,
                file_count: 5,
                history_entries: Some(1000),
                source_date: Some(Utc::now()),
            },
            cleanup: CleanupSummary {
                profile_count: 1,
                total_size: 1024,
                directories: vec![PathBuf::from("/test")],
            },
            safety: SafetySummary {
                backup_path: PathBuf::from("/backup.tar.gz"),
                backup_size: 2048,
            },
        };

        // Should be debug-printable
        let debug_str = format!("{:?}", summary);
        assert!(debug_str.contains("UninstallSummary"));
    }
}
