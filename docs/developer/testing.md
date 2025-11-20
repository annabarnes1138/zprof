# Testing Guide

Comprehensive guide to testing in zprof.

## Test Organization

```
zprof/
├── src/
│   └── **/*.rs           # Unit tests (#[cfg(test)] mod tests)
└── tests/
    ├── init_test.rs       # Initialization tests
    ├── create_test.rs     # Profile creation tests
    ├── use_test.rs        # Profile switching tests
    ├── framework_detection_test.rs
    └── snapshots/         # Insta snapshot files
```

## Running Tests

```bash
# Run all tests
cargo test

# Run specific test file
cargo test --test init_test

# Run specific test function
cargo test test_create_profile

# Run with output
cargo test -- --nocapture

# Run with logging
RUST_LOG=debug cargo test

# Run only unit tests
cargo test --lib

# Run only integration tests
cargo test --test '*'
```

## Unit Tests

Unit tests live in the same file as the code:

```rust
// src/core/profile.rs

pub fn validate_profile_name(name: &str) -> Result<()> {
    anyhow::ensure!(!name.contains(".."), "Invalid profile name");
    anyhow::ensure!(!name.contains("/"), "Invalid profile name");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_valid_name() {
        assert!(validate_profile_name("work").is_ok());
        assert!(validate_profile_name("my-profile").is_ok());
    }

    #[test]
    fn test_validate_invalid_name() {
        assert!(validate_profile_name("../etc").is_err());
        assert!(validate_profile_name("path/to/profile").is_err());
    }
}
```

## Integration Tests

Integration tests in `tests/` directory test complete workflows:

```rust
// tests/create_test.rs

use serial_test::serial;
use tempfile::TempDir;

#[test]
#[serial]  // Prevents parallel execution
fn test_create_profile_flow() {
    // Create temporary HOME
    let temp = TempDir::new().unwrap();
    std::env::set_var("HOME", temp.path());

    // Initialize zprof
    zprof::cli::init::execute(InitArgs {}).unwrap();

    // Create profile
    zprof::cli::create::execute(CreateArgs {
        profile_name: "test".to_string(),
        framework: Some("oh-my-zsh".to_string()),
        theme: Some("robbyrussell".to_string()),
        plugins: vec![],
    }).unwrap();

    // Verify profile exists
    let profile_path = temp.path()
        .join(".zsh-profiles/profiles/test");
    assert!(profile_path.exists());
    assert!(profile_path.join("profile.toml").exists());
}
```

### Using `serial_test`

Tests that modify `HOME` or global state must run serially:

```rust
use serial_test::serial;

#[test]
#[serial]  // Ensures tests don't run in parallel
fn test_modifies_home() {
    std::env::set_var("HOME", "/tmp/test");
    // Test code
}
```

### Using `tempfile`

Create isolated test environments:

```rust
use tempfile::TempDir;

#[test]
fn test_with_temp_dir() {
    let temp = TempDir::new().unwrap();
    let test_file = temp.path().join("test.txt");

    std::fs::write(&test_file, "content").unwrap();

    // temp is automatically cleaned up when dropped
}
```

## Snapshot Tests

Snapshot tests validate CLI output using `insta`:

```rust
use insta::assert_snapshot;

#[test]
fn test_list_command_output() {
    let output = run_command_capture_output("list");
    assert_snapshot!(output);
}
```

### Creating Snapshots

First run creates the snapshot:

```bash
cargo test test_list_command_output
```

This creates `tests/snapshots/test_name.snap`:

```
---
source: tests/list_test.rs
expression: output
---
Available profiles:

  work (active)
    Framework: oh-my-zsh
    Theme: robbyrussell
    Plugins: 3
```

### Updating Snapshots

When output changes intentionally:

```bash
# Review changes
cargo insta review

# Or update all
cargo insta accept
```

### Best Practices

- Use snapshots for user-facing output
- Keep snapshots small and focused
- Review snapshot diffs carefully
- Don't snapshot unstable output (timestamps, paths)

## Mocking

### Filesystem Mocking

Use `tempfile` for filesystem operations:

```rust
#[test]
fn test_file_operation() {
    let temp = TempDir::new().unwrap();
    let file_path = temp.path().join("test.toml");

    // Test code using file_path

    // Cleanup is automatic
}
```

### Environment Mocking

