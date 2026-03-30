# Canon Rule #233: CSS Cascade Order Is Build-Enforced Architecture

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13  

**Category:** styling-css
**Tags:** css, cascade, build, architecture
**Language:** EN

---

**Intro:**
Incorrect css ordering causes token conflicts and unstable themes. Order must be enforced at build level.

**Problem:**
css layers are loaded in wrong order causing inconsistencies and overrides

**Solution:**
enforce strict cascade order at bundler level with fixed sequence

**Signals:**
- token override
- theme break
- specificity issue

**Search Intent:**
how to enforce css cascade order build

**Keywords:**
css cascade architecture order, design system css layering, frontend css build order, token specificity issues css

---

## Principle

The CanonRS CSS cascade order is an architectural invariant enforced by the bundler.

It is not configurable.
It is not flexible.
It is not opinion.

---

## Mandatory Order

1. Primitives
2. Foundation
3. Themes
4. Semantic
5. Families
6. Root
7. Variants
8. UI
9. Blocks
10. Layouts
11. Globals

Any deviation is a structural violation.

---

## Forbidden Pattern

- Reordering files manually
- Injecting CSS before semantic layer
- Loading Families before Themes
- UI depending on layer order assumptions

---

## Canonical Enforcement

Bundler must concatenate layers strictly in declared sequence.

No `@layer` directives allowed in final artifact.

---

## Rationale

Improper ordering causes:

- Token shadowing
- Inconsistent specificity
- Theme instability
- Debug complexity
- Undocumented coupling

Order is enforced at build time, never by convention.

---

## Exceptions

**No exceptions.**