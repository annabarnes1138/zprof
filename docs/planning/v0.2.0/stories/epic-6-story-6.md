# Story 6.6: Add Init Dry-Run Mode

**Epic:** Epic 6 - Init Cleanup and Enhancement
**Priority:** P0
**Status:** TODO

## User Story

**As a** user considering zprof
**I want** see what init will do without committing
**So that** understand changes before proceeding

## Acceptance Criteria

- [ ] Add --dry-run flag
- [ ] Show what would happen without changes
- [ ] Detect potential issues (disk space, permissions)
- [ ] Show backup size estimate
- [ ] Show cleanup plan
- [ ] Integration test

## Files

- src/cli/init.rs

## Dependencies

Epic 3 (shares backup logic and manifest format)
