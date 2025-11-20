# Framework Installation Implementation Plan

**Date:** November 3, 2025  
**Purpose:** Detailed technical specifications for implementing actual framework installation

## Implementation Specifications by Framework

### 1. Oh-My-Zsh (PRIORITY 1 - Start Here)

**Repository**: `https://github.com/ohmyzsh/ohmyzsh.git`  
**Installation Directory**: `~/.oh-my-zsh`  
**Complexity**: Medium  

#### Implementation Steps:
1. **Git Clone**:
   ```bash
   git clone https://github.com/ohmyzsh/ohmyzsh.git ~/.oh-my-zsh
   ```

2. **Create Basic .zshrc Template**:
   ```bash
   # Path to your oh-my-zsh installation.
   export ZSH="$HOME/.oh-my-zsh"
   
   # Set name of the theme to load
   ZSH_THEME="{theme_name}"
   
   # Plugins to load
   plugins=({plugin_list})
   
   source $ZSH/oh-my-zsh.sh
   ```

3. **Directory Structure Verification**:
   - `~/.oh-my-zsh/plugins/` (built-in plugins)
   - `~/.oh-my-zsh/themes/` (built-in themes)  
   - `~/.oh-my-zsh/custom/plugins/` (custom plugins)
   - `~/.oh-my-zsh/custom/themes/` (custom themes)

#### Plugin Installation:
- **Built-in Plugins**: No installation needed, just enable in .zshrc
- **Custom Plugins**: Clone to `~/.oh-my-zsh/custom/plugins/<plugin-name>`
- **Popular Custom Plugins**:
  - `zsh-syntax-highlighting`: `https://github.com/zsh-users/zsh-syntax-highlighting.git`
  - `zsh-autosuggestions`: `https://github.com/zsh-users/zsh-autosuggestions.git`

### 2. Zap (PRIORITY 2 - Simple Implementation)

**Repository**: `https://github.com/zap-zsh/zap.git`  
**Installation Directory**: `~/.local/share/zap`  
**Complexity**: Low  

#### Implementation Steps:
1. **Git Clone**:
   ```bash
   git clone https://github.com/zap-zsh/zap.git ~/.local/share/zap
   ```

2. **Create Basic .zshrc Template**:
   ```bash
   # Initialize Zap
   [ -f ~/.local/share/zap/zap.zsh ] && source ~/.local/share/zap/zap.zsh
   
   # Install plugins
   {plugin_installations}
   
   # Load theme
   {theme_configuration}
   ```

3. **Directory Structure**:
   - `~/.local/share/zap/` (main installation)
   - `~/.local/share/zap/plugins/` (plugins directory)

#### Plugin Installation:
- **Plugin Format**: `plug "author/plugin-name"`
- **Zap handles git cloning automatically**
- **No manual plugin installation needed**

### 3. Zimfw (PRIORITY 3 - Medium Complexity)

**Repository**: `https://github.com/zimfw/zimfw.git`  
**Installation Directory**: `~/.zim`  
**Complexity**: Medium  

#### Implementation Steps:
1. **Download zimfw**:
   ```bash
   curl -fsSL https://raw.githubusercontent.com/zimfw/install/master/install.zsh | zsh
   ```
   OR
   ```bash
   git clone https://github.com/zimfw/zimfw.git ~/.zim
   ```

2. **Create .zimrc Configuration**:
   ```bash
   # Modules to load
   {module_list}
   ```

3. **Bootstrap Installation**:
   ```bash
   ~/.zim/zimfw.zsh install
   ```

4. **Create .zshrc Template**:
   ```bash
   # Initialize zimfw
   if [[ -s ${ZDOTDIR:-${HOME}}/.zim/init.zsh ]]; then
     source ${ZDOTDIR:-${HOME}}/.zim/init.zsh
   fi
   ```

#### Plugin Installation:
- **Module-based**: Add modules to `.zimrc`
- **Auto-installation**: Run `zimfw install` after configuration changes
- **No manual git cloning needed**

### 4. Prezto (PRIORITY 4 - High Complexity)

**Repository**: `https://github.com/sorin-ionescu/prezto.git`  
**Installation Directory**: `~/.zprezto`  
**Complexity**: High (complex symlink setup)  

#### Implementation Steps:
1. **Git Clone**:
   ```bash
   git clone --recursive https://github.com/sorin-ionescu/prezto.git ~/.zprezto
   ```

2. **Create Symlinks for Runcoms**:
   ```bash
   for rcfile in ~/.zprezto/runcoms/z*; do
     ln -s "$rcfile" "${ZDOTDIR:-$HOME}/.${rcfile:t}"
   done
   ```

3. **Configure .zpreztorc**:
   ```bash
   # Load modules
   zstyle ':prezto:load' pmodule \
     {module_list}
   
   # Set theme
   zstyle ':prezto:module:prompt' theme '{theme_name}'
   ```

#### Plugin Installation:
- **Module-based**: Enable modules in `.zpreztorc`
- **Mix of built-in and external**: Some modules included, others need manual installation
- **Complex dependency management**

### 5. Zinit (PRIORITY 5 - High Complexity)

**Repository**: `https://github.com/zdharma-continuum/zinit.git`  
**Installation Directory**: `~/.local/share/zinit/zinit.git`  
**Complexity**: High (advanced plugin manager)  

