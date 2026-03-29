# Canon Rule #111: Model First, CSS Second

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** architecture, css, components
**Version:** 1.0.0
**Date:** 2026-01-17

---

## Principle

**CSS never fixes an architecturally invalid model.**

If the solution requires "clever" CSS, the model is wrong.

---

## Definition

The *model* defines:
- Who owns the state
- Who renders each visual signal
- Where responsibilities live

CSS **only visually expresses** that model.
It does not redefine, correct, or compensate for architecture.

---

## Forbidden: Using CSS to

❌ Fix structural overlap
❌ Hide responsibility conflicts
❌ "Adjust" incorrect visual authority

Forbidden examples:
- `!important`
- `margin: -1px`
- compensatory padding
- specificity hacks

---

## Required

✅ Fix the HTML/hierarchy
✅ Redefine who owns the visual signal
✅ Remove the wrong element

---

## Canonical Diagnosis

If the solution requires:
- workarounds
- "temporary" exceptions
- defensive explanatory comments

👉 the model is incorrect.

---

## Justification

CSS is declarative, not corrective.
Using it to compensate for architecture creates silent technical debt.

---

## Final Rule

> **When CSS becomes a workaround, the error is in the model — always.**

---

## Related Canon Rules

- Canon Rule #107 — Token Architecture Theme Specificity
- Canon Rule #109 — Single Visual Authority
- Canon Rule #106 — UI Neutralizes Hostile CSS, Not Primitives
