# Canon Rule #214: lib.rs Owns Application Structure and UI Semantics

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** architecture, ssr, csr
**Version:** 1.0.0  
**Date:** 2026-02-03

---

## Principle

**`lib.rs` is the sole owner of application structure, UI composition, routing, and semantic intent.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

When `lib.rs` is treated as a secondary or optional file:

- UI logic leaks into `main.rs`
- SSR and CSR responsibilities blur
- Meta configuration becomes inconsistent
- Hydration and routing behavior diverge
- The application loses a single semantic root

This leads to architectural drift and non-deterministic rendering behavior.

---

## Forbidden Pattern

### Forbidden

```rust
// main.rs
Router::new()
    .leptos_routes(&opts, routes, || view! { <App/> })
```

UI structure is being constructed outside `lib.rs`.

---

## Canonical Pattern

### Canonical

```rust
// lib.rs
#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Router>
            <Routes>...</Routes>
        </Router>
    }
}
```

`lib.rs` defines what the application **is**, not how it is served.

---

## Rationale

This rule enforces a single semantic root.

- UI composition must be centralized
- SSR and CSR must share the same semantic tree
- `lib.rs` is the canonical description of the app
- Prevents duplication and divergence

This is an architectural invariant, not a stylistic choice.

---

## Enforcement

- Code review: UI code in `main.rs` is forbidden
- CI lint: forbid `view! { <App/> }` outside `lib.rs`
- Structural audits

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
