# Epic 0: GUI Foundation

**Priority:** P0 (Blocking - Must complete before Epic 1)
**Estimated Effort:** 2-3 days
**Owner:** TBD
**Status:** Proposed (Pending Sprint Change Approval)

## Overview

Establish the foundational Tauri-based GUI architecture for zprof, enabling a native desktop application with rich visual capabilities. This epic creates the base infrastructure that all subsequent GUI workflows will build upon, while maintaining full CLI compatibility.

## Problem Statement

The current TUI approach cannot deliver the visual capabilities required for zprof:
- Cannot preview themes visually
- Limited to 80x24 terminal constraints
- No support for multi-workflow instances
- Cannot run as separate process from terminal

A GUI solution using Tauri provides:
- Rich visual previews and graphics
- Flexible layouts and responsive design
- Multi-window/instance support
- Separate process architecture
- Native performance with web UI flexibility

## Goals

1. **Tauri Integration**: Successfully integrate Tauri framework into zprof codebase
2. **Base Application**: Create foundational window, navigation, and routing
3. **IPC Layer**: Establish robust communication between frontend and Rust backend
4. **First Real Screen**: Implement profile list view as proof-of-concept
5. **CLI Preservation**: Ensure all existing CLI commands remain fully functional

## User Stories

### Story 0.1: Install Tauri and Initialize Project

**As a** developer
**I want** Tauri installed and configured in the zprof project
**So that** we can build GUI applications with Rust backend

**Acceptance Criteria:**
- [ ] Install Tauri CLI: `cargo install tauri-cli`
- [ ] Initialize Tauri in project: `cargo tauri init`
- [ ] Choose Svelte as frontend framework
- [ ] Configure project structure:
  - `src-tauri/` for Tauri Rust backend
  - `src-ui/` for Svelte frontend
  - Keep existing `src/` for core business logic
- [ ] Update `.gitignore` for Tauri artifacts
- [ ] Configure `tauri.conf.json`:
  - App name: "zprof"
  - Window title: "zprof - Zsh Profile Manager"
  - Window size: 1200x800 (resizable)
  - macOS and Linux targets
- [ ] Add dependencies to `Cargo.toml`:
  ```toml
  [dependencies]
  tauri = { version = "2.0", features = ["shell-open"] }
  serde = { version = "1.0", features = ["derive"] }
  serde_json = "1.0"

  [build-dependencies]
  tauri-build = { version = "2.0" }
  ```
- [ ] Create basic `src-tauri/src/main.rs` entry point
- [ ] Verify build: `cargo tauri dev` launches empty window
- [ ] Verify production build: `cargo tauri build` succeeds
- [ ] Add build instructions to README.md

**Files:**
- `src-tauri/Cargo.toml` (NEW)
- `src-tauri/tauri.conf.json` (NEW)
- `src-tauri/build.rs` (NEW)
- `src-tauri/src/main.rs` (NEW)
- `src-tauri/src/lib.rs` (NEW)
- `src-ui/package.json` (NEW)
- `src-ui/vite.config.js` (NEW)
- `src-ui/src/main.js` (NEW)
- `src-ui/src/App.svelte` (NEW)
- `.gitignore` (updated)
- `README.md` (updated)

**Dependencies:**
- Node.js 18+ and npm/pnpm
- Tauri prerequisites (per platform)

---

### Story 0.2: Create Base Application Window and Navigation

**As a** user
**I want** a clean, intuitive application window with navigation
**So that** I can access different features of zprof

**Acceptance Criteria:**
- [ ] Create main application layout with:
  - Sidebar navigation (collapsible)
  - Main content area
  - Title bar (if using custom window controls)
- [ ] Implement navigation structure:
  - Profiles (list view)
  - Create Profile (wizard)
  - Settings
  - About
- [ ] Add routing with Svelte Router or similar:
  - `/profiles` - Profile list (default)
  - `/create` - Create wizard
  - `/settings` - Settings panel
  - `/about` - About/version info
- [ ] Create reusable UI components:
  - `Sidebar.svelte` - Navigation sidebar
  - `Header.svelte` - Top header/title bar
  - `Button.svelte` - Styled button component
  - `Card.svelte` - Content card component
