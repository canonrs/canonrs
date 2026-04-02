# Canon Rule #294: No Hidden Interactive Elements

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** accessibility
**Tags:** accessibility, focus, aria, dom
**Language:** EN

---

**Intro:**
Hidden elements must not receive focus or interaction. Violations break accessibility and UX.

**Problem:**
interactive elements exist inside hidden containers

**Solution:**
disable or remove interactivity when element is hidden

**Signals:**
- focus leaks
- keyboard navigation issues
- accessibility violations

**Search Intent:**
how to prevent focus on hidden elements

**Keywords:**
hidden focus accessibility, aria hidden focus issue, keyboard navigation hidden elements, wcag focus rules

---

## Principle

Hidden means non-interactive.

---

## Problem

Hidden interactive elements:

- trap focus
- confuse users
- violate WCAG

---

## Contract

- no focusable elements inside aria-hidden
- no interactive behavior when hidden

---

## Version History

- 1.0.0 — Initial definition
