# Story 3.4: Implement Safety Backup Before Uninstall

**Epic:** Epic 3 - Complete Uninstall System
**Priority:** P0
**Status:** ready-for-dev

## User Story

**As a** user running uninstall
**I want** a final backup created before removal
**So that** recover if something goes wrong

## Acceptance Criteria

- [ ] Create .zsh-profiles/backups/final-snapshot-{timestamp}.tar.gz
- [ ] Archive all profiles, history, backups
- [ ] Show backup location
- [ ] Abort if backup fails
- [ ] Add --no-backup flag

## Files

- src/cli/uninstall.rs
- src/backup/snapshot.rs (NEW)

## Dependencies

Epic 6 (shares backup logic)

## Dev Agent Record

**Context File:** epic-3-story-4.context.xml
**Status:** Ready for development
**Generated:** 2025-11-24