- [ ] Implement light/dark mode toggle:
  - Respect system theme preference
  - Manual toggle override
  - Persist preference to settings
- [ ] Style with Tailwind CSS or similar:
  - Clean, modern design
  - Consistent spacing and typography
  - Accessible color contrast
- [ ] Add keyboard shortcuts:
  - `Cmd/Ctrl + ,` - Settings
  - `Cmd/Ctrl + N` - New profile
  - `Cmd/Ctrl + Q` - Quit
- [ ] Handle window events:
  - Close button confirmation (if unsaved changes)
  - Window resize persists to settings
  - Window position restore on launch

**Files:**
- `src-ui/src/App.svelte` (updated)
- `src-ui/src/components/Sidebar.svelte` (NEW)
- `src-ui/src/components/Header.svelte` (NEW)
- `src-ui/src/components/Button.svelte` (NEW)
- `src-ui/src/components/Card.svelte` (NEW)
- `src-ui/src/lib/router.js` (NEW)
- `src-ui/src/lib/theme.js` (NEW)
- `src-ui/src/styles/main.css` (NEW)
- `src-tauri/src/main.rs` (add window event handlers)

**Design Notes:**
- Sidebar width: 240px (collapsed: 60px)
- Color palette: System-native or custom (to be defined by UX)
- Icons: Use Lucide icons or similar

---

### Story 0.3: Implement IPC Command Layer

**As a** developer
**I want** a robust IPC layer between frontend and backend
**So that** the GUI can interact with zprof's business logic

**Acceptance Criteria:**
- [ ] Create Tauri command module: `src-tauri/src/commands.rs`
- [ ] Implement core IPC commands:
  - `list_profiles()` → `Vec<ProfileInfo>`
  - `get_profile(name: String)` → `Result<Profile>`
  - `get_active_profile()` → `Option<String>`
  - `create_profile(config: ProfileConfig)` → `Result<String>`
  - `delete_profile(name: String)` → `Result<()>`
  - `activate_profile(name: String)` → `Result<()>`
  - `get_frameworks()` → `Vec<Framework>`
  - `get_plugins(framework: String)` → `Vec<Plugin>`
  - `get_themes(framework: String)` → `Vec<Theme>`
- [ ] Define shared types in `src-tauri/src/types.rs`:
  - `ProfileInfo` (id, name, framework, active, created_at)
  - `ProfileConfig` (framework, prompt_mode, plugins, etc.)
  - `Framework`, `Plugin`, `Theme` metadata
- [ ] Reuse existing business logic from `src/`:
  - Import and wrap existing functions
  - Convert between GUI types and core types
  - Handle errors and convert to JSON-serializable format
- [ ] Create frontend API client: `src-ui/src/lib/api.ts`
  - Wrapper functions for all IPC commands
  - TypeScript type definitions matching backend
  - Error handling and user-friendly messages
- [ ] Add error handling:
  - Return `Result<T, String>` from commands
  - Log errors to console and Tauri debug
  - Display error notifications in UI
- [ ] Add loading states:
  - Commands return immediately with loading indicator
  - Update UI when async operations complete
- [ ] Add integration tests:
  - Mock IPC calls in frontend tests
  - Test command handlers in Tauri backend tests

**Files:**
- `src-tauri/src/commands.rs` (NEW)
- `src-tauri/src/types.rs` (NEW)
- `src-tauri/src/error.rs` (NEW - error types)
- `src-tauri/src/main.rs` (register commands)
- `src-ui/src/lib/api.ts` (NEW)
- `src-ui/src/lib/types.ts` (NEW)
- `tests/gui_commands_test.rs` (NEW)

**Technical Notes:**
- Use `#[tauri::command]` attribute for all commands
- Serialize/deserialize with `serde_json`
- Use `anyhow::Result` internally, convert to `Result<T, String>` at IPC boundary

---

### Story 0.4: Create Profile List View (First Real Screen)

**As a** user
**I want** to see all my profiles in a clean list view
**So that** I can understand what profiles exist and which is active

