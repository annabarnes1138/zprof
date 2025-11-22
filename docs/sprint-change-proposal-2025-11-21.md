# Sprint Change Proposal - TUI to GUI Pivot

**Date:** 2025-11-21
**Project:** zprof
**Scope:** Major - Fundamental architectural pivot
**Status:** Proposed

---

## Executive Summary

After implementing 5 stories (1.1-1.5) of Epic 1 "Smart TUI", it has become clear that the Terminal User Interface (TUI) approach cannot deliver the user experience quality required for zprof. This proposal recommends a strategic pivot to a Graphical User Interface (GUI) using Tauri framework, with rollback of TUI-specific code (~2,000 lines) while preserving validated data models and business logic.

**Key Decision:** Adopt Tauri (Rust + Web) for native desktop GUI
**Impact:** Major - Affects Epic 1 (complete rewrite), moderate impact on Epic 2, minimal impact on Epics 3-6
**Timeline:** Similar to original estimate (~9-13 days vs 16 days planned)
**Risk:** Low - Data models proven, business logic unchanged, clear migration path
**Value:** High - Enables features impossible in TUI (theme preview, multi-workflow management)

---

## Issue Summary

### Problem Statement

The Terminal User Interface (TUI) approach for zprof's workflow management is inadequate for the complexity and quality of user experience required. After implementing Stories 1.1-1.5 of Epic 1, critical user requirements cannot be satisfied by TUI:

1. **Visual theme previewing** - Impossible in terminal environment
2. **Multi-workflow instance management** - Awkward and limited in TUI
3. **Separate process model** - Not natural for terminal-based interaction
4. **Rich, visually pleasing interface** - Limited by terminal constraints (80x24, no graphics, ASCII-only)

### Discovery Context

- **When discovered:** After completing 5 stories of Epic 1 (Smart TUI)
- **Code implemented:** ~2,000 lines of TUI code across 5 screens
- **Trigger:** User priorities explicitly require visual capabilities beyond TUI
- **Decision point:** Continue with inadequate solution or pivot to GUI

### Evidence

1. **User Requirements:**
   - "Visually pleasing (the ability to preview themes would be ideal)"
   - "The ability to launch other workflows within the same instance"
   - "Launch as a separate process so that we free up the terminal"

2. **TUI Limitations:**
   - Cannot display visual theme previews
   - Limited to 80x24 minimum terminal size
   - No rich graphics or color depth
   - Single-instance workflow blocking terminal

3. **Industry Patterns:**
   - Modern configuration tools use GUI (VS Code settings, Docker Desktop, Postman)
   - Users expect visual feedback for visual choices (themes, colors)

---

## Impact Analysis

### Epic Impact

#### Epic 1 (Smart TUI) → **MAJOR IMPACT - Complete Redefinition**

**Current:** "Smart TUI (Prompt Mode Branching)"
**New:** "Smart GUI Workflow (Prompt Mode Branching)"

**Stories to KEEP (Data Models - Completed & Valid):**
- ✅ Story 1.1: Add Prompt Mode to Manifest Schema (GUI-agnostic)
- ✅ Story 1.3: Create Prompt Engine Registry (GUI-agnostic)

**Stories to REPLACE (TUI → GUI):**
- ❌ Story 1.2: ~~Create Prompt Mode Selection TUI~~ → Create Prompt Mode Selection GUI Screen
- ❌ Story 1.4: ~~Create Prompt Engine Selection TUI~~ → Create Prompt Engine Selection GUI Screen
- ❌ Story 1.5: ~~Refactor Theme Selection for Conditional Display~~ → Create Theme Selection GUI with Visual Preview
- ❌ Story 1.6: ~~Update Generator for Prompt Engines~~ → Integrate GUI with Prompt Engine Generator (minor update)
- ❌ Story 1.7: ~~Integrate Prompt Mode into Create Workflow~~ → Integrate GUI Wizard with Backend Logic

**New Stories Required:**
- ➕ Epic 0 (NEW): GUI Foundation with 5 stories (Tauri setup, IPC, base UI)

#### Epic 2 (Presets) → **MODERATE IMPACT**

