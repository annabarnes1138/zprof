# zprof - Epic Breakdown

**Author:** Anna
**Date:** 2025-10-31
**Project Level:** 2
**Target Scale:** MVP - Level 2 (focused, 1-2 epics, 5-15 stories)

---

## Overview

This document provides the detailed epic breakdown for zprof, expanding on the high-level epic list in the [PRD](./PRD.md).

Each epic includes:

- Expanded goal and value proposition
- Complete story breakdown with user stories
- Acceptance criteria for each story
- Story sequencing and dependencies

**Epic Sequencing Principles:**

- Epic 1 establishes foundational infrastructure and initial functionality
- Subsequent epics build progressively, each delivering significant end-to-end value
- Stories within epics are vertically sliced and sequentially ordered
- No forward dependencies - each story builds only on previous work

---

## Epic 1: Core Profile Management & TUI Wizard

**Expanded Goal:**

This epic establishes the foundational infrastructure for zprof and delivers the core profile management capabilities. Users will be able to initialize the system, create profiles through an interactive TUI wizard with smart detection of existing configurations, switch between profiles instantly, and manage their profile collection. This epic provides immediate value by enabling risk-free experimentation with different zsh frameworks while preserving working configurations.

---

### Story 1.1a: Initialize zprof Directory Structure

As a developer,
I want to initialize zprof's directory structure in my home directory,
So that I have a clean foundation for managing multiple zsh profiles.

**Acceptance Criteria:**

1. `zprof init` command creates `~/.zsh-profiles/` directory structure with `profiles/`, `shared/`, and `cache/` subdirectories
2. Creates `shared/` directory with placeholder files: `aliases.zsh`, `env.zsh`, `functions.zsh`
3. Creates `shared/history/` directory for shared command history
4. Creates global configuration file `config.toml` with sensible defaults:
   - `shared_history = true`
   - `default_framework = "oh-my-zsh"` (optional)
