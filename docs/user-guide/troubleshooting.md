# Troubleshooting

Common issues and their solutions.

## Installation Issues

### `cargo install zprof` fails

**Symptom**: Error during compilation

**Solution**:
```bash
# Update Rust
rustup update stable

# Try again
cargo install zprof --force
```

### `zprof: command not found` after installation

**Symptom**: Command not recognized

**Solution**: Add `~/.cargo/bin` to your PATH:

```bash
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
source ~/.zshrc
```

---

## Initialization Issues

### `zprof init` says "already initialized" but directory is empty

**Symptom**: Can't reinitialize

**Solution**:
```bash
# Remove the directory
rm -rf ~/.zsh-profiles/

# Try again
zprof init
```

### Framework not detected during init

**Symptom**: `zprof init` doesn't find your oh-my-zsh/zimfw installation

**Solution**:

Check if framework is in the standard location:
```bash
ls -la ~/ | grep -E "\.oh-my-zsh|\.zim|\.zprezto|\.zinit|\.zap"
```

If it's in a non-standard location, create a profile manually:
```bash
zprof create default --framework oh-my-zsh
```

---

## Profile Issues

### Profile not activating after `zprof use`

**Symptom**: Switched profile but shell looks the same

**Solution**: Start a new shell:
```bash
exec zsh
```

Or open a new terminal window.

### `exec zsh` shows errors

**Symptom**: Errors when starting new shell

**Solution**:

1. Check which profile is active:
```bash
zprof current
```

2. Regenerate the profile:
```bash
zprof regenerate <profile-name>
```

3. If still broken, check the manifest:
```bash
zprof edit <profile-name>
# Fix any invalid values
```

### Can't delete active profile

**Symptom**: Error "cannot delete active profile"

**Solution**: Switch to a different profile first:
```bash
zprof use other-profile
zprof delete old-profile
```

### Profile creation wizard crashes

**Symptom**: TUI exits unexpectedly

**Solution**:

Use non-interactive mode:
```bash
zprof create myprofile --framework oh-my-zsh --theme robbyrussell
```

---

## Configuration Issues

### Manifest validation fails

**Symptom**: Error when editing `profile.toml`

**Common errors**:

**Invalid framework name**:
```
Error: Invalid framework 'ohmyzsh'
Valid options: oh-my-zsh, zimfw, prezto, zinit, zap
```

**Solution**: Use exact framework name from the error message.

**Invalid plugin**:
```
Error: Plugin 'nonexistent-plugin' not found in registry
```

**Solution**: Check available plugins:
```bash
# List all plugins (in future version)
# For now, use a known plugin or check framework docs
```

**Syntax error**:
```
Error: TOML parse error at line 5
```

**Solution**: Check TOML syntax (commas, quotes, brackets):
```toml
# Bad
[plugins]
enabled = [git docker]  # Missing quotes

# Good
[plugins]
enabled = ["git", "docker"]
```

### Shell config regeneration fails

**Symptom**: Error during `zprof regenerate`

**Solution**:

1. Check manifest syntax:
```bash
zprof edit <profile-name>
```

2. Verify framework is installed:
```bash
ls ~/.zsh-profiles/profiles/<profile-name>/.oh-my-zsh  # or .zim, etc.
```

3. Reinstall framework if missing:
```bash
zprof delete <profile-name>
zprof create <profile-name> --framework oh-my-zsh
```

---

## Performance Issues

### Slow shell startup

**Symptom**: Shell takes > 1 second to start

**Solutions**:

**Measure startup time**:
```bash
time zsh -i -c exit
```

**Identify slow plugins**:
```bash
# Add to top of .zshrc (temporarily)
zmodload zsh/zprof

# Add to bottom of .zshrc
zprof  # Shows timing breakdown
```

**Reduce plugins**:
```bash
zprof edit <profile-name>
# Remove unnecessary plugins from [plugins] section
```

**Switch to faster framework**:
```bash
# Try zimfw or zinit
zprof create fast-profile --framework zimfw
zprof use fast-profile
```

**Use async prompt**:
```bash
# Starship or Pure are faster than complex themes
zprof edit <profile-name>
# Change theme = "starship"
```

### zinit very slow on first load

**Symptom**: First shell startup after reboot is slow

**Solution**: This is normal—zinit compiles plugins on first load. Subsequent starts are fast.

---

## Framework-Specific Issues

### oh-my-zsh: "Insecure directories" warning

**Symptom**:
```
[oh-my-zsh] Insecure completion-dependent directories detected
```

**Solution**:
```bash
chmod 755 ~/.zsh-profiles/profiles/<profile-name>/.oh-my-zsh
chmod -R 755 ~/.zsh-profiles/profiles/<profile-name>/.oh-my-zsh/custom
```

