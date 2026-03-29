# Canon Rule #135: UI vs Layout Responsibility Boundary

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** architecture, ui, layout
**Version:** 1.0.0
**Date:** 2025-01-23

---

## Principle

**Layouts define spatial zones; UI components define interaction and behavior.**

---

## The Problem

When UI behavior leaks into layouts, or layouts leak into UI:

- Responsibilities blur
- Controllers attach to the wrong layer
- CSS zones become coupled to behavior
- Refactors break unrelated areas

This was observed in sidebar and header orchestration attempts.

---

## Forbidden Patterns

### ❌ Forbidden
```rust
// Layout controlling UI behavior
#[component]
pub fn DashboardLayout(children: Children) -> impl IntoView {
    view! {
        <SidebarProvider>
            {children()}
        </SidebarProvider>
    }
}
```

Providers and behavior belong to UI or Controllers, not layouts.

---

## Canonical Pattern

### ✅ Canonical
```rust
// Layout = zones only
#[component]
pub fn DashboardLayout(children: Children) -> impl IntoView {
    view! {
        <div data-layout="dashboard" class="layout-dashboard">
            {children()}
        </div>
    }
}
```

```rust
// UI controls behavior
<SidebarProvider>
    <Sidebar />
</SidebarProvider>
```

---

## Rationale

This boundary ensures:
- Predictable ownership
- Clear mental model
- Independent evolution of layout and UI
- Enforcement of Canon Zone Contracts

It is foundational for large systems.

---

## Enforcement

- CI rule: Layouts must not import Providers
- Static scan: forbid `ui::*` behavior imports in `layouts/`
- Code review boundary checks

---

## Exceptions

No exceptions. This rule is absolute.

---

## Version History

- **1.0.0** — Initial canonical version
