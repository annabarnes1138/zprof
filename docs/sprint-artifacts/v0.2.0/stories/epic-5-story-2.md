# Story 5.2: Update CLI Help Text

**Epic:** Epic 5 - Remove Deprecated Rollback Command
**Priority:** P2
**Status:** TODO

## User Story

**As a** user running zprof --help
**I want** see that rollback is deprecated
**So that** use the correct command

## Acceptance Criteria

- [ ] Update command list with (deprecated) marker
- [ ] Update rollback --help with migration info
- [ ] Ensure rollback still appears for discoverability

## Files

- src/main.rs
- src/cli/rollback.rs

## Dependencies

Epic 3 (Complete Uninstall System must be complete)
