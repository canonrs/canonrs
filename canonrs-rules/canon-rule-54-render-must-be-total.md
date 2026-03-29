# Canon Rule #54 — Render Must Be Total

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** state, ui
**Version:** 1.0.0
**Date:** 2025-01-16

---


## Golden Rule

**Rendering must always be total and non-panicking.  
Actions may be partial and may panic if required runtime is missing.**

Any closure executed during render (`to_html`, SSR, hydration)
MUST NOT assume the existence of runtime-only providers.

---

## The Problem

In Leptos/Tachys, **attribute closures are evaluated during rendering**, not interaction.

This includes closures used in:

- `disabled=`
- `class=`
- `style=`
- `hidden=`
- any reactive attribute or signal-based expression

If these closures call:

- `expect`
- `unwrap`
- `panic!`
- runtime-only assumptions

➡️ **The application will panic during SSR or render.**

---

## ❌ Forbidden Pattern (Render Panic)

```rust
let history = use_command_history();

disabled=move || {
    !history
        .expect("CommandHistory required")
        .can_undo()
        .get()
}
```

### Why this is illegal

- Executed during render
- Render must be total
- Render must not fail
- Runtime contexts may not exist yet

This causes **hard panics during SSR or initial render**.

---

## ✅ Correct Pattern (Total Render)

```rust
let history = use_command_history();

disabled=move || {
    match history {
        Some(h) => !h.can_undo().get(),
        None => true, // degrade safely
    }
}
```

### Why this is correct

- Render handles all possible states
- No assumptions
- SSR-safe
- Hydration-safe

---

## ✅ Where `expect()` IS allowed

`expect()` is allowed **only inside user-triggered actions**:

- `on:click`
- `on:submit`
- keyboard handlers
- command execution callbacks

```rust
on:click=move |_| {
    history
        .expect("CommandHistory required")
        .undo();
}
```

### Reason

- Actions run only after interaction
- Runtime is guaranteed
- Explicit failure is acceptable

---

## Canon Principle

> **Render must be total.  
> Actions may be partial.**

---

## Summary Checklist

- [ ] No `expect()` inside render closures
- [ ] No `unwrap()` inside reactive attributes
- [ ] Render degrades safely when context is missing
- [ ] Actions enforce required runtime explicitly
- [ ] SSR never panics due to missing providers

---

## Related Rules

- Canon Rule #50 — Provider Singleton Pattern
- Canon Rule #37 — Provider Taxonomy Boundaries
- Canon Rule #52 — Command History Runtime

