# Canon Rule #280: Interactive Must Be Safe Under Feature Flag Removal

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** interactive, build
**Version:** 1.0.0  
**Date:** 2026-02-13

---

## Principle

**Disabling any optional feature must not alter structural rendering or break hydration.**

---

## Problem

Feature coupling causes:

- Hidden rendering differences
- Inconsistent DOM trees
- Hydration mismatches
- Regression chains

---

## Forbidden Pattern

```rust
if resize_enabled {
    view! { <ResizeHandlePrimitive /> }
} else {
    view! {}
}
```

Feature removal must not change structural contracts without controlled fallback.

---

## Canonical Pattern

```rust
<ResizeHandlePrimitive
    attr:data-enabled=move || resize_enabled.get().to_string()
/>
```

Structural output remains stable.

---

## Rationale

Protects:
- SSR determinism
- DOM stability
- Feature isolation

---

## Enforcement

- Review: toggle each feature individually
- Hydration test matrix

---

## Exceptions

None.

---

## Version History

- 1.0.0 — Initial version
