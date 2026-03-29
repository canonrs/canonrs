# Canon Rule #112: UI Owns Visual Style

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** primitives, ui, css
**Version:** 1.0.0
**Date:** 2026-01-17

---

## Principle

**Only the UI layer can apply visual style.**

Primitives **never** own visual responsibility.
CSS **never** lives in primitives.
Primitives only expose structure and state.

---

## Definition

In CanonRS, the separation is absolute:

**Primitive:**
- Semantic HTML
- `data-*` attributes
- Declarative state (`data-state`)
- ZERO CSS
- ZERO tokens
- ZERO appearance

**UI:**
- Applies styling
- Consumes canonical tokens
- Defines visual affordances
- Translates state → visual

---

## Correct Examples

✅ `button[data-tabs-trigger]` styled in `style/ui/tabs.css`
✅ Primitive renders only `<button data-tabs-trigger>`
✅ Visual state comes from `data-state="active"`

---

## Anti-patterns (FORBIDDEN)

❌ CSS in `src/primitives/*`
❌ Primitive with `class=""`
❌ Primitive "knowing" how it looks
❌ Primitive importing tokens

---

## Violation Symptoms

- Duplicated CSS between UI and primitive
- Primitive difficult to reuse
- Style "leaking" to other components
- Visual refactors breaking structure

---

## Justification

Primitives are reusable structural blocks.
UI is where visual decisions live.
Mixing the two breaks scalability and governance.

---

## Final Rule

> **If there's appearance, there's UI.
> If there's structure, there's primitive.**

---

## Related Canon Rules

- Canon Rule #111 — Model First, CSS Second
- Canon Rule #108 — Visual Surfaces Contract
- Canon Rule #106 — UI Neutralizes Hostile CSS, Not Primitives
