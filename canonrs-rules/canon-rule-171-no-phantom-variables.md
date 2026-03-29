# Canon Rule #171: Phantom Variables Are Forbidden

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** css, tokens
**Version:** 1.0.0  
**Date:** 2026-01-28

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

