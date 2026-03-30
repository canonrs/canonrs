# Canon Rule #272: Global Mutable State Is Forbidden in wasm Scope

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13

**Category:** state-reactivity
**Tags:** state, wasm, global, mutability
**Language:** EN

---

**Intro:**
Global mutable state in wasm introduces race conditions and hidden coupling. State must be localized and reactive.

**Problem:**
global mutable state exists causing race conditions and coupling

**Solution:**
store all state in reactive signals or scoped structures

**Signals:**
- race condition
- memory leak
- state conflict

**Search Intent:**
why global state is bad wasm

**Keywords:**
wasm global mutable state issue, reactive state rust leptos, avoid static mut rust wasm, frontend state isolation

---

## Principle

**No global mutable state may exist in wasm runtime.**

---

## Problem

Global state causes:

- Race conditions
- Hidden coupling
- Memory leaks

---

## Forbidden Pattern

```rust
static mut GLOBAL_COUNTER: i32 = 0;
```

---

## Canonical Pattern

State lives inside ComponentState or Interactive signals.

---

## Rationale

Stateless wasm bridge.

---

## Enforcement

Clippy + code review.

---

## Exceptions

None.

---

## Version History

- 1.0.0 — Initial version