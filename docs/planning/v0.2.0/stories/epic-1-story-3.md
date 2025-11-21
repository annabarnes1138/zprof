# Story 1.3: Create Prompt Engine Registry

**Epic:** Epic 1 - Smart TUI (Prompt Mode Branching)
**Priority:** P0
**Status:** ready-for-dev

## User Story

**As a** developer
**I want** a centralized registry of supported prompt engines
**So that** users can select from known working engines

## Acceptance Criteria

- [ ] Create `src/prompts/mod.rs` and `src/prompts/engine.rs`
- [ ] Define `PromptEngine` enum (Starship, Powerlevel10k, OhMyPosh, Pure, Spaceship)
- [ ] Add metadata for each engine:
  - Name and description
  - Requires Nerd Font (bool)
  - Installation method (binary, git clone)
  - Initialization command
  - Cross-shell compatible (bool)
- [ ] Add unit tests

## Files

- `src/prompts/mod.rs` (NEW)
- `src/prompts/engine.rs` (NEW)

## Dependencies

None
## Dev Agent Record

### Context Reference
- Story context file: [epic-1-story-3.context.xml](epic-1-story-3.context.xml)
