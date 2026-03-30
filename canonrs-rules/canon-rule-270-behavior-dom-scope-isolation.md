# Canon Rule #270: Behavior Must Never Traverse Beyond Its Container Scope

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13

**Category:** behavior
**Tags:** dom, scope, behavior, isolation
**Language:** EN

---

**Intro:**
Global DOM queries introduce coupling and unpredictable side effects across components. Scope must be isolated.

**Problem:**
behavior queries global dom causing cross component interference

**Solution:**
restrict all dom queries to container scoped selectors

**Signals:**
- unexpected interaction
- coupling
- runtime bug

**Search Intent:**
how to scope dom queries behavior

**Keywords:**
dom query scope isolation, frontend behavior container pattern, avoid document query selector, component isolation dom

---

## Principle

**Behavior must operate strictly within its container element boundary.**

---

## Problem

Global DOM queries cause:

- Cross-component interference
- Coupling
- Unpredictable runtime effects

---

## Forbidden Pattern

```rust
document().query_selector_all(".datatable-row")
```

---

## Canonical Pattern

```rust
container.query_selector_all("[data-datatable-row]")
```

---

## Rationale

Isolation preserves composability.

---

## Enforcement

- Disallow document-wide query except registry
- Static review

---

## Exceptions

Registry initialization only.

---

## Version History

- 1.0.0 — Initial version