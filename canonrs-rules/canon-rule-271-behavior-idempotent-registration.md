# Canon Rule #271: Behavior Execution Must Be Idempotent

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13

**Category:** behavior
**Tags:** behavior, idempotency, runtime, events
**Language:** EN

---

**Intro:**
Repeated behavior registration causes duplicate listeners and inconsistent runtime state. Execution must be idempotent.

**Problem:**
behavior registration runs multiple times causing duplication and instability

**Solution:**
guard registration using container flags to ensure single execution

**Signals:**
- duplicate event
- multiple trigger
- runtime anomaly

**Search Intent:**
how to make behavior idempotent

**Keywords:**
idempotent behavior registration, frontend event duplication fix, wasm behavior guard pattern, leptos container attribute guard

---

## Principle

**Running register() multiple times must not change system state.**

---

## Problem

Hot reload may re-register behavior.

Without idempotence:

- Duplicate listeners
- Multiple event triggers

---

## Forbidden Pattern

No guard attribute.

---

## Canonical Pattern

```rust
if container.get_attribute("data-x-attached").as_deref() == Some("1") {
    return Ok(());
}
```

---

## Rationale

Deterministic runtime.

---

## Enforcement

Guard required in all behaviors.

---

## Exceptions

None.

---

## Version History

- 1.0.0 — Initial version