# Canon Rule #101: Workbench Assets Must Be Product-Scoped

**Status:** ENFORCED  
**Severity:** MEDIUM  
**Version:** 1.0.0  
**Date:** 2026-01-15

**Category:** governance
**Tags:** assets, design-system, ownership, monorepo
**Language:** EN

---

**Intro:**
Placing illustrative assets inside the design system creates coupling and violates separation of concerns. Design systems must remain focused on primitives and behavior, not product-specific visuals.

**Problem:**
illustrative assets stored in design system create coupling and misuse of scope

**Solution:**
keep illustrative assets inside product scope and restrict design system to primitives

**Signals:**
- asset coupling
- design drift
- wrong ownership

**Search Intent:**
prevent design system asset coupling

**Keywords:**
design system asset scope, product scoped assets, monorepo asset ownership, ui illustration separation

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