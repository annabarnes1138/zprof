# Story 6.2: Create Comprehensive Pre-zprof Backup

**Epic:** Epic 6 - Init Cleanup and Enhancement
**Priority:** P0
**Status:** TODO

## User Story

**As a** user running zprof init
**I want** all my shell config backed up automatically
**So that** restore it if needed

## Acceptance Criteria

- [ ] Create .zsh-profiles/backups/pre-zprof/ directory
- [ ] Backup all config files with permissions
- [ ] Backup framework directories as tarball
- [ ] Create backup-manifest.toml
- [ ] Skip if already exists
- [ ] Show summary
- [ ] Integration test

## Files

- src/cli/init.rs
- src/backup/pre_zprof.rs
- src/core/backup_manifest.rs

## Dependencies

Epic 3 (shares backup logic and manifest format)
