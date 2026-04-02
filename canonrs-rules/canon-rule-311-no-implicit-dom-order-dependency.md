# Canon Rule #311: No Implicit DOM Order Dependency

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** component-architecture
**Tags:** dom, order
**Language:** EN

---

**Intro:**
Component behavior must not depend on implicit DOM ordering.

**Problem:**
logic relies on child position

**Solution:**
use explicit attributes and selectors

**Signals:**
- fragile components
- order-based bugs
- unpredictable behavior

**Search Intent:**
how to avoid dom order dependency

**Keywords:**
dom order dependency, frontend architecture patterns, robust components, selector based logic

---

## Principle

Order must not define behavior.

---

## Problem

Order-based logic:

- breaks when structure changes
- creates hidden coupling
- reduces flexibility
- causes instability

---

## Patterns

### Forbidden Pattern

```
children[0]
```

---

### Canonical Pattern

```
querySelector("[data-rs-role='label']")
```

---

## Contract

### Enforcement

- no reliance on DOM order
- use explicit selectors

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
