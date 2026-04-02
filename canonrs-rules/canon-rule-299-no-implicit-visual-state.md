# Canon Rule #299: No Implicit Visual State

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** design-system
**Tags:** visual, state, css
**Language:** EN

---

**Intro:**
Visual state must be explicitly defined. Implicit styling leads to inconsistency.

**Problem:**
visual changes occur without explicit state mapping

**Solution:**
map all visual changes to explicit state attributes

**Signals:**
- inconsistent styling
- unclear state representation
- css conflicts

**Search Intent:**
how to map state to visual changes

**Keywords:**
state driven css, visual state mapping, design system consistency, frontend styling state

---

## Principle

Visual state must be explicit.

---

## Problem

Implicit visuals:

- break consistency
- confuse developers

---

## Contract

- all visuals must map to state
- no implicit styling

---

## Version History

- 1.0.0 — Initial definition
