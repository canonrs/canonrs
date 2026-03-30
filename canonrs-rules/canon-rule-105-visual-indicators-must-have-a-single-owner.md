# Canon Rule #105 — Visual Indicators Must Have a Single Owner

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-01-16

**Category:** component-architecture
**Tags:** ui, ownership, visual, architecture
**Language:** EN

---

**Intro:**
Splitting visual indicators across multiple components creates ambiguity and fragile styling. This leads to duplicated visuals and patch-based fixes.

**Problem:**
multiple layers render the same visual indicator causing duplication

**Solution:**
assign a single owner component for each visual indicator

**Signals:**
- duplicate visuals
- css hacks
- misalignment

**Search Intent:**
how to fix duplicate ui indicators

**Keywords:**
ui visual ownership pattern, component indicator duplication, ui architecture ownership, css visual conflict

---

## Principle

**A visual indicator MUST have exactly one canonical owner in the component hierarchy.**

Indicators such as:
- underline
- highlight
- focus ring
- active border
- selection marker

CANNOT be shared, duplicated, or split across multiple layers.

---

## The Problem

When two layers render parts of the same visual signal:

- The model becomes ambiguous
- Styling degenerates into patching
- Bugs are "fixed" with offsets or overrides
- Architecture silently rots

This is not a CSS issue — it is a **semantic ownership violation**.

---

## Forbidden Patterns

- Container draws baseline, child draws active line
- Parent draws inactive state, child draws active delta
- Two elements visually completing the same signal

**If two elements draw the same idea, the architecture is wrong.**

---

## Canonical Pattern

Choose ONE owner:

- Either the *item* owns the indicator  
- Or the *container* owns the indicator  

Never both.

All styling and tokens for that indicator MUST live in the owning layer's CSS.

---

## Real-World Trigger

This rule was formalized after a Tabs component rendered:

- a baseline on the list
- an active underline on the trigger

Resulting in duplicated lines and invalid fixes.

---

## Enforcement

- Visual duplication is a rule violation
- Fix requires redesign, not CSS tweaks
- Reviewers MUST reject patches that "align" or "mask" the issue

---

## Canonical Justification

> **Visual semantics are part of architecture.**  
> If ownership is unclear, implementation will decay.

---

## Version History

- **1.0.0** — Extracted from Tabs architecture failure