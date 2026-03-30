# Canon Rule #142 — Children Must Always Be Optional in Public Components

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-01-16

**Category:** component-architecture
**Tags:** components, api, children, ui
**Language:** EN

---

**Intro:**
Requiring children as mandatory props creates brittle APIs and breaks composability. Components must support absence of children without structural failure.

**Problem:**
mandatory children props create rigid and breaking component apis

**Solution:**
declare children as optional and safely render when present

**Signals:**
- component requires children
- unwrap children panic
- composition break

**Search Intent:**
how to make children optional leptos

**Keywords:**
leptos optional children prop, component api flexibility, children option pattern ui, safe children rendering leptos

---

## Context
Public UI components frequently evolve. Requiring `children` as a mandatory prop creates brittle APIs, breaks composability, and forces artificial wrappers.

In CanonRS, **absence of children must never break a component**.

## Rule
All public components (UI, Components, Blocks) **MUST declare `children` as optional**.

## Mandatory Pattern
```rust
#[component]
pub fn Component(
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    view! {
        <div>
            {children.map(|c| c())}
        </div>
    }
}
```

## Forbidden Patterns
- `children: Children` without `#[prop(optional)]`
- Calling `{children()}` without checking `Option`
- Enforcing structural children in UI-level components

## Rationale
- Prevents breaking changes
- Enables progressive composition
- Preserves backward compatibility
- Eliminates artificial layout wrappers

## Scope
- UI
- Components
- Blocks

## Exception
None.

Children are optional by design.