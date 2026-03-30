# Canon Rule #145 — UI Components Must Not Mutate Global State Implicitly

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-01-16

**Category:** state-reactivity
**Tags:** ui, state, callbacks, architecture
**Language:** EN

---

**Intro:**
Implicit global state mutations hide side effects and reduce predictability. UI components must expose behavior via explicit callbacks instead of mutating shared state directly.

**Problem:**
ui components mutate global state implicitly causing hidden side effects

**Solution:**
enforce explicit callbacks for all state mutations from ui components

**Signals:**
- hidden state change
- implicit side effect
- context mutation in ui

**Search Intent:**
how to avoid implicit state mutation ui

**Keywords:**
explicit callbacks ui pattern, avoid implicit state mutation, leptos ui state handling, callback driven state architecture

---

## Context
Implicit global state mutation hides side effects and breaks predictability.
UI components must be transparent: behavior is explicit via callbacks.

## Rule
UI components **MUST NOT mutate global state directly**.
All state changes **MUST occur via explicit callbacks or signals passed by the consumer**.

## Mandatory Pattern
```rust
#[component]
pub fn Button(
    #[prop(default = Callback::new(|_| {}))] on_click: Callback<()>,
) -> impl IntoView {
    view! {
        <button on:click=move |_| on_click(())>
            "Click"
        </button>
    }
}
```

## Forbidden Patterns
- `use_theme().set_mode(...)` inside UI
- Accessing Provider context internally to mutate state
- Side effects hidden inside render logic

## Rationale
- Makes data flow explicit
- Simplifies reasoning and debugging
- Enables reuse in different contexts
- Prevents architectural leakage

## Scope
- UI components
- Blocks

## Exception
Controllers may coordinate state explicitly, never implicitly.

UI renders. Callbacks act.