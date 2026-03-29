# Canon Rule #273: MutationObserver Requires Explicit Architectural Justification

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** behavior
**Version:** 1.0.0  
**Date:** 2026-02-13

---

## Principle

**MutationObserver must never be used implicitly.**

---

## Problem

Uncontrolled observers cause:

- Performance degradation
- Layout thrashing

---

## Enforcement

Mandatory architecture review approval.

---

## Exceptions

Documented reactive DOM sync only.

---

## Version History

- 1.0.0 — Initial version
