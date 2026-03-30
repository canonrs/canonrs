# Canon Rule #280: Interactive Must Be Safe Under Feature Flag Removal

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13

**Category:** core-runtime
**Tags:** interactive, feature-flags, hydration, ssr
**Language:** EN

---

**Intro:**
Feature flags that alter DOM structure create hydration mismatches and unstable rendering. Structural output must remain consistent.

**Problem:**
feature flags change dom structure causing hydration mismatch

**Solution:**
preserve structural output and control behavior via attributes

**Signals:**
- hydration mismatch
- dom divergence
- conditional rendering bugs

**Search Intent:**
how to use feature flags without breaking hydration

**Keywords:**
feature flags ssr hydration, dom stability rendering, conditional rendering pitfalls, frontend feature isolation

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