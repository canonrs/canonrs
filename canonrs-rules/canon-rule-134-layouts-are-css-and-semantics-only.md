# Canon Rule #134: Layouts Are CSS and Semantics Only

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** css, layout
**Version:** 1.0.0
**Date:** 2025-01-23

---

## Principle

**Layouts MUST provide only semantic containers and CSS contracts, never behavioral or reactive logic.**

---

## The Problem

When layouts include logic beyond structure:

- Layouts become stateful implicitly
- UI behavior leaks into structure
- CSS zones lose determinism
- SSR/CSR boundaries become unclear
- Layout reuse across apps breaks

This leads to fragile layouts that cannot be reasoned about statically.

---

## Forbidden Patterns

### ❌ Forbidden
```rust
#[component]
pub fn Layout(children: Children) -> impl IntoView {
    let open = create_rw_signal(true);

    view! {
        <div>
            <button on:click=move |_| open.set(false) />
            {children()}
        </div>
    }
}
```

Layouts MUST NOT manage state or events.

---

## Canonical Pattern

### ✅ Canonical
```rust
#[component]
pub fn Layout(children: Children) -> impl IntoView {
    view! {
        <div class="layout" data-layout="app">
            {children()}
        </div>
    }
}
```

All behavior is delegated to UI or Controllers.

---

## Rationale

Layouts define **where things live**, not **how they behave**.

This rule:
- Preserves layout purity
- Guarantees CSS zone correctness
- Enables static reasoning
- Aligns with Canon Layout Zones Contract

---

## Enforcement

- Lint: forbid signals and event handlers in layouts
- Code review: layouts must be stateless
- CI: detect reactive primitives in `layouts/`

---

## Exceptions

No exceptions. This rule is absolute.

---

## Version History

- **1.0.0** — Initial canonical version
