//! Backup management for zprof
//!
//! This module handles creating and managing backups of shell configurations,
//! particularly the pre-zprof backup created during initialization.

pub mod pre_zprof;
pub mod snapshot;

// Re-export commonly used items
// These will be used in later stories (3.2, 3.3, 3.7)
#[allow(unused_imports)]
pub use pre_zprof::{backup_exists, create_backup, move_configs_to_backup, validate_backup};
pub use snapshot::SafetySummary;
