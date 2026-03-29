# Canon Rule #159: UI CSS Must Be Fail-Safe, Token-Only, and Attribute-Scoped

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** css, tokens, design-system
**Version:** 1.0.0  
**Date:** 2026-01-26

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
