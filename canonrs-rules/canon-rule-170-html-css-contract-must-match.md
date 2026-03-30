# Canon Rule #170: HTML and CSS Must Share the Same Contract

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-01-28

**Category:** component-architecture
**Tags:** html, css, contracts, ui
**Language:** EN

---

**Intro:**
Mismatch between rendered HTML attributes and CSS selectors results in missing styles. The contract between markup and styling must be consistent.

**Problem:**
html attributes do not match css selectors causing broken styling

**Solution:**
ensure html and css share identical contract using data attributes

**Signals:**
- no styles applied
- selector mismatch
- ui broken

**Search Intent:**
how to fix css selector not

**Keywords:**
html css contract mismatch, data attribute styling pattern, css selector issue frontend, ui contract consistency

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
