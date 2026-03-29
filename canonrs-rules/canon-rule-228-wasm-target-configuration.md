# Canon Rule #228: Workspace Must Define WASM Target Configuration

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** wasm, workspace, build
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**Workspace MUST have `.cargo/config.toml` configuring `wasm32-unknown-unknown` target with optimization flags.**

---

## Problem

Without WASM target configuration:
- `cargo build --target wasm32-unknown-unknown` fails with "target not found"
- WASM builds use default optimization (bloated output)
- Inconsistent builds between developers
- CI fails on WASM compilation

**Observable symptoms**:
```bash
cargo build --target wasm32-unknown-unknown
error: failed to run custom build command
target `wasm32-unknown-unknown` not found
```

---

## Forbidden Pattern

### ❌ Forbidden
```
# No .cargo/config.toml exists
```

**Why this violates**: Cargo doesn't know how to build for WASM target.

---

## Canonical Pattern

### ✅ Canonical
```toml
# .cargo/config.toml (workspace root)
[build]
target-dir = "target"

[target.wasm32-unknown-unknown]
rustflags = [
    "-C", "opt-level=z",
    "-C", "lto=fat",
    "-C", "codegen-units=1",
]
```

**Why this complies**: 
- Defines WASM target explicitly
- Optimizes for size (`opt-level=z`)
- Enables LTO for smaller binaries
- Single codegen unit for maximum optimization

---

## Rationale

### Architectural invariants
1. **Target availability**: WASM must be buildable without manual setup
2. **Size optimization**: Browser WASM needs aggressive size reduction
3. **Build consistency**: All developers use same optimization flags

### Bugs prevented
- "Target not found" build failures
- Bloated WASM (10MB+ vs 500KB optimized)
- Inconsistent performance between dev/prod
- CI/CD failures on WASM builds

### Why not opinion
WASM target doesn't exist by default in Cargo. Configuration is mandatory, not optional.

---

## Enforcement

### CI validation
```bash
#!/bin/bash
# check-wasm-target.sh

if [[ ! -f .cargo/config.toml ]]; then
    echo "❌ .cargo/config.toml not found"
    exit 1
fi

if ! grep -q "wasm32-unknown-unknown" .cargo/config.toml; then
    echo "❌ WASM target not configured"
    exit 1
fi

# Verify target actually works
cargo build --target wasm32-unknown-unknown --lib -p canonrs-site --no-default-features --features=hydrate || {
    echo "❌ WASM build failed"
    exit 1
}

echo "✅ WASM target configured"
```

### Required setup
```bash
# Install target (one-time per machine)
rustup target add wasm32-unknown-unknown

# Verify
rustup target list --installed | grep wasm32
```

---

## Exceptions

**No exceptions. This rule is absolute.**

All Leptos workspaces MUST configure WASM target. Without it, hydrate builds fail.

---

## Version History

- **1.0.0** (2026-02-03) — Initial version
