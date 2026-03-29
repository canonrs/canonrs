# Canon Rule #276: Interactive Must Remain Deterministic Under SSR Without wasm

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** interactive
**Version:** 1.0.0  
**Date:** 2026-02-13

---

## Principle

**Interactive rendering must not depend on wasm execution.**

---

## Problem

SSR mismatch causes hydration errors.

---

## Enforcement

All wasm logic must be cfg-gated.

---

## Exceptions

None.

---

## Version History

- 1.0.0 — Initial version
