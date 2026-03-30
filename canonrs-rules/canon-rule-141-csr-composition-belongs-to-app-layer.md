# Canon Rule #141: CSR Composition Belongs to the App Layer

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2025-01-23

**Category:** component-architecture
**Tags:** csr, architecture, composition, ssr
**Language:** EN

---

**Intro:**
Placing CSR behavior outside the application layer introduces SSR crashes, hydration issues, and runtime leakage. Design systems and layouts become environment-aware and lose portability.

**Problem:**
csr logic outside app layer breaks ssr boundaries and architecture separation

**Solution:**
centralize all csr composition and orchestration in the application layer only

**Signals:**
- hydration mismatch
- cfg wasm leakage
- ssr crash
- logic duplication

**Search Intent:**
where to place csr logic in architecture

**Keywords:**
csr composition app layer, leptos csr architecture, ssr csr separation pattern, runtime logic layering ui

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