# Canon Rule #106 — UI Neutralizes Hostile CSS, Not Primitives

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-01-16

**Category:** component-architecture
**Tags:** primitives, css, ui, layering
**Language:** EN

---

**Intro:**
Applying CSS fixes inside primitives breaks architectural layering and contaminates core abstractions. Global CSS issues must be handled at the UI layer.

**Problem:**
css fixes applied in primitives break layer separation

**Solution:**
handle css neutralization only in ui layer while keeping primitives pure

**Signals:**
- css leakage
- primitive contamination
- layer violation

**Search Intent:**
how to isolate css in ui layer

**Keywords:**
ui css neutralization pattern, primitive purity css, layer separation ui css, design system layering css

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