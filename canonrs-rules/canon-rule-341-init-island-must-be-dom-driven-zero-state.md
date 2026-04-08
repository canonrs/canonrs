# Canon Rule #341: Init Island Must Be DOM-Driven and Zero State

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-07

**Category:** island-architecture
**Tags:** island, init, dom, state, events, web_sys, hydration
**Language:** EN

---

**Intro:**
Init islands are responsible for simple runtime behavior and must operate exclusively as DOM mutation engines. They must not hold internal state, perform business logic, or interpret props. All state must live in the DOM via `data-rs-state`.

**Problem:**
When init islands introduce signals, internal state, prop transformations, or logic, they break the CanonRS architecture by duplicating state outside the DOM and creating hydration inconsistencies and unpredictable behavior.

**Solution:**
Init islands must read state from the DOM, react to browser events via `web_sys`, and mutate DOM attributes using canonical helpers. They must remain stateless and purely mechanical.

**Signals:**
- usage of `signal`, `RwSignal`, `ReadSignal`
- presence of `match`, `if` logic unrelated to DOM state
- prop transformation (`unwrap_or`, enum mapping, parsing)
- local variables storing state (e.g. `is_open`, `active`)
- event handlers defined via `on:click`, `on:mouseenter`
- duplicated state between DOM and Rust

**Search Intent:**
init island state management, leptos island dom driven, remove signals from island, rust web_sys dom mutation pattern

**Keywords:**
init island, dom driven state, data-rs-state, web_sys events, zero state island, hydration safe behavior

---

## Principle

Init islands are **stateless DOM mutation layers**.  
They do not own state — they **read and write DOM state only**.

---

## Patterns

### Forbidden Pattern
```rust
#[island]
pub fn CollapsibleIsland(...) -> impl IntoView {
    let (is_open, set_open) = signal(false);

    view! {
        <button on:click=move |_| set_open.set(!is_open.get())>
            "Toggle"
        </button>
    }
}
```

### Canonical Pattern
```rust
#[island]
pub fn CollapsibleIsland(...) -> impl IntoView {
    let node_ref = NodeRef::<leptos::html::Div>::new();

    Effect::new(move |_| {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;
        use leptos::web_sys;

        let Some(root_html) = node_ref.get() else { return };
        let root: web_sys::Element = (*root_html).clone().unchecked_into();

        let root_cb = root.clone();
        let cb = Closure::<dyn Fn(web_sys::MouseEvent)>::wrap(Box::new(move |_| {
            let state = root_cb.get_attribute("data-rs-state").unwrap_or_default();
            let is_open = state.contains("open");

            if is_open {
                let _ = root_cb.set_attribute("data-rs-state", "closed");
            } else {
                let _ = root_cb.set_attribute("data-rs-state", "open");
            }
        }));

        let _ = root.add_event_listener_with_callback("click", cb.as_ref().unchecked_ref());
        cb.forget();
    });

    view! {
        <div node_ref=node_ref data-rs-state="closed"></div>
    }
}
```

---

## Contract

### Enforcement

- MUST use `#[island]`
- MUST use `web_sys` for event handling
- MUST use `add_event_listener_with_callback`
- MUST read state from `data-rs-state`
- MUST write state to `data-rs-state`
- MUST NOT use signals or reactive state
- MUST NOT store state in Rust variables
- MUST NOT use `on:*` event bindings
- MUST NOT transform props (`match`, parsing, enum mapping)
- MUST NOT contain business logic
- MUST NOT control layout or rendering
- ARIA attributes MUST reflect DOM state

### Exceptions

None.

---

## Version History

- 1.0.0 - Initial definition — establishes DOM-driven, zero-state contract for init islands (2026-04-07)
