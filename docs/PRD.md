# zprof Product Requirements Document (PRD)

**Author:** Anna
**Date:** 2025-10-31
**Project Level:** 2
**Target Scale:** MVP - Level 2 (focused, 1-2 epics, 5-15 stories)

---

## Goals and Background Context

### Goals

- **Enable instant, risk-free zsh configuration experimentation** - Developers can safely try new frameworks and plugins without breaking their working setup
- **Eliminate context-switching friction** - Reduce 5-15 minute configuration changes to sub-second profile switches
- **Enable shareable shell environments** - Create foundation for distributing standardized configurations across teams and the community

### Background Context

Developers working across multiple projects, clients, or experimental setups waste 5-15 minutes per context switch manually reconfiguring their zsh shell environments. Each context may require different frameworks (oh-my-zsh vs zimfw vs prezto), plugin sets, and themes. Current solutions force developers to choose between a single bloated .zshrc with complex conditionals, error-prone manual file swapping, heavyweight container solutions, or abandoning customization entirely.

zprof brings "virtual environment" thinking to shell management - enabling developers to create, switch between, and share multiple isolated zsh configurations as easily as switching Node versions with nvm. The MVP focuses on core profile switching, an interactive creation wizard, YAML manifests, and export/import capabilities to establish the foundation for a shareable ecosystem.

---

## Requirements

### Functional Requirements

**Profile Management**
- **FR001:** System shall initialize zsh-profiles directory structure in user's home directory (`~/.zsh-profiles/`) with profiles, shared, and cache subdirectories
- **FR002:** System shall list all available profiles with visual indicator showing currently active profile
- **FR003:** System shall switch active profile by updating ZDOTDIR environment variable and executing new shell instance
- **FR004:** System shall display currently active profile name and metadata
- **FR005:** System shall delete profiles with confirmation prompt and cleanup of associated files

**Profile Creation**
- **FR006:** System shall detect existing zsh framework configuration and prompt user to "Import current setup" or "Start fresh" when creating first profile
- **FR007:** System shall provide interactive TUI wizard for profile creation with framework selection (oh-my-zsh, zimfw, prezto, zinit, zap)
- **FR008:** System shall allow users to browse and select plugins during profile creation with popular recommendations
- **FR009:** System shall allow users to select theme during profile creation
- **FR010:** System shall generate YAML manifest (profile.yml) from wizard selections
- **FR011:** System shall automatically install selected framework and plugins during profile creation

**YAML Manifest System**
- **FR012:** System shall parse and validate YAML profile manifests containing name, framework, plugins, theme, and environment variables
- **FR013:** System shall generate functional .zshrc and .zshenv files from YAML manifest specifications
- **FR014:** System shall support manual YAML editing with validation and error reporting

**Export/Import**
- **FR015:** System shall export profiles to portable .zprof archive files containing manifest and configuration files
- **FR016:** System shall import profiles from local .zprof archive files
- **FR017:** System shall import profiles from GitHub repositories using `github:<user>/<repo>` syntax

**Cross-Profile Features**
- **FR018:** System shall maintain shared command history across all profiles stored in `~/.zsh-profiles/shared/.zsh_history`

### Non-Functional Requirements

- **NFR001:** Profile switching shall complete in under 500ms (excluding framework installation time)
- **NFR002:** System shall not modify or corrupt user's existing dotfiles; all operations must be non-destructive with automatic backups
- **NFR003:** TUI shall be responsive and navigable via keyboard on machines with 2GB RAM and standard terminal emulators (iTerm2, Terminal.app, Alacritty)

---

## User Journeys

### User Journey: Context-Switching Developer Experimenting with New Framework

**Persona:** Sarah, mid-level backend developer working on multiple client projects

**Scenario:** Sarah wants to try zimfw for her side project but can't risk breaking her oh-my-zsh work setup

**Journey:**

1. **Discovery & Installation**
   - Sarah hears about zprof from a colleague
   - Installs zprof via Homebrew: `brew install zprof`
   - Runs `zprof init` to set up the tool

2. **Creating First Profile (Preserving Current Setup)**
   - Runs `zprof create work`
   - zprof detects existing oh-my-zsh configuration
   - Prompt appears: "Existing oh-my-zsh detected. Import current setup? (y/n)"
   - Sarah chooses "y"
   - System captures her current oh-my-zsh config, plugins, and theme into "work" profile
   - Her existing setup continues working unchanged

3. **Experimenting Safely**
   - Runs `zprof create experimental`
   - No existing config to import, so TUI wizard launches
   - Framework selection menu appears - Sarah chooses zimfw
   - Browses recommended plugins, selects a few
   - Picks a theme to try
   - Wizard installs zimfw and generates "experimental" profile

