# Canon Rule #290: Interactive Components Must Emit Events

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** behavior
**Tags:** events, interaction, components, architecture
**Language:** EN

---

**Intro:**
Interactive components must emit events to integrate with the system. Silent state changes break composability.

**Problem:**
components mutate state without emitting events

**Solution:**
emit standardized events such as rs-change on state updates

**Signals:**
- no event emitted
- integration failure
- non-reactive components

**Search Intent:**
how to emit events in ui components

**Keywords:**
custom event frontend, rs-change event pattern, component interaction events, ui event architecture

---

## Principle

Interaction must be observable.

---

## Problem

Silent components:

- cannot be composed
- cannot trigger reactions
- break system integration

---

## Patterns

### Forbidden Pattern

```
state = "on"
<!-- no event -->
```

---

### Canonical Pattern

```
dispatchEvent(new CustomEvent("rs-change", { detail: value }))
```

---

## Contract

### Enforcement

- all interactive components must emit events
- rs-change is required for value updates

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
