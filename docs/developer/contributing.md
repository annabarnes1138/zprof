# Contributing to zprof

Thank you for your interest in contributing to zprof! This guide will help you get started.

## Getting Started

### Prerequisites

- **Rust**: 1.70 or later (`rustup update stable`)
- **Git**: For version control
- **zsh**: For testing (obviously!)

### Clone the Repository

```bash
git clone https://github.com/annabarnes1138/zprof.git
cd zprof
```

### Build from Source

```bash
# Build debug version
cargo build

# Build release version
cargo build --release

# The binary will be at:
# ./target/debug/zprof (debug)
# ./target/release/zprof (release)
```

### Run Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test init_test

# Run with logging
RUST_LOG=debug cargo test

# Update snapshot tests
cargo insta review
```

## Development Workflow

### 1. Create a Branch

```bash
git checkout -b feature/my-new-feature
# or
git checkout -b fix/bug-description
```

### 2. Make Changes

Follow the [Architecture Overview](architecture.md) to understand the codebase structure.

**Key guidelines:**
- Write tests for new features
- Update documentation for user-facing changes
- Follow Rust naming conventions
- Add comments for complex logic
- Keep functions small and focused

### 3. Test Your Changes

```bash
# Run all tests
cargo test

# Test a specific command manually
cargo run -- create test-profile

# Clean up test artifacts
rm -rf ~/.zsh-profiles/profiles/test-*
```

### 4. Format and Lint

```bash
# Format code
cargo fmt

# Run clippy (linter)
cargo clippy --all-targets --all-features -- -D warnings
```

### 5. Commit Changes

Write clear, descriptive commit messages:

```bash
git add .
git commit -m "Add support for antigen framework

- Implement Framework trait for antigen
- Add detection in detector.rs
- Update plugin registry with antigen-specific plugins
- Add integration tests

Closes #42"
```

**Commit message format:**
- First line: Brief summary (50 chars or less)
- Blank line
- Detailed description (wrap at 72 chars)
- Reference issues/PRs if applicable

### 6. Push and Create Pull Request

```bash
git push origin feature/my-new-feature
```

Then create a pull request on GitHub.

## Code Style

### Rust Conventions

Follow the official [Rust Style Guide](https://doc.rust-lang.org/beta/style-guide/).

**Key points:**
- Use `snake_case` for functions and variables
- Use `PascalCase` for types and traits
- Use `SCREAMING_SNAKE_CASE` for constants
- Prefer explicit types for public APIs
- Document public items with `///` comments

### Project-Specific Conventions

**Error handling:**
```rust
use anyhow::{Context, Result};

pub fn my_function() -> Result<()> {
    some_operation()
        .context("Helpful error message for users")?;
    Ok(())
}
```

**File operations:**
```rust
// Use the filesystem module for all file operations
use crate::core::filesystem;

filesystem::copy_with_backup(src, dest)?;
```

**CLI commands:**
```rust
// All commands follow this structure
#[derive(Debug, Args)]
pub struct MyCommandArgs {
    // Arguments
}

pub fn execute(args: MyCommandArgs) -> Result<()> {
    // Implementation
    Ok(())
}
```

## Testing Guidelines

### Unit Tests

Place unit tests in the same file:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_profile_name() {
        assert!(validate_profile_name("valid-name").is_ok());
        assert!(validate_profile_name("../invalid").is_err());
    }
}
```

### Integration Tests

Place integration tests in `tests/`:

```rust
use serial_test::serial;

#[test]
#[serial]  // Prevents parallel execution if tests modify HOME
fn test_create_profile() {
    with_temp_home(|| {
        // Test implementation
    });
}
```

### Snapshot Tests

Use `insta` for CLI output validation:

```rust
#[test]
fn test_list_output() {
    let output = run_zprof_command(&["list"]);
    insta::assert_snapshot!(output);
}
```

Update snapshots:
```bash
cargo insta review
```

## Adding Features

### Adding a New Framework

See [Adding Frameworks](adding-frameworks.md) for a detailed step-by-step guide.

**Quick overview:**
1. Create `src/frameworks/myframework.rs`
2. Implement `Framework` trait
3. Add to `FrameworkType` enum
4. Add detection logic
5. Update generator
6. Add tests

### Adding a New Command

1. Create `src/cli/mycommand.rs`:

```rust
use anyhow::Result;
use clap::Args;

