# Canon Rule #293: No Duplicate State Outside DOM

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** state-reactivity
**Tags:** state, dom, duplication, architecture
**Language:** EN

---

**Intro:**
State duplication between DOM and internal logic leads to divergence and inconsistency.

**Problem:**
state exists in multiple sources causing desynchronization

**Solution:**
use DOM as single source of truth via data attributes

**Signals:**
- inconsistent UI state
- mismatch between DOM and logic
- stale values

**Search Intent:**
how to avoid duplicated state in frontend

**Keywords:**
single source of truth dom, state duplication frontend, dom state pattern, reactive ui state

---

## Principle

DOM is the source of truth.

---

## Problem

Duplicated state:

- diverges
- becomes stale
- breaks reactivity

---

## Contract

- state must be reflected in DOM
- no parallel state storage

---

## Version History

- 1.0.0 — Initial definition