- Stories reference TUI components → Update to GUI components
- ASCII art mockups → Replace with visual mockups
- Story 2.4: Preset cards become actual visual cards (enhanced by GUI)
- No fundamental changes to preset logic

#### Epics 3-6 → **LOW IMPACT**

- Mostly CLI-focused with minimal GUI needs
- Epic 3 (Uninstall): Could benefit from confirmation GUI
- Epic 4 (Nerd Fonts): Enhanced by font preview GUI
- Epic 5 (Remove Rollback): No impact (pure CLI)
- Epic 6 (Init Cleanup): No impact (pure CLI)

### Artifact Impact

#### PRD (v0.1.0 archived)

**Changes Required:**
1. **NFR003:** Remove TUI requirement → Add GUI performance requirements
2. **FR007:** Change "TUI wizard" → "GUI wizard"
3. **FR009:** Add "with live visual preview" to theme selection
4. **New FR010:** Explicitly preserve CLI functionality
5. **New FR011:** Multi-workflow management capability
6. **Out of Scope:** Remove "No GUI" constraint
7. **UI Design Goals:** Complete section rewrite for GUI platform

#### Architecture Document

**Changes Required:**
1. **Technology Stack:** Add Tauri, remove Ratatui/Crossterm
2. **Project Structure:** Replace `src/tui/` with `src/gui/` + `src-ui/` (web frontend)
3. **Data Flow:** Update profile creation workflow diagram
4. **Module Responsibilities:** Add GUI/IPC section
5. **New Sections:**
   - GUI/CLI separation architecture
   - IPC communication patterns
   - Window management and state
   - Platform-specific considerations

#### UX Design (NEW ARTIFACT NEEDED)

**Required Deliverables:**
1. Visual wireframes for all workflows
2. Component library specification
3. Visual design system (colors, typography, spacing)
4. Theme preview implementation approach
5. Navigation and interaction patterns

#### Technical Decisions Document

**New Decision Required:**
- **TD-003: GUI Technology Selection (Tauri)**
  - Decision: Use Tauri (Rust backend + web frontend)
  - Rationale: Native performance, rich UI, Rust integration, cross-platform
  - Alternatives considered: Native Rust GUI (egui/iced), embedded browser

---

## Recommended Approach

### Selected Path: Strategic Rollback + GUI Pivot

This hybrid approach combines:
1. **Rollback** of TUI-specific implementation (~2,000 lines)
2. **Preservation** of validated data models and business logic
3. **Pivot** to Tauri-based GUI architecture

### Rationale

**Why Rollback TUI Code:**
1. **Clean slate** - Avoids maintaining parallel TUI+GUI codebases
2. **Eliminates technical debt** - No confusion between two UI paradigms
3. **Focus** - Team energy on one excellent solution vs. two mediocre ones
4. **Low risk** - Git preserves history if needed, data models remain valid

**Why Tauri for GUI:**
1. **Native + Web hybrid** - Native performance with rich web UI capabilities
2. **Rust integration** - Seamlessly reuses all existing business logic
3. **Visual capabilities** - Theme preview, rich layouts, proper windowing
4. **Cross-platform** - macOS, Linux, Windows (future-proof)
5. **Separate process** - Natural architecture for your requirement
6. **Proven technology** - Used by production applications (similar to VS Code architecture)

**What We Keep (Valuable Work):**
- ✅ Story 1.1: Manifest schema with PromptMode enum (~100 lines)
- ✅ Story 1.3: Prompt engine registry and metadata (~237 lines)
- ✅ All framework support code (oh-my-zsh, zimfw, etc.)
- ✅ All shell generator logic
- ✅ All business logic in `src/core/`, `src/frameworks/`, `src/shell/`
- ✅ ~10,700 lines of core functionality

**What We Remove (UI-Specific):**
- ❌ TUI screens: ~2,000 lines across 5 files
  - prompt_mode_select.rs (310 lines)
  - prompt_engine_select.rs (345 lines)
  - theme_select.rs (493 lines)
  - framework_select.rs (315 lines)
  - plugin_browser.rs (501 lines)
  - tui/mod.rs (75 lines)
- ❌ Dependencies: Ratatui, Crossterm

### Effort & Timeline Estimate

