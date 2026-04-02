# Canon Rule #291: data-rs-value Is Canonical Output

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** behavior
**Tags:** data-attributes, value, state, integration
**Language:** EN

---

**Intro:**
Components must expose their state through a canonical data attribute to enable system-wide integration.

**Problem:**
component state is not accessible externally

**Solution:**
expose value via data-rs-value attribute

**Signals:**
- missing data-rs-value
- integration failure
- state inaccessible

**Search Intent:**
how to expose component value via dom

**Keywords:**
data attribute state, dom value exposure, frontend component integration, canonical value pattern

---

## Principle

Value must be externalized.

---

## Problem

Hidden state:

- cannot be consumed
- breaks integration
- isolates components

---

## Patterns

### Forbidden Pattern

```
state stored internally only
```

---

### Canonical Pattern

```
data-rs-value="selected-option"
```

---

## Contract

### Enforcement

- all interactive components must expose data-rs-value
- value must reflect current state

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
