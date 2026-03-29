# Canon Rule #61: No Relative CSS Imports

**Status:** ENFORCED


**Severity:** MEDIUM
**Scope:** build, css
**Version:** 1.0.0
**Date:** 2025-01-16

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

