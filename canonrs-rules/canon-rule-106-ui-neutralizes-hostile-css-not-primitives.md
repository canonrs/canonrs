# Canon Rule #106 — UI Neutralizes Hostile CSS, Not Primitives

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** primitives, css
**Version:** 1.0.0  
**Date:** 2026-01-16

---

## Principle

**CSS neutralization is a UI concern, never a primitive concern.**

Primitives MUST remain pure:
- semantic HTML
- data-attributes
- zero styling logic
- zero browser behavior

If external or global CSS interferes, the fix belongs to the UI layer.

---

## The Problem

Global resets and base styles may affect primitives unintentionally.

Incorrect responses include:
- adding CSS to primitives
- adding wrapper hacks
- modifying primitive semantics

These actions violate CanonRS layering.

---

## Canonical Response

- Primitives stay untouched
- UI applies controlled neutralization
- CSS selectors target `button[data-*]`, not generic elements
- Techniques like `all: revert` are allowed **only in UI CSS**

---

## Forbidden Patterns

- CSS inside primitives
- Primitive-specific overrides
- Runtime fixes to compensate for styling leaks

---

## Canonical Pattern
```
Primitive → HTML + data-attributes  
UI        → ergonomics + resilience  
CSS       → token-driven interpretation
```

Layer boundaries must never collapse.

---

## Canonical Justification

> **If primitives know about CSS, the system is already broken.**

---

## Version History

- **1.0.0** — Extracted from Tabs / button reset incident
