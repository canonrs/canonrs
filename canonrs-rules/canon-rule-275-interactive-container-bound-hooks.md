# Canon Rule #275: Interactive Hooks Must Be Container-Bound

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** interactive
**Version:** 1.0.0  
**Date:** 2026-02-13

---

## Principle

**Hooks must bind strictly to container_id.**

---

## Problem

Global hook attachment breaks isolation.

---

## Enforcement

All hooks require explicit container_id.

---

## Exceptions

None.

---

## Version History

- 1.0.0 — Initial version
