# Canon Rule #61: No Relative CSS Imports

**Status:** ENFORCED


**Severity:** MEDIUM
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** styling-css
**Tags:** css, imports, monorepo
**Language:** EN

---

**Intro:**
Relative CSS imports create brittle dependencies that break with refactors and prevent package portability. Only canonical package imports should be used across boundaries.

**Problem:**
relative css imports couple files to directory structure

**Solution:**
use canonical package imports instead of relative paths

**Signals:**
- ../../css import
- broken imports after move
- path fragility

**Search Intent:**
why relative css imports are bad in monorepo

**Keywords:**
relative css imports problem, monorepo css imports, canonical css packages, postcss path resolution

---

---

## Principle

Relative imports in CSS **DO NOT SCALE** and are forbidden outside the local package.

---

## Forbidden

```css
@import "../../theme.css";
@import "../../../../tokens.css";
```

---

## Allowed

```css
@import "@canonrs/tailwind/tokens.css";
```

---

## Rationale

Relative imports:
- break monorepos
- break refactors
- break publishing
- hide architectural boundaries

Canonical imports encode **intent**, not topology.

---

## Enforcement Checklist

- [ ] Zero `../` imports across package boundaries
- [ ] Only `@canonrs/*` used
- [ ] Reviewed in PR
