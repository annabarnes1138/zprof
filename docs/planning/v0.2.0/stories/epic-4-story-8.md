# Story 4.8: Integrate Font Installation into Create Workflow

**Epic:** Epic 4 - Nerd Font Auto-Installation
**Priority:** P1
**Status:** TODO

## User Story

**As a** user creating profile with Starship
**I want** automatic font installation offered
**So that** prompt works immediately

## Acceptance Criteria

- [ ] After prompt engine selection, check if Nerd Font required
- [ ] Show font selection TUI if needed
- [ ] Download and install selected font
- [ ] Show terminal config instructions
- [ ] Handle skip gracefully
- [ ] Store font choice in manifest
- [ ] Integration test

## Files

- src/cli/create.rs
- src/cli/create_wizard.rs
- src/core/manifest.rs

## Dependencies

Epic 1 (for PromptEngine integration)
