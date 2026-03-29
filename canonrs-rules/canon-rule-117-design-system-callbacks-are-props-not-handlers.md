# Canon Rule #117: Design System Callbacks Are Props, Not Handlers

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** components, design-system
**Version:** 1.0.0
**Date:** 2026-01-17

---

## Principle

**UI components receive callbacks as props. They DO NOT create handlers inline.**

Every `on:*=move |_| { }` creates an externref.
Design systems with hundreds of components = thousands of refs = runtime crash.

---

## The Problem

Current pattern (FORBIDDEN):
```rust
#[component]
pub fn Button(on_click: Callback<()>) -> impl IntoView {
    view! {
        <button on:click=move |_| on_click.run(())>  // ❌ externref!
    }
}
```

Each Button instance = 1 externref
100 buttons = 100 refs = Table.grow() crash

---

## Canonical Pattern (REQUIRED)
```rust
#[component]
pub fn Button(
    on_click: Callback<()>,
    #[prop(into)] id: String,
) -> impl IntoView {
    view! {
        <button data-action="click" data-action-id=id>  // ✅ zero externref
    }
}
```

Shell handles clicks:
```javascript
document.addEventListener("click", e => {
    const btn = e.target.closest("[data-action='click']");
    if (!btn) return;
    window.dispatchEvent(new CustomEvent("leptos:action", {
        detail: { id: btn.dataset.actionId }
    }));
});
```

---

## Enforcement

UI Components MUST:
- Accept callbacks as props
- Render data-* attributes
- Never use `on:*=move`

UI Components MUST NOT:
- Create inline handlers
- Call `Callback::new` in render
- Use closures with captured state

---

## Related Canon Rules

- Canon Rule #116 — WASM Externref Table Limits
- Canon Rule #102 — Runtime JS Is Shell Infrastructure

---
