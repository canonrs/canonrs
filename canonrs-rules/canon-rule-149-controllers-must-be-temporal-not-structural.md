# Canon Rule #149 — Controllers Must Be Temporal, Not Structural

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** architecture, ui, layout
**Version:** 1.0.0
**Date:** 2026-01-16

---
## Context
Controllers exist to manage **time**, **async**, and **coordination**.
They are not layout managers and must not define DOM structure.

Structural logic inside controllers breaks composability and reuse.

## Rule
Controllers **MUST manage temporal logic only**:
- async flows
- timers
- transitions
- coordination between signals

Controllers **MUST NOT**:
- render elements
- decide layout
- own visual structure

## Forbidden Pattern
```rust
if loading {
    view! { <Spinner/> }
}
```

## Correct Pattern
```rust
controller.on_load.emit(())
```

UI decides how loading is represented.

## Rationale
- Keeps controllers reusable
- Preserves UI purity
- Prevents architectural leakage
- Aligns with Canon layer hierarchy

## Scope
- Controllers
- Components

## Exception
None.

Controllers manage *when*, never *what*.
