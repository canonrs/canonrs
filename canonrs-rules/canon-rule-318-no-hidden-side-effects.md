# Canon Rule #318: No Hidden Side Effects

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** state-reactivity
**Tags:** side-effects, state
**Language:** EN

---

**Intro:**
All side effects must be explicit and traceable to avoid unpredictable behavior.

**Problem:**
side effects occur implicitly

**Solution:**
make all side effects explicit

**Signals:**
- unexpected updates
- difficult debugging
- inconsistent state

**Search Intent:**
how to manage side effects in frontend

**Keywords:**
side effects frontend, reactive architecture, explicit state changes, predictable ui

---

## Principle

Side effects must be explicit.

---

## Problem

Hidden side effects:

- create unpredictability
- break mental model
- complicate debugging
- cause race conditions

---

## Patterns

### Forbidden Pattern

```
state.set("x") inside render
```

---

### Canonical Pattern

```
effect(() => { ... })
```

---

## Contract

### Enforcement

- no implicit side effects
- all effects must be declared

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
