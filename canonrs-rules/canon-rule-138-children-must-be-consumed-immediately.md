# Canon Rule #138: Children Must Be Consumed Immediately

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** leptos, layout, components
**Version:** 1.0.0
**Date:** 2025-01-23

---

## Principle

**`Children` MUST be consumed immediately in the component that receives them and MUST NOT be forwarded across abstraction layers.**

---

## The Problem

In Leptos, `Children` is implemented as `FnOnce`.

When `Children` is:
- stored,
- forwarded,
- captured by closures,
- or passed through multiple components,

the result is:

- Ownership violations
- Move errors during compilation
- Non-deterministic render behavior
- Incompatibility with `view!` macro closures
- SSR/CSR boundary breakage

This issue surfaced repeatedly when Layouts attempted to forward `Children` to Blocks or UI components.

---

## Forbidden Patterns

### ❌ Forbidden
```rust
#[component]
pub fn DashboardLayout(children: Children) -> impl IntoView {
    view! {
        <SidebarProvider>
            {children()} // ❌ consumed inside closure, not immediately
        </SidebarProvider>
    }
}
```

### ❌ Forbidden
```rust
#[component]
pub fn LayoutWrapper(children: Children) -> impl IntoView {
    let slot = children; // ❌ storing Children
    view! { <div /> }
}
```

---

## Canonical Pattern

### ✅ Canonical
```rust
#[component]
pub fn DashboardLayout(children: Children) -> impl IntoView {
    let content = children(); // ✅ consume immediately

    view! {
        <div data-layout="dashboard">
            {content}
        </div>
    }
}
```

### ✅ Canonical (Preferred)
```rust
// Layouts act as wrappers only
#[component]
pub fn DashboardLayout(children: Children) -> impl IntoView {
    view! {
        <div data-layout="dashboard">
            {children()}
        </div>
    }
}
```

Composition MUST happen at the application level.

---

## Rationale

This rule protects:

- Rust ownership invariants
- Leptos rendering model
- SSR determinism
- Architectural clarity

Layouts are **structural**, not compositional routers.

Passing `Children` across layers introduces hidden ownership coupling and violates Leptos execution semantics.

---

## Enforcement

- Static analysis: detect forwarding of `Children`
- CI: forbid storing `Children` in structs or closures
- Code review checklist

---

## Exceptions

No exceptions. This rule is absolute.

---

## Version History

- **1.0.0** — Initial canonical version
