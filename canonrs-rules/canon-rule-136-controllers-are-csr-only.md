# Canon Rule #136: Controllers Are CSR-Only

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** ssr, csr, behavior
**Version:** 1.0.0
**Date:** 2025-01-23

---

## Principle

**Controllers MUST exist and execute exclusively in CSR (wasm32), never during SSR.**

---

## The Problem

When Controllers run during SSR:

- Hydration mismatches occur
- Event handlers bind to non-existent DOM
- Side effects execute twice (SSR + CSR)
- Panics happen due to browser-only APIs
- Debugging becomes non-deterministic

This directly violates SSR safety guarantees.

---

## Forbidden Patterns

### ❌ Forbidden
```rust
#[component]
pub fn SidebarController() -> impl IntoView {
    // Runs during SSR — forbidden
    create_effect(move |_| {
        web_sys::window().unwrap();
    });

    view! { <></> }
}
```

Controllers MUST NOT exist without CSR gating.

---

## Canonical Pattern

### ✅ Canonical
```rust
#[cfg(target_arch = "wasm32")]
#[component]
pub fn SidebarController() -> impl IntoView {
    create_effect(move |_| {
        // Browser-only logic
    });

    view! { <></> }
}
```

```rust
// Usage
{#[cfg(target_arch = "wasm32")]
 { view! { <SidebarController /> } }}
```

---

## Rationale

Controllers orchestrate **behavior**, not structure.

SSR renders structure only.
CSR attaches behavior afterward.

This rule:
- Guarantees hydration correctness
- Prevents double execution
- Enforces clean SSR/CSR boundary
- Aligns with Canon Rule #129 (SSR Event Safety)

---

## Enforcement

- CI: forbid Controllers without `cfg(target_arch = "wasm32")`
- Static scan: detect browser APIs in non-CSR code
- Code review checklist

---

## Exceptions

No exceptions. This rule is absolute.

---

## Version History

- **1.0.0** — Initial canonical version
