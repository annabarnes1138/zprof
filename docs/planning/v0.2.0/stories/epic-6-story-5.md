# Story 6.5: Add Backup Verification

**Epic:** Epic 6 - Init Cleanup and Enhancement
**Priority:** P0
**Status:** TODO

## User Story

**As a** user who backed up configs
**I want** verification that backup is complete
**So that** trust I can restore later

## Acceptance Criteria

- [ ] Verify all files backed up
- [ ] Check file sizes match
- [ ] Verify checksums for critical files
- [ ] Show verification results
- [ ] Handle verification failures
- [ ] Add --skip-verification flag
- [ ] Unit tests

## Files

- src/backup/verify.rs (NEW)
- src/backup/pre_zprof.rs
- src/cli/init.rs

## Dependencies

Epic 3 (shares backup logic and manifest format)
