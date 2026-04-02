# Canon Rule #306: Components Must Declare Behavior

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** behavior
**Tags:** behavior, architecture
**Language:** EN

---

**Intro:**
Interactive components must explicitly declare their behavior to ensure predictable attachment.

**Problem:**
behavior is implicit or missing

**Solution:**
declare behavior via data-rs-behavior

**Signals:**
- missing interaction
- behavior not attached
- inconsistent UX

**Search Intent:**
how to attach behavior to components

**Keywords:**
component behavior binding, data rs behavior, frontend interaction, ui architecture

---

## Principle

Behavior must be explicit.

---

## Problem

Implicit behavior:

- cannot be reliably attached
- breaks behavior registry
- causes inconsistent interaction
- increases coupling

---

## Patterns

### Forbidden Pattern

```
<button onclick="...">
```

---

### Canonical Pattern

```
data-rs-behavior="toggle"
```

---

## Contract

### Enforcement

- all interactive components must declare behavior
- use data-rs-behavior

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
