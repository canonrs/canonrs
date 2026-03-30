# Canon Rule 153 — Layouts Must Be Event-Free During Hydration

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-01-16

**Category:** core-runtime
**Tags:** hydration, layout, ssr, events
**Language:** EN

---

**Intro:**
Attaching DOM event listeners inside layouts during SSR hydration causes non-deterministic runtime failures due to unstable DOM nodes. This leads to panics and inconsistent behavior.

**Problem:**
event listeners in layouts during hydration cause runtime mismatch and failures

**Solution:**
remove all event bindings from layouts and delegate to csr controllers or ui

**Signals:**
- unreachable panic
- callback removed
- hydration failure
- dom instability

**Search Intent:**
why events break during hydration leptos

**Keywords:**
leptos hydration event issue, layout event listener ssr, hydration callback error, ssr event timing problem

---

## Status
**Mandatory**

## Context

During SSR + Hydration, the DOM generated on the server **must be structurally and behaviorally identical** to the DOM expected by the client at hydration time.

Any DOM event listener (`on:click`, `on:input`, etc.) attached inside **Layouts or Shells** during hydration introduces a high risk of runtime failure, including:

- `callback removed before attaching`
- `unreachable` panics during hydrate
- Silent DOM replacement by the runtime

This issue is **not related to Providers, UI components, or business logic**, but to **event attachment timing** during hydration.

## Problem

Layouts (AppLayout, AppShell, PageLayout, etc.) are rendered:

- On the server (SSR)
- Immediately re-walked by the client during hydration

If an event listener exists at this level, the runtime may attempt to attach a callback to a node that:

- Was moved
- Was replaced
- Was optimized away
- Has not yet stabilized in the hydration phase

This causes **non-deterministic hydration failures**.

## Rule

**Layouts and Shells MUST NOT contain DOM event listeners.**

This includes, but is not limited to:

- `on:click`
- `on:input`
- `on:change`
- Any interactive callback bound directly in a Layout or Shell

Layouts are **structural only**.

## Allowed

✅ Structural HTML  
✅ Static content  
✅ Slots (`Children`, `Option<Children>`)  
✅ Pure UI composition  
✅ Providers (context only, no events)  
✅ CSS classes and `data-*` attributes  

## Forbidden

❌ Any `on:*` DOM event in Layouts  
❌ Business logic inside Layouts  
❌ State mutation in Layouts  
❌ Controllers instantiated in Layouts  
❌ Conditional rendering based on reactive state during hydration  

## Correct Architecture

### ❌ Invalid

```rust
#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <Button on:click=toggle_theme>
            "🌓"
        </Button>
    }
}
```

### ✅ Valid

```rust
#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <ThemeToggleSlot />
    }
}
```

```rust
#[cfg(not(feature = "ssr"))]
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let events = use_app_controller();

    view! {
        <Button on:click=move |_| events.toggle_theme()>
            "🌓"
        </Button>
    }
}
```

## Responsibility Split

- **Layout**: structure only (SSR-safe)
- **UI**: rendering only
- **Controller / App layer**: event handling (CSR-only)
- **Provider**: state ownership, no UI

## Rationale

This rule guarantees:

- Deterministic hydration
- Zero runtime panics
- Clear architectural boundaries
- Predictable SSR/CSR behavior

Violating this rule **will eventually cause hydration failure**, even if it appears to work temporarily.

## Enforcement

Any Layout or Shell containing DOM events is considered **architecturally invalid** and must be refactored.

---

**Summary**  
> Layouts define structure.  
> UI renders visuals.  
> Controllers handle behavior.  
> Hydration must be silent.
