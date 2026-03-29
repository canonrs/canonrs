# Canon Rule #133: Children Consumption Locality

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** leptos, state
**Version:** 1.0.0
**Date:** 2025-01-23

---

## Principle

**`Children` MUST be consumed in the closest possible component to their final render location.**

---

## The Problem

`Children` in Leptos is `FnOnce`. When passed across component layers:

- Move errors occur
- Closures created by `view!` cannot safely capture it
- SSR rendering panics or fails
- Workarounds introduce unnecessary indirection

This caused repeated architectural failures.

---

## Forbidden Patterns

### ❌ Forbidden
```rust
fn Wrapper(children: Children) -> impl IntoView {
    view! {
        <Provider>
            {children()}
        </Provider>
    }
}
```

This moves `Children` into a closure boundary.

---

## Canonical Pattern

### ✅ Canonical
```rust
fn Wrapper(children: Children) -> impl IntoView {
    let content = children();
    view! {
        <Provider>
            {content}
        </Provider>
    }
}
```

Or consume `Children` only in the final composing component.

---

## Rationale

This rule enforces:
- Ownership determinism
- Predictable SSR behavior
- Clear reactive boundaries

It is a direct enforcement of reactive safety.

---

## Enforcement

- Static analysis: detect `children()` inside `view!` closures
- Code review rejection of forwarded `Children`
- CI lint rule: `Children` must be consumed exactly once

---

## Exceptions

No exceptions. This rule is absolute.

---

## Version History

- **1.0.0** — Initial canonical version
