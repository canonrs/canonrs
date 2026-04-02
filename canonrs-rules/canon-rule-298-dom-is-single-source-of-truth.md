# Canon Rule #298: DOM Is Single Source of Truth

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** governance
**Tags:** dom, architecture, state
**Language:** EN

---

**Intro:**
All system state must be derivable from DOM to ensure consistency across layers.

**Problem:**
state is stored outside DOM and becomes inconsistent

**Solution:**
use DOM attributes as canonical state representation

**Signals:**
- state divergence
- inconsistent rendering
- debugging complexity

**Search Intent:**
how to use dom as source of truth

**Keywords:**
dom state architecture, frontend state source of truth, data attributes state, reactive dom pattern

---

## Principle

DOM is canonical.

---

## Problem

External state:

- diverges
- becomes stale
- increases complexity

---

## Contract

- all state must be visible in DOM
- no hidden internal state

---

## Version History

- 1.0.0 — Initial definition
