# Canon Rule #169: Token Import Order Is Architectural

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** css, tokens
**Version:** 1.0.0  
**Date:** 2026-01-28

---

## Principle

**Token import order defines system correctness.**

---

## Problem

Out-of-order tokens cause partial styling and broken components.

---

## Forbidden Pattern

❌ Importing UI CSS before core tokens.

---

## Canonical Pattern

✅ Core → Tokens → Themes → Variants → UI → Layouts → Blocks.

---

## Enforcement

- Build-time checks
- Canonical entrypoint review

---

## Exceptions

None.

