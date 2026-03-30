# Canon Rule #274: Interactive Features Must Be Strictly Isolated

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13

**Category:** state-reactivity
**Tags:** interactive, state, isolation, signals
**Language:** EN

---

**Intro:**
Cross feature state mutation creates hidden dependencies and regression chains. Features must remain isolated.

**Problem:**
features share state causing coupling and unpredictable regressions

**Solution:**
scope all signals and state per feature module

**Signals:**
- hidden dependency
- regression chain
- state conflict

**Search Intent:**
how to isolate feature state signals

**Keywords:**
feature state isolation signals, reactive state modularization, frontend signal scoping, leptos feature boundaries

---

## Principle

**Each feature must operate independently without signal cross-coupling.**

---

## Problem

Cross-feature mutation leads to:

- Hidden dependencies
- Regression chains

---

## Enforcement

Feature signals must be scoped to module.

---

## Exceptions

None.

---

## Version History

- 1.0.0 — Initial version