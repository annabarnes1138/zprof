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

### Story 1.1: Initialize zprof Directory Structure

As a developer,
I want to initialize zprof's directory structure in my home directory,
So that I have a clean foundation for managing multiple zsh profiles.

**Acceptance Criteria:**

1. `zprof init` command creates `~/.zsh-profiles/` directory structure with `profiles/`, `shared/`, and `cache/` subdirectories
2. Shared command history file `.zsh_history` is created in `shared/` directory
3. Global configuration file `config.yml` is created with sensible defaults
4. Command outputs success message confirming initialization
5. Running `zprof init` when already initialized warns user but does not corrupt existing data

**Prerequisites:** None (first story)

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

### Story 1.5: Profile Creation with Import Current Setup

As a developer with an existing zsh configuration,
I want to import my current setup as a zprof profile,
So that I can preserve my working configuration before experimenting.

**Acceptance Criteria:**

1. When framework detected, `zprof create <name>` prompts "Import current setup? (y/n)"
2. On "y", system copies current framework files to new profile directory
3. Profile includes detected framework, plugins, theme, and custom configurations
4. YAML manifest is generated from imported configuration
5. Original dotfiles remain untouched and functional
6. Success message confirms profile creation with imported details

**Prerequisites:** Story 1.4 (requires framework detection)

---

### Story 1.6: TUI Wizard Framework Selection

As a developer,
I want an interactive menu to select a zsh framework,
So that I can easily choose the framework for my new profile.

**Acceptance Criteria:**

1. TUI displays list of supported frameworks (oh-my-zsh, zimfw, prezto, zinit, zap)
2. Each framework shows brief description and key characteristics
3. Keyboard navigation with arrow keys and enter to select
4. Selected framework is highlighted visually
5. TUI is responsive and works in 80x24 terminal minimum
6. Supports both light and dark terminal themes

**Prerequisites:** Story 1.1 (requires initialized directory structure)

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
3. Final confirmation screen shows all selections (framework, plugins, theme)
4. On confirmation, system installs selected framework and plugins
5. Generates profile.yml manifest with all selections
6. Creates functional .zshrc and .zshenv in profile directory
7. Installation progress is displayed with clear status messages
8. Success message confirms profile is ready to use

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

## Epic 2: YAML Manifests & Export/Import

**Expanded Goal:**

This epic transforms profiles from purely TUI-generated configurations into declarative, version-controllable YAML manifests. It enables power users to manually edit profile configurations and establishes the foundation for a shareable profile ecosystem through export/import capabilities. Users will be able to package profiles into portable archives and share them via local files or GitHub repositories, enabling team standardization and community contribution.

---

### Story 2.1: Parse and Validate YAML Manifests

As a developer,
I want zprof to parse and validate my profile YAML manifests,
So that I can ensure my profile configuration is correct before applying it.

**Acceptance Criteria:**

1. System reads profile.yml files and validates schema (name, framework, plugins, theme, environment variables)
2. Validation checks for required fields and correct data types
3. Clear error messages identify specific validation failures with line numbers
4. Successfully validated manifests are marked as ready for use
5. Invalid manifests prevent profile activation with helpful guidance

**Prerequisites:** Story 1.5 or 1.8 (requires profiles with generated manifests)

---

### Story 2.2: Generate Shell Configuration from YAML

As a developer,
I want zprof to automatically generate .zshrc and .zshenv from my YAML manifest,
So that my declarative configuration is translated into functional shell files.

**Acceptance Criteria:**

1. System generates .zshrc with framework initialization, plugin loading, and theme activation
2. System generates .zshenv with environment variables from manifest
3. Generated files include header comments indicating they're auto-generated from manifest
4. Re-generation from manifest overwrites previous generated files (manifest is source of truth)
5. Generated configuration is syntactically valid zsh code
6. Process completes in under 1 second for typical profiles

**Prerequisites:** Story 2.1 (requires YAML parsing)

---

### Story 2.3: Manual YAML Editing with Live Validation

As a power user,
I want to manually edit my profile YAML and receive validation feedback,
So that I can quickly customize profiles without using the TUI wizard.

**Acceptance Criteria:**

1. `zprof edit <profile-name>` opens profile.yml in user's $EDITOR
2. After saving, system validates YAML and reports any errors
3. If valid, regenerates .zshrc and .zshenv from updated manifest
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

1. `zprof export <profile-name>` creates .zprof archive file (tar.gz)
2. Archive contains profile.yml manifest and any custom configuration files
3. Archive includes metadata (export date, zprof version, framework version)
4. Export excludes cache files and installed framework binaries (manifest describes installation)
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
2. Validates profile.yml manifest within archive
3. Checks for name conflicts and prompts for resolution (rename/overwrite/cancel)
4. Installs specified framework and plugins per manifest
5. Creates new profile in `~/.zsh-profiles/profiles/`
6. Success message confirms import and lists profile details
7. Handles corrupted archives gracefully with clear error messages

**Prerequisites:** Story 2.4 (complements export functionality)

---

### Story 2.6: Import Profile from GitHub Repository

As a developer,
I want to import profiles directly from GitHub repositories,
So that I can easily adopt shared team configurations or community profiles.

**Acceptance Criteria:**

1. `zprof import github:<user>/<repo>` clones or downloads repository
2. Searches repo root for profile.yml manifest
3. Validates manifest and prompts for name conflicts
4. Installs framework and plugins per manifest
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
