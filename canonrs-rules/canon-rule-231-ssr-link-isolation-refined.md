# Canon Rule #231 (Refined): SSR Crates Must Be Link-Time Isolated From WASM Graph

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** interactive, wasm, build
**Version:** 1.1.0  
**Date:** 2026-02-04  

---

## Principle

**No server-side crate or server-only dependency may appear in the WASM target dependency graph — even transitively.**

This must be enforced at:

- Cargo resolution phase
- Feature resolution phase
- Link graph phase

Not merely at `cfg` usage level.

---

## Clarification: Why cfg Alone Is Insufficient

Incorrect assumption:

```rust
#[cfg(feature = "ssr")]
use axum::Router;
```

This removes code usage during compilation,
but DOES NOT guarantee dependency isolation if:

- Feature is enabled in the crate
- Dependency is not properly feature-gated
- Bin separation is not enforced

Cargo resolves dependencies before dead-code elimination.

---

## Real Root Cause of Leakage

Leakage occurs when:

1. Single crate builds both SSR and CSR
2. Optional deps are enabled globally
3. Features are not bound to specific bins
4. Library crate exposes SSR deps transitively

Example violation:

```toml
[dependencies]
axum = { optional = true }

[features]
ssr = ["axum"]
hydrate = []
```

If bin does not require-features isolation,
resolver may include `axum` in graph.

---

## Canonical Isolation Model

### Separate Binaries

```toml
[[bin]]
name = "app-ssr"
path = "src/bin/ssr.rs"
required-features = ["ssr"]

[[bin]]
name = "app-csr"
path = "src/bin/csr.rs"
required-features = ["hydrate"]
```

---

### Feature Scoped Dependencies

```toml
[dependencies]
leptos = { version = "0.8", default-features = false }

axum = { version = "0.8", optional = true }
tower = { version = "0.5", optional = true }

[features]
ssr = ["leptos/ssr", "axum", "tower"]
hydrate = ["leptos/hydrate"]
```

---

### Hard Compile Guard

In SSR-only crate:

```rust
#[cfg(target_arch = "wasm32")]
compile_error!("❌ SSR crate compiled for WASM target");
```

---

## Graph Verification

```bash
cargo tree --target wasm32-unknown-unknown -p app-csr \
  | grep -E "axum|tower|tokio"
```

Must return nothing.

This validates link-time purity.

---

## Architectural Invariants

- SSR and WASM artifacts must have disjoint link graphs
- No server runtime symbols in browser binary
- Feature resolution must not cross product boundaries
- Library crates must not leak server deps transitively

---

## Enforcement in CI

```bash
# WASM graph purity check
if cargo tree --target wasm32-unknown-unknown -p app-csr \
   | grep -E "axum|tower|tokio"; then
    echo "❌ SSR dependency leaked into WASM graph"
    exit 1
fi
```

---

## Forbidden Patterns

### ❌ Single-bin architecture
### ❌ Global feature flags
### ❌ SSR deps without optional gating
### ❌ Products exposing server crates via shared libs

---

## Rationale

Rust linking is graph-based.

Even if `cfg` removes code, Cargo:

1. Resolves dependency graph
2. Determines features
3. Links reachable artifacts

Graph isolation is stronger than conditional compilation.

---

## Exceptions

None.

If code must be shared:

- Extract shared logic into feature-neutral crate
- Keep server adapter layer separate
- WASM crate must not depend on server adapter crate

---

## Summary

This rule enforces:

- Link-graph integrity
- WASM artifact purity
- Feature boundary enforcement
- Zero SSR contamination

Version 1.1 clarifies that leakage is caused by
feature resolution and bin architecture,
not merely by `cfg` usage.

