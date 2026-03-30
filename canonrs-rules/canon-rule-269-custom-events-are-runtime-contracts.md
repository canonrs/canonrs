# Canon Rule #269: Custom Events Are Public Runtime Contracts

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-02-13

**Category:** behavior
**Tags:** events, runtime, contracts, behavior
**Language:** EN

---

**Intro:**
Unstructured event naming creates hidden coupling and breaking changes. Events must follow stable contracts.

**Problem:**
custom events lack naming contract causing instability and breakage

**Solution:**
use canonical namespaced event pattern canon component action

**Signals:**
- event mismatch
- silent failure
- integration break

**Search Intent:**
how to name custom events consistently

**Keywords:**
custom event naming convention, frontend runtime contracts events, canonical event namespace pattern, behavior event stability

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