| Phase | Activities | Effort |
|-------|-----------|--------|
| **Phase 1: Rollback & Setup** | Delete TUI files, install Tauri, initialize project structure | 0.5 days |
| **Phase 2: GUI Foundation** | Epic 0 - Base window, navigation, IPC layer, profile list | 2-3 days |
| **Phase 3: Core Workflows** | Epic 1 GUI stories - Wizard screens with visual preview | 5-7 days |
| **Phase 4: Integration** | Wire GUI to business logic, testing, documentation | 2-3 days |
| **Total** | | **9-13 days** |

**Comparison:** Original Epic 1 estimate was ~3 days (7 stories), but full v0.2.0 was 16 days. GUI approach is comparable with significantly better UX outcome.

### Risk Assessment

| Risk | Level | Mitigation |
|------|-------|------------|
| Learning curve for Tauri | Medium | Excellent documentation, active community, similar to existing Rust patterns |
| Theme preview implementation complexity | Medium | Start with basic rendering, iterate based on feedback |
| Platform compatibility issues | Low | Tauri handles cross-platform abstractions |
| CLI functionality regression | Low | Integration tests ensure CLI remains functional |
| Team morale from pivot | Low | Clear communication, exciting better outcome |

**Overall Risk:** **Low** - Clear path forward, proven technology, minimal business logic changes

---

## Detailed Change Proposals

### 1. Epic 1 Redefinition

**File:** `docs/planning/v0.2.0/epic-1-smart-tui.md`

**Change:** Rename and reframe epic

```diff
- # Epic 1: Smart TUI (Prompt Mode Branching)
+ # Epic 1: Smart GUI Workflow (Prompt Mode Branching)

  ## Overview

- Enable intelligent prompt selection by introducing "prompt mode" branching in the TUI.
+ Enable intelligent prompt selection through a visual GUI workflow.
  Users choose between standalone prompt engines (Starship, Powerlevel10k, etc.)
- and framework-built-in themes, with the UI adapting to show only relevant options.
+ and framework-built-in themes using an intuitive visual interface with theme
+ previewing, plugin browsing, and multi-workflow management.
```

---

### 2. Epic 1 Story Updates

#### Story 1.2: Prompt Mode Selection (TUI → GUI)

**File:** `docs/planning/v0.2.0/epic-1-smart-tui.md`

**Before:**
```markdown
### Story 1.2: Create Prompt Mode Selection TUI
- Create `src/tui/prompt_mode_select.rs`
- Show binary choice screen
- Keyboard navigation (↑↓, Enter, Esc)
```

**After:**
```markdown
### Story 1.2: Create Prompt Mode Selection GUI Screen

**Acceptance Criteria:**
- [ ] Create GUI component for prompt mode selection
- [ ] Display two large, visually distinct cards:
  - "Standalone Prompt Engine" (with icon and examples)
  - "Framework Built-in Themes" (with icon and examples)
- [ ] Show help text with visual examples
- [ ] Support both mouse and keyboard navigation
- [ ] Return selected PromptMode to wizard state

**Files:**
- Frontend: `src-ui/components/PromptModeSelect.svelte` (NEW)
- Backend IPC: `src/gui/commands.rs` (update)
```

---

#### Story 1.4: Engine Selection (TUI → GUI)

**Before:**
```markdown
### Story 1.4: Create Prompt Engine Selection TUI
- Display list of engines with descriptions
- Show warning if engine requires Nerd Font
- Keyboard navigation
```

**After:**
```markdown
### Story 1.4: Create Prompt Engine Selection GUI Screen

**Acceptance Criteria:**
- [ ] Create GUI component displaying engines as visual cards
- [ ] Each card shows:
  - Engine name and icon
  - Description and key features
  - Cross-shell compatibility badge
  - Nerd Font requirement indicator
  - Screenshot/preview of example prompt (future enhancement)
- [ ] Support search/filter by name or features
- [ ] Support both mouse and keyboard shortcuts

**Files:**
- Frontend: `src-ui/components/PromptEngineSelect.svelte` (NEW)
- Frontend: `src-ui/components/EngineCard.svelte` (NEW)
- Backend IPC: `src/gui/commands.rs` (update)
```

---

#### Story 1.5: Theme Selection with Preview (NEW CAPABILITY)

