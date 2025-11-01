# Story 1.4: Framework Detection for Smart Profile Creation

Status: ready-for-dev

## Story

As a developer,
I want zprof to detect my existing zsh framework configuration,
so that I can preserve my current setup when creating my first profile.

## Acceptance Criteria

1. System scans for oh-my-zsh, zimfw, prezto, zinit, and zap installations
2. Detection identifies framework type, installed plugins, and active theme
3. If framework detected, system captures configuration details for import
4. Detection completes in under 2 seconds
5. Gracefully handles multiple frameworks or corrupted installations

## Tasks / Subtasks

- [ ] Implement framework detection infrastructure (AC: #1)
  - [ ] Create `frameworks/mod.rs` with Framework trait definition per architecture Pattern 6
  - [ ] Create `frameworks/detector.rs` for main detection orchestration
  - [ ] Define FrameworkInfo struct to hold detection results (type, plugins, theme, config_path)
- [ ] Implement oh-my-zsh detection (AC: #1, #2, #3)
  - [ ] Create `frameworks/oh_my_zsh.rs` implementing Framework trait
  - [ ] Check for `~/.oh-my-zsh/` directory existence
  - [ ] Parse `~/.zshrc` to extract plugins array and theme variable (ZSH_THEME)
  - [ ] Return FrameworkInfo with detected configuration
- [ ] Implement zimfw detection (AC: #1, #2, #3)
  - [ ] Create `frameworks/zimfw.rs` implementing Framework trait
  - [ ] Check for `~/.zim/` or `~/.zimfw/` directory existence
  - [ ] Parse `~/.zimrc` to extract zmodule declarations
  - [ ] Return FrameworkInfo with detected configuration
- [ ] Implement prezto detection (AC: #1, #2, #3)
  - [ ] Create `frameworks/prezto.rs` implementing Framework trait
  - [ ] Check for `~/.zprezto/` directory existence
  - [ ] Parse `~/.zpreztorc` to extract loaded modules (zstyle ':prezto:load' pmodule)
  - [ ] Return FrameworkInfo with detected configuration
- [ ] Implement zinit detection (AC: #1, #2, #3)
  - [ ] Create `frameworks/zinit.rs` implementing Framework trait
  - [ ] Check for `~/.zinit/` or `~/.local/share/zinit/` directory existence
  - [ ] Parse `~/.zshrc` for zinit plugin declarations (zinit load, zinit light)
  - [ ] Return FrameworkInfo with detected configuration
- [ ] Implement zap detection (AC: #1, #2, #3)
  - [ ] Create `frameworks/zap.rs` implementing Framework trait
  - [ ] Check for `~/.local/share/zap/` directory existence
  - [ ] Parse `~/.zshrc` for zap plugin declarations (plug)
  - [ ] Return FrameworkInfo with detected configuration
- [ ] Implement detection orchestration (AC: #1, #4, #5)
  - [ ] In `frameworks/detector.rs`, implement detect_existing_framework() function
  - [ ] Scan for all five frameworks in parallel for speed
  - [ ] If multiple frameworks detected, return the one with most recent .zshrc modification
  - [ ] If no framework detected, return None
  - [ ] Complete detection in under 2 seconds (AC: #4)
- [ ] Handle edge cases gracefully (AC: #5)
  - [ ] Handle corrupted .zshrc files (invalid syntax) without crashing
  - [ ] Handle missing plugin/theme declarations in config files
  - [ ] Handle symlinked framework directories
  - [ ] Return partial FrameworkInfo if some details missing
- [ ] Add user-friendly error handling (AC: All)
  - [ ] Use anyhow::Context for all file operations following Pattern 2
  - [ ] Log warnings for corrupted configs but don't fail
  - [ ] Provide helpful debug logging for troubleshooting detection issues
- [ ] Write unit and integration tests (AC: All)
  - [ ] Test each framework detection in isolation with mock file systems
  - [ ] Test oh-my-zsh detection with sample .zshrc
  - [ ] Test zimfw detection with sample .zimrc
  - [ ] Test prezto detection with sample .zpreztorc
  - [ ] Test zinit detection with sample .zshrc
  - [ ] Test zap detection with sample .zshrc
  - [ ] Test multiple frameworks scenario (most recent wins)
  - [ ] Test no framework detected scenario
  - [ ] Test corrupted config file handling
  - [ ] Test performance meets < 2 second requirement

## Dev Notes

### Architecture Constraints

**Module Structure:**
- Primary: `frameworks/detector.rs`, `frameworks/mod.rs`
- Secondary: `frameworks/oh_my_zsh.rs`, `frameworks/zimfw.rs`, `frameworks/prezto.rs`, `frameworks/zinit.rs`, `frameworks/zap.rs`
- All modules must follow patterns defined in architecture.md Pattern 6 (Framework Trait)
- Error handling via anyhow::Result with context (Pattern 2)
- No file modifications (read-only detection)

**Framework Trait (from architecture.md Pattern 6):**
```rust
pub trait Framework {
    fn name(&self) -> &str;
    fn detect() -> Option<FrameworkInfo>;
    fn install(profile_path: &Path) -> Result<()>;  // Not used in this story
    fn get_plugins() -> Vec<Plugin>;                 // Not used in this story
    fn get_themes() -> Vec<Theme>;                   // Not used in this story
}
```

**Data Structures to Define:**
```rust
// In frameworks/mod.rs
#[derive(Debug, Clone)]
pub struct FrameworkInfo {
    pub framework_type: FrameworkType,
    pub plugins: Vec<String>,
    pub theme: String,
    pub config_path: PathBuf,
    pub install_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FrameworkType {
    OhMyZsh,
    Zimfw,
    Prezto,
    Zinit,
    Zap,
}
```

**Detection Strategies by Framework:**

1. **oh-my-zsh:**
   - Directory: `~/.oh-my-zsh/`
   - Config: `~/.zshrc` with `export ZSH="$HOME/.oh-my-zsh"` and `source $ZSH/oh-my-zsh.sh`
   - Plugins: `plugins=(git docker kubectl)` array in .zshrc
   - Theme: `ZSH_THEME="robbyrussell"` variable in .zshrc

2. **zimfw:**
   - Directory: `~/.zim/` or `~/.zimfw/`
   - Config: `~/.zimrc` with zmodule declarations
   - Plugins: `zmodule ohmyzsh/ohmyzsh --root plugins/git` lines
   - Theme: `zmodule romkatv/powerlevel10k` or similar

3. **prezto:**
   - Directory: `~/.zprezto/`
   - Config: `~/.zpreztorc`
   - Plugins: `zstyle ':prezto:load' pmodule 'git' 'docker'` lines
   - Theme: `zstyle ':prezto:module:prompt' theme 'powerlevel10k'`

4. **zinit:**
   - Directory: `~/.zinit/` or `~/.local/share/zinit/`
   - Config: `~/.zshrc` with zinit declarations
   - Plugins: `zinit load zdharma-continuum/fast-syntax-highlighting` lines
   - Theme: `zinit ice lucid; zinit light romkatv/powerlevel10k`

5. **zap:**
   - Directory: `~/.local/share/zap/`
   - Config: `~/.zshrc` with zap source and plug declarations
   - Plugins: `plug "zsh-users/zsh-autosuggestions"` lines
   - Theme: Usually plug command for theme

**Error Handling:**
- Use `anyhow::Context` for all file operations
- Detection failures should warn but not error (return None instead)
- Log warnings with `log::warn!` for troubleshooting
- Never crash on malformed config files

**Testing Strategy:**
- Unit tests in each framework module for detection logic
- Integration tests in `tests/framework_detection_test.rs`
- Create temporary test .zshrc files with known configurations
- Test edge cases: empty configs, malformed syntax, missing files

**Performance Target (AC: #4):**
- Expected execution time: < 2 seconds for all five framework checks
- Use parallel scanning if possible to improve speed
- Cache file reads to avoid redundant I/O

### Project Structure Notes

**File Locations:**
- `src/frameworks/mod.rs` - Framework trait and common types
- `src/frameworks/detector.rs` - Main detection orchestration
- `src/frameworks/oh_my_zsh.rs` - oh-my-zsh specific detection
- `src/frameworks/zimfw.rs` - zimfw specific detection
- `src/frameworks/prezto.rs` - prezto specific detection
- `src/frameworks/zinit.rs` - zinit specific detection
- `src/frameworks/zap.rs` - zap specific detection
- `tests/framework_detection_test.rs` - Integration tests

**Dependencies (may need to add for parsing):**
```toml
[dependencies]
regex = "1.10"  # For parsing .zshrc plugin/theme declarations
```

**Parsing Strategy:**
- Use simple regex patterns to extract plugin arrays and theme variables
- Don't try to execute zsh code - just parse declarative config
- Be lenient with whitespace and formatting variations

### Learnings from Previous Story

Previous story (1.3) not yet implemented - no predecessor context to incorporate.

### References

- [Source: docs/epics.md#Story-1.4]
- [Source: docs/PRD.md#FR006-detect-existing-framework]
- [Source: docs/architecture.md#Pattern-6-Framework-Trait]
- [Source: docs/architecture.md#Pattern-2-Error-Handling]
- [Source: docs/architecture.md#Epic-1-Story-1.4-Mapping]
- [Source: docs/architecture.md#Module-Structure-frameworks]

## Dev Agent Record

### Context Reference

- docs/stories/1-4-framework-detection-for-smart-profile-creation.context.xml

### Agent Model Used

<!-- Will be populated by dev agent during implementation -->

### Debug Log References

<!-- Will be populated by dev agent during implementation -->

### Completion Notes List

<!-- Will be populated by dev agent during implementation -->

### File List

<!-- Will be populated by dev agent during implementation -->

## Change Log

- 2025-10-31: Story drafted by SM agent (Bob)
