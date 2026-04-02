# Canon Rule #319: Behaviors Must Be Stateless

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** behavior
**Tags:** behavior, stateless
**Language:** EN

---

**Intro:**
Behaviors must not retain internal state to remain predictable and re-attachable.

**Problem:**
behavior stores internal state

**Solution:**
derive state from DOM

**Signals:**
- inconsistent behavior
- state desync
- debugging difficulty

**Search Intent:**
how to build stateless behaviors

**Keywords:**
stateless behavior dom, frontend behavior patterns, dom driven state, reactive ui

---

## Principle

Behavior must be stateless.

---

## Problem

Stateful behavior:

- breaks reattachment
- desynchronizes with DOM
- creates hidden logic
- causes inconsistency

---

## Patterns

### Forbidden Pattern

```
let open = false;
```

---

### Canonical Pattern

```
element.getAttribute("data-rs-state")
```

---

## Contract

### Enforcement

- no state in behavior
- state must be read from DOM

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
