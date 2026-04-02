# Canon Rule #314: State Must Not Live in Behavior

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** behavior
**Tags:** state, behavior, architecture
**Language:** EN

---

**Intro:**
Behavior must not own state. State must be stored in DOM or signals to remain observable and consistent.

**Problem:**
state stored inside behavior logic

**Solution:**
externalize state to DOM attributes

**Signals:**
- hidden state
- inconsistent behavior
- debugging difficulty

**Search Intent:**
how to manage state outside behavior

**Keywords:**
state externalization dom, behavior state anti pattern, frontend architecture, observable state

---

## Principle

State must not be hidden.

---

## Problem

State inside behavior:

- becomes invisible
- cannot be shared
- breaks integration
- creates inconsistencies

---

## Patterns

### Forbidden Pattern

```
let isOpen = false;
```

---

### Canonical Pattern

```
data-rs-state="open"
```

---

## Contract

### Enforcement

- no state stored in behavior
- state must exist in DOM

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