#[derive(Debug, Args)]
pub struct MyCommandArgs {
    /// Profile name
    pub profile: String,
}

pub fn execute(args: MyCommandArgs) -> Result<()> {
    println!("Executing my command for profile: {}", args.profile);
    Ok(())
}
```

2. Register in `src/main.rs`:

```rust
#[derive(Debug, Subcommand)]
enum Commands {
    // ... existing commands
    MyCommand(mycommand::MyCommandArgs),
}

match cli.command {
    // ... existing matches
    Commands::MyCommand(args) => mycommand::execute(args)?,
}
```

3. Add integration test in `tests/mycommand_test.rs`

### Adding a New Plugin

Update `src/frameworks/plugin.rs`:

```rust
Plugin {
    name: "my-plugin",
    description: "Does something useful",
    compatibility: PluginCompatibility {
        supported_managers: &[
            ManagerSupport {
                framework: FrameworkType::OhMyZsh,
                repo_url: None,  // Built-in
                recommended: true,
            },
            ManagerSupport {
                framework: FrameworkType::Zap,
                repo_url: Some("user/my-plugin"),
                recommended: false,
            },
        ],
        dependencies: &[],
    },
},
```

## Documentation

### User-Facing Documentation

Update docs in `docs/user-guide/` for user-facing changes:
- New commands → Update `commands.md`
- New frameworks → Update `frameworks.md`
- Troubleshooting → Update `troubleshooting.md`

### Developer Documentation

Update docs in `docs/developer/` for architectural changes:
- Architecture changes → Update `architecture.md`
- New patterns → Update this guide
- Technical decisions → Update `technical-decisions.md`

### Code Documentation

Document public APIs with doc comments:

```rust
/// Creates a new profile with the given name.
///
/// # Arguments
///
/// * `name` - The profile name (must not contain `/` or `..`)
/// * `framework` - The framework to install
///
/// # Errors
///
/// Returns an error if:
/// - Profile name is invalid
/// - Profile already exists
/// - Framework installation fails
///
/// # Examples
///
/// ```no_run
/// create_profile("work", FrameworkType::OhMyZsh)?;
/// ```
pub fn create_profile(name: &str, framework: FrameworkType) -> Result<()> {
    // Implementation
}
```

## Pull Request Guidelines

### Before Submitting

- [ ] All tests pass (`cargo test`)
- [ ] Code is formatted (`cargo fmt`)
- [ ] No clippy warnings (`cargo clippy`)
- [ ] Documentation updated (if needed)
- [ ] Commit messages are clear
- [ ] Branch is up to date with main

### PR Description Template

```markdown
## Description
Brief description of changes.

## Type of Change
- [ ] Bug fix (non-breaking change fixing an issue)
- [ ] New feature (non-breaking change adding functionality)
- [ ] Breaking change (fix or feature that would break existing functionality)
- [ ] Documentation update

## Testing
Describe how you tested your changes.

## Checklist
- [ ] Tests added/updated
- [ ] Documentation updated
- [ ] No breaking changes (or documented)
```

### Review Process

1. **Automated checks**: CI runs tests on all platforms
2. **Code review**: Maintainer reviews code
3. **Feedback**: Address any comments or requested changes
4. **Merge**: PR is merged once approved

## Getting Help

- **Questions**: Open a [Discussion](https://github.com/annabarnes1138/zprof/discussions)
- **Bugs**: Open an [Issue](https://github.com/annabarnes1138/zprof/issues)
- **Chat**: Join our community (link TBD)

## Code of Conduct

Be respectful, inclusive, and constructive. We want zprof to be a welcoming project for all contributors.

## License

By contributing, you agree that your contributions will be licensed under the same license as the project (see LICENSE file).

## Thank You!

Every contribution, no matter how small, helps make zprof better. We appreciate your time and effort! 🎉
