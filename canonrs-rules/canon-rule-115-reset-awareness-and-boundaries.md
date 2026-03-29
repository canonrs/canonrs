# Canon Rule #115: Reset Awareness & CSS Boundaries

**Status:** ENFORCED
**Severity:** MEDIUM
**Scope:** css, components
**Version:** 1.0.0
**Date:** 2026-01-17

---

## Principle

**UI components must assume the existence of global resets.**

Global reset is not a bug.
Ignoring it is.

---

## Definition

In enterprise systems:

- CSS reset exists
- Global normalization exists
- Inevitable inheritance exists

The role of UI component is to:
- neutralize reset **explicitly**
- without violating primitives
- without depending on fragile order

---

## Allowed

✅ `all: revert` in UI
✅ Re-establish `font`, `display`, `box-sizing`
✅ Increase specificity with `button[data-*]`

---

## Forbidden

❌ Fix reset inside primitive
❌ Use `!important` to "win"
❌ Create wrappers just to fight CSS
❌ Rely on browser defaults

---

## Violation Symptoms

- Component appearance changes between apps
- Works isolated, breaks integrated
- Cascading local corrections
- Dependency on CSS load order

---

## Justification

Reset is infrastructure.
UI must be resilient to it.
Primitive must be ignorant of it.

---

## Final Rule

> **Reset is not fought.
> Reset is bounded.**

---

## Related Canon Rules

- Canon Rule #106 — UI Neutralizes Hostile CSS, Not Primitives
- Canon Rule #112 — UI Owns Visual Style
- Canon Rule #110 — Reset Awareness and Boundaries (duplicate number, needs cleanup)
