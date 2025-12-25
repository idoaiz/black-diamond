SHELL := /bin/bash

.PHONY: check install-hooks

check:
	@echo "Running CI checks..."
	@cargo fmt --all -- --check
	@cargo clippy --all-targets --all-features -- -D warnings
	@cargo test --workspace --verbose
	@cargo build --release

install-hooks:
	@echo "Installing pre-commit hooks (requires python & pip)."
	@pip install --user pre-commit || true
	@pre-commit install || true
	@echo "Pre-commit hooks installed."
