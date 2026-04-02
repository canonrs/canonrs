# Canon Rule #297: Overlay State Must Be Deterministic

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** state-reactivity
**Tags:** overlay, state, deterministic
**Language:** EN

---

**Intro:**
Overlay behavior must be fully determined by state. Implicit conditions lead to inconsistency.

**Problem:**
overlay visibility depends on implicit logic

**Solution:**
bind overlay behavior strictly to explicit state

**Signals:**
- inconsistent overlay visibility
- unpredictable UI
- state mismatch

**Search Intent:**
how to control overlay visibility via state

**Keywords:**
overlay state control, deterministic ui state, reactive overlay behavior, frontend state patterns

---

## Principle

State drives behavior.

---

## Problem

Implicit logic:

- creates bugs
- breaks predictability

---

## Contract

- overlay visibility must depend only on state
- no implicit conditions

---

## Version History

- 1.0.0 — Initial definition
