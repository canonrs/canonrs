# Canon Rule #300: No Side Effects in Primitives

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** component-architecture
**Tags:** primitives, side-effects, architecture
**Language:** EN

---

**Intro:**
Primitives must be pure and predictable. Side effects break determinism and SSR safety.

**Problem:**
primitives contain logic or side effects

**Solution:**
keep primitives pure and move logic to behaviors

**Signals:**
- unpredictable rendering
- hydration issues
- side effects in view

**Search Intent:**
how to keep components pure in leptos

**Keywords:**
pure components frontend, no side effects ui, leptos primitives rules, ssr safe components

---

## Principle

Primitives are pure.

---

## Problem

Side effects:

- break SSR
- reduce predictability
- introduce bugs

---

## Contract

- primitives must be pure functions
- no side effects allowed

---

## Version History

- 1.0.0 — Initial definition
