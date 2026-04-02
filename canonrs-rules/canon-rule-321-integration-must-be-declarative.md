# Canon Rule #321: Integration Must Be Declarative

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** governance
**Tags:** integration, declarative
**Language:** EN

---

**Intro:**
Component integration must be defined declaratively through attributes, not imperative code.

**Problem:**
integration is manual and imperative

**Solution:**
use declarative data-rs-* attributes

**Signals:**
- complex wiring
- tight coupling
- brittle integrations

**Search Intent:**
how to build declarative frontend integration

**Keywords:**
declarative ui integration, data attributes integration, frontend architecture, decoupled components

---

## Principle

Integration must be declarative.

---

## Problem

Imperative integration:

- increases complexity
- creates tight coupling
- reduces scalability
- breaks composability

---

## Patterns

### Forbidden Pattern

```
componentA.listen(componentB)
```

---

### Canonical Pattern

```
data-rs-filter-target="table"
```

---

## Contract

### Enforcement

- integration must use attributes
- no imperative wiring

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
