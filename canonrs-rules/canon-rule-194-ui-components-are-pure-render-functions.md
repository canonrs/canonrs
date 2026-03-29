# Canon Rule #194: UI Components Are Pure Render Functions

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** ui, architecture
**Version:** 1.0.0  
**Date:** 2026-01-30

---

## Principle

**UI components MUST be pure render functions with no side effects or behavior.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Without this rule:

- UI starts executing logic
- Rendering becomes stateful
- Hydration mismatches occur
- Behavior becomes untraceable

Observable failures:

- Effects inside UI components
- DOM access during render
- Conditional rendering based on runtime state

Architectural impact:

- Broken SSR guarantees
- Unpredictable hydration
- Tight coupling between view and logic

---

## Forbidden Pattern

### ❌ Forbidden

```rust
#[component]
pub fn Button() {
    Effect::new(|| { ... }); // ❌ side effect
}
```

```rust
let el = document.get_element_by_id(...); // ❌ DOM access
```

UI must never *do* anything.

---

## Canonical Pattern

### ✅ Canonical

```rust
#[component]
pub fn Button() -> impl IntoView {
    view! {
        <button data-button />
    }
}
```

Behavior is attached externally via behaviors.

---

## Rationale

This rule ensures:

- Deterministic rendering
- Stable SSR/CSR output
- Maximum reusability
- Clear mental model

UI components describe structure — nothing else.

---

## Enforcement

- Review: no effects, no DOM, no logic in `/ui`
- CI: forbid `window`, `document`, `Effect` in UI
- UI props must be serializable and declarative

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
