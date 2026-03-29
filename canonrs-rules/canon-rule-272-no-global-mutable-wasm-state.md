# Canon Rule #272: Global Mutable State Is Forbidden in wasm Scope

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** behavior
**Version:** 1.0.0  
**Date:** 2026-02-13

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
