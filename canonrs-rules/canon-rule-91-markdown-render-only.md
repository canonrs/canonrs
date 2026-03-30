# Canon Rule #91: Markdown and Code Blocks Are Render-Only

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-01-14

**Category:** component-architecture
**Tags:** markdown, rendering, ui
**Language:** EN

---

**Intro:**
Embedding behavior inside markdown components creates SSR coupling and unreliable runtime behavior. Markdown must remain purely declarative.

**Problem:**
markdown components include behavior instead of pure rendering

**Solution:**
limit markdown components to structure and delegate behavior externally

**Signals:**
- event handlers in markdown
- clipboard logic inside render
- ssr coupling

**Search Intent:**
why markdown components should not have behavior

**Keywords:**
markdown render only pattern, separate behavior from content, leptos markdown components, ssr safe markdown rendering

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
