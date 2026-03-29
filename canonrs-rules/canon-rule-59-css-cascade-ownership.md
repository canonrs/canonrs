# Canon Rule #59: CSS Cascade Ownership

**Status:** ENFORCED


**Severity:** HIGH
**Scope:** css, architecture
**Version:** 1.0.0
**Date:** 2025-01-16

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

