# Canon Rule #197: Feature Flags Are Architectural Boundaries

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-03

**Category:** governance
**Tags:** feature-flags, architecture, build, boundaries
**Language:** EN

---

**Intro:**
Using feature flags as conditional shortcuts breaks architectural boundaries and introduces cross-target leakage. Flags must define strict separation.

**Problem:**
feature flags are misused causing cross target code leakage and instability

**Solution:**
treat feature flags as hard architectural boundaries defining code location

**Signals:**
- type mismatch
- duplicate crates
- hydration panic

**Search Intent:**
how to use feature flags as architecture boundaries

**Keywords:**
feature flags architecture rust, ssr csr boundary enforcement, build graph separation, frontend feature flag misuse

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