**Acceptance Criteria:**
- [ ] Create `ProfileList.svelte` view component
- [ ] Display profiles as cards in a grid layout:
  - Profile name (large, bold)
  - Framework name and icon
  - Prompt mode (engine name or "Built-in theme")
  - Plugin count (e.g., "12 plugins")
  - Created date (relative: "2 days ago")
  - Active indicator (badge/checkmark)
- [ ] Add profile actions (on hover/click):
  - "Activate" button (if not active)
  - "Edit" button (future - show as disabled)
  - "Delete" button (with confirmation dialog)
  - "Duplicate" button (future - show as disabled)
- [ ] Handle empty state:
  - Show welcome message
  - Large "Create Profile" CTA button
  - Quick start guide link
- [ ] Add "Create New Profile" button in header
- [ ] Implement search/filter:
  - Search by profile name
  - Filter by framework
  - Filter by active/inactive
- [ ] Add sorting options:
  - Sort by name (A-Z, Z-A)
  - Sort by created date (newest, oldest)
  - Sort by last used (future)
- [ ] Integrate with IPC commands:
  - Call `list_profiles()` on mount
  - Call `activate_profile()` on activate button
  - Call `delete_profile()` on delete confirmation
  - Refresh list after mutations
- [ ] Add loading skeleton during data fetch
- [ ] Add error state if profile loading fails
- [ ] Implement delete confirmation dialog:
  - Show profile name
  - Warn about data loss
  - "Delete" (destructive) vs "Cancel" buttons

**Files:**
- `src-ui/src/views/ProfileList.svelte` (NEW)
- `src-ui/src/components/ProfileCard.svelte` (NEW)
- `src-ui/src/components/EmptyState.svelte` (NEW)
- `src-ui/src/components/ConfirmDialog.svelte` (NEW)
- `src-ui/src/components/SearchBar.svelte` (NEW)
- `src-ui/src/App.svelte` (add route)

**Design Mockup:**
```
┌──────────────────────────────────────────────────────────┐
│  [☰] zprof                        [+] New Profile   [⚙]  │
├──────────────────────────────────────────────────────────┤
│  🔍 Search profiles...         Framework: [All ▾]  ⋮Sort │
├──────────────────────────────────────────────────────────┤
│  ┌─────────────────┐  ┌─────────────────┐              │
│  │ ✓ work          │  │   personal      │              │
│  │ oh-my-zsh       │  │   zimfw         │              │
│  │ Starship        │  │   Built-in      │              │
│  │ 12 plugins      │  │   8 plugins     │              │
│  │ 2 days ago      │  │   1 week ago    │              │
│  │ [Active]        │  │ [Activate] [×]  │              │
│  └─────────────────┘  └─────────────────┘              │
└──────────────────────────────────────────────────────────┘
```

---

### Story 0.5: Ensure CLI Compatibility

**As a** developer
**I want** all existing CLI commands to work without regression
**So that** users can choose between GUI and CLI

**Acceptance Criteria:**
- [ ] Verify all CLI commands still work:
  - `zprof init`
  - `zprof create <name>`
  - `zprof list`
  - `zprof use <name>`
  - `zprof delete <name>`
  - `zprof show <name>`
  - All other existing commands
- [ ] Add CLI integration tests:
  - Test each command in isolation
  - Test with GUI running in background
  - Test with GUI closed
- [ ] Ensure no dependency conflicts:
  - CLI binary size doesn't bloat from GUI deps
  - CLI startup time remains fast (<100ms)
  - GUI dependencies are optional at compile time
- [ ] Add feature flags if needed:
  - `gui` feature (default enabled)
  - CLI compiles without GUI if feature disabled
  - `cargo build --no-default-features` works
- [ ] Add `zprof gui` command:
  - Launch GUI application from CLI
  - `zprof gui --help` shows GUI options
  - `zprof gui --version` shows version info
- [ ] Update help text:
  - Mention GUI availability in `zprof --help`
  - Add "GUI" section to command list
  - Document keyboard shortcuts
- [ ] Add E2E test suite:
  - CLI creates profile → GUI displays it
  - GUI creates profile → CLI can use it
  - CLI activates profile → GUI shows active badge
  - GUI deletes profile → CLI doesn't see it
