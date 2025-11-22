# Technical Decisions - zprof

**Project:** zprof
**Last Updated:** 2025-11-21

---

## Architecture Decisions

### AD-001: Profile Storage Directory Name
**Date:** 2025-10-31
**Status:** Decided
**Context:** Need to determine the directory name for storing zprof profiles in user's home directory.
**Decision:** Use `~/.zsh-profiles/` instead of `~/.zprof/`
**Rationale:**
- More descriptive and self-documenting - clearly indicates these are zsh profile configurations
- Avoids potential confusion with other "prof" tools (profilers, etc.)
- Follows Unix convention of descriptive hidden directory names
**Alternatives Considered:**
- `~/.zprof/` - Too abbreviated, not immediately clear what it contains
- `~/.zshprofiles/` - Missing hyphen makes it less readable
**Consequences:**
- All documentation and code must reference `~/.zsh-profiles/`
- Update FR001 to reflect correct directory structure

---

### AD-002: Profile Creation Smart Detection
**Date:** 2025-10-31
**Status:** Decided
**Context:** Need to determine behavior of `zprof create` command - should it always start from scratch, or intelligently detect existing configurations?
**Decision:** Implement smart detection with user choice (Option B)
- When `zprof create <name>` is run, system detects if existing zsh framework configuration exists
- If detected: Prompt user "Import current setup?" or "Start fresh?"
- If not detected or user chooses "Start fresh": Launch TUI wizard from scratch
**Rationale:**
- Provides best UX for first-time users who want to preserve existing setup
- Doesn't force assumptions - user maintains control
- Reduces friction for adoption (users don't have to manually recreate their current config)
- Still allows clean slate experimentation when desired
**Alternatives Considered:**
- Option A: Always launch wizard from scratch - loses user's existing config, high friction
- Option C: Separate commands (create/import-current/clone) - more complex CLI surface, steeper learning curve
**Consequences:**
- FR006 documents detection and prompt behavior
- User journey reflects two-path creation flow
- Implementation requires framework detection logic

---

### AD-003: GUI Technology Selection (Tauri)
**Date:** 2025-11-21
**Status:** Decided
**Context:** After implementing Stories 1.1-1.5 of Epic 1 with TUI (Terminal User Interface), user requirements revealed critical limitations:
- Cannot preview themes visually in terminal
- Limited to 80x24 terminal size constraints
- No support for multi-workflow instances
- Cannot run as separate process (blocks terminal)
- Explicit requirement: "Visually pleasing (the ability to preview themes would be ideal)"
- Explicit requirement: "Launch as a separate process so that we free up the terminal"

**Decision:** Pivot from TUI (Ratatui/Crossterm) to GUI using Tauri framework
- Tauri 2.0+ for native desktop application
- Svelte 4+ for rich web UI frontend
- Rust backend reusing all existing business logic
- Target macOS and Linux initially

**Rationale:**
1. **Visual Capabilities:** Tauri enables theme preview, rich graphics, flexible layouts impossible in TUI
2. **Rust Integration:** Seamlessly reuses 100% of existing business logic (~10,700 lines)
3. **Native Performance:** Native app with web rendering - best of both worlds
4. **Separate Process:** Natural architecture - GUI doesn't block terminal
5. **Multi-Window Support:** Can manage multiple workflows simultaneously
6. **Cross-Platform:** Single codebase for macOS, Linux, future Windows
7. **Proven Technology:** Used by production apps (similar to VS Code Electron but Rust-based)
8. **Small Binary:** Tauri bundles are smaller than Electron alternatives
9. **Developer Experience:** Excellent documentation, active community, modern tooling

**Alternatives Considered:**
1. **Continue with TUI (Ratatui):**
   - ❌ Cannot display visual theme previews
   - ❌ Limited to ASCII/ANSI art
   - ❌ Poor UX for complex visual choices
   - ❌ Blocks terminal during use

2. **Native Rust GUI (egui/iced):**
   - ✅ Native performance
   - ✅ Rust integration
   - ❌ Limited component ecosystem
   - ❌ Steeper learning curve for UI development
   - ❌ Less polished/mature than web-based UIs

3. **Electron:**
   - ✅ Mature ecosystem
   - ✅ Rich UI capabilities
   - ❌ Large bundle size (~100MB+)
   - ❌ Poor integration with Rust backend
   - ❌ Memory overhead from Chromium

4. **Qt/GTK Rust bindings:**
   - ✅ Native widgets
   - ❌ Complex C++ bindings
   - ❌ Platform-specific quirks
   - ❌ Smaller Rust community

**Consequences:**
- **Rollback:** Remove TUI code (~2,000 lines across 5 files)
  - Delete: src/tui/prompt_mode_select.rs (310 lines)
  - Delete: src/tui/prompt_engine_select.rs (345 lines)
  - Delete: src/tui/theme_select.rs (493 lines)
  - Delete: src/tui/framework_select.rs (315 lines)
  - Delete: src/tui/plugin_browser.rs (501 lines)
  - Delete: src/tui/mod.rs (75 lines)
  - Remove dependencies: Ratatui, Crossterm
- **Preserve:** Data models and business logic remain unchanged
  - Keep: Story 1.1 (Manifest schema with PromptMode enum)
  - Keep: Story 1.3 (Prompt engine registry)
  - Keep: All src/core/, src/frameworks/, src/shell/ logic
- **New Epic:** Epic 0 (GUI Foundation) with 5 stories (~2-3 days)
- **Epic 1 Updates:** Rewrite Stories 1.2, 1.4, 1.5, 1.6, 1.7 for GUI
- **CLI Preservation:** All CLI commands remain fully functional via feature flags
- **Build Process:** New build requirements (Node.js, Tauri CLI)
- **Documentation:** Update architecture, PRD, build instructions
- **Timeline:** Similar effort (9-13 days vs 16 days planned) with better UX outcome

**Migration Path:**
1. Create git tag `v0.2.0-tui-archived` ✅
2. Remove TUI files and dependencies
3. Install Tauri and initialize project structure (Epic 0, Story 0.1)
4. Build GUI foundation (Epic 0, Stories 0.2-0.5)
5. Implement GUI workflows (Epic 1, Stories 1.2-1.7 rewritten)
6. Update all documentation

**Risk Mitigation:**
- Learning curve: Allocate time for team training, start simple
- Type safety: Use TypeScript, comprehensive integration tests
- Platform compatibility: Test early on both macOS and Linux
- CLI regression: Maintain full CLI test suite, feature flags for optional GUI

**References:**
- Sprint Change Proposal: docs/sprint-change-proposal-2025-11-21.md
- Epic 0: docs/planning/v0.2.0/epic-0-gui-foundation.md
- TUI Archive Tag: v0.2.0-tui-archived

---

## Technology Stack

### Core Technologies
- **Language:** Rust 1.70+
- **CLI Framework:** Clap 4.5+
- **GUI Framework:** Tauri 2.0+ (native desktop)
- **Frontend:** Svelte 4+ (web UI)
- **Config Format:** TOML 0.9
- **Serialization:** Serde 1.0

### Build Requirements
- Rust toolchain 1.70+
- Node.js 18+ and npm/pnpm
- Tauri prerequisites (varies by platform)
- Platform: macOS 11+, Linux (Ubuntu 20.04+, Fedora, Arch)

---

## Integration Decisions

_To be populated as integration approaches are decided_

---

## Data & Storage Decisions

_To be populated as data storage approaches are decided_

---

## Notes

This document captures technical decisions, preferences, and constraints discussed during planning phases. It serves as input for the Architecture workflow and ensures consistency across implementation.
