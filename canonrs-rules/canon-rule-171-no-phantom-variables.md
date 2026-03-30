# Canon Rule #171: Phantom Variables Are Forbidden

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-01-28

**Category:** design-system
**Tags:** css, variables, tokens, validation
**Language:** EN

---

**Intro:**
Referencing undefined variables creates silent UI failures and unpredictable styling. Every variable must be traceable to a defined token.

**Problem:**
undefined css variables are used causing silent failures

**Solution:**
ensure all variables are declared and traceable in token definitions

**Signals:**
- undefined variable
- silent failure
- broken ui

**Search Intent:**
how to detect undefined css variables

**Keywords:**
css phantom variables issue, design system variable validation, undefined css variable fix, token completeness check

---

## Principle

**Every referenced variable must be defined and traceable.**

---

## Problem

Phantom variables create silent UI failures.

---

## Forbidden Pattern

❌ Referencing variables not defined in any token file.

---

## Canonical Pattern

✅ All variables originate from core or base tokens.

---

## Enforcement

- CSS variable scan
- Token completeness check

---

## Exceptions

None.
