# Canon Rule #268: Behavior Cleanup Is Mandatory and Deterministic

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13

**Category:** behavior
**Tags:** behavior, cleanup, wasm, listeners
**Language:** EN

---

**Intro:**
Missing cleanup in behaviors causes memory leaks and duplicated listeners in SPA environments. Lifecycle must be deterministic.

**Problem:**
event listeners and side effects are not cleaned up causing leaks and duplication

**Solution:**
implement deterministic cleanup using lifecycle hooks for all behaviors

**Signals:**
- memory leak
- duplicate listener
- event storm

**Search Intent:**
how to cleanup event listeners wasm

**Keywords:**
wasm event listener cleanup, behavior lifecycle rust, frontend memory leak listeners, leptos on_cleanup pattern

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

### Forbidden

```rust
container.add_event_listener_with_callback("click", cb)?;
closure.forget(); // no cleanup ever
```

Listener lives forever. No detach. No guard reset.

---

## Canonical Pattern

### Canonical

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