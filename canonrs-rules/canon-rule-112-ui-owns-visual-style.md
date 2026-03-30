# Canon Rule #112: UI Owns Visual Style

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-01-17

**Category:** component-architecture
**Tags:** ui, primitives, css, layering
**Language:** EN

---

**Intro:**
Mixing visual styling into primitives breaks separation of concerns and reduces reusability. UI layer must exclusively control appearance while primitives remain structural.

**Problem:**
visual styling is applied in primitives instead of ui layer

**Solution:**
restrict styling to ui layer and keep primitives purely structural

**Signals:**
- style leakage
- duplicate css
- primitive coupling

**Search Intent:**
how to separate ui and primitives

**Keywords:**
ui layer styling pattern, primitive vs ui separation, design system layering, css ownership ui layer

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