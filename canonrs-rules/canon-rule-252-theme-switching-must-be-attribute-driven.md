# Canon Rule #252: Theme Switching Must Be Attribute-Driven

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** css, theming
**Version:** 1.0.0
**Date:** 2026-02-13

---

## Principle

Theme switching MUST occur exclusively via DOM attribute mutation:

    <html data-theme="canonrs-theme">

JavaScript style mutation is forbidden.

---

## Problem

Imperative theme systems cause:

- Inline style pollution
- Hydration mismatches
- CSS specificity conflicts
- Impossible debugging

---

## Canonical Pattern

CSS:

[data-theme="canonrs-theme"] { ... }
[data-theme="clean-slate"] { ... }

Runtime:

document.documentElement.setAttribute("data-theme", "clean-slate");

---

## Forbidden

- Modifying CSS variables individually at runtime
- Injecting `<style>` blocks
- Using JS-based theming engines

---

## Rationale

Theme authority belongs to CSS cascade, not JavaScript.

---

## Exceptions

None.