4. **Switching Between Profiles**
   - Runs `zprof use experimental` - new shell launches with zimfw in < 1 second
   - Experiments with zimfw features for 30 minutes
   - When done, runs `zprof use work` - back to familiar oh-my-zsh setup instantly
   - No configuration files were modified; no risk of breaking anything

5. **Outcome**
   - Sarah discovers she prefers zimfw's performance
   - Over the next week, she refines her "experimental" profile
   - Eventually makes "experimental" her daily driver
   - Keeps "work" profile as backup, never worrying about lost productivity

**Key Touchpoints:**
- Installation: < 2 minutes (Homebrew)
- First profile creation (import): < 2 minutes
- Experiment profile creation (wizard): 5-10 minutes
- Profile switching: < 1 second
- Peace of mind: Priceless - can always return to working setup

---

## UX Design Principles

1. **Familiar Mental Models** - Leverage patterns developers already know from nvm, pyenv, Docker contexts, and git commands
2. **Safety First** - All destructive operations require confirmation; non-destructive by default with automatic backups
3. **Progressive Disclosure** - Simple commands for common tasks; advanced features (YAML editing) available but not required
4. **Immediate Feedback** - Clear status indicators, informative error messages, confirmation of successful operations

---

## User Interface Design Goals

**Platform & Interaction:**
- **Target Platform:** Terminal-based CLI with TUI (Text User Interface) for interactive wizards
- **Core Screens/Views:**
  - Command-line interface for all core operations (init, list, use, create, delete, export, import, current)
  - Interactive TUI wizard for profile creation (framework selection, plugin browser, theme picker)
  - Text output for list/status commands with visual indicators for active profile
- **Key Interaction Patterns:**
  - Keyboard-driven navigation (arrow keys, tab, enter)
  - Autocomplete for profile names (shell completion)
  - Consistent command structure: `zprof <verb> <noun>` pattern

**Design Constraints:**
- Must work in standard terminal emulators (iTerm2, Terminal.app, Alacritty, Kitty, WezTerm)
- No mouse required - fully keyboard navigable
- Readable in both light and dark terminal themes
- ASCII-safe fallbacks for systems without Unicode support
- Responsive on 80x24 minimum terminal size

---

## Epic List

**Epic 1: Core Profile Management & TUI Wizard**

Goal: Establish foundation infrastructure and enable users to create, switch, and manage multiple zsh profiles

Estimated Story Count: 8-10 stories

Delivers: Full profile lifecycle (init, create with smart detection, list, use, delete, current) + interactive TUI wizard for framework/plugin/theme selection

**Epic 2: YAML Manifests & Export/Import**

Goal: Enable declarative profile definitions and shareable profile ecosystem

Estimated Story Count: 5-7 stories

Delivers: YAML manifest generation and validation, manual YAML editing support, profile export to .zprof archives, import from local files and GitHub repos

**Total:** 2 epics, 13-17 stories

> **Note:** Detailed epic breakdown with full story specifications is available in [epics.md](./epics.md)

---

## Out of Scope

**Features Explicitly Excluded from MVP (Phase 2+):**

- **Multi-shell support** - bash, fish, nushell, PowerShell support deferred to Phase 2
- **Context-aware auto-switching** - Automatic profile switching based on directory, time, location, or git repo triggers deferred to Phase 3
- **Public profile registry/marketplace** - Community profile discovery and search features deferred to Phase 2
- **Nested profiles/inheritance** - Profile composition and override capabilities deferred to Phase 2
- **Terminal emulator integration** - Auto-theme switching for iTerm2, Alacritty, etc. deferred to Phase 2
- **Configuration-as-Code playbooks** - Ansible-style idempotent operations deferred to Phase 3
- **Background daemon** - Process monitoring and automated operations deferred to Phase 3
- **Profile diff/merge/branch operations** - Git-style profile comparison and merging deferred to Phase 2
- **Team/enterprise features** - Private registries, centralized management, SSO deferred to Phase 3

**Nice-to-Have Features (Defer if Time-Constrained):**

- Profile templates/starter kits
- Profile validation/health checks beyond basic YAML validation
- Usage analytics dashboard
- Plugin dependency resolution and conflict management

**Scope Boundaries:**

- MVP focuses exclusively on zsh; other shells are future work
- Import/export supports GitHub and local files only; no registry infrastructure
- Framework support limited to five major frameworks (oh-my-zsh, zimfw, prezto, zinit, zap)
- No GUI - terminal UI only
- No Windows native support (WSL only)
