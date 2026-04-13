# CanonRS — Makefile
# Entrada única: make dev

.PHONY: dev release tokens

dev:
	cargo run -p canonrs-orchestrator

release:
	CANON_RELEASE=1 cargo run -p canonrs-orchestrator

tokens:
	cargo run --bin tokens-engine --manifest-path canonrs-tokens/Cargo.toml
