# Canon Rule #189: Overlays Must Be Anchor-Relative

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** overlays, primitives
**Version:** 1.1.0  
**Date:** 2026-02-03

---

## Principle

**Every overlay MUST be positioned relative to an explicit anchor element.**

Viewport-relative overlays are architecturally invalid.

---

## Forbidden Patterns

❌ Viewport-centered overlays  
❌ Fixed positioning without anchor  
❌ Positioning without trigger reference  

```css
[data-dropdown] {
  position: fixed;
  top: 50%;
}
```

---

## Canonical Pattern

```rust
use_floating_position(
  "trigger-id",
  "overlay-id",
  FloatingConfig {
    placement: Placement::Bottom,
    offset: 8.0,
    flip: true,
  },
);
```

---

## Architectural Invariants

- Spatial causality is preserved
- UX remains predictable
- Scroll and resize remain stable
- Accessibility remains coherent

---

## Enforcement

- Overlay primitives require `anchor_id`
- CI rejects viewport-relative overlays
- Review blocks unanchored overlays

---

## Exceptions

**None. Anchor-relative is mandatory.**

---
