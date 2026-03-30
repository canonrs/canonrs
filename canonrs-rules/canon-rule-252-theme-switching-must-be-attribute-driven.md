# Canon Rule #252: Theme Switching Must Be Attribute-Driven

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-02-13

**Category:** design-system
**Tags:** css, theming, runtime, attributes
**Language:** EN

---

**Intro:**
Imperative theme manipulation introduces inconsistencies and hydration issues. Theme switching must rely on declarative attributes.

**Problem:**
themes are applied via javascript mutations causing instability and conflicts

**Solution:**
apply themes exclusively via dom data attributes and css cascade

**Signals:**
- hydration mismatch
- style conflict
- debug difficulty

**Search Intent:**
how to implement css attribute theme switching

**Keywords:**
data theme attribute switching, css theming declarative pattern, frontend theme toggle css, avoid js theme mutation

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