# Canon Rule #287: State Must Control Visibility

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** state-reactivity
**Tags:** state, visibility, dom, css
**Language:** EN

---

**Intro:**
Component state must explicitly control DOM visibility. Implicit or disconnected state leads to inconsistent UI behavior.

**Problem:**
state is defined but does not affect visibility or rendering

**Solution:**
bind state directly to DOM visibility via attributes or CSS selectors

**Signals:**
- state changes without visual effect
- inconsistent UI behavior
- hidden elements still interactive

**Search Intent:**
how to bind state to visibility in leptos

**Keywords:**
state visibility binding, leptos state dom control, css state selectors, reactive ui visibility

---

## Principle

State without effect is invalid.

---

## Problem

Unbound state:

- creates dead logic
- breaks UX expectations
- introduces bugs

---

## Patterns

### Forbidden Pattern

```
data-rs-state="loading"
<!-- no visual change -->
```

---

### Canonical Pattern

```
[data-rs-state="loading"] .overlay {
  display: flex;
}
```

---

## Contract

### Enforcement

- every state must control DOM or behavior
- no passive state declarations

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
