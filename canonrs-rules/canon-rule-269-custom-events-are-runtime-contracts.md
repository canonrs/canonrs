# Canon Rule #269: Custom Events Are Public Runtime Contracts

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** behavior, interactive
**Version:** 1.0.0  
**Date:** 2026-02-13

---

## Principle

**All canon:* custom events are part of the public runtime API and semver-bound.**

---

## Problem

Without formal contract:

- Breaking rename breaks Interactive
- Hidden coupling
- Silent behavior failures

---

## Forbidden Pattern

```rust
CustomEvent::new("resize-event")
```

Unscoped, unstable, non-canonical.

---

## Canonical Pattern

```rust
CustomEvent::new("canon:resize:move")
```

Pattern: `canon:{component}:{action}`

---

## Rationale

Runtime events must be versioned surfaces.

---

## Enforcement

- Event naming audit
- CI string pattern validation

---

## Exceptions

> No exceptions.

---

## Version History

- 1.0.0 — Initial version
