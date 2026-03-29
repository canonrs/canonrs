# Canon Rule #140: Layouts Are Structural, Not Behavioral

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** layout, leptos, architecture
**Version:** 1.0.0
**Date:** 2025-01-23

---

## Principle

**Layouts MUST define structure and zones only and MUST NOT contain behavior, state, or logic.**

---

## The Problem

When layouts contain behavior:

- State leaks across routes
- Ownership rules are violated
- SSR execution becomes non-deterministic
- Layouts become tightly coupled to app logic
- Reuse across products becomes impossible

This manifested as:
- Signals inside layouts
- Providers with implicit logic
- Event handlers embedded in layout components
- Conditional rendering based on business state

---

## Forbidden Patterns

### ❌ Forbidden
```rust
#[component]
pub fn DashboardLayout(children: Children) -> impl IntoView {
    let open = create_rw_signal(true); // ❌ state in layout

    view! {
        <div>
            <button on:click=move |_| open.update(|v| *v = !*v) />
            {children()}
        </div>
    }
}
```

### ❌ Forbidden
```rust
#[component]
pub fn LayoutWithLogic(children: Children) -> impl IntoView {
    <Show when=some_condition>
        {children()}
    </Show>
}
```

---

## Canonical Pattern

### ✅ Canonical
```rust
#[component]
pub fn DashboardLayout(children: Children) -> impl IntoView {
    view! {
        <div data-layout="dashboard" class="layout-dashboard">
            {children()}
        </div>
    }
}
```

Behavior and state MUST live in:
- Controllers (CSR-only)
- Application composition layer
- Dedicated UI components

---

## Rationale

Layouts are **spatial contracts**, not interactive entities.

Their role is to:
- Define zones
- Apply CSS structure
- Provide semantic wrappers

By keeping layouts behavior-free, we guarantee:
- SSR safety
- Ownership simplicity
- Compositional freedom
- Predictable rendering

---

## Enforcement

- Static analysis: forbid signals in layouts
- CI: block event handlers in `/layouts`
- Code review checklist

---

## Exceptions

No exceptions. This rule is absolute.

---

## Version History

- **1.0.0** — Initial canonical version
