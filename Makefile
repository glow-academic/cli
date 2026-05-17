# Glow CLI — Development Makefile
#
# Quick reference:
#   make                    → build debug
#   make run                → start CLI dev server on :9000
#   make exec health        → cargo run -- health (with debug logs)
#   make exec personas list → cargo run -- personas list
#   make test               → run all tests
#   make check              → fmt + clippy + test (same as CI)
#   make release            → optimized build
#   make install            → install to ~/.cargo/bin

.PHONY: build run exec test test-unit test-integration fmt clippy check release install clean watch coverage help sync-types

# Grab everything after the first word (the make target) as CLI args
RUN_ARGS := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))

# Default target
build:
	cargo build

# Start the CLI dev server (serves CLI spec + exec endpoint)
run:
	cargo run -- serve --port 9000

# Run a CLI command directly: make exec health | make exec personas list
exec:
	RUST_LOG=debug cargo run -- $(or $(RUN_ARGS),--help)

# Run all tests
test:
	cargo test

# Run only unit tests (inline #[cfg(test)] modules)
test-unit:
	cargo test --lib

# Run only integration tests (tests/ directory)
test-integration:
	cargo test --test cli_integration

# Auto-format code
fmt:
	cargo fmt

# Lint
clippy:
	cargo clippy -- -D warnings

# Run everything CI runs (format check + lint + test)
check: fmt clippy test

# Optimized release build
release:
	cargo build --release
	@echo "\nBinaries:"
	@ls -lh target/release/glow target/release/glw

# Install both binaries to ~/.cargo/bin
install:
	cargo install --path .

# ── Type Generation ────────────────────────────────────────
# Regenerate Rust types from live API servers.
# Requires both servers running: learnloop-api (8100) and glow-api (8000).
ADMIN_API_URL ?= http://localhost:8100
GLOW_API_URL  ?= http://localhost:8000
PYTHON        ?= python3.11

sync-types: ## Fetch OpenAPI specs and regenerate Rust types from both servers
	@echo "Generating admin types from $(ADMIN_API_URL)..."
	@$(PYTHON) scripts/generate-rust-types.py --url $(ADMIN_API_URL) --output src/admin/api/latest.rs
	@echo "Generating glow types from $(GLOW_API_URL)..."
	@$(PYTHON) scripts/generate-rust-types.py --url $(GLOW_API_URL) --output src/glow/api/latest.rs
	@echo "{\"platform-api\":{\"version\":\"$$(curl -sf $(ADMIN_API_URL)/ | jq -r .version)\",\"synced_at\":\"$$(date -u +%%Y-%%m-%%dT%%H:%%M:%%SZ)\"},\"glow-api\":{\"version\":\"$$(curl -sf $(GLOW_API_URL)/ | jq -r .version)\",\"synced_at\":\"$$(date -u +%%Y-%%m-%%dT%%H:%%M:%%SZ)\"}}" | jq . > api-versions.json
	@echo "✅ Types updated. Run 'cargo build' to check for breaking changes."

sync-types-admin: ## Regenerate admin types only
	@$(PYTHON) scripts/generate-rust-types.py --url $(ADMIN_API_URL) --output src/admin/api/latest.rs

sync-types-glow: ## Regenerate glow types only
	@$(PYTHON) scripts/generate-rust-types.py --url $(GLOW_API_URL) --output src/glow/api/latest.rs

# Remove build artifacts
clean:
	cargo clean

# Auto-rebuild on file changes (requires: cargo install cargo-watch)
watch:
	cargo watch -x run

# Test coverage (requires: cargo install cargo-llvm-cov && rustup component add llvm-tools-preview)
coverage:
	cargo llvm-cov --open

coverage-ci:
	cargo llvm-cov --lcov --output-path lcov.info

help:
	@echo "Available targets:"
	@echo "  build            Build debug binary"
	@echo "  run <args>       Build + run (e.g. make run health, make run personas list)"
	@echo "  test             Run all tests"
	@echo "  test-unit        Run unit tests only"
	@echo "  test-integration Run integration tests only"
	@echo "  fmt              Auto-format code"
	@echo "  clippy           Run linter"
	@echo "  check            fmt + clippy + test (mirrors CI)"
	@echo "  release          Optimized release build"
	@echo "  install          Install to ~/.cargo/bin"
	@echo "  clean            Remove build artifacts"
	@echo "  watch            Auto-rebuild on file changes (needs cargo-watch)"
	@echo "  coverage         Run tests with coverage report (needs cargo-llvm-cov)"

# Catch trailing args from "make run ..." so Make doesn't error on them
%:
	@:
