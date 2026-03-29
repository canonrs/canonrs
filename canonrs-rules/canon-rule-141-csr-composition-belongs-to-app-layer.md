# Canon Rule #141: CSR Composition Belongs to the App Layer

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** csr, architecture
**Version:** 1.0.0
**Date:** 2025-01-23

---

## Principle

**All CSR-only composition, event wiring, and behavioral orchestration MUST occur in the application layer, never in layouts or design-system components.**

---

## The Problem

When CSR behavior is embedded outside the app layer:

- SSR crashes or hydrates incorrectly
- `cfg(target_arch = "wasm32")` leaks everywhere
- Design System becomes environment-aware
- Logic duplication across products
- Impossible-to-test side effects

This occurred when:
- Controllers were placed inside layouts
- UI components embedded CSR logic
- Layouts attempted to orchestrate events

---

## Forbidden Patterns

### ❌ Forbidden
```rust
#[component]
pub fn DashboardLayout(children: Children) -> impl IntoView {
    view! {
        <div>
            {children()}
            <SidebarController /> // ❌ CSR logic in layout
        </div>
    }
}
```

### ❌ Forbidden
```rust
#[component]
pub fn Sidebar() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        // ❌ UI component aware of runtime
        document().add_event_listener(...)
    }
}
```

---

## Canonical Pattern

### ✅ Canonical
```rust
#[component]
pub fn AppLayout() -> impl IntoView {
    view! {
        <DashboardLayout>
            <SidebarProvider>
                <Sidebar />
                <SidebarInset />
            </SidebarProvider>

            {#[cfg(target_arch = "wasm32")]
            { view! { <SidebarController /> } }}
        </DashboardLayout>
    }
}
```

- CSR logic is colocated with routing
- Layouts remain pure
- Design System remains SSR-safe

---

## Rationale

The application layer is the **only layer aware of runtime context**.

This separation guarantees:
- Clean SSR boundaries
- Portable Design System
- Zero conditional compilation leakage
- Clear ownership of side effects

CSR orchestration is **not reusable UI** — it is app-specific glue.

---

## Enforcement

- CI: forbid controllers in `/layouts` and `/packages-rust`
- Lint: detect `cfg(target_arch)` outside app layer
- Code review enforcement

---

## Exceptions

No exceptions. This rule is absolute.

---

## Version History

- **1.0.0** — Initial canonical version
