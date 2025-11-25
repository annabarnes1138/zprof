# Story 3.6: Add Uninstall Confirmation Screen

**Epic:** Epic 3 - Complete Uninstall System
**Priority:** P0
**Status:** done

## User Story

**As a** user about to uninstall
**I want** see a summary and confirm my choice
**So that** don't accidentally remove shell config

## Acceptance Criteria

- [x] Show detailed summary (restoration plan, cleanup plan, safety backup)
- [x] Default to No
- [x] Add --yes flag to skip
- [x] Include file counts and sizes
- [x] Highlight destructive operations

## Tasks/Subtasks

- [x] Create UninstallSummary data structures
  - [x] UninstallSummary with restoration, cleanup, safety
  - [x] RestorationSummary with option, file_count, history_entries, source_date
  - [x] RestoreOption enum (PreZprof, PromoteProfile, NoRestore)
- [x] Implement formatting helpers
  - [x] format_size() for human-readable bytes (B, KB, MB, GB)
  - [x] format_timestamp() for readable dates
  - [x] format_number() for thousands separators
- [x] Implement format_summary() function
  - [x] Header with box drawing
  - [x] Restoration section with all details
  - [x] Cleanup section with file counts and sizes
  - [x] Safety section with backup info
  - [x] Destructive operation warning
- [x] Implement show_confirmation() function
  - [x] Display formatted summary
  - [x] Show warning box
  - [x] Use dialoguer::Confirm with default(false)
- [x] Update uninstall.rs to use new confirmation
  - [x] Import new module
  - [x] Build UninstallSummary from restoration plan
  - [x] Calculate cleanup summary (profile count, total size)
  - [x] Call show_confirmation() instead of old confirm_uninstall()
- [x] Write comprehensive unit tests
  - [x] format_size() tests (B, KB, MB, GB)
  - [x] format_timestamp() tests
  - [x] format_number() tests (thousands separators)
  - [x] format_summary() tests for all 3 restoration options
  - [x] RestoreOption variant tests
  - [x] UninstallSummary creation tests
- [x] Run full test suite
  - [x] All 300 tests passing
  - [x] No regressions

## Files

### Created
- src/tui/uninstall_confirm.rs

### Modified
- src/tui/mod.rs (added uninstall_confirm module)
- src/cli/uninstall.rs (integrated new confirmation screen)

## Dependencies

Story 3.5 (CleanupSummary struct)
Story 3.4 (SafetySummary struct)
Story 3.1 (BackupManifest for restoration details)

## Dev Agent Record

**Context File:** epic-3-story-6.context.xml
**Status:** Implementation complete
**Generated:** 2025-11-24
**Completed:** 2025-11-25

### Debug Log

1. Created [src/tui/uninstall_confirm.rs](src/tui/uninstall_confirm.rs) with:
   - UninstallSummary, RestorationSummary, RestoreOption data structures
   - show_confirmation() function using dialoguer with default(false)
   - format_summary() to build comprehensive text summary
   - format_size() for human-readable file sizes
   - format_timestamp() for readable dates
   - format_number() for thousands separators in numbers

2. Updated [src/tui/mod.rs](src/tui/mod.rs:13):
   - Added pub mod uninstall_confirm

3. Updated [src/cli/uninstall.rs](src/cli/uninstall.rs):
   - Imported uninstall_confirm module and CleanupSummary
   - Replaced confirm_uninstall() with build_uninstall_summary()
   - Added build_cleanup_summary() to scan profiles directory
   - Added calculate_dir_size() helper for directory size calculation
   - Added count_history_lines() to count history entries
   - Integrated show_confirmation() call with proper summary building

4. Fixed format string issues:
   - Rust doesn't support {:,} for thousands separator
   - Implemented custom format_number() function

5. All tests passing (300/300)

### Completion Notes

✅ All acceptance criteria satisfied:
- AC1: Detailed summary shows restoration plan, cleanup plan, safety backup with all relevant details
- AC2: Confirmation defaults to No (dialoguer::Confirm::default(false))
- AC3: --yes flag already existed in UninstallArgs, now properly bypasses new confirmation
- AC4: File counts, sizes, and history entries all displayed in human-readable format
- AC5: Destructive operations highlighted with warning box and clear messaging

### Technical Highlights

**Data Flow:**
1. Safety backup created BEFORE confirmation (for accurate size reporting)
2. Restoration option selected → Build summary from:
   - Backup manifest (for PreZprof option)
   - Profile directory scan (for PromoteProfile option)
   - Zero values (for Clean option)
3. Cleanup summary calculated by scanning profiles directory
4. Complete UninstallSummary passed to show_confirmation()
5. User must explicitly confirm (default: No)

**Quality:**
- Comprehensive unit tests (11 new tests)
- Clean separation of concerns (formatting helpers)
- Human-readable output (sizes in MB/GB, thousands separators)
- Follows existing code patterns (dialoguer for confirmation)
- Zero regressions (all 300 tests passing)
