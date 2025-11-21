# Story 5.7: Add Integration Test for Deprecation Warning

**Epic:** Epic 5 - Remove Deprecated Rollback Command
**Priority:** P2
**Status:** TODO

## User Story

**As a** developer
**I want** tests verifying the deprecation warning
**So that** ensure users see helpful messages

## Acceptance Criteria

- [ ] Test deprecation warning is shown
- [ ] Test user can decline
- [ ] Test --force flag skips warning
- [ ] Test help text shows (deprecated)
- [ ] Snapshot test for message format
- [ ] Verify rollback still works if confirmed

## Files

- tests/rollback_deprecation_test.rs (NEW)

## Dependencies

Epic 3 (Complete Uninstall System must be complete)
