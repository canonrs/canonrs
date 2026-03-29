# Canon Rule #132: Layout Composition Over Abstraction

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** layout, leptos
**Version:** 1.0.0
**Date:** 2025-01-23

---

## Principle

**Layouts MUST be structural wrappers and MUST NOT orchestrate composition logic or slot abstraction.**

---

## The Problem

When layouts attempt to manage slots, render props, or complex composition:

- Ownership errors with `Children` and `ViewFn`
- SSR hydration mismatches
- Non-deterministic render order
- Tight coupling between layout and app-specific structure
- Hard-to-debug move semantics failures

These issues surfaced directly in Leptos 0.8 during multi-slot layout design.

---

## Forbidden Patterns

### ❌ Forbidden
```rust
#[component]
pub fn DashboardLayout(
    sidebar: Children,
    header: Children,
    children: Children,
) -> impl IntoView {
    view! {
        <Sidebar>{sidebar()}</Sidebar>
        <Header>{header()}</Header>
        <main>{children()}</main>
    }
}
```

Layouts MUST NOT orchestrate content wiring.

---

## Canonical Pattern

### ✅ Canonical
```rust
#[component]
pub fn DashboardLayout(children: Children) -> impl IntoView {
    view! {
        <div class="layout-dashboard" data-layout="dashboard">
            {children()}
        </div>
    }
}
```

Composition is handled by the consuming application.

---

## Rationale

Layouts define **structure and zones**, not **content relationships**.

This rule:
- Preserves ownership clarity
- Prevents SSR/CSR divergence
- Keeps layouts reusable and stable

This is an architectural invariant.

---

## Enforcement

- Code review rejection of slot-based layouts
- Static detection of multiple `Children` in layouts
- CI rule forbidding composition logic in layouts

---

## Exceptions

No exceptions. This rule is absolute.

---

## Version History

- **1.0.0** — Initial canonical version