**Before:**
```markdown
### Story 1.5: Refactor Theme Selection for Conditional Display
- Modify `src/tui/theme_select.rs`
- Skip if PromptEngine mode
- Show framework-specific themes
```

**After:**
```markdown
### Story 1.5: Create Theme Selection GUI with Visual Preview

**Acceptance Criteria:**
- [ ] Create GUI component for theme selection with preview
- [ ] Display framework-specific themes as visual cards with:
  - Theme name and description
  - **Live preview of prompt appearance** ⭐
  - Color scheme preview
  - Popularity indicator
- [ ] Preview updates in real-time when hovering
- [ ] Support search and category filtering
- [ ] Allow "preview in terminal" simulation

**Files:**
- Frontend: `src-ui/components/ThemeSelect.svelte` (NEW)
- Frontend: `src-ui/components/ThemePreview.svelte` (NEW - KILLER FEATURE)
- Frontend: `src-ui/lib/theme-renderer.ts` (NEW)
- Backend: `src/frameworks/theme.rs` (add preview metadata)
```

---

### 3. New Epic 0: GUI Foundation

**File:** `docs/planning/v0.2.0/epic-0-gui-foundation.md` (NEW)

**Summary:** 5 stories establishing Tauri integration

1. **Story 0.1:** Install Tauri and Initialize Project
2. **Story 0.2:** Create Base Application Window and Navigation
3. **Story 0.3:** Implement IPC Command Layer
4. **Story 0.4:** Create Profile List View (First Real Screen)
5. **Story 0.5:** Ensure CLI Compatibility

**Estimated Effort:** 2-3 days
**Priority:** P0 (Blocking - must complete before Epic 1)

See full epic document for detailed acceptance criteria.

---

### 4. PRD Updates

**File:** `docs/planning/archive/v0.1.0/PRD.md` (or create v0.2.0 PRD)

#### Non-Functional Requirements

```diff
  - NFR001: Profile switching < 500ms
  - NFR002: Non-destructive operations with backups
- - NFR003: TUI responsive on 2GB RAM, standard terminals
+ - NFR003: GUI responsive on 4GB RAM, 60fps animations, macOS 11+, Linux
+ - NFR004: GUI startup < 2 seconds
+ - NFR005: Light/dark mode support, respect system theme
+ - NFR006: CLI commands remain fully functional
```

#### Functional Requirements

```diff
  - FR006: Detect existing config, prompt import/fresh
- - FR007: Interactive TUI wizard for profile creation
+ - FR007: Interactive GUI wizard for profile creation
- - FR008: Browse and select plugins with recommendations
+ - FR008: Browse and select plugins with search, filtering, recommendations
- - FR009: Select theme during profile creation
+ - FR009: Select theme with live visual preview during profile creation
+ - FR010: Support profile creation via both GUI and CLI
+ - FR011: Enable multiple workflows simultaneously in separate windows
```

#### Out of Scope

```diff
  - MVP focuses on zsh only
  - Import/export: GitHub and local files only
  - Five frameworks supported
- - No GUI - terminal UI only
+ - GUI provides visual workflows; CLI remains fully functional
+ - GUI targets macOS and Linux initially; Windows via WSL
- - No Windows native support (WSL only)
+ - Web-based theme preview for common formats; complex themes may need verification
```

---

### 5. Architecture Document Updates

**File:** `docs/developer/architecture.md`

#### Technology Stack Table

```diff
  | Component | Technology | Purpose |
  |-----------|------------|---------|
  | Language | Rust 1.70+ | Performance, safety |
  | CLI Framework | Clap 4.5+ | Argument parsing |
- | TUI Framework | Ratatui + Crossterm | Interactive wizards |
+ | GUI Framework | Tauri 2.0+ | Native desktop application |
+ | Frontend | Svelte 4+ | Rich web UI components |
  | Config Format | TOML 0.9 | Profile manifests |
```

#### Project Structure

```diff
  zprof/
  ├── src/
  │   ├── cli/           # Command implementations
  │   ├── core/          # Core business logic
  │   ├── frameworks/    # Framework support
- │   ├── tui/           # Terminal UI
+ │   ├── gui/           # GUI IPC commands
  │   ├── archive/       # Import/export
  │   └── shell/         # Shell integration
+ ├── src-tauri/        # Tauri Rust backend
+ ├── src-ui/           # Web frontend (Svelte)
+ │   ├── components/   # Reusable UI components
+ │   ├── views/        # Main application views
+ │   └── lib/          # Utilities and API client
  └── tests/            # Integration tests
```

