# Story 1.6: Update Generator for Prompt Engines

**Epic:** Epic 1 - Smart TUI (Prompt Mode Branching)
**Priority:** P0
**Status:** ready-for-dev

## User Story

**As a** developer
**I want** the generator to handle prompt engines correctly
**So that** shell configs initialize engines instead of framework themes

## Acceptance Criteria

- [ ] Modify `src/shell/generator.rs`
- [ ] If `prompt_mode = PromptEngine`:
  - Disable framework theme (`ZSH_THEME=""` for oh-my-zsh)
  - Add engine initialization (e.g., `eval "$(starship init zsh)"`)
  - Handle framework-specific syntax
- [ ] If `prompt_mode = FrameworkTheme`:
  - Use existing theme logic
- [ ] Add prompt engine installation to profile creation
- [ ] Validate generated configs with `zsh -n`
- [ ] Add tests for each engine × framework combination

## Files

- `src/shell/generator.rs`
- `src/prompts/installer.rs` (NEW - for engine installation)
- `tests/generator_test.rs`

## Dependencies

- Story 1.1 (Manifest schema)
- Story 1.3 (Prompt Engine Registry)
## Dev Agent Record

### Context Reference
- Story context file: [epic-1-story-6.context.xml](epic-1-story-6.context.xml)
