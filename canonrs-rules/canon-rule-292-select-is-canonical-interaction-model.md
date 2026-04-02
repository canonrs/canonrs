# Canon Rule #292: Select Is Canonical Interaction Model

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** behavior
**Tags:** select, interaction, architecture, canonical
**Language:** EN

---

**Intro:**
All interactive components must follow a unified interaction contract. Divergence breaks composability.

**Problem:**
components implement inconsistent interaction patterns

**Solution:**
standardize all interactions on Select model: state + value + event

**Signals:**
- inconsistent component behavior
- missing events
- integration failures

**Search Intent:**
how to standardize interaction patterns in ui components

**Keywords:**
select pattern ui, interaction architecture frontend, unified component behavior, canonical ui model

---

## Principle

Select defines interaction contract.

---

## Problem

Without standard:

- components diverge
- integration breaks
- system becomes inconsistent

---

## Contract

- must expose data-rs-value
- must expose data-rs-state
- must emit rs-change

---

## Version History

- 1.0.0 — Initial definition
