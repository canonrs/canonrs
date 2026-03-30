# Canon Rule #266: Primitives Must Never Generate IDs

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-02-13

**Category:** component-architecture
**Tags:** primitives, ids, ssr, hydration
**Language:** EN

---

**Intro:**
Auto-generated ids create nondeterministic markup and hydration issues. Identity must be externally controlled.

**Problem:**
primitives generate ids causing mismatch between ssr and csr

**Solution:**
require ids to be passed externally and forbid internal generation

**Signals:**
- hydration warning
- id mismatch
- runtime conflict

**Search Intent:**
why primitives should not generate ids

**Keywords:**
deterministic id generation ssr, leptos hydration id mismatch, frontend component identity, primitive id contract

---

## Principle

**Primitives MUST receive IDs externally and MUST NOT generate, mutate, or auto-derive IDs internally.**

---

## Problem

Without this rule:

- SSR and CSR ID mismatch
- Non-deterministic markup
- Hydration warnings
- Broken behavior attachment

Observable symptoms:

- `format!("button-{}", counter)`
- Static mut counters
- Atomic counters in primitives
- Random UUID generation

Architectural impact:

- Hydration instability
- Cross-instance conflicts
- Hard-to-debug runtime bugs

---

## Forbidden Pattern

### Forbidden

```rust
let id = format!("btn-{}", COUNTER.fetch_add(1, Ordering::SeqCst));
```

Why this violates:

ID generation is side-effectful.
Primitive becomes stateful.

---

## Canonical Pattern

### Canonical

```rust
#[prop(optional)] id: Option<String>
```

Interactive:

```rust
let table_id = "users-table".to_string();
```

Why this complies:

- Deterministic render
- Identity controlled at higher layer
- SSR consistent with CSR

---

## Rationale

Protects:

- Hydration determinism
- Identity contract integrity
- Behavior registry reliability

ID is architectural identity.
It cannot be implicit.

---

## Enforcement

- Static grep for `format!(".*{}")` patterns in primitives
- Reject use of atomic counters
- Reject UUID generation
- Review checklist requires ID prop presence

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version