---

## Implementation Handoff

### Scope Classification: **MAJOR**

This is a fundamental architectural change requiring:
- New technology adoption (Tauri)
- Complete UI reimplementation
- Multi-role coordination
- Documentation overhaul

### Primary Recipients & Responsibilities

#### 1. Solution Architect
**Responsibilities:**
- Design GUI/backend architecture with Tauri
- Define IPC communication patterns and data contracts
- Update architecture documentation
- Review technology stack decisions
- Create implementation guidelines for team

**Deliverables:**
- Architecture decision record for Tauri selection
- IPC API specification
- Updated architecture.md document
- Development guidelines for GUI components

---

#### 2. UX Designer
**Responsibilities:**
- Create visual wireframes for all workflows
- Design component library and visual system
- Specify theme preview visualization approach
- Design multi-workflow management UI
- Conduct user testing and iterate

**Deliverables:**
- Wireframes for all GUI screens (6+ screens)
- Visual design system specification
- Component library documentation
- Theme preview mockups and technical approach
- Interaction patterns guide

---

#### 3. Product Manager
**Responsibilities:**
- Update PRD with GUI requirements
- Revise NFRs for GUI performance and platform support
- Adjust epic definitions and story breakdown
- Timeline and release planning
- Stakeholder communication about pivot

**Deliverables:**
- Updated PRD (create v0.2.0 version)
- Revised epic definitions (Epic 0 + Epic 1)
- Communication plan for stakeholders
- Success metrics for GUI adoption

---

#### 4. Development Team
**Responsibilities:**
- Learn Tauri framework and development workflow
- Implement GUI foundation (Epic 0: 5 stories)
- Build GUI workflows (Epic 1: 5 stories rewritten)
- Maintain CLI functionality and compatibility
- Write tests (unit, integration, E2E)

**Deliverables:**
- Working Tauri application
- GUI screens matching UX specifications
- IPC command implementations
- Integration tests ensuring CLI/GUI interop
- Developer documentation

---

#### 5. Scrum Master
**Responsibilities:**
- Update sprint-status.yaml with new epic/story structure
- Resequence remaining epics based on dependencies
- Manage team capacity during learning curve
- Track progress on GUI implementation
- Facilitate retrospectives on pivot decision

**Deliverables:**
- Updated sprint-status.yaml
- Revised v0.2.0 timeline
- Team velocity tracking during transition
- Retrospective findings and lessons learned

---

### Implementation Timeline

**Week 1: Planning & Setup**
- Architecture design and documentation
- UX wireframes (initial drafts)
- Team Tauri training
- Development environment setup

**Week 2-3: Core Implementation**
- Epic 0: GUI Foundation (5 stories)
- Epic 1: GUI Workflows (5 stories)
- Iterative UX feedback and refinement

**Week 4: Integration & Launch**
- Testing (unit, integration, cross-platform)
- Documentation completion
- Internal dogfooding
- Release preparation

---

## Success Criteria

### Technical Success
- [ ] Tauri successfully integrated into zprof
- [ ] GUI application builds and runs on macOS and Linux
- [ ] All 5 Epic 0 stories completed and tested
- [ ] All 5 Epic 1 GUI stories completed and tested
- [ ] Theme preview displays accurate prompt rendering
- [ ] CLI commands remain 100% functional
- [ ] No regression in profile switching performance (<500ms)
- [ ] Integration tests pass for CLI/GUI interoperability

### User Experience Success
- [ ] Users can create profile via GUI in < 3 minutes
- [ ] Theme preview shows visual difference between themes
- [ ] Multiple workflow instances can run simultaneously
- [ ] GUI operates as separate process (terminal remains usable)
- [ ] Light/dark mode works and respects system preferences
- [ ] Application startup < 2 seconds

