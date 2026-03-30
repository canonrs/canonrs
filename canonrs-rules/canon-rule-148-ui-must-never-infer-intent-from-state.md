# Canon Rule #148 — UI Must Never Infer Intent From State

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-01-16

**Category:** state-reactivity
**Tags:** state, intent, ui, callbacks
**Language:** EN

---

**Intro:**
Inferring intent from state transitions introduces implicit behavior and duplicated logic in UI components. This leads to unpredictable flows and hidden side effects.

**Problem:**
ui infers intent from state changes causing implicit behavior

**Solution:**
express intent explicitly via callbacks or commands and update state after

**Signals:**
- implicit logic
- duplicated behavior
- side effects

**Search Intent:**
how to separate intent from state in ui

**Keywords:**
intent vs state ui, explicit callbacks pattern, state transition logic frontend, ui event architecture

---

## Context
State represents **what is**, not **why it changed**.
Inferring intent from state leads to implicit behavior, duplicated logic and unpredictable flows.

This mistake appears when UI tries to "guess" actions based on state transitions.

## Rule
UI components **MUST NOT infer intent from state changes**.
Intent **MUST be explicit**, expressed via callbacks or commands.

## Forbidden Pattern
```rust
if open.get() {
    on_opened.emit(())
}
```

## Correct Pattern
```rust
on_toggle.emit(ToggleIntent::Open)
```

State updates happen **after** intent emission.

## Rationale
- Separates cause from effect
- Prevents hidden side-effects
- Enables deterministic flows
- Makes audit and testing trivial

## Scope
- UI
- Components
- Controllers

## Exception
None.

State is data. Intent is an event.