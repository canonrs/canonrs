# Canon Rule #313: No DOM Mutation Outside Behavior Layer

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-02

**Category:** component-architecture
**Tags:** dom, mutation, architecture
**Language:** EN

---

**Intro:**
DOM mutation must be centralized in the behavior layer to ensure predictability and separation of concerns.

**Problem:**
components mutate DOM directly

**Solution:**
restrict DOM mutation to behavior layer only

**Signals:**
- unpredictable UI updates
- duplicated logic
- inconsistent DOM state

**Search Intent:**
how to centralize dom mutations in frontend

**Keywords:**
dom mutation architecture, behavior layer pattern, frontend separation of concerns, ui mutation control

---

## Principle

DOM mutation belongs to behavior layer only.

---

## Problem

When UI or primitives mutate DOM:

- logic becomes fragmented
- SSR/CSR consistency breaks
- behavior duplication occurs
- debugging becomes difficult

---

## Patterns

### Forbidden Pattern

```
view! {
  <div onclick=...>
}
```

---

### Canonical Pattern

```
data-rs-behavior="toggle"
```

Mutation handled in behavior.

---

## Contract

### Enforcement

- no DOM mutation in primitives or UI
- behaviors own all DOM changes

---

### Exceptions

None.

---

## Version History

- 1.0.0 — Initial definition