5. Command outputs success message confirming initialization
6. Running `zprof init` when already initialized warns user but does not corrupt existing data
7. Does NOT migrate existing configuration (that's Story 1.1b)

**Prerequisites:** None (first story)

---

### Story 1.1b: Migrate Existing Configuration During Init

As a developer with an existing zsh setup,
I want zprof init to detect and migrate my current configuration,
So that I can adopt zprof without losing my working shell environment.

**Acceptance Criteria:**

1. During `zprof init`, system detects existing framework (oh-my-zsh, zinit, prezto, zimfw, zap) in home directory
2. If framework detected, shows detailed migration preview:
   - What will be backed up (`.zshrc` → `.zshrc.pre-zprof`)
   - What will be moved (framework → `profiles/default/<framework>/`)
   - What will be categorized (configs → `shared/` or `profiles/default/`)
3. Requires explicit user confirmation: "Proceed with migration? [y/N]"
4. On confirmation, creates first profile named "default" (user can choose different name)
5. Migration process:
   - Backs up original `.zshrc` to `profiles/default/.zshrc.pre-zprof`
   - Moves framework installation to `profiles/default/<framework>/`
   - Parses `.zshrc` and categorizes configurations using heuristics:
     - Generic aliases → `shared/aliases.zsh`
     - Generic env vars → `shared/env.zsh`
     - Framework-specific configs → `profiles/default/<framework-config>.zsh`
     - Profile-specific overrides → `profiles/default/aliases.zsh`, `profiles/default/env.zsh`
   - Optionally moves `.zsh_history` to `shared/history/` (based on user preference)
6. Generates new minimal `~/.zshrc` containing only zprof bootstrap code
7. Creates `profiles/default/profile.toml` manifest from detected configuration
8. Outputs detailed success message showing what was migrated and where
9. Informs user about rollback: "To undo, run: zprof rollback"
10. If no framework detected, proceeds with clean initialization (Story 1.1a behavior)

**Prerequisites:** Story 1.1a (requires basic directory structure), Story 1.4 (requires framework detection)

---

### Story 1.2: List Available Profiles

As a developer,
I want to see all my available profiles with a visual indicator for the active one,
So that I know what profiles exist and which one I'm currently using.

**Acceptance Criteria:**

1. `zprof list` command displays all profiles in `~/.zsh-profiles/profiles/`
2. Active profile is visually indicated (e.g., with `*` or arrow)
3. Each profile shows its name and framework type
4. Output is human-readable and formatted clearly
5. Command handles empty profile directory gracefully with helpful message

**Prerequisites:** Story 1.1 (requires initialized directory structure)

---

### Story 1.3: Display Current Active Profile

As a developer,
I want to quickly check which profile is currently active,
So that I can confirm my shell environment context.

**Acceptance Criteria:**

1. `zprof current` command displays the currently active profile name
2. Output includes profile metadata (framework, creation date)
3. If no profile is active, displays clear message
4. Command executes in under 100ms for quick reference

**Prerequisites:** Story 1.1 (requires initialized directory structure)

---

### Story 1.4: Framework Detection for Smart Profile Creation

As a developer,
I want zprof to detect my existing zsh framework configuration,
So that I can preserve my current setup when creating my first profile.

**Acceptance Criteria:**

1. System scans for oh-my-zsh, zimfw, prezto, zinit, and zap installations
2. Detection identifies framework type, installed plugins, and active theme
3. If framework detected, system captures configuration details for import
4. Detection completes in under 2 seconds
5. Gracefully handles multiple frameworks or corrupted installations

**Prerequisites:** Story 1.1 (requires initialized directory structure)

---

### Story 1.5: Quick Profile Creation via CLI

As a developer,
I want to quickly create a new profile from the command line,
So that I can add profiles without going through the full TUI wizard.

**Acceptance Criteria:**

1. `zprof create <name>` creates a new empty profile with default settings
2. User can optionally specify framework with `--framework` flag (e.g., `zprof create work --framework zinit`)
3. If no framework specified, uses `default_framework` from config.toml or prompts user
4. Creates profile directory structure with minimal `profile.toml` manifest
5. Generates basic `.zshrc` and `.zshenv` for the profile
6. Success message confirms profile creation and suggests using TUI wizard for customization
7. Validates profile name (alphanumeric + hyphens, no conflicts with existing profiles)

**Prerequisites:** Story 1.1a (requires initialized directory structure)

---

### Story 1.6: TUI Wizard Framework Selection

As a developer,
I want an interactive menu to select a zsh framework,
So that I can easily choose the framework for my new profile.

**Acceptance Criteria:**

1. TUI displays list of supported frameworks (oh-my-zsh, zinit, prezto, zimfw, zap, vanilla)
2. Each framework shows brief description and key characteristics:
   - oh-my-zsh: "Most popular, extensive plugin ecosystem"
   - zinit: "Fast, modern plugin manager"
   - prezto: "Configuration framework with curated modules"
   - zimfw: "Fast, modular framework"
   - zap: "Minimal plugin manager"
   - vanilla: "No framework, pure zsh (maximum performance)"
3. Keyboard navigation with arrow keys and enter to select
4. Selected framework is highlighted visually
5. TUI is responsive and works in 80x24 terminal minimum
6. Supports both light and dark terminal themes
7. Selecting "vanilla" skips plugin/theme selection and creates minimal profile

**Prerequisites:** Story 1.1a (requires initialized directory structure)

---

### Story 1.7: TUI Wizard Plugin Browser

As a developer,
I want to browse and select plugins for my profile,
So that I can customize my shell with useful tools.

**Acceptance Criteria:**

1. TUI displays popular plugins with descriptions for selected framework
2. Multi-select interface allows checking/unchecking plugins
3. At least 10-15 popular plugins per framework with recommendations
4. Search/filter capability for finding specific plugins
5. Selected plugins are highlighted and counted
6. Can proceed with no plugins selected (minimal setup)

**Prerequisites:** Story 1.6 (requires framework selection)

---

### Story 1.8: TUI Wizard Theme Selection and Profile Generation

As a developer,
I want to select a theme and finalize my profile creation,
So that I have a complete, working zsh profile.

**Acceptance Criteria:**

1. TUI displays available themes for selected framework with previews/descriptions
2. Single-select interface for choosing one theme
3. If vanilla zsh selected, skips theme selection (no themes for vanilla)
4. Final confirmation screen shows all selections (framework, plugins, theme)
5. On confirmation, system installs selected framework and plugins to profile directory
6. Generates `profile.toml` manifest with all selections
7. Creates functional `.zshrc` and `.zshenv` in profile directory with proper sourcing order:
   - Shared configs (`shared/aliases.zsh`, `shared/env.zsh`, `shared/functions.zsh`)
   - Framework initialization (profile-scoped)
   - Profile-specific overrides (`aliases.zsh`, `env.zsh`)
8. Installation progress is displayed with clear status messages
9. Success message confirms profile is ready to use with `zprof use <name>`

**Prerequisites:** Story 1.7 (requires plugin selection)

---

### Story 1.9: Switch Active Profile

As a developer,
I want to switch between my profiles quickly,
So that I can change my shell environment for different contexts.

**Acceptance Criteria:**

1. `zprof use <profile-name>` updates ZDOTDIR to point to selected profile
2. New shell instance is launched with selected profile active
3. Switching completes in under 500ms
4. Shared command history is accessible in new profile
5. Clear confirmation message shows which profile is now active
6. Handles invalid profile names with helpful error message

**Prerequisites:** Story 1.5 or 1.8 (requires at least one profile to exist)

---

### Story 1.10: Delete Profile

As a developer,
I want to delete profiles I no longer need,
So that I can keep my profile collection clean and manageable.

**Acceptance Criteria:**

1. `zprof delete <profile-name>` prompts for confirmation before deletion
2. Confirmation shows profile name and warns action is irreversible
3. On confirmation, removes profile directory and all contents
4. Cannot delete currently active profile (requires switching first)
5. Success message confirms deletion
6. Shared history and other profiles remain unaffected

**Prerequisites:** Story 1.9 (requires profile switching capability)

---

### Story 1.11: Rollback to Pre-zprof State

As a developer who wants to uninstall zprof,
I want to restore my original shell configuration,
So that I can revert to my pre-zprof setup if needed.

**Acceptance Criteria:**

1. `zprof rollback` command checks for backup file (`.zshrc.pre-zprof`) in profiles
2. Shows what will be restored and what will be moved back:
   - Restore: `~/.zshrc` from backup
   - Move: Framework back to home directory (if applicable)
   - Keep: `~/.zsh-profiles/` directory for reference
3. Requires explicit confirmation: "Continue? [y/N]"
4. On confirmation:
   - Restores original `.zshrc` from backup
   - Moves framework back to original location (e.g., `~/.oh-my-zsh`)
   - Leaves `~/.zsh-profiles/` intact but inactive
5. Success message confirms rollback and provides instructions:
   - Restart shell or run: `source ~/.zshrc`
   - Can safely delete `~/.zsh-profiles/` manually if desired
6. If no backup found, provides clear error message
7. Cannot rollback if backup was modified or deleted

**Prerequisites:** Story 1.1b (requires migration with backup)

---

## Epic 2: TOML Manifests & Export/Import

**Expanded Goal:**

This epic transforms profiles from purely TUI-generated configurations into declarative, version-controllable TOML manifests. It enables power users to manually edit profile configurations and establishes the foundation for a shareable profile ecosystem through export/import capabilities. Users will be able to package profiles into portable archives and share them via local files or GitHub repositories, enabling team standardization and community contribution.

---

### Story 2.1: Parse and Validate TOML Manifests

As a developer,
I want zprof to parse and validate my profile TOML manifests,
So that I can ensure my profile configuration is correct before applying it.

**Acceptance Criteria:**

1. System reads `profile.toml` files and validates schema (name, framework, plugins, theme, environment variables)
2. Validation checks for required fields and correct data types
3. Clear error messages identify specific validation failures with line numbers
4. Successfully validated manifests are marked as ready for use
5. Invalid manifests prevent profile activation with helpful guidance
6. TOML parsing handles explicit typing and prevents indentation errors

**Prerequisites:** Story 1.5 or 1.8 (requires profiles with generated manifests)

---

### Story 2.2: Generate Shell Configuration from TOML

As a developer,
I want zprof to automatically generate .zshrc and .zshenv from my TOML manifest,
So that my declarative configuration is translated into functional shell files.

**Acceptance Criteria:**

1. System generates `.zshrc` with proper sourcing order:
   - Shared configs (`shared/aliases.zsh`, `shared/env.zsh`, `shared/functions.zsh`)
   - Framework initialization (profile-scoped)
   - Profile-specific overrides (`aliases.zsh`, `env.zsh`)
   - History configuration (shared or profile-specific)
2. System generates `.zshenv` with environment variables from manifest
3. Generated files include header comments indicating they're auto-generated from manifest
4. Re-generation from manifest overwrites previous generated files (manifest is source of truth)
5. Generated configuration is syntactically valid zsh code
6. Process completes in under 1 second for typical profiles

**Prerequisites:** Story 2.1 (requires TOML parsing)

---

### Story 2.3: Manual TOML Editing with Live Validation

As a power user,
I want to manually edit my profile TOML and receive validation feedback,
So that I can quickly customize profiles without using the TUI wizard.

**Acceptance Criteria:**

1. `zprof edit <profile-name>` opens `profile.toml` in user's $EDITOR
2. After saving, system validates TOML and reports any errors
3. If valid, regenerates `.zshrc` and `.zshenv` from updated manifest
4. If invalid, preserves old configuration and shows validation errors
5. User can retry edit or cancel without breaking profile
6. Changes take effect on next `zprof use <profile-name>`

**Prerequisites:** Story 2.2 (requires manifest-to-shell generation)

---

### Story 2.4: Export Profile to Archive

As a developer,
I want to export my profile to a portable .zprof archive,
So that I can share it with teammates or use it on other machines.

**Acceptance Criteria:**

1. `zprof export <profile-name>` creates `.zprof` archive file (tar.gz)
2. Archive contains `profile.toml` manifest and any custom configuration files (aliases.zsh, env.zsh)
3. Archive includes metadata (export date, zprof version, framework version)
4. Export excludes:
   - Cache files
   - Installed framework binaries (manifest describes installation)
   - Generated `.zshrc` and `.zshenv` (regenerated on import)
   - Backup files (`.zshrc.pre-zprof`)
5. Archive saved to current directory with filename `<profile-name>.zprof`
6. Success message displays archive path and size

**Prerequisites:** Story 2.1 (requires manifest system)

---

### Story 2.5: Import Profile from Local Archive

As a developer,
I want to import a profile from a local .zprof archive,
So that I can use shared profiles on my machine.

**Acceptance Criteria:**

1. `zprof import <file.zprof>` extracts archive and validates contents
2. Validates `profile.toml` manifest within archive
3. Checks for name conflicts and prompts for resolution (rename/overwrite/cancel)
4. Installs specified framework and plugins per manifest to profile directory (profile-scoped)
5. Restores custom config files (`aliases.zsh`, `env.zsh`) to profile directory
6. Generates `.zshrc` and `.zshenv` from manifest with proper sourcing order
7. Creates new profile in `~/.zsh-profiles/profiles/`
8. Success message confirms import and lists profile details
9. Handles corrupted archives gracefully with clear error messages

**Prerequisites:** Story 2.4 (complements export functionality)

---

### Story 2.6: Import Profile from GitHub Repository

As a developer,
I want to import profiles directly from GitHub repositories,
So that I can easily adopt shared team configurations or community profiles.

**Acceptance Criteria:**

1. `zprof import github:<user>/<repo>` clones or downloads repository
2. Searches repo root for `profile.toml` manifest
3. Validates manifest and prompts for name conflicts
4. Installs framework and plugins per manifest to profile directory (profile-scoped)
5. Creates new profile from GitHub source
6. Supports both public and private repos (uses git credentials)
7. Success message includes source repo URL for reference
8. Handles network errors and missing manifests gracefully

**Prerequisites:** Story 2.5 (builds on local import)

---

## Story Guidelines Reference

**Story Format:**

```
**Story [EPIC.N]: [Story Title]**

As a [user type],
I want [goal/desire],
So that [benefit/value].

**Acceptance Criteria:**
1. [Specific testable criterion]
2. [Another specific criterion]
3. [etc.]

**Prerequisites:** [Dependencies on previous stories, if any]
```

**Story Requirements:**

- **Vertical slices** - Complete, testable functionality delivery
- **Sequential ordering** - Logical progression within epic
- **No forward dependencies** - Only depend on previous work
- **AI-agent sized** - Completable in 2-4 hour focused session
- **Value-focused** - Integrate technical enablers into value-delivering stories

---

**For implementation:** Use the `create-story` workflow to generate individual story implementation plans from this epic breakdown.
