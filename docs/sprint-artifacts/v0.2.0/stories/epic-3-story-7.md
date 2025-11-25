# Story 3.7: Handle Edge Cases and Validation

**Epic:** Epic 3 - Complete Uninstall System
**Priority:** P0
**Status:** done

## User Story

**As a** developer
**I want** robust error handling for uninstall
**So that** users don't end up in broken states

## Acceptance Criteria

- [x] Validate preconditions
- [x] Handle missing pre-zprof backup gracefully
- [x] Handle file conflicts during restoration
- [x] Handle partial restoration failures
- [x] Add comprehensive error messages

## Tasks/Subtasks

- [x] Create src/backup/restore.rs module with validation and error handling
- [x] Implement ValidationReport for precondition checking
- [x] Add file conflict resolution logic (Overwrite/Backup/Skip)
- [x] Implement rollback mechanism for failed restorations
- [x] Add checksum validation for restored files
- [x] Update uninstall.rs to use new validation
- [x] Write comprehensive edge case tests (20 scenarios)
- [x] Write validation tests (9 scenarios)
- [x] Verify all tests pass (305 unit + 29 new integration tests)

## Files

**Modified:**
- src/cli/uninstall.rs (integrated validation and rollback-enabled restoration)
- src/backup/mod.rs (added restore module exports)

**Created:**
- src/backup/restore.rs (560 lines - validation, conflict handling, rollback logic)
- tests/uninstall_edge_cases_test.rs (21 test scenarios)
- tests/uninstall_validation_test.rs (9 test scenarios)

## Dependencies

Epic 6 (shares backup logic)

## Dev Agent Record

**Context File:** epic-3-story-7.context.xml
**Status:** Review
**Generated:** 2025-11-24
**Completed:** 2025-11-25

### Debug Log

Implementation plan:
1. ✅ Created comprehensive validation system with ValidationReport
2. ✅ Implemented file conflict resolution (interactive + non-interactive modes)
3. ✅ Added rollback mechanism for partial failures
4. ✅ Implemented checksum validation for corruption detection
5. ✅ Enhanced error messages with recovery instructions
6. ✅ Integrated new validation into uninstall command
7. ✅ Created 29 comprehensive tests covering all edge cases

### Completion Notes

Successfully implemented robust error handling and validation for the uninstall system:

**Key Features:**
- **Precondition Validation**: Checks zprof installation, HOME validity, write permissions, backup existence, and active shells
- **Graceful Degradation**: Missing pre-zprof backup disables "Restore Original" option with clear messaging
- **Conflict Resolution**: Interactive prompts (or non-interactive backup strategy) for existing files
- **Rollback Support**: Automatic rollback on partial failures preserves system state
- **Checksum Validation**: Detects file corruption during restoration
- **Comprehensive Error Messages**: All errors include recovery instructions and backup locations

**Test Coverage:**
- 305 existing unit tests passing
- 20 new edge case tests (symlinks, permissions, conflicts, rollback, checksums)
- 9 new validation tests (preconditions, report structure, multi-issue handling)
- All tests passing, zero regressions

**Files:**
- [src/backup/restore.rs:1-560](src/backup/restore.rs#L1-L560) - Core validation and restoration logic
- [src/cli/uninstall.rs:59-84](src/cli/uninstall.rs#L59-L84) - Validation integration
- [src/cli/uninstall.rs:162-166](src/cli/uninstall.rs#L162-L166) - Rollback-enabled restoration
- [tests/uninstall_edge_cases_test.rs:1-531](tests/uninstall_edge_cases_test.rs) - Edge case coverage
- [tests/uninstall_validation_test.rs:1-143](tests/uninstall_validation_test.rs) - Validation coverage

## File List

- src/backup/restore.rs (NEW - 560 lines)
- src/backup/mod.rs (MODIFIED - added restore exports)
- src/cli/uninstall.rs (MODIFIED - validation integration)
- tests/uninstall_edge_cases_test.rs (NEW - 531 lines)
- tests/uninstall_validation_test.rs (NEW - 143 lines)

## Change Log

- 2025-11-25: Implemented comprehensive error handling and validation system (Story 3.7)
