# Canon Rule #308: All Interactions Must Emit Events

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** behavior
**Tags:** events, interaction
**Language:** EN

---

**Intro:**
All state changes must emit events to enable reactive integration across the system.

**Problem:**
state changes are not externally observable

**Solution:**
emit canonical events after state changes

**Signals:**
- silent updates
- integration failure
- missing reactions

**Search Intent:**
how to emit events on state change

**Keywords:**
ui events architecture, state change events, frontend events, rs change pattern

---

## Principle

Interactions must be observable.

---

## Problem

Silent changes:

- break integration
- isolate components
- prevent system reaction
- reduce composability

---

## Patterns

### Forbidden Pattern

```
state.set("on");
```

---

### Canonical Pattern

```
dispatchEvent(new CustomEvent("rs-change"))
```

---

## Contract

### Enforcement

- emit event on every state change
- use canonical event naming

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
