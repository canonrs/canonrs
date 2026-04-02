# Canon Rule #284: Children Must Be Structurally Stable

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** component-architecture
**Tags:** children, composition, dom, structure
**Language:** EN

---

**Intro:**
Dynamic children insertion or removal alters DOM shape and breaks hydration consistency.

**Problem:**
optional children modify node count causing mismatch between ssr and client

**Solution:**
ensure children count and position remain stable regardless of state

**Signals:**
- hydration mismatch
- missing node errors
- inconsistent children length

**Search Intent:**
how to prevent children mismatch in leptos components

**Keywords:**
leptos children mismatch, dom children stability, ssr hydration children, component composition rules

---

## Principle

Children define structure, not state.

---

## Problem

Optional nodes:

- change child count
- reorder DOM
- break hydration

---

## Patterns

### Forbidden Pattern

```
{if show_hint {
  <Hint/>
}}
```

---

### Canonical Pattern

```
<Hint hidden={!show_hint}/>
```

---

## Contract

### Enforcement

- children count must remain constant
- optional content must be hidden, not removed

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