#### Implementation Steps:
1. **Git Clone**:
   ```bash
   git clone https://github.com/zdharma-continuum/zinit.git ~/.local/share/zinit/zinit.git
   ```

2. **Create .zshrc Template**:
   ```bash
   # Initialize zinit
   ZINIT_HOME="${XDG_DATA_HOME:-${HOME}/.local/share}/zinit/zinit.git"
   [ ! -d $ZINIT_HOME ] && mkdir -p "$(dirname $ZINIT_HOME)"
   [ ! -d $ZINIT_HOME/.git ] && git clone https://github.com/zdharma-continuum/zinit.git "$ZINIT_HOME"
   source "${ZINIT_HOME}/zinit.zsh"
   
   # Load plugins
   {plugin_installations}
   
   # Load theme
   {theme_configuration}
   ```

#### Plugin Installation:
- **Dynamic loading**: Plugins loaded on-demand
- **Advanced syntax**: `zinit load "author/plugin"`
- **Auto-installation**: Zinit handles all git operations
- **No manual plugin management needed**

## Git Operations Infrastructure

### Required Dependencies

Add to `Cargo.toml`:
```toml
[dependencies]
git2 = "0.18"
tokio = { version = "1.0", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
```

### Core Git Operations Module

Create `src/git.rs`:
```rust
use anyhow::{Context, Result};
use git2::{Repository, RemoteCallbacks, Progress, FetchOptions};
use std::path::Path;

pub fn clone_repository(url: &str, destination: &Path) -> Result<()> {
    // Implementation with progress callbacks
}

pub fn is_git_repository(path: &Path) -> bool {
    // Check if directory is a git repository
}

pub fn get_repository_version(path: &Path) -> Result<Option<String>> {
    // Get git tag or commit hash for version detection
}
```

### Error Handling Strategy

```rust
#[derive(Debug, thiserror::Error)]
pub enum InstallationError {
    #[error("Network error: {0}")]
    Network(#[from] git2::Error),
    
    #[error("Permission denied: {0}")]
    Permission(String),
    
    #[error("Framework already installed at {path}")]
    AlreadyInstalled { path: String },
    
    #[error("Invalid repository URL: {url}")]
    InvalidUrl { url: String },
}
```

## Implementation Phases

### Phase 1: Foundation (Week 1)
1. **Git Infrastructure** (8-12 hours)
   - Add git2 dependency
   - Implement basic clone operations
   - Add network error handling
   - Create progress callbacks

2. **Oh-My-Zsh Implementation** (8-12 hours)
   - Replace `unimplemented!()` in `oh_my_zsh.rs`
   - Implement git clone to `~/.oh-my-zsh`
   - Create .zshrc template generation
   - Add basic plugin support

### Phase 2: Simple Frameworks (Week 2)
3. **Zap Implementation** (4-6 hours)
   - Replace `unimplemented!()` in `zap.rs`
   - Implement git clone to `~/.local/share/zap`
   - Create .zshrc template with zap initialization

4. **Testing Infrastructure** (6-8 hours)
   - Update tests to verify real installations
   - Add integration tests with temporary directories
   - Mock git operations for unit tests

### Phase 3: Advanced Frameworks (Week 3)
5. **Zimfw Implementation** (8-12 hours)
   - Download and bootstrap zimfw
   - Create .zimrc configuration
   - Handle module installation

6. **Plugin System Enhancement** (8-12 hours)
   - Implement custom plugin installation
   - Add plugin dependency resolution
   - Handle different plugin installation methods per framework

### Phase 4: Complex Frameworks (Week 4)
7. **Prezto Implementation** (12-16 hours)
   - Recursive git clone
   - Complex symlink setup
   - Module configuration management

8. **Zinit Implementation** (12-16 hours)
   - Advanced installation setup
   - Dynamic plugin loading configuration
   - Complex .zshrc generation

## Success Criteria

### Phase 1 Success
- ✅ User can run `zprof create work` and select Oh-My-Zsh
- ✅ Actual oh-my-zsh gets downloaded from GitHub
- ✅ Profile contains working .zshrc with oh-my-zsh
- ✅ `zprof use work` loads functional oh-my-zsh environment

### Full Implementation Success
- ✅ All 5 frameworks can be installed from wizard
- ✅ Custom plugins can be installed for each framework
- ✅ Framework version detection works
- ✅ Error handling covers network failures
- ✅ Progress indicators show real installation progress
- ✅ Tests verify actual installations work

## Risk Mitigation

### Network Failures
- **Detection**: Check connectivity before attempting downloads
- **Retry Logic**: Exponential backoff for temporary failures
- **Offline Mode**: Graceful degradation when network unavailable
- **User Messaging**: Clear error messages with suggested fixes

### Installation Conflicts
- **Detection**: Check for existing framework installations
- **Resolution**: Prompt user for backup/overwrite/skip
- **Backup**: Create backups before replacing existing installations
- **Rollback**: Ability to restore previous state on failure

### Framework Changes
- **Version Pinning**: Clone specific stable versions/tags
- **Validation**: Verify cloned repositories have expected structure
- **Fallback**: Graceful handling of repository structure changes
- **Documentation**: Keep installation procedures up to date

This plan provides a clear roadmap from the current "placeholder" implementation to fully functional framework installation.