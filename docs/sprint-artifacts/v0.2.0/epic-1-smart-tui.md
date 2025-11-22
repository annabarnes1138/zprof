# Epic 1: Smart TUI (Prompt Mode Branching)

**Priority:** P0 (Must Have)
**Estimated Effort:** 3 days
**Owner:** TBD

## Overview

Enable intelligent prompt selection by introducing "prompt mode" branching in the TUI. Users choose between standalone prompt engines (Starship, Powerlevel10k, etc.) and framework-built-in themes, with the UI adapting to show only relevant options.

## Problem Statement

Currently, the TUI shows all theme options regardless of whether the user wants a framework theme or a standalone prompt engine. This is confusing because:
- Prompt engines (Starship, P10k) **replace** the framework's theme system
- Users don't understand the difference between themes and prompt engines
- No guidance on which approach to choose
- Can accidentally configure both (engine gets ignored or conflicts)

## Goals

1. **Clear mental model**: Users understand engines vs themes
2. **Smart branching**: TUI only shows relevant options based on mode
3. **No redundancy**: Don't show theme picker after selecting an engine
4. **Framework independence**: All prompt engines work with all frameworks

## User Stories

### Story 1.1: Add Prompt Mode to Manifest Schema

**As a** developer
**I want** the manifest to support prompt mode discrimination
**So that** configs can represent either engine-based or theme-based prompts

**Acceptance Criteria:**
- [ ] Add `prompt_mode` field to `[profile]` section (enum: "prompt_engine" | "framework_theme")
- [ ] Add `prompt_engine` field (optional, used when mode = "prompt_engine")
- [ ] Rename `theme` field to `framework_theme` (optional, used when mode = "framework_theme")
- [ ] Implement backward compatibility (old manifests default to `framework_theme`)
- [ ] Update validation to ensure only one is set based on mode
- [ ] Update tests for new schema

**Files:**
- `src/core/manifest.rs`
- `tests/manifest_validation_test.rs`

---

### Story 1.2: Create Prompt Mode Selection TUI

**As a** user creating a profile
**I want** to choose how I want my prompt configured
**So that** I only see relevant options for my choice

**Acceptance Criteria:**
- [ ] Create `src/tui/prompt_mode_select.rs`
- [ ] Show binary choice screen after framework selection:
  ```
  How do you want to handle your prompt?

  > Standalone prompt engine (Starship, Powerlevel10k, Pure...)
    Framework's built-in themes (robbyrussell, agnoster...)
  ```
- [ ] Include help text explaining the difference
- [ ] Return selected `PromptMode` enum value
- [ ] Keyboard navigation (↑↓, Enter, Esc)

**Files:**
- `src/tui/prompt_mode_select.rs` (NEW)
- `src/tui/wizard.rs` (integrate new screen)

---

### Story 1.3: Create Prompt Engine Registry

**As a** developer
**I want** a centralized registry of supported prompt engines
**So that** users can select from known working engines

**Acceptance Criteria:**
- [ ] Create `src/prompts/mod.rs` and `src/prompts/engine.rs`
- [ ] Define `PromptEngine` enum (Starship, Powerlevel10k, OhMyPosh, Pure, Spaceship)
- [ ] Add metadata for each engine:
  - Name and description
  - Requires Nerd Font (bool)
  - Installation method (binary, git clone)
  - Initialization command
  - Cross-shell compatible (bool)
- [ ] Add unit tests

**Files:**
- `src/prompts/mod.rs` (NEW)
- `src/prompts/engine.rs` (NEW)

---

### Story 1.4: Create Prompt Engine Selection TUI

**As a** user who chose "Standalone prompt engine"
**I want** to select which engine to use
**So that** I can pick the prompt that fits my needs

**Acceptance Criteria:**
- [ ] Create `src/tui/prompt_engine_select.rs`
- [ ] Display list of engines with descriptions:
  ```
  Select a prompt engine:

  > Starship (cross-shell, Rust-powered, async)
    Powerlevel10k (Zsh-only, highly customizable)
    Oh-My-Posh (cross-shell, many themes)
    Pure (minimal, async, fast)
    Spaceship (feature-rich, pretty)
  ```
- [ ] Show warning if engine requires Nerd Font
- [ ] Return selected `PromptEngine`
- [ ] Keyboard navigation

**Files:**
- `src/tui/prompt_engine_select.rs` (NEW)

---

### Story 1.5: Refactor Theme Selection for Conditional Display