```rust
#[test]
fn test_with_env_var() {
    std::env::set_var("TEST_VAR", "value");

    // Test code

    std::env::remove_var("TEST_VAR");
}
```

### Dependency Injection

For testability, use traits:

```rust
// Production code
pub trait UserInput {
    fn confirm(&self, prompt: &str) -> Result<bool>;
}

pub struct RealUserInput;
impl UserInput for RealUserInput {
    fn confirm(&self, prompt: &str) -> Result<bool> {
        // Real implementation
    }
}

// Test code
struct MockUserInput {
    response: bool,
}

impl UserInput for MockUserInput {
    fn confirm(&self, _prompt: &str) -> Result<bool> {
        Ok(self.response)
    }
}

#[test]
fn test_with_mock() {
    let mock = MockUserInput { response: true };
    let result = some_function(&mock);
    assert!(result.is_ok());
}
```

## Test Helpers

Create test utilities in `tests/common/mod.rs`:

```rust
// tests/common/mod.rs

use tempfile::TempDir;

pub fn create_test_profile(name: &str, framework: &str) -> TempDir {
    let temp = TempDir::new().unwrap();
    std::env::set_var("HOME", temp.path());

    // Create profile structure
    // ...

    temp
}

pub fn run_zprof_command(args: &[&str]) -> String {
    // Capture command output
    // ...
}
```

Use in tests:

```rust
// tests/some_test.rs

mod common;

#[test]
fn test_using_helper() {
    let temp = common::create_test_profile("test", "oh-my-zsh");
    // Test code
}
```

## Performance Testing

Measure operation performance:

```rust
use std::time::Instant;

#[test]
fn test_profile_switch_performance() {
    let start = Instant::now();

    switch_profile("work").unwrap();

    let duration = start.elapsed();
    assert!(duration.as_millis() < 500, "Switch took {}ms", duration.as_millis());
}
```

## Testing Checklist

Before submitting a PR, ensure:

- [ ] All tests pass: `cargo test`
- [ ] New features have tests
- [ ] Tests run in isolation (don't depend on order)
- [ ] No hardcoded paths (use `tempfile`)
- [ ] Serial tests use `#[serial]`
- [ ] Snapshots reviewed: `cargo insta review`
- [ ] No skipped/ignored tests without reason

## CI/CD

Tests run automatically on:
- Every push
- Every pull request
- Linux, macOS, Windows

View results in GitHub Actions.

## Debugging Tests

### Print Debugging

```rust
#[test]
fn test_debug() {
    let value = some_function();
    println!("Debug: {:?}", value);  // Only shows with --nocapture
    assert_eq!(value, expected);
}

// Run with:
cargo test test_debug -- --nocapture
```

### Using `dbg!`

```rust
#[test]
fn test_with_dbg() {
    let value = dbg!(some_function());  // Prints to stderr
    assert_eq!(value, expected);
}
```

### Conditional Compilation

```rust
#[test]
fn test_something() {
    #[cfg(debug_assertions)]
    println!("Debug mode only");

    // Test code
}
```

## Coverage

Generate coverage report:

```bash
# Install tarpaulin
cargo install cargo-tarpaulin

# Run coverage
cargo tarpaulin --out Html --output-dir coverage/

# Open coverage/index.html
```

## Common Testing Patterns

### Parameterized Tests

```rust
#[test]
fn test_multiple_frameworks() {
    for framework in &["oh-my-zsh", "zimfw", "prezto"] {
        let result = validate_framework(framework);
        assert!(result.is_ok(), "Failed for {}", framework);
    }
}
```

### Table-Driven Tests

```rust
#[test]
fn test_profile_name_validation() {
    let test_cases = vec![
        ("valid", true),
        ("also-valid", true),
        ("../invalid", false),
        ("path/invalid", false),
    ];

    for (input, expected) in test_cases {
        let result = validate_profile_name(input).is_ok();
        assert_eq!(result, expected, "Failed for input: {}", input);
    }
}
```

### Setup/Teardown

```rust
#[test]
fn test_with_setup() {
    // Setup
    let temp = TempDir::new().unwrap();
    std::env::set_var("HOME", temp.path());
    create_test_files(&temp);

    // Test
    let result = run_test();
    assert!(result.is_ok());

    // Teardown (automatic with Drop)
}
```

## Further Reading

- [Rust Testing Documentation](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Insta Documentation](https://insta.rs/)
- [Serial Test Documentation](https://docs.rs/serial_test/)
