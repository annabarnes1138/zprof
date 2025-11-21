# Story 4.3: Check for Existing Nerd Font Installation

**Epic:** Epic 4 - Nerd Font Auto-Installation
**Priority:** P1
**Status:** TODO

## User Story

**As a** user
**I want** zprof to detect if I already have Nerd Fonts
**So that** not prompted if unnecessary

## Acceptance Criteria

- [ ] Create src/fonts/detector.rs
- [ ] Implement macOS detection (~/Library/Fonts, /Library/Fonts)
- [ ] Implement Linux detection (~/.local/share/fonts, /usr/share/fonts)
- [ ] Search for *Nerd*.{ttf,otf} patterns
- [ ] Cache detection result
- [ ] Add unit and integration tests

## Files

- src/fonts/detector.rs (NEW)
- tests/font_detection_test.rs (NEW)

## Dependencies

Epic 1 (for PromptEngine integration)
