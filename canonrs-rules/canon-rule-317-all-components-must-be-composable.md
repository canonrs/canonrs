# Canon Rule #317: All Components Must Be Composable

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** component-architecture
**Tags:** composition, architecture
**Language:** EN

---

**Intro:**
Components must be designed for composition without hidden constraints or coupling.

**Problem:**
components cannot be reused or composed

**Solution:**
design components with explicit contracts

**Signals:**
- rigid components
- duplication
- poor reuse

**Search Intent:**
how to design composable components

**Keywords:**
component composition, reusable ui components, frontend architecture patterns, composability

---

## Principle

Composition is mandatory.

---

## Problem

Non-composable components:

- increase duplication
- reduce flexibility
- break scalability
- create tight coupling

---

## Patterns

### Forbidden Pattern

```
component requires fixed structure
```

---

### Canonical Pattern

```
component accepts children
```

---

## Contract

### Enforcement

- components must accept composition
- no rigid internal structure

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
