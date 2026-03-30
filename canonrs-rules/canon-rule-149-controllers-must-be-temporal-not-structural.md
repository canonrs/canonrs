# Canon Rule #149 — Controllers Must Be Temporal, Not Structural

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-01-16

**Category:** behavior
**Tags:** controllers, architecture, async, coordination
**Language:** EN

---

**Intro:**
Controllers mixing structural rendering with temporal logic break composability and reuse. Controllers must focus exclusively on timing and coordination concerns.

**Problem:**
controllers define structure instead of managing temporal logic

**Solution:**
limit controllers to async flows and coordination and move rendering to ui

**Signals:**
- controller renders ui
- structure in controller
- coupling ui logic

**Search Intent:**
what should controllers handle in architecture

**Keywords:**
controller temporal logic pattern, async coordination architecture, separation controller ui, leptos controller design

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