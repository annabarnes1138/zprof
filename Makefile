# Makefile for zprof development

.PHONY: help build run test test-unit test-integration clean lint fix fmt check install dev watch release

# Default target
help:
	@echo "zprof Development Commands"
	@echo "=========================="
	@echo ""
	@echo "Building:"
	@echo "  make build           Build the project in debug mode"
	@echo "  make release         Build optimized release binary"
	@echo "  make install         Install zprof to ~/.cargo/bin"
	@echo ""
	@echo "Running:"
	@echo "  make run             Run zprof (pass args with ARGS='...')"
	@echo "  make dev             Build and run in dev mode"
	@echo "  make watch           Watch files and rebuild on changes"
	@echo ""
	@echo "Testing:"
	@echo "  make test            Run all tests"
	@echo "  make test-unit       Run unit tests only"
	@echo "  make test-integration Run integration tests only"
	@echo "  make test-watch      Run tests in watch mode"
	@echo ""
	@echo "Code Quality:"
	@echo "  make lint            Run clippy linter"
	@echo "  make fmt             Format code with rustfmt"
	@echo "  make check           Run all checks (fmt, lint, test)"
	@echo "  make fix             Auto-fix issues (fmt + clippy --fix)"
	@echo ""
	@echo "Cleanup:"
	@echo "  make clean           Remove build artifacts"
	@echo ""
	@echo "Examples:"
	@echo "  make run ARGS='init'"
	@echo "  make run ARGS='list'"
	@echo "  make run ARGS='create myprofile'"

# Build commands
build:
	cargo build

release:
	cargo build --release

install:
	cargo install --path .

# Run commands
run:
	cargo run -- $(ARGS)

dev: build
	./target/debug/zprof $(ARGS)

watch:
	cargo watch -x build

# Testing commands
test:
	cargo test

test-unit:
	cargo test --lib

test-integration:
	cargo test --test '*'

test-watch:
	cargo watch -x test

# Code quality
lint:
	cargo clippy --all-targets --all-features -- -D warnings

fmt:
	cargo fmt

check: fmt lint test
	@echo "✅ All checks passed!"

fix:
	cargo fmt
	cargo clippy --fix --allow-dirty --allow-staged

# Cleanup
clean:
	cargo clean

# Snapshot testing
snapshots-review:
	cargo insta review

snapshots-accept:
	cargo insta accept

# Documentation
docs:
	cargo doc --open

# Coverage (requires tarpaulin: cargo install cargo-tarpaulin)
coverage:
	cargo tarpaulin --out Html --output-dir coverage

# Benchmark (if you add benchmarks later)
bench:
	cargo bench
