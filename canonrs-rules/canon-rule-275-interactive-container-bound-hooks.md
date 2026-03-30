# Canon Rule #275: Interactive Hooks Must Be Container-Bound

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13

**Category:** state-reactivity
**Tags:** interactive, hooks, container, state
**Language:** EN

---

**Intro:**
Global hook binding breaks isolation and introduces cross component interference. Hooks must be scoped.

**Problem:**
hooks attach globally causing interference and coupling

**Solution:**
bind all hooks explicitly to container id

**Signals:**
- unexpected behavior
- cross interaction
- state leak

**Search Intent:**
how to scope hooks to container

**Keywords:**
container bound hooks pattern, frontend hook isolation, reactive hook scoping rust, leptos container id binding

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