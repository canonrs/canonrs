# Canon Rule #276: Interactive Must Remain Deterministic Under SSR Without wasm

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13

**Category:** core-runtime
**Tags:** ssr, wasm, determinism, interactive
**Language:** EN

---

**Intro:**
Interactive logic depending on wasm execution breaks SSR determinism and causes hydration mismatches. SSR must be independent.

**Problem:**
interactive rendering depends on wasm causing mismatch between ssr and csr

**Solution:**
gate all wasm logic and ensure deterministic ssr output

**Signals:**
- hydration mismatch
- ssr divergence
- runtime inconsistency

**Search Intent:**
how to ensure ssr deterministic interactive

**Keywords:**
ssr deterministic rendering leptos, wasm gating interactive logic, frontend hydration consistency, ssr csr symmetry rust

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