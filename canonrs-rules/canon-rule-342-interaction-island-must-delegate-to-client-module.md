# Canon Rule #342: Interaction Island Must Delegate to Client Module

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-07

**Category:** interaction-architecture
**Tags:** interaction, island, client, wasm, delegation
**Language:** EN

---

**Intro:**
Any component that requires complex interaction must delegate its behavior to a dedicated client interaction module. The island must not implement interaction logic.

**Problem:**
Implementing interaction logic inside islands introduces:
- state outside the DOM (violates source of truth)
- SSR/hydration inconsistencies
- duplicated logic across components
- tight coupling between UI and behavior

Islands become mini frameworks instead of thin bridges.

**Solution:**
Interaction islands must:
- only bootstrap the interaction
- call a client module (`init_all` or equivalent)
- never implement behavior logic

All interaction logic must live in:
```
/opt/docker/monorepo/packages-rust/rs-canonrs/canonrs-client/src/interactions/
```

**Signals:**
- pointer events inside island (`pointerdown`, `pointermove`, `pointerup`)
- drag logic implemented in island
- keyboard navigation logic inside island
- layout mutation (`getBoundingClientRect`, inline styles)
- signals controlling interaction state
- loops managing DOM interaction state

---

## Principle

Interaction logic belongs to the **client interaction layer**, not to the island.

```
Primitive = contract
UI = proxy
Island = bootstrap only
Interaction = behavior engine
DOM = source of truth
```

---

## Patterns

### Forbidden Pattern
```rust
#[island]
pub fn SliderIsland(...) -> impl IntoView {
    let (value, set_value) = signal(0.0);

    let on_pointer_move = move |e| {
        // ❌ interaction logic inside island
        set_value.set(...);
    };
}
```

### Canonical Pattern
```rust
#[island]
pub fn SliderInit() -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::prelude::*;
        use leptos::wasm_bindgen::JsCast;

        let f = Closure::wrap(Box::new(move || {
            crate::interactions::slider::init_all();
        }) as Box<dyn Fn()>);

        leptos::web_sys::window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .ok();

        f.forget();
    }

    view! { <></> }
}
```

---

## Contract

### Enforcement

- Island MUST NOT:
  - implement pointer logic
  - implement drag logic
  - implement keyboard navigation logic (complex)
  - mutate layout directly
  - hold interaction state via signals

- Island MUST:
  - call client interaction module
  - execute via `requestAnimationFrame` or equivalent
  - remain stateless

- Interaction logic MUST:
  - live in `canonrs-client`
  - read/write DOM via `data-rs-*`
  - not depend on SSR

### Applies to

- resizable
- slider
- carousel
- scroll_area
- virtual_list (if interactive)
- any drag-based component

---

## Exceptions

Allowed inside island:
- simple event binding (init category)
- DOM reads without behavior logic
- ARIA sync

NOT allowed:
- full interaction engine

---

## Version History

- 1.0.0 - Initial definition — formalizes separation between Island and Interaction (2026-04-07)
