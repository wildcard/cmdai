.PHONY: help build test clean fmt lint audit check bench install

# Default target
help:
	@echo "Available targets:"
	@echo "  build     - Build the project in debug mode"
	@echo "  release   - Build optimized release binary"
	@echo "  test      - Run all tests"
	@echo "  clean     - Clean build artifacts"
	@echo "  fmt       - Format code with rustfmt"
	@echo "  lint      - Run clippy lints"
	@echo "  audit     - Run security audit"
	@echo "  check     - Run all quality checks (fmt, lint, audit, test)"
	@echo "  bench     - Run benchmarks"
	@echo "  install   - Install cmdai locally"

# Build commands
build:
	cargo build

release:
	cargo build --release

# Test commands
test:
	cargo test --verbose

test-contract:
	cargo test --test "*contract*" --verbose

test-integration:
	cargo test --test "*integration*" --verbose

test-property:
	cargo test --test "*property*" --verbose

# Code quality
fmt:
	cargo fmt --all

fmt-check:
	cargo fmt --all -- --check

lint:
	cargo clippy --all-targets --all-features -- -D warnings

audit:
	cargo audit

# Combined quality check
check: fmt-check lint audit test

# Performance
bench:
	cargo bench

# Binary size check (must be < 50MB)
size-check: release
	@SIZE=$$(stat -f%z target/release/cmdai 2>/dev/null || stat -c%s target/release/cmdai); \
	echo "Binary size: $$SIZE bytes"; \
	if [ $$SIZE -gt 52428800 ]; then \
		echo "❌ Binary size ($$SIZE bytes) exceeds 50MB limit"; \
		exit 1; \
	else \
		echo "✅ Binary size ($$SIZE bytes) is within 50MB limit"; \
	fi

# Utility commands
clean:
	cargo clean

install:
	cargo install --path .

# Development setup
setup:
	rustup component add rustfmt clippy
	cargo install cargo-audit cargo-deny cargo-llvm-cov

# Documentation
doc:
	cargo doc --no-deps --open

# Run with debug logging
run-debug:
	RUST_LOG=debug cargo run --

# Profile release build
profile: release
	@echo "Release binary info:"
	@ls -la target/release/cmdai
	@echo ""
	@echo "Dependencies:"
	@ldd target/release/cmdai 2>/dev/null || otool -L target/release/cmdai 2>/dev/null || echo "Dependencies check not available"