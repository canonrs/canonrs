# Canon Rule #188: Overlay Positioning Is Not UI Logic

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-01-30

**Category:** component-architecture
**Tags:** overlays, architecture, controllers, primitives
**Language:** EN

---

**Intro:**
Embedding positioning logic in UI or controllers leads to duplication and divergence. Positioning must remain centralized and reusable.

**Problem:**
overlay positioning logic is implemented in ui or controllers causing duplication

**Solution:**
delegate positioning entirely to dedicated primitives

**Signals:**
- duplicate logic
- geometry in ui
- inconsistent overlays

**Search Intent:**
how to separate overlay positioning from ui logic

**Keywords:**
overlay positioning separation, ui vs primitive architecture, popover logic centralization, frontend overlay design pattern

---

## Principle

**Controllers and UI components MUST NOT compute or decide overlay positioning.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Without this rule:

- Controllers accumulate geometry logic
- UI code becomes unportable
- Multiple components reimplement positioning
- Bugs multiply with every new overlay

Observable failures:

- Tooltip logic duplicated in multiple files
- Dropdown behavior diverges from Popover
- Fixes applied inconsistently

Architectural impact:

- Loss of primitive abstraction
- Explosion of ad-hoc logic
- System becomes unscalable

---

## Forbidden Pattern

### Forbidden

```rust
let rect = trigger.get_bounding_client_rect();
let x = rect.left() + 10.0;
let y = rect.bottom() + 8.0;
```

```rust
// Controller deciding placement
if is_near_bottom {
    placement = Top;
}
```

This embeds layout decisions in UI logic.

---

## Canonical Pattern

### Canonical

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

Controllers only declare intent, never geometry.

---

## Rationale

This rule preserves:

- Reusability of overlays
- Single source of truth for positioning
- Predictable behavior across the system
- Ability to evolve positioning without touching UI

Positioning is infrastructure, not UI.

---

## Enforcement

- Review checklist: no `getBoundingClientRect` in controllers
- CI scan for geometry math outside primitives
- Primitive API is the only allowed entry point

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version