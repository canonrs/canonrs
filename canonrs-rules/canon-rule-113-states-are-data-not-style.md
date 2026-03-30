# Canon Rule #113: States Are Data, Not Style

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-01-17

**Category:** design-system
**Tags:** state, css, data-attributes, components
**Language:** EN

---

**Intro:**
Encoding state as visual style creates tight coupling and limits flexibility. State must remain semantic and independent from appearance to support scalable theming.

**Problem:**
state is encoded as visual style instead of semantic data

**Solution:**
express state via data attributes and map to visuals only in css

**Signals:**
- style coupling
- theme break
- implicit state

**Search Intent:**
how to separate state from style

**Keywords:**
state vs style css, data attributes state pattern, semantic state ui, css state mapping

---

## Principle

**State is data, not appearance.**

State declares **what it is**.
CSS decides **how it looks**.

---

## Definition

States in CanonRS:

- Are expressed via `data-*`
- Are semantic (`active`, `inactive`, `disabled`)
- Carry no embedded visual meaning

Example:
```html
<button data-state="active">
```

This state **does NOT say**:
- color
- underline
- background
- animation

It only declares: **active**.

---

## Required

✅ Use `data-state` for states
✅ Map state → visual only in CSS
✅ Keep state independent of theme

---

## Anti-patterns (FORBIDDEN)

❌ States encoded as visual classes (`.is-blue`)
❌ Style logic in Rust component
❌ Implicit states via DOM (e.g., `:first-child`)

---

## Direct Benefits

- Swappable themes without touching component
- Consistent accessibility
- Predictable SSR
- UI scalable across multiple products

---

## Justification

When state becomes style, you lose semantics.
When state is data, the system gains flexibility.

---

## Final Rule

> **State describes truth.
> Style describes appearance.**

---

## Related Canon Rules

- Canon Rule #108 — Visual Surfaces Contract
- Canon Rule #112 — UI Owns Visual Style
- Canon Rule #107 — Token Architecture Theme Specificity