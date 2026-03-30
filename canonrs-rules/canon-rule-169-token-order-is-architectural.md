# Canon Rule #169: Token Import Order Is Architectural

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-01-28

**Category:** styling-css
**Tags:** css, tokens, order, cascade
**Language:** EN

---

**Intro:**
Incorrect token import order causes partial styling and broken component rendering. The cascade must follow a strict architectural sequence.

**Problem:**
tokens are imported in wrong order causing inconsistent styles

**Solution:**
enforce canonical import order from core tokens to layouts and blocks

**Signals:**
- partial styles
- broken components
- order conflict

**Search Intent:**
how to fix css token import order issues

**Keywords:**
css token order cascade, design system token import order, frontend css layering tokens, token dependency order

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
