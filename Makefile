.PHONY: lint
lint:
	cargo fmt --all
	cargo clippy --workspace --all-targets

