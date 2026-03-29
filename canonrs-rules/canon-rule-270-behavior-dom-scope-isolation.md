# Canon Rule #270: Behavior Must Never Traverse Beyond Its Container Scope

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** behavior
**Version:** 1.0.0  
**Date:** 2026-02-13

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
