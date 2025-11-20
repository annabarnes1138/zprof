# zprof v0.2.0 Planning

**Release Target**: TBD
**Status**: Planning in progress

## Overview

Version 0.2.0 focuses on making zprof more accessible while maintaining its power-user capabilities through smart UX and comprehensive installation workflows.

## Key Features

### 1. Smart TUI (Prompt Mode Branching)
Enable users to choose between standalone prompt engines (Starship, Powerlevel10k) and framework-built-in themes with intelligent UX that doesn't overwhelm beginners.

### 2. Quick Setup Presets
Provide curated preset configurations (Minimal, Performance, Fancy, Developer) for one-click profile creation while still offering advanced customization.

### 3. Complete Uninstall System
Allow users to safely try zprof with confidence they can completely remove it and restore either their original configuration or any profile as their root config.

### 4. Nerd Font Auto-Installation
Automatically download and install Nerd Fonts when users select prompts that require them, with clear terminal configuration instructions.

## Epics

### Core Features

1. **[Epic 1: Smart TUI (Prompt Mode Branching)](epic-1-smart-tui.md)** - P0
   - Intelligent prompt selection flow
   - Differentiate prompt engines from framework themes
   - Progressive disclosure UX pattern
   - 7 stories, ~3 days

2. **[Epic 2: Quick Setup Presets](epic-2-presets.md)** - P0
   - Curated preset configurations (Minimal, Performance, Fancy, Developer)
   - Quick vs Custom setup paths
   - One-click profile creation
   - 8 stories, ~3 days

3. **[Epic 3: Complete Uninstall System](epic-3-uninstall.md)** - P0
   - Comprehensive uninstall with restoration options
   - Restore original or promote profile to root
   - Safety backups before cleanup
   - 8 stories, ~4 days

4. **[Epic 4: Nerd Font Auto-Installation](epic-4-nerd-fonts.md)** - P1
   - Automatic font detection and installation
   - Terminal configuration instructions
   - Font management commands
   - 10 stories, ~3 days

### Technical Improvements

5. **[Epic 5: Remove Deprecated Rollback](epic-5-remove-rollback.md)** - P2
   - Deprecate rollback command in favor of uninstall
   - Migration guide and documentation
   - Graceful transition with warnings
   - 7 stories, ~1 day

6. **[Epic 6: Init Cleanup and Enhancement](epic-6-init-cleanup.md)** - P0
   - Comprehensive pre-zprof backup during init
   - Move root configs to backup location
   - Clean HOME directory state
   - 7 stories, ~2 days

## Stories

Story files will be created in `stories/` as epics are finalized.

## Design Decisions

Key architectural and UX decisions will be documented here as they're made.

## Timeline

Planning phase: Current
Implementation: TBD
Release: TBD

## Success Criteria

- [ ] New users can create a working profile in < 2 minutes
- [ ] Users can uninstall zprof cleanly with confidence
- [ ] Prompt selection doesn't confuse beginners
- [ ] Power users still have full control
- [ ] Nerd Fonts automatically installed when needed
- [ ] Init process creates comprehensive backups
- [ ] Clean migration path from deprecated rollback

## Release Summary

**Total Effort:** ~16 days
**Total Stories:** 47 stories across 6 epics
**Priority Breakdown:**
- P0 (Must Have): 4 epics (Smart TUI, Presets, Uninstall, Init Cleanup)
- P1 (Should Have): 1 epic (Nerd Fonts)
- P2 (Nice to Have): 1 epic (Rollback Deprecation)

**Story Count by Epic:**
1. Epic 1 (Smart TUI): 7 stories
2. Epic 2 (Presets): 8 stories
3. Epic 3 (Uninstall): 8 stories
4. Epic 4 (Nerd Fonts): 10 stories
5. Epic 5 (Rollback): 7 stories
6. Epic 6 (Init Cleanup): 7 stories