**As a** user who chose "Framework themes"
**I want** to see only themes compatible with my framework
**So that** I don't pick incompatible options

**Acceptance Criteria:**
- [ ] Modify `src/tui/theme_select.rs` to accept `PromptMode`
- [ ] If mode = `PromptEngine`: Skip theme selection entirely
- [ ] If mode = `FrameworkTheme`: Show framework-specific themes
- [ ] Filter theme registry by selected framework
- [ ] Update tests

**Files:**
- `src/tui/theme_select.rs`
- `src/tui/wizard.rs`

---

### Story 1.6: Update Generator for Prompt Engines

**As a** developer
**I want** the generator to handle prompt engines correctly
**So that** shell configs initialize engines instead of framework themes

**Acceptance Criteria:**
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

**Files:**
- `src/shell/generator.rs`
- `src/prompts/installer.rs` (NEW - for engine installation)
- `tests/generator_test.rs`

---

### Story 1.7: Integrate Prompt Mode into Create Workflow

**As a** user running `zprof create`
**I want** the new prompt mode flow to be seamless
**So that** profile creation is intuitive

**Acceptance Criteria:**
- [ ] Update `src/cli/create.rs` to use new TUI flow
- [ ] New flow order:
  1. Framework selection
  2. **Prompt mode selection** (NEW)
  3. **IF engine**: Prompt engine selection
  4. **IF theme**: Theme selection (existing)
  5. Plugin selection
  6. Confirmation
- [ ] Update confirmation screen to show prompt info correctly
- [ ] Integration test for full workflow
- [ ] Update user-facing docs

**Files:**
- `src/cli/create.rs`
- `tests/create_workflow_test.rs`
- `docs/user-guide/quick-start.md`

---

## Technical Design

### Data Model

```rust
// src/core/manifest.rs
pub enum PromptMode {
    PromptEngine { engine: PromptEngine },
    FrameworkTheme { theme: String },
}

// src/prompts/engine.rs
pub enum PromptEngine {
    Starship,
    Powerlevel10k,
    OhMyPosh,
    Pure,
    Spaceship,
}

pub struct EngineMetadata {
    pub name: &'static str,
    pub description: &'static str,
    pub requires_nerd_font: bool,
    pub installation: InstallMethod,
    pub init_command: &'static str,
    pub cross_shell: bool,
}

pub enum InstallMethod {
    Binary { url: &'static str },
    GitClone { repo: &'static str },
    FrameworkPlugin { plugin_name: &'static str },
}
```

### TOML Examples

**With Prompt Engine:**
```toml
[profile]
framework = "oh-my-zsh"
prompt_mode = "prompt_engine"
prompt_engine = "starship"
```

**With Framework Theme:**
```toml
[profile]
framework = "oh-my-zsh"
prompt_mode = "framework_theme"
framework_theme = "robbyrussell"
```

### Generated Config Examples

**Starship with oh-my-zsh:**
```bash
export ZSH="$ZDOTDIR/.oh-my-zsh"
ZSH_THEME=""  # Disabled for external prompt
plugins=(git zsh-autosuggestions)
source $ZSH/oh-my-zsh.sh

# Starship initialization
eval "$(starship init zsh)"
```

**robbyrussell theme:**
```bash
export ZSH="$ZDOTDIR/.oh-my-zsh"
ZSH_THEME="robbyrussell"
plugins=(git zsh-autosuggestions)
source $ZSH/oh-my-zsh.sh
```

## Dependencies

- None (foundational epic)

## Risks & Mitigations

**Risk:** Users confused by prompt mode choice
**Mitigation:** Clear help text, visual examples in TUI

**Risk:** Engine installation fails
**Mitigation:** Graceful error handling, fallback to framework theme

**Risk:** Breaking existing profiles
**Mitigation:** Backward compatibility migration (default to `framework_theme`)

## Testing Strategy

- Unit tests for new data structures
- Integration tests for TUI flows (mock input)
- Snapshot tests for generated configs
- Manual testing with all engine × framework combinations

## Success Criteria

- [ ] Users can choose between engines and themes
- [ ] TUI only shows relevant options
- [ ] Generated configs work correctly
- [ ] Backward compatibility maintained
- [ ] Documentation updated
- [ ] All tests passing

## Out of Scope

- Engine-specific configuration (Starship config.toml customization) - deferred to v0.3.0
- Automatic Nerd Font installation - covered in Epic 4
- Profile editing to change prompt mode - deferred to v0.3.0
