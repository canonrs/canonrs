# Canon Rule #302: State Must Be Serializable in DOM

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** governance
**Tags:** dom, state, serialization
**Language:** EN

---

**Intro:**
All component state must be externally observable through the DOM to guarantee SSR/CSR consistency, enable behavior integration, and support system-wide orchestration.

**Problem:**
component state is hidden in memory and not exposed

**Solution:**
serialize all state into data-rs-* attributes

**Signals:**
- hydration mismatch
- integration failure
- state inaccessible

**Search Intent:**
how to expose component state through dom

**Keywords:**
dom state serialization, data attributes state, ssr consistency, frontend integration

---

## Principle

State must be externalized and observable.

---

## Problem

When state exists only in memory:

- behaviors cannot consume it
- external systems cannot react
- SSR and client diverge
- debugging becomes opaque

This creates systemic isolation and breaks Canon integration contracts.

---

## Patterns

### Forbidden Pattern

```
let selected = create_signal("a");
```

(no DOM representation)

---

### Canonical Pattern

```
data-rs-value="a"
data-rs-state="active"
```

State is always reflected in DOM.

---

## Contract

### Enforcement

- all state must exist in DOM
- use data-rs-* attributes exclusively
- value must always reflect current state

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
