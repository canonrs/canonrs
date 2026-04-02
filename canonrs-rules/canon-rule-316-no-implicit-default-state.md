# Canon Rule #316: No Implicit Default State

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** state-reactivity
**Tags:** state, defaults
**Language:** EN

---

**Intro:**
Default state must be explicitly declared to avoid ambiguity and inconsistent initialization.

**Problem:**
state defaults are implicit

**Solution:**
always define explicit initial state

**Signals:**
- inconsistent initial render
- unpredictable behavior
- missing attributes

**Search Intent:**
how to define default state in components

**Keywords:**
default state frontend, explicit state initialization, component state patterns, predictable ui

---

## Principle

State must be explicit.

---

## Problem

Implicit defaults:

- create ambiguity
- lead to inconsistent rendering
- break determinism
- complicate debugging

---

## Patterns

### Forbidden Pattern

```
let state;
```

---

### Canonical Pattern

```
data-rs-state="idle"
```

---

## Contract

### Enforcement

- all components must declare default state
- no implicit initialization

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