### zimfw: `.zimrc` not found

**Symptom**: zimfw errors about missing `.zimrc`

**Solution**: Regenerate profile:
```bash
zprof regenerate <profile-name>
```

zprof auto-generates `.zimrc` from `profile.toml`.

### prezto: Modules not loading

**Symptom**: Plugins don't activate

**Solution**: Check module names match prezto's conventions:

```toml
# profile.toml
[plugins]
enabled = [
    "git",              # prezto module name
    "syntax-highlighting",  # not "zsh-syntax-highlighting"
    "autosuggestions"   # not "zsh-autosuggestions"
]
```

See prezto docs for exact module names.

### zinit: Turbo mode not working

**Symptom**: Plugins still load slowly

**Solution**: zprof enables turbo automatically for compatible plugins. Verify with:
```bash
cat ~/.zsh-profiles/profiles/<profile-name>/.zshrc | grep "zinit ice"
```

### zap: Plugin not found

**Symptom**: "Could not find plugin..."

**Solution**: zap requires GitHub URLs. Ensure plugin is in zprof's registry, or use full URL:

```toml
[plugins]
enabled = ["zsh-users/zsh-autosuggestions"]  # Include username
```

---

## Import/Export Issues

### Export fails with permission error

**Symptom**: Can't write `.zprof` file

**Solution**: Check target directory permissions:
```bash
# Export to home directory instead
zprof export <profile-name> --output ~/<profile-name>.zprof
```

### Import fails: "Invalid archive"

**Symptom**: Corrupted or incompatible archive

**Solution**:

1. Verify it's a valid archive:
```bash
file <profile-name>.zprof
# Should show: gzip compressed data
```

2. Try extracting manually:
```bash
mkdir test-extract
tar -xzf <profile-name>.zprof -C test-extract
ls test-extract  # Should see profile.toml
```

3. If corrupted, ask sender to re-export.

### GitHub import fails: Repository not found

**Symptom**: "Failed to clone repository"

**Solutions**:

**Public repo**: Verify URL is correct:
```bash
# Both work
zprof import github:username/repo
zprof import --github username/repo
```

**Private repo**: Set up Git credentials first:
```bash
# SSH
ssh-add ~/.ssh/id_rsa

# Or use HTTPS with token
git config --global credential.helper store
```

**No profile.toml in repo**: The repository must have `profile.toml` in its root.

---

## Rollback Issues

### Rollback finds no backup

**Symptom**: "No backup file found"

**Reason**: No `.zshrc.pre-zprof` was created

**Solutions**:

1. Check cache backups:
```bash
ls ~/.zsh-profiles/cache/backups/
```

2. If you have a backup elsewhere, restore manually:
```bash
cp ~/my-backup/.zshrc ~/.zshrc
```

3. If no backup exists, create a new `.zshrc` from scratch.

### After rollback, shell still uses profile

**Symptom**: Rollback completed but profile still active

**Solution**: The rollback preserves `~/.zsh-profiles/`. To fully uninstall:

```bash
zprof rollback
rm -rf ~/.zsh-profiles/
# Edit ~/.zshenv and remove zprof-managed section
```

---

## General Debugging

### Enable verbose logging

**For development/troubleshooting**:
```bash
export RUST_LOG=debug
zprof <command>
```

### Check zprof version

```bash
zprof --version
```

### Verify installation integrity

```bash
# Check directory structure
ls -la ~/.zsh-profiles/

# Should see:
# profiles/
# shared/
# cache/
# config.toml
```

### Check active profile configuration

```bash
cat ~/.zshenv | grep ZDOTDIR
# Should show: export ZDOTDIR="/path/to/profile"
```

### Reset to clean state

**Nuclear option** (destroys all profiles):
```bash
# Backup first!
cp -r ~/.zsh-profiles ~/zprof-backup

# Remove everything
rm -rf ~/.zsh-profiles/
rm ~/.zshenv  # Or edit to remove zprof section

# Start over
zprof init
```

---

## Getting Help

If you're still stuck:

1. **Check existing issues**: https://github.com/annabarnes1138/zprof/issues
2. **Search discussions**: https://github.com/annabarnes1138/zprof/discussions
3. **Open a new issue**: Provide:
   - `zprof --version`
   - Operating system and version
   - Shell version (`zsh --version`)
   - Complete error message
   - Steps to reproduce

**Include relevant logs**:
```bash
export RUST_LOG=debug
zprof <command> 2>&1 | tee zprof-debug.log
# Attach zprof-debug.log to your issue
```

## See Also

- [Command Reference](commands.md) - Detailed command documentation
- [Understanding Profiles](profiles.md) - How profiles work
- [FAQ](faq.md) - Frequently asked questions
