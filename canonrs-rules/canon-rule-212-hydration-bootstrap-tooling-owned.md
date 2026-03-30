# Canon Rule #212: Hydration Bootstrap Is Tooling-Owned

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-03

**Category:** core-runtime
**Tags:** hydration, wasm, bootstrap, tooling
**Language:** EN

---

**Intro:**
Manual wasm bootstrap conflicts with tooling lifecycle and causes duplicated initialization. Ownership must remain in tooling.

**Problem:**
application controls wasm bootstrap causing lifecycle conflicts

**Solution:**
delegate bootstrap to tooling and expose only entrypoint

**Signals:**
- double init
- unexpected behavior
- bootstrap conflict

**Search Intent:**
how to fix wasm bootstrap conflict leptos

**Keywords:**
wasm bootstrap leptos, hydration lifecycle control, frontend wasm startup pattern, tooling owned bootstrap

---

## Principle

**The WASM hydration bootstrap MUST be owned by the build tooling, not the application.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Manual bootstrap:

- Conflicts with `cargo-leptos`
- Breaks tool-controlled lifecycle
- Causes double initialization
- Produces non-deterministic behavior

---

## Forbidden Pattern

### Forbidden

```rust
#[wasm_bindgen(start)]
fn run() {
    hydrate();
}
```

Application hijacks the bootstrap phase.

---

## Canonical Pattern

### Canonical

```rust
#[cfg(feature = "hydrate")]
fn main() {
    app::hydrate();
}
```

Tooling controls startup; app exposes entrypoint only.

---

## Rationale

Bootstrap is infrastructure responsibility.

- Tools orchestrate WASM lifecycle
- Apps must remain passive
- Prevents duplicated initialization
- Enforces clear ownership

---

## Enforcement

- Static analysis for `wasm_bindgen(start)`
- CI failure on forbidden pattern
- Mandatory code review check

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version