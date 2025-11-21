# Story 4.5: Implement Font Download

**Epic:** Epic 4 - Nerd Font Auto-Installation
**Priority:** P1
**Status:** TODO

## User Story

**As a** user who selected a font
**I want** it downloaded automatically
**So that** don't manually find and download

## Acceptance Criteria

- [ ] Create src/fonts/download.rs
- [ ] Download from nerdfonts.com GitHub releases
- [ ] Show progress bar
- [ ] Verify download
- [ ] Extract zip/tar.gz
- [ ] Handle errors (network, timeout)
- [ ] Retry logic
- [ ] Clean up temp files

## Files

- src/fonts/download.rs (NEW)
- tests/font_download_test.rs (NEW)

## Dependencies

Epic 1 (for PromptEngine integration)
