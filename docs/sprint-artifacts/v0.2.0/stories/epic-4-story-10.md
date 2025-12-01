# Story 4.10: Update Documentation

Status: ready-for-dev

## Story

As a user,
I want documentation about Nerd Fonts,
so that I understand why they're needed and how to manage them.

## Acceptance Criteria

1. Create `docs/user-guide/nerd-fonts.md` - Comprehensive Nerd Fonts guide
2. Update `docs/user-guide/quick-start.md` - Mention font installation step
3. Update `docs/user-guide/commands.md` - Document `zprof font` commands
4. Update `docs/user-guide/troubleshooting.md` - Add font-related issues
5. Update `docs/user-guide/faq.md` - Add font FAQs
6. Include terminal configuration examples
7. Add troubleshooting for common font issues
8. Add screenshots/examples if applicable

## Dev Agent Context

### Story Requirements from Epic

This story creates and updates documentation to help users understand Nerd Fonts, why they're needed, how to install them, and how to troubleshoot issues. Documentation must be comprehensive, clear, and include all major terminals.

**Documentation Scope:**
1. **New Guide**: Complete Nerd Fonts explanation and reference
2. **Quick Start**: Brief mention of automatic font installation
3. **Commands Reference**: Full `zprof font` command documentation
4. **Troubleshooting**: Common font display issues and solutions
5. **FAQ**: Questions about fonts, requirements, customization

**Target Audience:**
- New users unfamiliar with Nerd Fonts
- Existing users troubleshooting font issues
- Advanced users wanting to customize fonts

### Architecture Compliance

**Documentation Structure:**
```
docs/
├── user-guide/
│   ├── nerd-fonts.md          (NEW - comprehensive guide)
│   ├── quick-start.md         (UPDATE - add font mention)
│   ├── commands.md            (UPDATE - add font commands)
│   ├── troubleshooting.md     (UPDATE - add font issues)
│   └── faq.md                 (UPDATE - add font FAQs)
```

**Content Guidelines:**
- Use clear, jargon-free language
- Provide concrete examples (not abstract explanations)
- Include command outputs (actual terminal output)
- Step-by-step instructions with screenshots if possible
- Link between related topics
- Test all commands and examples before documenting

### File Structure Requirements

**New File:** `docs/user-guide/nerd-fonts.md`

**Modified Files:**
- `docs/user-guide/quick-start.md`
- `docs/user-guide/commands.md`
- `docs/user-guide/troubleshooting.md`
- `docs/user-guide/faq.md`

### Implementation Guidance

**1. New File: `docs/user-guide/nerd-fonts.md`**

Complete comprehensive guide covering:

```markdown
# Nerd Fonts Guide

## What are Nerd Fonts?

Nerd Fonts are specially patched fonts that include thousands of icons and symbols used by modern prompt engines like Starship, Powerlevel10k, and Oh-My-Posh. Without a Nerd Font, these prompts will display broken characters (boxes □, question marks ?) instead of beautiful icons.

### Examples

**With Nerd Font:**
```
 ~/code/zprof  main  2 󰃵
```

**Without Nerd Font:**
```
□ ~/code/zprof □ main □ 2 □
```

## Why Does zprof Need Nerd Fonts?

Modern prompt engines (Starship, Powerlevel10k, Oh-My-Posh, Spaceship) use icons to display:
- Git status (, , )
- Programming language versions (, , , )
- Folder icons (, , )
- Status indicators (, , )

These icons are NOT standard Unicode characters. They're part of the Nerd Fonts extended character set.

## Automatic Installation

When you create a profile with a prompt engine that requires Nerd Fonts, zprof will:

1. Detect if you already have a Nerd Font installed
2. If not, show a font selection menu
3. Download your chosen font from nerdfonts.com
4. Install it to your system font directory
5. Show terminal configuration instructions

### Example Workflow

```bash
$ zprof create work
Select a framework: oh-my-zsh
Select a prompt engine: Starship

Nerd Font Required
------------------
Starship uses icons and symbols that require a Nerd Font to display correctly.
We can automatically download and install a font for you.

[Font selection menu appears]
> FiraCode Nerd Font (recommended)
  JetBrainsMono Nerd Font (recommended)
  ...

Downloading FiraCode Nerd Font...
✓ Downloaded 18.4 MB

Installing fonts to ~/Library/Fonts/...
✓ Copied 12 font files
✓ Updated font cache
✓ Installation complete

