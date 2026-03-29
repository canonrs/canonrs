# Canon Rule #183: Overlay Positioning Is a Primitive, Not a Controller Concern

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** architecture, overlays, primitives
**Version:** 1.0.0  
**Date:** 2026-01-30

---

## Principle

**All overlay positioning logic MUST live in a dedicated primitive, never in controllers or CSS alone.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Without this rule:

- Overlays appear centered on screen instead of anchored to triggers
- Controllers start performing layout calculations
- Inline styles leak into runtime DOM
- Tooltip, Popover, Dropdown implementations diverge
- SSR/CSR hydration breaks due to structural mismatch

Architectural impact:

- Controller bloat
- Logic duplication
- Non-reusable overlays
- Unstable rendering contracts

---

## Forbidden Pattern

### ❌ Forbidden

```rust
let rect = anchor.get_bounding_client_rect();
overlay.style().set_property("top", "...").unwrap();
overlay.style().set_property("left", "...").unwrap();
```

This violates the rule by coupling geometry calculation to controllers.

---

## Canonical Pattern

### ✅ Canonical

```rust
use_floating_position(
    "trigger-id",
    "overlay-id",
    FloatingConfig {
        placement: Placement::Top,
        offset: 8.0,
        flip: true,
    },
);
```

The primitive owns all positioning logic and exposes results via CSS variables.

---

## Rationale

This rule protects core architectural invariants:

- Controllers manage state only
- Geometry is reusable and centralized
- CSS remains declarative
- Overlays behave consistently across the system

This is not opinion — it prevents systemic architectural failure.

---

## Enforcement

- Code review: no geometry logic outside primitives
- CI grep for `getBoundingClientRect` usage
- Overlay checklist during PR review

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
