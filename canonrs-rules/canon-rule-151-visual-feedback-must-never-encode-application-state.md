# Canon Rule #151 — Visual Feedback Must Never Encode Application State

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** state, ui, components
**Version:** 1.0.0
**Date:** 2026-01-16

---
## Context
Visual indicators exist to reflect state, not define it.
Encoding application logic into visual feedback creates hidden coupling.

Examples include icons, colors, spinners or badges driving logic decisions.

## Rule
Visual feedback **MUST be a pure projection** of state.
It **MUST NOT**:
- encode decisions
- influence application flow
- act as state source

## Forbidden Pattern
```rust
if icon == "error" {
    retry()
}
```

## Correct Pattern
```rust
match state {
    State::Error => show_error(),
    _ => {}
}
```

Visuals subscribe to state; they never drive it.

## Rationale
- Prevents implicit logic
- Keeps state authoritative
- Improves testability
- Aligns with data-first architecture

## Scope
- UI
- Components
- Blocks

## Exception
None.

Visuals describe reality. They do not decide it.
