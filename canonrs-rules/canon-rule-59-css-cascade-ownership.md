# Canon Rule #59: CSS Cascade Ownership

**Status:** ENFORCED


**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** styling-css
**Tags:** css, cascade, architecture
**Language:** EN

---

**Intro:**
Multiple CSS bundles without defined precedence create unpredictable styling behavior. A single stylesheet must own the final cascade to ensure deterministic rendering.

**Problem:**
multiple css bundles create undefined cascade precedence

**Solution:**
enforce single stylesheet entrypoint with defined cascade ownership

**Signals:**
- style conflicts
- random overrides
- multiple css files

**Search Intent:**
how to manage css cascade ownership

**Keywords:**
css cascade control, single stylesheet architecture, css layering strategy, design system precedence

---

---

## Principle

There must be **exactly ONE owner** of the final CSS cascade.

Mixing multiple style bundles without defined precedence is forbidden.

---

## Canonical Ownership

```
Design Tokens  → Lowest
Design Themes  → Middle
Application    → Highest
```

Design system CSS **MUST NOT override application intent**.

---

## Forbidden Pattern

```html
<link rel="stylesheet" href="/canon.css">
<link rel="stylesheet" href="/app.css">
```

Order ambiguity = undefined behavior.

---

## Required Pattern

```rust
<Stylesheet id="app" href="/app.css"/>
```

Single entrypoint.

---

## Enforcement Checklist

- [ ] One stylesheet in HTML
- [ ] No parallel CSS bundles
- [ ] App owns last cascade layer
