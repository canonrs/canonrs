# Canon Rule #268: Behavior Cleanup Is Mandatory and Deterministic

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** behavior, wasm
**Version:** 1.0.0  
**Date:** 2026-02-13

---

## Principle

**Every Behavior must implement deterministic cleanup of listeners, guards, and side-effects.**

---

## Problem

Without deterministic cleanup:

- Memory leaks in SPA navigation
- Duplicate listeners
- Intermittent runtime bugs
- Event storms
- Hard-to-reproduce state corruption

---

## Forbidden Pattern

### ❌ Forbidden

```rust
container.add_event_listener_with_callback("click", cb)?;
closure.forget(); // no cleanup ever
```

Listener lives forever. No detach. No guard reset.

---

## Canonical Pattern

### ✅ Canonical

```rust
let closure = Closure::wrap(Box::new(move |_| { ... }) as Box<dyn FnMut(_)>);

container.add_event_listener_with_callback(
    "click",
    closure.as_ref().unchecked_ref()
)?;

on_cleanup(move || {
    let _ = container.remove_event_listener_with_callback(
        "click",
        closure.as_ref().unchecked_ref()
    );
    let _ = container.remove_attribute("data-x-attached");
});
```

---

## Rationale

Prevents:
- Memory leaks
- Double registration
- Runtime instability

Cleanup is not optional. It is architectural integrity.

---

## Enforcement

- Code review mandatory
- CI lint for missing guard removal
- Behavior template compliance check

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
