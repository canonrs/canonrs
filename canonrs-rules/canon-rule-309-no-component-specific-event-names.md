# Canon Rule #309: No Component-Specific Event Names

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** governance
**Tags:** events, naming
**Language:** EN

---

**Intro:**
Event naming must be standardized to enable cross-component integration.

**Problem:**
components emit custom event names

**Solution:**
use canonical event names

**Signals:**
- inconsistent events
- integration complexity
- duplicated logic

**Search Intent:**
how to standardize ui event names

**Keywords:**
event naming frontend, canonical events, ui architecture patterns, event consistency

---

## Principle

Events must be unified.

---

## Problem

Custom event names:

- break integration
- increase complexity
- prevent reuse
- create fragmentation

---

## Patterns

### Forbidden Pattern

```
dispatchEvent(new CustomEvent("toggle-change"))
```

---

### Canonical Pattern

```
dispatchEvent(new CustomEvent("rs-change"))
```

---

## Contract

### Enforcement

- no component-specific event names
- use canonical event vocabulary

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
