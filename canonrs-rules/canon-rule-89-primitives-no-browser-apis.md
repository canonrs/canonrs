# Canon Rule #89: Primitives Must Never Touch Browser APIs

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** primitives, ssr
**Version:** 1.0.0  
**Date:** 2026-01-14

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

