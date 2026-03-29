# Canon Rule #197: Feature Flags Are Architectural Boundaries

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** architecture, build
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**Feature flags define hard architectural boundaries, not conditional shortcuts.**

A feature flag decides **where code is allowed to exist**, not merely if it compiles.

---

## Problem

Misusing feature flags causes:

- SSR code compiled into WASM
- CSR code linked into server binaries
- Duplicate runtime crates
- Non-deterministic build graphs

Observed failures:
- `axum::Body` type mismatches
- Multiple `axum-core` versions
- Hydration panics
- WASM bloat

---

## Canonical Feature Roles

```
ssr     → server rendering, adapters, HTML
hydrate → client behaviors, DOM, WASM
none    → pure shared contracts only
```

---

## Forbidden Patterns

❌ `cfg(any(feature = "ssr", feature = "hydrate"))`  
❌ Runtime logic inside shared crates  
❌ Feature flags used to “hide” architecture violations  

---

## Canonical Pattern

```rust
#[cfg(feature = "ssr")]
pub mod ui;

#[cfg(feature = "hydrate")]
pub mod behaviors;

pub mod shared;
```

---

## Enforcement

- Every `#[cfg]` maps to a runtime boundary
- CI validates dependency graph per feature
- Feature misuse is a build failure

---

## Exceptions

**None. Feature flags are architectural contracts.**

---
