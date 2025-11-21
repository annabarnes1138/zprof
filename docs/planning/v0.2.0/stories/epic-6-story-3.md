# Story 6.3: Move Root Configs to Backup Location

**Epic:** Epic 6 - Init Cleanup and Enhancement
**Priority:** P0
**Status:** ready-for-dev

## User Story

**As a** user completing zprof init
**I want** root config files moved out of HOME
**So that** only zprof integration remains

## Acceptance Criteria

- [ ] Move config files to backup (not copy)
- [ ] Move framework directories
- [ ] Create new .zshenv with zprof integration
- [ ] Keep .zsh_history in place
- [ ] Handle edge cases (symlinks, read-only)
- [ ] Show confirmation
- [ ] Integration test

## Files

- src/backup/pre_zprof.rs
- src/cli/init.rs

## Dependencies

Epic 3 (shares backup logic and manifest format)

## Dev Agent Record

### Context Reference
- [epic-6-story-3.context.xml](epic-6-story-3.context.xml)
