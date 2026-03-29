# Canon Rule #114: Single Visual Authority

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** components, css
**Version:** 1.0.0
**Date:** 2026-01-17

---

## Principle

**A component has a single visual authority.**

Never more than one layer should draw the same visual affordance.

---

## Definition

In CanonRS, **each visual signal has exactly one owner**:

- Active line
- Hover background
- Focus ring
- Indicator border
- Elevation shadow

If two elements draw the same signal → **architectural BUG**.

---

## Correct Examples

✅ Selection line drawn **only** in `TabsTrigger`
✅ `TabsList` defines only layout, not active indicator
✅ Focus ring applied only to the real interactive element

---

## Anti-patterns (FORBIDDEN)

❌ List with `border-bottom` + trigger with `border-bottom`
❌ Wrapper drawing visual state
❌ "Compensations" with negative margin
❌ Two selectors competing for the same affordance

---

## Violation Symptoms

- Duplicated lines
- "Fighting" borders
- Corrections with `-1px`
- Style depends on DOM order

---

## Justification

Single visual authority guarantees:
- predictability
- readability
- safe refactoring
- absence of workarounds

---

## Final Rule

> **If two elements draw the same thing,
> the architecture is wrong.**

---

## Related Canon Rules

- Canon Rule #105 — Visual Indicators Must Have a Single Owner
- Canon Rule #111 — Model First, CSS Second
- Canon Rule #112 — UI Owns Visual Style