- [ ] Document build process:
  - How to build GUI version
  - How to build CLI-only version
  - Platform-specific notes (macOS app bundle, Linux AppImage)

**Files:**
- `src/main.rs` (add `gui` subcommand)
- `src/cli/gui.rs` (NEW - launch GUI command)
- `Cargo.toml` (add feature flags)
- `tests/cli_gui_interop_test.rs` (NEW)
- `README.md` (update build instructions)
- `docs/developer/building.md` (NEW or update)

**Success Criteria:**
- All existing CLI tests pass
- No performance regression in CLI
- GUI and CLI can interoperate seamlessly
- Documentation clear for both usage paths

---

## Technical Design

### Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    zprof Application                     │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌──────────────┐              ┌──────────────┐        │
│  │  CLI Entry   │              │  GUI Entry   │        │
│  │  (main.rs)   │              │ (tauri/main) │        │
│  └──────┬───────┘              └──────┬───────┘        │
│         │                             │                 │
│         │  ┌──────────────────────────┼────────┐       │
│         │  │  IPC Layer (Tauri)       │        │       │
│         │  │  commands.rs             │        │       │
│         │  └──────────────────────────┼────────┘       │
│         │                             │                 │
│         └─────────────┬───────────────┘                 │
│                       │                                  │
│              ┌────────▼────────┐                        │
│              │  Core Business  │                        │
│              │     Logic       │                        │
│              │  (src/core,     │                        │
│              │   frameworks,   │                        │
│              │   shell, etc.)  │                        │
│              └─────────────────┘                        │
│                                                          │
├─────────────────────────────────────────────────────────┤
│                  Frontend (src-ui)                       │
│  ┌────────┐  ┌──────────┐  ┌────────────┐             │
│  │ Views  │→ │Components│→ │ API Client │             │
│  └────────┘  └──────────┘  └────────────┘             │
│                                  ↑                       │
│                                  │ IPC                  │
│                         (invoke Tauri commands)         │
└─────────────────────────────────────────────────────────┘
```

### Project Structure After Epic 0

```
zprof/
├── src-tauri/              # Tauri Rust backend
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   ├── build.rs
│   ├── icons/              # App icons
│   └── src/
│       ├── main.rs         # Tauri entry point
│       ├── lib.rs          # Public library interface
│       ├── commands.rs     # IPC command handlers
│       ├── types.rs        # Shared types (GUI-specific)
│       └── error.rs        # Error handling
│
├── src-ui/                 # Svelte frontend
│   ├── package.json
│   ├── vite.config.js
│   ├── tsconfig.json
│   ├── public/
│   │   └── favicon.ico
│   └── src/
│       ├── main.js
│       ├── App.svelte
│       ├── components/     # Reusable UI components
│       │   ├── Sidebar.svelte
│       │   ├── Header.svelte
│       │   ├── Button.svelte
│       │   ├── Card.svelte
│       │   ├── ProfileCard.svelte
│       │   ├── EmptyState.svelte
│       │   ├── ConfirmDialog.svelte
│       │   └── SearchBar.svelte
│       ├── views/          # Main application views
│       │   ├── ProfileList.svelte
│       │   ├── CreateWizard.svelte (placeholder)
│       │   ├── Settings.svelte (placeholder)
│       │   └── About.svelte
│       ├── lib/
│       │   ├── api.ts      # IPC client wrapper
│       │   ├── types.ts    # TypeScript types
│       │   ├── router.js   # Routing logic
│       │   └── theme.js    # Theme management
│       └── styles/
│           └── main.css
│
├── src/                    # Existing Rust core (mostly unchanged)
│   ├── main.rs             # CLI entry (add gui subcommand)
│   ├── cli/
│   │   ├── gui.rs          # NEW - GUI launch command
│   │   └── ...             # Existing CLI commands
│   ├── core/               # Business logic (unchanged)
│   ├── frameworks/         # Framework support (unchanged)
│   ├── shell/              # Shell generation (unchanged)
│   └── ...
│
├── tests/
│   ├── gui_commands_test.rs (NEW)
│   ├── cli_gui_interop_test.rs (NEW)
│   └── ...                 # Existing tests
│
├── Cargo.toml              # Root workspace
├── README.md               # Updated with GUI instructions
└── docs/
    └── developer/
        └── building.md     # Build instructions
