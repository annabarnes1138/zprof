# Story 5.1: Add Deprecation Warning to Rollback Command

**Epic:** Epic 5 - Remove Deprecated Rollback Command
**Priority:** P2
**Status:** TODO

## User Story

**As a** user running zprof rollback
**I want** a clear message directing me to the new command
**So that** know what to use instead

## Acceptance Criteria

- [ ] Show deprecation warning with details
- [ ] Default to No (must confirm)
- [ ] Add --force flag to skip warning
- [ ] Add to CLI help: "(deprecated)"
- [ ] Log deprecation

## Files

- src/cli/rollback.rs

## Dependencies

Epic 3 (Complete Uninstall System must be complete)
