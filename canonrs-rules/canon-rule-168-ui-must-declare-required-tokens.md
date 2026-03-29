# Canon Rule #168: UI Must Declare Required Tokens

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** tokens, ui
**Version:** 1.0.0  
**Date:** 2026-01-28

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