```

### Data Flow Examples

**Profile List Loading:**
```
User opens app
  ↓
ProfileList.svelte mounts
  ↓
Calls api.listProfiles()
  ↓
invoke('list_profiles') via Tauri IPC
  ↓
commands::list_profiles() in Rust
  ↓
Reads from src/core/profile.rs
  ↓
Returns Vec<ProfileInfo> as JSON
  ↓
Frontend displays in ProfileCard components
```

**Profile Activation:**
```
User clicks "Activate" button
  ↓
ProfileCard emits activate event
  ↓
Calls api.activateProfile(name)
  ↓
invoke('activate_profile', { name }) via IPC
  ↓
commands::activate_profile(name) in Rust
  ↓
Calls existing src/cli/use.rs logic
  ↓
Returns Result<()>
  ↓
Frontend refreshes profile list
  ↓
Active badge appears on activated profile
```

## Dependencies

**Blocks:**
- Epic 1 (Smart GUI Workflow) - Requires GUI foundation
- Epic 2 (Presets) - GUI preset selection needs base UI

**Depends on:**
- Existing business logic in `src/core/`, `src/frameworks/`, `src/shell/`
- Manifest schema from Story 1.1 (PromptMode enum)
- Prompt engine registry from Story 1.3

## Risks & Mitigations

**Risk:** Tauri learning curve delays implementation
**Mitigation:** Excellent documentation, start simple, iterate. Allocate time for team learning.

**Risk:** Frontend/backend type mismatches cause bugs
**Mitigation:** Use TypeScript on frontend, codegen types from Rust if possible, integration tests.

**Risk:** IPC performance bottleneck for large data
**Mitigation:** Start simple, optimize if needed. Profile list unlikely to have >100 items.

**Risk:** Platform-specific build issues
**Mitigation:** Test on macOS and Linux early, document platform-specific setup, CI for both platforms.

**Risk:** CLI regression from GUI dependencies
**Mitigation:** Feature flags, separate binaries if needed, comprehensive CLI test suite.

## Testing Strategy

- **Unit tests:** Tauri command handlers, API client functions
- **Integration tests:** IPC round-trips, CLI/GUI interop
- **E2E tests:** User workflows (launch app, view profiles, activate)
- **Manual testing:** UI/UX, cross-platform compatibility
- **Snapshot tests:** UI component rendering (Svelte Testing Library)

## Success Criteria

- [ ] Tauri successfully integrated and builds on macOS and Linux
- [ ] Base application window launches with navigation
- [ ] Profile list view displays actual profiles from disk
- [ ] All IPC commands work and are tested
- [ ] All existing CLI commands pass tests
- [ ] `zprof gui` command launches GUI application
- [ ] Light/dark mode works
- [ ] Profile activation works from GUI
- [ ] Delete profile works with confirmation
- [ ] Documentation updated with build instructions
- [ ] No performance regression in CLI

## Out of Scope

- Profile creation wizard (Epic 1)
- Theme preview (Epic 1, Story 1.5)
- Plugin browsing (Epic 1)
- Settings customization (v0.3.0)
- Multi-window support (v0.3.0)
- Auto-update mechanism (v0.3.0)
- Crash reporting (v0.4.0)
- Windows support (future)

## Notes

This epic establishes the foundation for all GUI work in zprof. Once complete, Epic 1 can proceed with visual workflows, theme previews, and the full create wizard experience.

**Estimated Timeline:**
- Story 0.1: 0.5 days (setup)
- Story 0.2: 0.5 days (base UI)
- Story 0.3: 0.5 days (IPC layer)
- Story 0.4: 0.5-1 day (profile list)
- Story 0.5: 0.5 day (CLI compatibility)
- **Total: 2.5-3.5 days**

---

**Epic Status:** Proposed (Pending approval of Sprint Change Proposal)
**Created:** 2025-11-21
**Last Updated:** 2025-11-21
