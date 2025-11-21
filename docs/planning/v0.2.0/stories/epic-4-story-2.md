# Story 4.2: Detect Font Requirements from Prompt Engine

**Epic:** Epic 4 - Nerd Font Auto-Installation
**Priority:** P1
**Status:** TODO

## User Story

**As a** user selecting a prompt engine
**I want** know if it requires Nerd Fonts
**So that** install them if needed

## Acceptance Criteria

- [ ] Add requires_nerd_font bool to PromptEngine
- [ ] Mark engines (Starship, P10k, OhMyPosh, Spaceship = true; Pure = false)
- [ ] Show Nerd Font requirement in TUI
- [ ] Add icon for fonts required
- [ ] Add unit tests

## Files

- src/prompts/engine.rs
- src/tui/prompt_engine_select.rs

## Dependencies

Epic 1 (for PromptEngine integration)
