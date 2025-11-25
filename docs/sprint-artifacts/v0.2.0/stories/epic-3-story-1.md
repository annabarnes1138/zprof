# Story 3.1: Enhance Init to Create Pre-zprof Backup

**Epic:** Epic 3 - Complete Uninstall System
**Priority:** P0
**Status:** ready-for-dev

## Dev Agent Record

### Context Reference
- docs/sprint-artifacts/v0.2.0/stories/epic-3-story-1.context.xml

## User Story

**As a** user running zprof init
**I want** my current shell config automatically backed up
**So that** restore it later if needed

## Acceptance Criteria

- [ ] Create .zsh-profiles/backups/pre-zprof/ directory
- [ ] Backup all root shell config files (.zshrc, .zshenv, etc.)
- [ ] Backup .zsh_history
- [ ] Create backup-manifest.toml with metadata
- [ ] Skip if already exists
- [ ] Add unit tests

## Files

- src/cli/init.rs
- src/backup/pre_zprof.rs (NEW)
- src/core/backup_manifest.rs (NEW)

## Dependencies

Epic 6 (shares backup logic)