Configure iTerm2 to use this font:
1. Open iTerm2 → Preferences (Cmd+,)
2. Select Profiles tab → Select your profile
3. Go to Text tab
4. Change Font to "FiraCode Nerd Font Mono"
5. Restart your terminal
```

## Available Fonts

zprof offers 6 curated Nerd Fonts:

### FiraCode Nerd Font (Recommended)
- **Description**: Programming ligatures, clean and modern
- **Best for**: Starship, Oh-My-Posh, general programming
- **Preview**: ⚡ ⬢  →  ✓   λ ≡
- **Features**: Ligatures for `=>`, `!=`, `===`, etc.

### JetBrainsMono Nerd Font (Recommended)
- **Description**: Designed for developers, excellent readability
- **Best for**: Long coding sessions, small font sizes
- **Preview**: ⚡ ⬢  →  ✓   λ ≡

### Meslo Nerd Font (Recommended for Powerlevel10k)
- **Description**: Optimized for Powerlevel10k prompt
- **Best for**: Powerlevel10k users
- **Preview**: ⚡ ⬢  →  ✓   λ ≡

### Hack Nerd Font
- **Description**: Classic programming font
- **Best for**: Traditional look, high contrast
- **Preview**: ⚡ ⬢  →  ✓   λ ≡

### CascadiaCode Nerd Font
- **Description**: Microsoft's modern programming font
- **Best for**: Windows/cross-platform consistency
- **Preview**: ⚡ ⬢  →  ✓   λ ≡

### UbuntuMono Nerd Font
- **Description**: Clean, widely compatible
- **Best for**: Linux users, minimalist design
- **Preview**: ⚡ ⬢  →  ✓   λ ≡

## Manual Font Installation

If automatic installation doesn't work, you can install fonts manually:

1. Visit https://www.nerdfonts.com/font-downloads
2. Download your preferred font (click the Download button)
3. Extract the ZIP file
4. Install fonts to:
   - **macOS**: Drag `.ttf` files to `~/Library/Fonts/`
   - **Linux**: Copy `.ttf` files to `~/.local/share/fonts/`
5. Refresh font cache:
   - **Linux**: Run `fc-cache -fv`
   - **macOS**: No action needed
6. Restart your terminal

## Terminal Configuration

After installing a Nerd Font, configure your terminal to use it:

### iTerm2 (macOS)
1. Open iTerm2 → Preferences (Cmd+,)
2. Go to Profiles → Select your profile
3. Click the Text tab
4. Click "Change" next to Font
5. Search for your Nerd Font (e.g., "FiraCode Nerd Font Mono")
6. Select the Regular variant
7. Click OK and close Preferences
8. Restart iTerm2

### Terminal.app (macOS)
1. Open Terminal → Preferences (Cmd+,)
2. Go to Profiles → Select your profile
3. Click the Font tab
4. Click "Change"
5. Find your Nerd Font in the font picker
6. Select it and close the font picker
7. Set as Default if desired
8. Restart Terminal

### VS Code Integrated Terminal
1. Open Settings (Cmd+, or Ctrl+,)
2. Search for "terminal font"
3. Find "Terminal › Integrated: Font Family"
4. Enter the font name: `FiraCode Nerd Font Mono`
5. Close Settings
6. Reload VS Code (Cmd+R or Ctrl+R)

### Alacritty
1. Open `~/.config/alacritty/alacritty.yml` (or `.toml`)
2. Add or update:
   ```yaml
   font:
     normal:
       family: "FiraCode Nerd Font Mono"
   ```
3. Save the file
4. Restart Alacritty

### Kitty
1. Open `~/.config/kitty/kitty.conf`
2. Add or update:
   ```
   font_family FiraCode Nerd Font Mono
   ```
3. Save the file
4. Restart Kitty

### GNOME Terminal (Linux)
1. Open Terminal → Preferences
2. Select your profile
3. Uncheck "Use the system fixed width font"
4. Click the font button
5. Search for your Nerd Font
6. Select it and close
7. Restart GNOME Terminal

### Konsole (KDE)
1. Open Settings → Edit Current Profile
2. Go to Appearance tab
3. Click "Select Font"
4. Find your Nerd Font
5. Click OK
6. Restart Konsole

## Font Management Commands

### List Installed Fonts

```bash
$ zprof font list
```

Shows all installed Nerd Fonts and which profiles use them.

### Install New Font

```bash
$ zprof font install
```

Interactive font installation (same as during profile creation).

### Get Font Information

```bash
$ zprof font info FiraCode
```

Shows details about a specific font including terminal configuration.

See [Commands Reference](commands.md#font-commands) for full documentation.

## Troubleshooting

### Prompt shows boxes (□) or question marks (?)

**Cause**: Terminal is not using a Nerd Font.

**Solutions**:
1. Verify font is installed: `zprof font list`
2. If not installed: `zprof font install`
3. Configure terminal (see Terminal Configuration above)
4. **Important**: Restart your terminal completely (not just new tab)

### Font doesn't appear in terminal's font picker

**Cause**: Font cache not refreshed or font not properly installed.

**Solutions**:
1. **Linux**: Run `fc-cache -fv`
2. **macOS**: Font Book may need refresh - open Font Book app and verify font appears
3. Verify installation path:
   - macOS: `ls ~/Library/Fonts/*Nerd*`
   - Linux: `ls ~/.local/share/fonts/*Nerd*`
4. Reinstall: `zprof font install`

### Icons work in some programs but not terminal

**Cause**: Terminal is using a different font than other applications.

**Solution**: Terminal fonts are configured separately. Follow Terminal Configuration instructions for your specific terminal.

### After restarting, font reverts to default

**Cause**: Profile not set as default or terminal has multiple profiles.

**Solution**:
- **iTerm2**: Check Preferences → Profiles → Set as Default
- **Terminal.app**: Check Preferences → Profiles → Set as Default
- **GNOME/Konsole**: Ensure edited profile is the default profile

### Font looks different after installation

**Cause**: Selected variant (Bold, Italic) instead of Regular.

**Solution**: In terminal settings, explicitly select "Regular" or "Mono" variant.

### Want to use different font

```bash
$ zprof font install
```

Install additional fonts. Update terminal configuration to use new font.

## FAQ

### Do I need Nerd Fonts for all profiles?

Only if using prompt engines: Starship, Powerlevel10k, Oh-My-Posh, or Spaceship.

Pure prompt does NOT require Nerd Fonts (it uses standard ASCII).

### Can I use my own Nerd Font?

Yes! Install any Nerd Font from https://www.nerdfonts.com/, then configure your terminal manually. zprof will detect it during profile creation.

### Will fonts work across all terminals?

Fonts are system-wide, but each terminal must be configured individually. Install once, configure each terminal separately.

### How do I uninstall a Nerd Font?

**macOS**: Delete from `~/Library/Fonts/`
**Linux**: Delete from `~/.local/share/fonts/` and run `fc-cache -fv`

### Which font should I choose?

- **Best overall**: FiraCode Nerd Font (ligatures + clean design)
- **Best readability**: JetBrainsMono Nerd Font
- **For Powerlevel10k**: Meslo Nerd Font (optimized)
- **Traditional**: Hack Nerd Font

All work with all prompt engines - choose based on personal preference.

### Do Nerd Fonts slow down terminal?

No. Font rendering is handled by the terminal emulator, not zprof or the prompt engine.

### Can I skip font installation?

Yes, but prompts will display broken characters. Install later with `zprof font install`.

## More Resources

- Official Nerd Fonts website: https://www.nerdfonts.com/
- Font preview gallery: https://www.programmingfonts.org/
- zprof font commands: [Commands Reference](commands.md#font-commands)
- Troubleshooting: [Troubleshooting Guide](troubleshooting.md#font-issues)
```

**2. Update: `docs/user-guide/quick-start.md`**

Add font mention in profile creation section:

```markdown
## Creating Your First Profile

1. Run the creation command:
   ```bash
   zprof create my-profile
   ```

2. Select your framework (oh-my-zsh recommended for beginners)

3. Select your prompt engine (Starship recommended)

   **Note**: If you select a prompt engine that requires Nerd Fonts (Starship, Powerlevel10k, Oh-My-Posh), zprof will automatically offer to install a font for you. This ensures icons and symbols display correctly. See [Nerd Fonts Guide](nerd-fonts.md) for details.

4. Select plugins (use arrow keys, Space to select, Enter to confirm)

5. [Rest of steps...]
```

**3. Update: `docs/user-guide/commands.md`**

Add font commands section:

```markdown
## Font Commands

Manage Nerd Fonts for prompt engines.

### `zprof font list`

List all installed Nerd Fonts and show which profiles use them.

**Usage:**
```bash
zprof font list
```

**Example Output:**
```
Installed Nerd Fonts:

✓ FiraCode Nerd Font
  ~/Library/Fonts/FiraCodeNerdFont-Regular.ttf (+ 11 more files)
  Used by: work, personal

✓ JetBrainsMono Nerd Font
  ~/Library/Fonts/JetBrainsMonoNerdFont-Regular.ttf (+ 7 more files)
  (Not used by any profile)
```

### `zprof font install`

Install a new Nerd Font interactively.

**Usage:**
```bash
zprof font install
```

Launches the font selection interface, downloads the selected font, installs it to your system, and shows terminal configuration instructions.

### `zprof font info <font-name>`

Show detailed information about a specific font.

**Usage:**
```bash
zprof font info FiraCode
zprof font info JetBrainsMono
```

**Arguments:**
- `<font-name>` - Font identifier (e.g., FiraCode, JetBrainsMono)

**Example Output:**
```
FiraCode Nerd Font
------------------
Description: Programming ligatures, clean and modern
File Format: TrueType (.ttf)
Recommended for: Starship, Oh-My-Posh

Installation Status: ✓ Installed
  Location: ~/Library/Fonts/
  Files: 12

Terminal Configuration:
[... terminal-specific instructions ...]
```

**See Also:**
- [Nerd Fonts Guide](nerd-fonts.md) - Complete font documentation
- [Troubleshooting](troubleshooting.md#font-issues) - Fix font display issues
```

**4. Update: `docs/user-guide/troubleshooting.md`**

Add font issues section:

```markdown
## Font Issues

### Prompt shows boxes (□) or question marks (?) instead of icons

**Symptoms:**
- Git branch shows □ instead of 
- Language versions show □ instead of , , etc.
- Prompt displays `□ ~/code  □ main` instead of ` ~/code   main`

**Cause:** Terminal not configured to use a Nerd Font.

**Solution:**

1. Check if Nerd Font is installed:
   ```bash
   zprof font list
   ```

2. If no fonts shown, install one:
   ```bash
   zprof font install
   ```

3. Configure your terminal to use the font:
   - See [Nerd Fonts Guide - Terminal Configuration](nerd-fonts.md#terminal-configuration)

4. **Important**: Completely restart your terminal (close and reopen, not just new tab)

### Font installed but terminal doesn't show it in font picker

**Cause:** Font cache not refreshed or font not installed to correct location.

**Solution:**

**On Linux:**
```bash
fc-cache -fv
fc-list | grep -i "nerd"
```

**On macOS:**
1. Open Font Book app
2. Check if font appears
3. If not, manually install:
   ```bash
   ls ~/Library/Fonts/*Nerd*
   ```
4. If files present but Font Book doesn't show them, try:
   ```bash
   sudo atsutil databases -remove
   atsutil server -shutdown
   atsutil server -ping
   ```

### Icons display correctly in VS Code but not iTerm2

**Cause:** VS Code and iTerm2 have separate font configurations.

**Solution:** Configure each terminal independently. See [Terminal Configuration](nerd-fonts.md#terminal-configuration) for your specific terminal.

### Profile creation failed during font installation

**Cause:** Network error, GitHub unavailable, or permission issue.

**Solution:**

1. Profile was still created (font installation is optional)

2. Install font manually later:
   ```bash
   zprof font install
   ```

3. Check font in profile:
   ```bash
   cat ~/.zprof/profiles/YOUR_PROFILE/profile.toml
   ```
   Look for `nerd_font = "..."` or `nerd_font_skipped = true`

### Want to change font after profile creation

**Solution:**

1. Install new font:
   ```bash
   zprof font install
   ```

2. Update terminal configuration to use new font

3. (Optional) Update profile manifest:
   ```bash
   # Edit the profile.toml manually
   vim ~/.zprof/profiles/YOUR_PROFILE/profile.toml
   # Change: nerd_font = "NewFontName"
   ```

### Nerd Font works in login shell but not in tmux

**Cause:** tmux may override terminal settings or not inherit font configuration.

**Solution:**

1. Ensure terminal is configured BEFORE starting tmux
2. Add to `~/.tmux.conf`:
   ```
   set -g default-terminal "screen-256color"
   set -ga terminal-overrides ",*256col*:Tc"
   ```
3. Restart tmux completely:
   ```bash
   tmux kill-server
   tmux
   ```
```

**5. Update: `docs/user-guide/faq.md`**

Add font FAQs:

```markdown
## Nerd Fonts

### What are Nerd Fonts and why do I need them?

Nerd Fonts are specially patched fonts that include thousands of icons and symbols used by modern prompt engines (Starship, Powerlevel10k, Oh-My-Posh). Without them, your prompt will show boxes (□) or question marks (?) instead of beautiful icons.

See the [Nerd Fonts Guide](nerd-fonts.md) for complete details.

### Do all profiles require Nerd Fonts?

No. Only profiles using these prompt engines require Nerd Fonts:
- Starship
- Powerlevel10k
- Oh-My-Posh
- Spaceship

The Pure prompt does NOT require Nerd Fonts (uses standard ASCII).

### Can I use my own Nerd Font not in zprof's list?

Yes! Install any font from https://www.nerdfonts.com/, configure your terminal to use it, and zprof will detect it during profile creation. The 6 fonts zprof offers are curated recommendations, but any Nerd Font will work.

### Which Nerd Font should I choose?

**Best overall**: FiraCode Nerd Font (programming ligatures + clean design)
**Best readability**: JetBrainsMono Nerd Font (designed for long coding sessions)
**For Powerlevel10k**: Meslo Nerd Font (specifically optimized)

All work with all prompt engines. Choose based on personal preference. You can preview fonts at https://www.programmingfonts.org/.

### Can I skip font installation during profile creation?

Yes. Select "Skip font installation" in the font selection menu. Your prompt will display broken characters until you install a font later with:
```bash
zprof font install
```

### How do I change fonts after creating a profile?

```bash
# Install new font
zprof font install

# Configure terminal to use new font
# (See Nerd Fonts Guide for terminal-specific instructions)
```

Fonts are system-wide, so changing your terminal's font setting immediately changes all prompts.

### Do Nerd Fonts work on Windows?

Nerd Fonts YES, zprof auto-installation NO (not in v0.2.0).

Windows users can:
1. Manually download from https://www.nerdfonts.com/
2. Install via Windows font installer (double-click .ttf files)
3. Configure terminal (Windows Terminal, WSL, etc.)

### Will installing fonts affect other terminals or applications?

Fonts are installed system-wide, making them available to all applications. However, each terminal must be configured individually to use them. Installing a Nerd Font won't change existing terminal configurations.

### How do I uninstall a Nerd Font?

**macOS**:
```bash
rm ~/Library/Fonts/*NerdFont*
```

**Linux**:
```bash
rm ~/.local/share/fonts/*NerdFont*
fc-cache -fv
```

Or use Font Book (macOS) / Font Manager (Linux) GUI applications.

### My terminal shows the font but icons still don't display

Ensure you selected the "Mono" variant in your terminal's font settings. Some terminals have separate settings for regular text vs terminal use. Also verify font name is exact: `"FiraCode Nerd Font Mono"` not just `"FiraCode"`.

### Can I use Nerd Fonts with Oh My Zsh themes?

Yes! Oh My Zsh themes will automatically use Nerd Font icons if a Nerd Font is installed and configured. Many popular Oh My Zsh themes (like agnoster, powerlevel9k) benefit from Nerd Fonts.
```

### Critical Reminders

**DO:**
- ✅ Create comprehensive `nerd-fonts.md` guide
- ✅ Update all mentioned existing docs
- ✅ Include terminal configuration for ALL major terminals
- ✅ Provide concrete examples with actual command output
- ✅ Add troubleshooting for common issues
- ✅ Cross-reference between docs (use markdown links)
- ✅ Test all commands before documenting
- ✅ Use clear, beginner-friendly language
- ✅ Add "See Also" sections for related topics

**DON'T:**
- ❌ Don't use technical jargon without explanation
- ❌ Don't skip terminal configuration examples
- ❌ Don't forget to test commands
- ❌ Don't duplicate content (link instead)
- ❌ Don't skip troubleshooting section
- ❌ Don't forget Windows users (explain manual process)
- ❌ Don't add screenshots without descriptive alt text
- ❌ Don't leave broken internal links

### Acceptance Criteria Expanded

1. **Create `docs/user-guide/nerd-fonts.md`**
   - What are Nerd Fonts (with visual examples)
   - Why zprof needs them
   - Automatic installation workflow
   - All 6 available fonts with descriptions
   - Manual installation instructions
   - Terminal configuration for 7+ terminals
   - Font management commands overview
   - Troubleshooting section
   - FAQ section
   - External resources links

2. **Update `docs/user-guide/quick-start.md`**
   - Add note about font installation in profile creation
   - Link to nerd-fonts.md
   - Mention it's automatic for compatible prompts

3. **Update `docs/user-guide/commands.md`**
   - Add "Font Commands" section
   - Document `zprof font list`
   - Document `zprof font install`
   - Document `zprof font info <name>`
   - Include usage examples
   - Include sample output
   - Cross-reference to nerd-fonts.md

4. **Update `docs/user-guide/troubleshooting.md`**
   - Add "Font Issues" section
   - Broken characters troubleshooting
   - Font not appearing in picker
   - Icons work in some programs not others
   - Font reverts after restart
   - Each with cause and solution

5. **Update `docs/user-guide/faq.md`**
   - What are Nerd Fonts
   - Do all profiles need them
   - Can I use my own font
   - Which font to choose
   - Can I skip installation
   - How to change fonts later
   - Windows support
   - Affect on other applications
   - How to uninstall

6. **Include terminal configuration examples**
   - iTerm2 (with screenshots if available)
   - Terminal.app
   - VS Code
   - Alacritty (with config file example)
   - Kitty (with config file example)
   - GNOME Terminal
   - Konsole

7. **Add troubleshooting for common issues**
   - Broken characters (most common)
   - Font cache issues
   - Multiple terminal configuration
   - tmux compatibility
   - Font picker issues
   - Each with step-by-step solution

8. **Add screenshots/examples if applicable**
   - Before/after Nerd Font installation
   - Terminal configuration screens
   - Font selection menu
   - Use descriptive alt text for accessibility

## Tasks / Subtasks

- [ ] Create `docs/user-guide/nerd-fonts.md` (AC: 1)
  - [ ] Introduction section
  - [ ] What are Nerd Fonts (with examples)
  - [ ] Why needed section
  - [ ] Automatic installation workflow
  - [ ] Available fonts (all 6 with details)
  - [ ] Manual installation instructions
  - [ ] Terminal configuration for all terminals
  - [ ] Font management commands overview
  - [ ] Troubleshooting section
  - [ ] FAQ section
  - [ ] Resources/links
- [ ] Update `docs/user-guide/quick-start.md` (AC: 2)
  - [ ] Add font note in profile creation section
  - [ ] Link to nerd-fonts.md
- [ ] Update `docs/user-guide/commands.md` (AC: 3)
  - [ ] Add "Font Commands" section
  - [ ] Document `zprof font list`
  - [ ] Document `zprof font install`
  - [ ] Document `zprof font info`
  - [ ] Include examples and output
  - [ ] Cross-reference to guide
- [ ] Update `docs/user-guide/troubleshooting.md` (AC: 4, 7)
  - [ ] Add "Font Issues" section
  - [ ] Broken characters issue
  - [ ] Font cache issue
  - [ ] Multi-terminal issue
  - [ ] tmux issue
  - [ ] Each with cause + solution
- [ ] Update `docs/user-guide/faq.md` (AC: 5)
  - [ ] Add Nerd Fonts section
  - [ ] 10+ font-related FAQs
  - [ ] Clear, concise answers
  - [ ] Cross-references to guides
- [ ] Terminal configuration examples (AC: 6)
  - [ ] iTerm2 step-by-step
  - [ ] Terminal.app step-by-step
  - [ ] VS Code step-by-step
  - [ ] Alacritty with config
  - [ ] Kitty with config
  - [ ] GNOME Terminal step-by-step
  - [ ] Konsole step-by-step
- [ ] Test all commands (AC: all)
  - [ ] Run every command documented
  - [ ] Verify output matches docs
  - [ ] Test on macOS and Linux if possible
- [ ] Review and proofread (AC: all)
  - [ ] Check spelling and grammar
  - [ ] Verify all links work
  - [ ] Ensure consistent terminology
  - [ ] Check code block formatting

## Dev Agent Record

### Context Reference

Epic: [epic-4-nerd-fonts.md](../epic-4-nerd-fonts.md)
Tech Spec: [tech-spec-epic-4.md](../../tech-spec-epic-4.md)
Previous Story: [epic-4-story-9.md](epic-4-story-9.md) (Font Commands)
Existing Docs: [docs/user-guide/](../../../docs/user-guide/)

### Agent Model Used

<!-- Will be filled during implementation -->

### Debug Log References

<!-- Will be filled during implementation -->

### Completion Notes List

<!-- Will be filled during implementation -->

### File List

<!-- Will be filled during implementation -->
