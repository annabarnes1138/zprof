# Epic 2: Quick Setup Presets

**Priority:** P0 (Must Have)
**Estimated Effort:** 3 days
**Owner:** TBD

## Overview

Provide curated preset configurations for one-click profile creation. Users can choose between "Quick Setup" (select a preset) or "Custom Setup" (full wizard), lowering the barrier to entry for beginners while preserving flexibility for power users.

## Problem Statement

Current profile creation requires users to make multiple decisions:
- Choose a framework (5 options)
- Select plugins (hundreds available)
- Pick a theme or prompt engine
- Configure environment variables

This is overwhelming for new users who just want "something good that works." Power users still want full control.

## Goals

1. **Fast onboarding**: New users can create a working profile in < 2 minutes
2. **Curated quality**: Presets represent proven, well-tested configurations
3. **No forced choice**: Advanced users can still access full wizard
4. **Educational**: Presets teach users about zsh configuration patterns

## User Stories

### Story 2.1: Define Preset Data Model

**As a** developer
**I want** a data-driven preset system
**So that** adding new presets requires no code changes

**Acceptance Criteria:**
- [ ] Create `src/presets/mod.rs` module
- [ ] Define `Preset` struct with all config fields:
  - id, name, icon, description
  - target_user (who it's for)
  - framework, prompt_mode, plugins
  - env vars, shell options
- [ ] Create `PresetConfig` that can generate a `Manifest`
- [ ] Add `PRESET_REGISTRY` constant with 4-5 presets
- [ ] Implement `Manifest::from_preset()` method
- [ ] Add unit tests

**Files:**
- `src/presets/mod.rs` (NEW)
- `src/core/manifest.rs`

---

### Story 2.2: Define Initial Preset Catalog

**As a** product manager
**I want** 4-5 well-defined presets
**So that** users have clear, differentiated options

**Acceptance Criteria:**
- [ ] **Minimal** preset defined:
  - Framework: Zap
  - Prompt: Pure
  - Plugins: 3 essential (autosuggestions, syntax-highlighting, git)
  - Target: "Beginners who want simplicity"
- [ ] **Performance** preset defined:
  - Framework: Zinit
  - Prompt: Starship
  - Plugins: 5 optimized with turbo mode
  - Target: "Users with slow shells"
- [ ] **Fancy** preset defined:
  - Framework: Oh-My-Zsh
  - Prompt: Powerlevel10k
  - Plugins: 12 feature-rich
  - Target: "Make my terminal Instagram-worthy"
- [ ] **Developer** preset defined:
  - Framework: Zimfw
  - Prompt: Starship
  - Plugins: 8 dev-focused (docker, kubectl, git, fzf, etc.)
  - Target: "Professional devs who code daily"
- [ ] Each preset documented with rationale

**Files:**
- `src/presets/mod.rs`
- `docs/planning/v0.2.0/preset-definitions.md` (NEW)

---

### Story 2.3: Create Quick vs Custom Selection Screen

**As a** user running `zprof create`
**I want** to choose between preset and custom setup
**So that** I can pick the path that fits my expertise

**Acceptance Criteria:**
- [ ] Create `src/tui/setup_mode_select.rs`
- [ ] Display initial binary choice:
  ```
  How would you like to set up your profile?

  > Quick Setup (recommended presets)
    Custom Setup (choose your own components)
  ```
- [ ] Return `SetupMode` enum (Quick | Custom)
- [ ] Default selection: Quick Setup
- [ ] Keyboard navigation
- [ ] Help text explaining each option

**Files:**
- `src/tui/setup_mode_select.rs` (NEW)

---

### Story 2.4: Create Preset Selection TUI

**As a** user who chose Quick Setup
**I want** to see preset options as cards
**So that** I can quickly understand and choose

**Acceptance Criteria:**
- [ ] Create `src/tui/preset_select.rs`
- [ ] Display preset cards with full details:
  ```
  ┌────────────────────────────────────────────────┐
  │ ✨ Minimal                                      │
  │ Fast startup, clean prompt, essential plugins  │
  │                                                 │
  │ Framework: Zap                                 │
  │ Prompt: Pure                                   │
  │ Plugins: 3 (autosuggestions, syntax, git)     │
  │ Target: Beginners who want simplicity          │
  └────────────────────────────────────────────────┘
  ```
- [ ] Highlight recommended preset (Minimal for first-time users)
- [ ] Show "Customize (advanced)" option at bottom
- [ ] Return selected `Preset` or `Custom` signal
- [ ] Keyboard navigation (↑↓, Enter, Esc)

**Files:**
- `src/tui/preset_select.rs` (NEW)

---

### Story 2.5: Create Profile from Preset

**As a** user selecting a preset
**I want** the profile created automatically
**So that** I can start using it immediately

**Acceptance Criteria:**
- [ ] Create `src/cli/create_from_preset.rs` helper
- [ ] Generate `Manifest` from `Preset`
- [ ] Install framework (same as custom flow)
- [ ] Install prompt engine if needed
- [ ] Install all plugins from preset
- [ ] Generate shell configs
- [ ] Show confirmation screen with summary
- [ ] Offer to activate profile immediately
- [ ] Handle errors gracefully (network, permissions)

**Files:**
- `src/cli/create_from_preset.rs` (NEW helper)
- `src/cli/create.rs` (integrate preset path)

---

### Story 2.6: Add Preset Preview/Details Screen

**As a** user browsing presets
**I want** to see detailed information before committing
**So that** I understand what I'm installing

**Acceptance Criteria:**
- [ ] Add "Preview" action to preset selection (press `p` key)
- [ ] Show detailed screen:
  ```
  Performance Preset - Details

  Description:
  Blazing fast, async prompt, optimized loading

  Configuration:
  - Framework: Zinit (turbo mode enabled)
  - Prompt: Starship (async rendering)
  - Plugins:
    • git (version control)
    • zsh-autosuggestions (command suggestions)
    • fast-syntax-highlighting (zinit optimized)
    • fzf (fuzzy finding)
    • history-substring-search (smart history)

  Estimated startup time: < 100ms

  [Enter] Select  [Esc] Back
  ```
- [ ] Return to selection screen on Esc

**Files:**
- `src/tui/preset_select.rs`

---

### Story 2.7: Add Non-Interactive Preset Flag

**As a** power user
**I want** to create profiles from presets via CLI
**So that** I can script profile creation

**Acceptance Criteria:**
- [ ] Add `--preset <name>` flag to `zprof create`
- [ ] Example: `zprof create work --preset performance`
- [ ] Skip all TUI, use preset directly
- [ ] Show summary after creation
- [ ] Return error if preset doesn't exist
- [ ] Update CLI help text
- [ ] Add integration test

**Files:**
- `src/cli/create.rs`
- `tests/create_preset_test.rs` (NEW)

---

### Story 2.8: Update Documentation

**As a** user
**I want** clear documentation on presets
**So that** I know which to choose

**Acceptance Criteria:**
- [ ] Update `docs/user-guide/quick-start.md` with preset flow
- [ ] Create `docs/user-guide/presets.md` explaining each preset
- [ ] Include comparison table
- [ ] Add screenshots or ASCII art of TUI
- [ ] Document `--preset` flag in commands.md

**Files:**
- `docs/user-guide/quick-start.md`
- `docs/user-guide/presets.md` (NEW)
- `docs/user-guide/commands.md`

---

## Technical Design

### Preset Data Structure

```rust
// src/presets/mod.rs

pub struct Preset {
    pub id: &'static str,
    pub name: &'static str,
    pub icon: &'static str,
    pub description: &'static str,
    pub target_user: &'static str,
    pub config: PresetConfig,
}

pub struct PresetConfig {
    pub framework: FrameworkType,
    pub prompt_mode: PromptMode,
    pub plugins: Vec<&'static str>,
    pub env_vars: HashMap<&'static str, &'static str>,
    pub shell_options: Vec<&'static str>,
}

pub const PRESETS: &[Preset] = &[
    Preset {
        id: "minimal",
        name: "Minimal",
        icon: "✨",
        description: "Fast startup, clean prompt, essential plugins only",
        target_user: "Beginners who want simplicity",
        config: PresetConfig {
            framework: FrameworkType::Zap,
            prompt_mode: PromptMode::PromptEngine {
                engine: PromptEngine::Pure
            },
            plugins: vec!["zsh-autosuggestions", "zsh-syntax-highlighting", "git"],
            env_vars: HashMap::new(),
            shell_options: vec!["HIST_IGNORE_DUPS", "AUTO_CD"],
        },
    },
    // ... more presets
];
```

### Workflow Integration

```
zprof create myprofile
         ↓
Setup Mode Selection (Quick vs Custom)
         ↓
    ┌─────┴──────┐
    ↓            ↓
  Quick        Custom
    ↓            ↓
Preset       Framework
Selection    Selection
    ↓            ↓
  Create      (existing
from Preset   wizard)
```

## Dependencies

- Epic 1 (Smart TUI) - Preset configs use `PromptMode`

## Risks & Mitigations

**Risk:** Presets become outdated
**Mitigation:** Data-driven design makes updates easy, version control presets

**Risk:** Users want preset customization
**Mitigation:** v0.3.0 feature - "Edit preset before creating"

**Risk:** Preset installation fails mid-way
**Mitigation:** Rollback to clean state, clear error messages

## Testing Strategy

- Unit tests for `Manifest::from_preset()`
- Integration tests for full preset creation flow
- Snapshot tests for generated configs from each preset
- Manual testing of TUI interactions

## Success Criteria

- [ ] Users can create profiles in < 2 minutes using presets
- [ ] Presets generate valid, working configurations
- [ ] Custom setup path still accessible
- [ ] CLI `--preset` flag works
- [ ] Documentation clear and helpful
- [ ] All tests passing

## Post-Review Follow-ups

### Story 2.1 Review Items (2025-11-22)

- [x] [Med] Fix clippy uninlined_format_args warnings in test assertions (Story 2.1) - **COMPLETED** ✅
- [ ] [Low] Consider adding test for PresetConfig with empty plugins array (Story 2.1) - edge case coverage (optional enhancement)
- [ ] [Future] Add preset validation to ensure plugin names exist in plugin registry (Story 2.1) - quality improvement (future work)

### Story 2.4 Review Items (2025-11-23)

- [ ] [Med] Clarify AC #3 "Show preview characters" interpretation (Story 2.4) - Current implementation shows emoji icons (✨, ⚙️, 👨‍💻, 🚀). If requirement means actual rendered prompt examples/screenshots, additional work needed. **Awaiting product owner clarification.**
- [ ] [Low] Add integration test for preset selection reachability (Story 2.4) - Test full quick setup flow: `create → quick mode → preset selection → profile creation` (can be done in Story 2.5 or later)

## Out of Scope

- Preset customization UI (v0.3.0)
- User-created/shared presets (v0.4.0)
- Preset marketplace (future)
