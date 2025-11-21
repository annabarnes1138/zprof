# Story 6.4: Handle Re-initialization

**Epic:** Epic 6 - Init Cleanup and Enhancement
**Priority:** P0
**Status:** ready-for-dev

## User Story

**As a** user running zprof init again
**I want** it to be safe and not destroy backups
**So that** fix broken installations without risk

## Acceptance Criteria

- [ ] Detect if already initialized
- [ ] Show different message for re-init
- [ ] Preserve existing backup
- [ ] Recreate directory structure if needed
- [ ] Don't move files again
- [ ] Show summary of existing installation
- [ ] Add --force flag to re-backup
- [ ] Integration test

## Files

- src/cli/init.rs

## Dependencies

Epic 3 (shares backup logic and manifest format)

## Dev Agent Record

### Context Reference
- [epic-6-story-4.context.xml](epic-6-story-4.context.xml)
