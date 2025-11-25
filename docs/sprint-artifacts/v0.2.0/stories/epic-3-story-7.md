# Story 3.7: Handle Edge Cases and Validation

**Epic:** Epic 3 - Complete Uninstall System
**Priority:** P0
**Status:** ready-for-dev

## User Story

**As a** developer
**I want** robust error handling for uninstall
**So that** users don't end up in broken states

## Acceptance Criteria

- [ ] Validate preconditions
- [ ] Handle missing pre-zprof backup gracefully
- [ ] Handle file conflicts during restoration
- [ ] Handle partial restoration failures
- [ ] Add comprehensive error messages

## Files

- src/cli/uninstall.rs
- src/backup/restore.rs (NEW)
- tests/uninstall_test.rs (NEW)

## Dependencies

Epic 6 (shares backup logic)

## Dev Agent Record

**Context File:** epic-3-story-7.context.xml
**Status:** Ready for development
**Generated:** 2025-11-24
