# Canon Rule #225 (v2.0): Workspace Dependency Version Policy Must Prevent ABI Drift Without Blocking Patch Compatibility

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** workspace, build
**Version:** 2.0.0  
**Date:** 2026-02-04  

---

## Principle

**Workspace-level runtime dependencies MUST prevent multiple-version ABI conflicts while preserving controlled patch compatibility.**

The objective is:
- Zero duplicate critical crates in the dependency graph
- Deterministic builds
- No accidental semver drift
- No artificial ecosystem lock

---

## Problem

Using wide semver ranges:

```toml
axum = "0.8"
leptos = "0.8"
```

allows Cargo to resolve different patch versions across the graph:

```
axum 0.8.7
axum 0.8.9
```

This causes:

- Type mismatches across crate boundaries
- Duplicate symbol linkage
- Subtle ABI conflicts
- Non-deterministic CI vs local builds

But over-constraining with `=X.Y.Z` everywhere may:

- Break compatibility with upstream ecosystem crates
- Cause unnecessary resolution conflicts
- Increase maintenance burden

---

## Canonical Version Strategy

### Critical Runtime Crates

```toml
[workspace.dependencies]

# Server stack
axum = "0.8.8"
tower = "0.5.3"
tokio = { version = "1.42.0", features = ["full"] }

# Leptos ecosystem
leptos = "0.8.15"
leptos_meta = "0.8.5"
leptos_router = "0.8.11"
leptos_axum = "0.8.7"

# WASM
wasm-bindgen = "0.2.95"
```

✔ Minor version stability  
✔ Patch updates allowed  
✔ Compatible with ecosystem constraints  
✔ Prevents accidental major drift  

---

### Lockfile Is The Real Freeze Point

Determinism is enforced by:

```
Cargo.lock (committed)
```

Not by forcing `=X.Y.Z` everywhere.

Build reproducibility depends on:
- Committed lockfile
- CI `cargo fetch --locked`
- `cargo build --locked`

---

### Duplicate Version Detection

Multiple versions of critical crates are forbidden.

Verification:

```bash
cargo tree --workspace -d | grep -E "axum|leptos|tokio"
```

Must produce zero duplicate entries.

Enforced via:

```toml
# deny.toml
[bans]
multiple-versions = "deny"

[[bans.deny]]
name = "axum"

[[bans.deny]]
name = "leptos"
```

---

## Forbidden Pattern

### Forbidden

```toml
axum = "0.8"
leptos = "0.8"
tokio = "1"
```

Too permissive.

---

### Forbidden

```toml
axum = "=0.8.8"
```

Blocks compatible ecosystem patch updates unnecessarily.

---

## Rationale

Rust uses nominative typing:

```
axum::Body (0.8.7) ≠ axum::Body (0.8.9)
```

Version drift at patch level can still produce separate crate instantiations.

The correct balance:

- Minor stability for API surface
- Patch flexibility
- Graph duplication prohibition

---

## Enforcement

### CI

```bash
cargo fetch --locked
cargo build --locked

# No duplicates
cargo tree --workspace -d && exit 1
```

### Pre-commit

```bash
cargo update -p axum --dry-run
```

Must not upgrade across minor without architectural review.

---

## Exceptions

Allowed:
- Dev-dependencies
- Build-dependencies
- Proc-macro crates
- Crates proven ABI-isolated

Forbidden:
- Multiple versions of any runtime crate in final artifact
- Unbounded semver ranges

---

## Summary

This rule enforces:

- Graph purity
- ABI safety
- Ecosystem compatibility
- Deterministic builds

Without over-constraining legitimate patch evolution.

---

Version 2.0 replaces strict `=X.Y.Z` pinning with **controlled minor pinning + lockfile enforcement + duplicate banning**.

