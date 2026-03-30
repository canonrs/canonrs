# Canon Rule #89: Primitives Must Never Touch Browser APIs

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-01-14

**Category:** component-architecture
**Tags:** primitives, ssr, browser
**Language:** EN

---

**Intro:**
Accessing browser APIs inside primitives breaks SSR compatibility and violates architectural separation. Primitives must remain environment-agnostic.

**Problem:**
primitives access browser apis causing ssr and hydration issues

**Solution:**
restrict primitives to html and delegate behavior to runtime js

**Signals:**
- window usage in primitives
- hydration errors
- ssr breakage

**Search Intent:**
why primitives should not use browser apis

**Keywords:**
no browser api primitives, ssr safe components design, leptos primitives architecture, separation runtime js

---

## Principle

**UI Primitives are structural only. They MUST NOT access browser APIs.**

---

## Absolute Prohibitions

Primitives MUST NOT reference:

- window
- document
- navigator
- clipboard
- web_sys
- wasm_bindgen
- execCommand

---

## Correct Responsibility Split

```text
Primitive → HTML + data-* attributes
Runtime JS → browser APIs + behavior
```

---

## Justification

- Preserves SSR compatibility
- Prevents hydration breakage
- Maintains architectural layering
- Enables deterministic testing

---

## Enforcement

Any browser API reference inside `primitives/` is a hard violation.

---

## Related Rules

- Canon Rule #87 — Leptos SSR Script Placement
- Canon Rule #91 — Markdown Is Render-Only
