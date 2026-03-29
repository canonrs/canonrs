# Canon Rule #91: Markdown and Code Blocks Are Render-Only

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** components, ui
**Version:** 1.0.0  
**Date:** 2026-01-14

---

## Principle

**Markdown components MUST only render structure, never behavior.**

---

## Forbidden Responsibilities

Markdown components MUST NOT:

- Copy to clipboard
- Register click handlers
- Manage imperative state
- Call browser APIs

---

## Correct Pattern

```html
<pre data-copy-button data-copy-text="...">
```

Behavior is handled by runtime JS via event delegation.

---

## Justification

- Markdown is static content
- Behavior must survive hydration
- Prevents SSR/runtime coupling

---

## Related Rules

- Canon Rule #87 — Leptos SSR Script Placement
- Canon Rule #89 — Primitives No Browser APIs

