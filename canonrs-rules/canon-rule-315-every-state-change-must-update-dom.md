# Canon Rule #315: Every State Change Must Update DOM

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** state-reactivity
**Tags:** state, dom, reactivity
**Language:** EN

---

**Intro:**
State changes must always be reflected in DOM to maintain synchronization across system layers.

**Problem:**
state changes not reflected in DOM

**Solution:**
update data-rs-* attributes on every change

**Signals:**
- stale UI
- mismatch between state and DOM
- integration failure

**Search Intent:**
how to sync state with dom

**Keywords:**
state dom sync, reactive dom update, frontend state architecture, data attribute sync

---

## Principle

DOM is the source of truth.

---

## Problem

If DOM is not updated:

- behaviors read stale state
- integrations break
- UI becomes inconsistent
- debugging becomes unreliable

---

## Patterns

### Forbidden Pattern

```
state.set("on");
```

(no DOM update)

---

### Canonical Pattern

```
element.setAttribute("data-rs-state", "on");
```

---

## Contract

### Enforcement

- every state change updates DOM
- DOM must reflect current state

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
