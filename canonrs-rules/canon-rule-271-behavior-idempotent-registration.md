# Canon Rule #271: Behavior Execution Must Be Idempotent

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** behavior
**Version:** 1.0.0  
**Date:** 2026-02-13

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
