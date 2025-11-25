//! Backup management for zprof
//!
//! This module handles creating and managing backups of shell configurations,
//! particularly the pre-zprof backup created during initialization.

pub mod pre_zprof;
pub mod restore;
pub mod snapshot;

// Re-export commonly used items
#[allow(unused_imports)]
pub use pre_zprof::{backup_exists, create_backup, move_configs_to_backup, validate_backup};
#[allow(unused_imports)]
pub use restore::{
    validate_preconditions, ValidationReport, ConflictResolution,
    handle_file_conflict, restore_pre_zprof_with_validation,
    validate_checksum, rollback_restoration, check_disk_space,
};
pub use snapshot::SafetySummary;