### Project Success
- [ ] Timeline maintained (9-13 days comparable to original 16-day plan)
- [ ] Team successfully adopts Tauri with minimal friction
- [ ] Architecture documentation updated and accurate
- [ ] UX design artifacts created and validated
- [ ] Stakeholders informed and supportive of pivot

---

## Risks & Mitigations

| Risk | Probability | Impact | Mitigation |
|------|------------|--------|------------|
| Tauri learning curve delays timeline | Medium | Medium | Allocate dedicated learning time, use excellent documentation, start with simple screens |
| Theme preview technically complex | Medium | Medium | MVP: simple text rendering, iterate to improve accuracy |
| Cross-platform issues | Low | Medium | Tauri handles abstractions, test early on both platforms |
| CLI regression | Low | High | Comprehensive integration tests, manual verification |
| Team resistance to pivot | Low | Medium | Clear communication of benefits, involve team in decisions |
| Window management complexity | Low | Low | Use Tauri built-in capabilities, defer advanced features |

---

## Appendix

### Code Removal Summary

**Files to Delete:**
- `src/tui/prompt_mode_select.rs` (310 lines)
- `src/tui/prompt_engine_select.rs` (345 lines)
- `src/tui/theme_select.rs` (493 lines)
- `src/tui/framework_select.rs` (315 lines)
- `src/tui/plugin_browser.rs` (501 lines)
- `src/tui/mod.rs` (75 lines)

**Total Removed:** ~2,039 lines of TUI code

**Files to Keep (Data Models):**
- `src/core/manifest.rs` (PromptMode enum and schema)
- `src/prompts/engine.rs` (PromptEngine registry)
- `src/prompts/mod.rs`

**Dependencies to Remove:**
```toml
# Remove from Cargo.toml
ratatui = "0.29.0"
crossterm = "0.28.1"
```

**Dependencies to Add:**
```toml
# Add to Cargo.toml
[dependencies]
tauri = { version = "2.0", features = ["shell-open"] }
serde_json = "1.0"

[build-dependencies]
tauri-build = { version = "2.0" }
```

### New File Structure

```
zprof/
├── src-tauri/              # Tauri Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json     # Tauri configuration
│   ├── build.rs
│   └── src/
│       ├── main.rs         # Tauri app entry
│       └── lib.rs
├── src-ui/                 # Web frontend
│   ├── package.json
│   ├── vite.config.js
│   ├── src/
│   │   ├── App.svelte      # Main app component
│   │   ├── components/     # Reusable components
│   │   │   ├── Sidebar.svelte
│   │   │   ├── ProfileCard.svelte
│   │   │   ├── PromptModeSelect.svelte
│   │   │   ├── EngineCard.svelte
│   │   │   ├── ThemePreview.svelte
│   │   │   └── ...
│   │   ├── views/          # Main views
│   │   │   ├── ProfileList.svelte
│   │   │   ├── CreateWizard.svelte
│   │   │   └── Settings.svelte
│   │   └── lib/
│   │       ├── api.ts      # IPC client
│   │       ├── router.ts
│   │       └── theme-renderer.ts
│   └── public/
└── src/                    # Existing Rust code
    ├── gui/                # NEW - GUI-specific code
    │   ├── mod.rs
    │   ├── commands.rs     # Tauri command handlers
    │   └── types.rs        # Shared types
    ├── cli/                # Existing CLI (unchanged)
    ├── core/               # Existing business logic (unchanged)
    ├── frameworks/         # Existing (unchanged)
    └── ...
```

---

## Conclusion

This Sprint Change Proposal recommends a strategic pivot from TUI to GUI using Tauri framework. The approach:

✅ **Preserves** valuable work (data models, business logic)
✅ **Enables** features impossible in TUI (theme preview, multi-workflow)
✅ **Maintains** similar timeline (9-13 days vs 16 days)
✅ **Reduces** technical debt (one UI paradigm vs two)
✅ **Delivers** better user experience aligned with stated priorities

The pivot is **low risk** due to preserved business logic and proven technology, with **high value** from dramatically improved UX capabilities.

**Recommendation:** Approve this proposal and proceed with Epic 0 (GUI Foundation) implementation.

---

**Prepared by:** Claude (BMM Correct-Course Workflow)
**Date:** 2025-11-21
**For Review by:** Anna (Product Owner), Development Team, Solution Architect
