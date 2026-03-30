# Canon Rule #168: UI Must Declare Required Tokens

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-01-28

**Category:** design-system
**Tags:** tokens, ui, css, validation
**Language:** EN

---

**Intro:**
Using undefined CSS variables results in invisible or broken UI elements. Token usage must be explicit and verifiable to ensure reliable rendering.

**Problem:**
ui references undefined tokens causing broken or invisible styles

**Solution:**
ensure all tokens are defined before being used by ui components

**Signals:**
- missing variable
- invisible ui
- broken style

**Search Intent:**
how to validate css tokens in ui components

**Keywords:**
css token validation ui, design system token usage, undefined css variable issue, ui token contract

---

## Principle

**Every UI component must rely only on tokens that are explicitly defined.**

---

## Problem

Using undefined CSS variables leads to invisible or broken UI.

---

## Forbidden Pattern

❌ UI CSS referencing tokens that do not exist in base tokens.

---

## Canonical Pattern

✅ Tokens are defined first, UI consumes only canonical tokens.

---

## Enforcement

- Token audit
- CSS variable validation

---

## Exceptions

None.
