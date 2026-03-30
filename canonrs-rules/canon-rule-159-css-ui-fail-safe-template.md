# Canon Rule #159: UI CSS Must Be Fail-Safe, Token-Only, and Attribute-Scoped

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-01-26

**Category:** design-system
**Tags:** css, tokens, ui, attributes
**Language:** EN

---

**Intro:**
UI styles break when primitives fail to emit expected structure or tokens are inconsistently applied. This leads to fragile styling and unpredictable rendering.

**Problem:**
ui css depends on structure or tokens that may be missing causing fragile styles

**Solution:**
enforce token-only css scoped by data attributes with fail-safe defaults

**Signals:**
- missing styles
- fragile css
- inconsistent rendering

**Search Intent:**
how to make ui css resilient

**Keywords:**
fail safe css ui pattern, data attribute css scoping, token based css architecture, resilient design system css

---

## Principle

**Every UI CSS file MUST be fail-safe by construction: token-only, data-attribute–scoped, and resilient to partial or missing UI/Primitive wiring.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Without this rule, UI styling becomes fragile and unsafe.

Typical failures observed:

- CSS silently breaks when a Primitive forgets to emit a 