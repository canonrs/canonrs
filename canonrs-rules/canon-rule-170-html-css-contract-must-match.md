# Canon Rule #170: HTML and CSS Must Share the Same Contract

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** ui, components
**Version:** 1.0.0  
**Date:** 2026-01-28

---

## Principle

**Rendered HTML attributes must exactly match CSS selectors.**

---

## Problem

CSS using data-* while HTML renders classes (or vice versa) breaks styling.

---

## Forbidden Pattern

❌ CSS selectors that do not match rendered HTML.

---

## Canonical Pattern

✅ Data-attribute driven contracts shared by Rust and CSS.

---

## Enforcement

- Runtime inspection
- Component contract review

---

## Exceptions

None.

