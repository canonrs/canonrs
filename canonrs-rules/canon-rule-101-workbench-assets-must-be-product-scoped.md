# Canon Rule #101: Workbench Assets Must Be Product-Scoped

**Status:** ENFORCED  
**Severity:** MEDIUM  
**Scope:** build, design-system
**Version:** 1.0.0  
**Date:** 2026-01-15

---

## Principle

**Illustrative assets belong to the product, never to the design system.**

The design system defines rules and UI primitives.  
Products define narrative, illustration, and branding.

---

## Rule

- `rs-design` MAY contain:
  - UI icons
  - Control glyphs
  - Interaction symbols

- `rs-design` MUST NOT contain:
  - Illustrations
  - Pillars
  - Marketing or storytelling visuals

- Products (Workbench, apps) MUST own their own illustrative SVGs.

---

## Forbidden
```
packages-rust/rs-design/assets/symbols/pillars/*.svg
```

---

## Required
```
products/<product-name>/shell/public/assets/
```

---

## Canonical Justification

> A design system encodes behavior, not narrative.

---
