# Canon Rule #139: Slots Are UI-Level Only

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2025-01-23

**Category:** component-architecture
**Tags:** slots, ui, layout, ownership
**Language:** EN

---

**Intro:**
Using slots in layouts introduces ownership complexity and breaks SSR safety due to Fn and lifetime boundaries. This leads to brittle composition and logic-heavy layouts.

**Problem:**
slots are defined in layouts causing ownership and composition issues

**Solution:**
restrict slots to ui components and keep layouts purely compositional

**Signals:**
- ownership explosion
- fnonce issues
- layout complexity

**Search Intent:**
how to use slots correctly in ui components

**Keywords:**
ui slots pattern, leptos slot ownership, layout vs ui slots, component slot architecture

---

## Principle

**Component slots MUST be defined and consumed exclusively at the UI component level, never in Layouts or Shells.**

---

## The Problem

Slots introduce:

- Typed child ownership
- Complex lifetime semantics
- Fn/FnOnce boundaries
- Render-time branching

When used in Layouts:

- Ownership explodes across layers
- SSR execution becomes unsafe
- Layouts become logic-heavy
- Composition becomes brittle

This directly contradicts Canon layout philosophy.

---

## Forbidden Patterns

### ❌ Forbidden
```rust
#[component]
pub fn DashboardLayout(
    sidebar: SidebarSlot,
    header: HeaderSlot,
) -> impl IntoView {
    view! { ... }
}
```

Layouts MUST NOT define slots.

---

## Canonical Pattern

### ✅ Canonical
```rust
// UI-level component
#[slot]
pub struct HeaderActions {
    children: ChildrenFn,
}

#[component]
pub fn Header(actions: HeaderActions) -> impl IntoView {
    view! {
        <header>
            {(actions.children)()}
        </header>
    }
}
```

```rust
// Layout uses composition only
#[component]
pub fn DashboardLayout(children: Children) -> impl IntoView {
    view! {
        <div data-layout="dashboard">
            {children()}
        </div>
    }
}
```

---

## Rationale

Layouts define **where things go**, not **what they are**.

Slots define **what goes inside a component**.

Mixing the two:
- Violates separation of concerns
- Breaks ownership guarantees
- Makes layouts untestable
- Prevents reuse across apps

---

## Enforcement

- CI: forbid `#[slot]` usage in layouts
- Static scan: slot structs under `/layouts`
- Code review validation

---

## Exceptions

No exceptions. This rule is absolute.

---

## Version History

- **1.0.0** — Initial canonical version