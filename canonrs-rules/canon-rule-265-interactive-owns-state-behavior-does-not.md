# Canon Rule #265: Interactive Owns All State — Behavior Owns None

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** interactive, behavior, state
**Version:** 1.0.0
**Date:** 2026-02-13

---

## Principle

**All feature state MUST live inside Interactive. Behavior MUST NOT maintain internal mutable state beyond ephemeral runtime tracking.**

---

## Problem

Without this rule:

- Behavior holds hidden state
- State diverges between SSR and CSR
- Debugging becomes impossible
- Hydration mismatches appear

Observable symptoms:

- `Rc<RefCell<...>>` storing business logic
- Sorting state in behavior
- Pagination index stored in behavior
- Filter strings stored in DOM only

Architectural impact:

- Split source of truth
- CSR-only correctness
- SSR inconsistency

---

## Forbidden Pattern

### ❌ Forbidden

```rust
struct DataTableState {
    current_page: usize,
    filter_query: String,
}
```

or

```rust
let state = Rc<RefCell<MyComplexState>>::new(...);
```

Why this violates:

Behavior becomes orchestrator.
It duplicates Interactive responsibilities.

---

## Canonical Pattern

### ✅ Canonical

```rust
// Interactive owns state
pub struct DataTableState {
    pub current_page: RwSignal<usize>,
    pub filter_query: RwSignal<String>,
}

// Behavior only dispatches
container.dispatch_event(&event);
```

Why this complies:

- Single source of truth
- State is reactive
- SSR-safe
- Behavior purely runtime bridge

---

## Rationale

Protects:

- State determinism
- SSR/CSR symmetry
- Debug visibility
- Predictable architecture layering

State ownership is not optional.
It defines the CanonRS architecture.

---

## Enforcement

- Reject any complex state structs inside behavior
- Reject `Rc<RefCell>` for business logic
- Allow only ephemeral flags (`is_resizing`, etc.)
- Mandatory state presence inside Interactive for features

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
