# Canon Rule #145 — UI Components Must Not Mutate Global State Implicitly

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** state, ui, architecture
**Version:** 1.0.0
**Date:** 2026-01-16

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
