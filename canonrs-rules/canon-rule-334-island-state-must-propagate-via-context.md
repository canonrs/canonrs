# Canon Rule #334: Island State Must Propagate Via Context

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-05

**Category:** islands-architecture
**Tags:** islands, context, signal, state, leptos
**Language:** EN

---

**Intro:**
When multiple islands in the same tree need to share reactive state, the only correct pattern is `provide_context(RwSignal)` at the root island and `use_context` in child islands. Direct prop drilling of signals or DOM-based state sharing violates the signal contract.

**Problem:**
Islands cannot share state via SSR component tree. Passing signals as props is not supported. DOM mutation as a workaround breaks the SSOT principle.

**Solution:**
Root island publishes state via `provide_context`. Child islands consume via `use_context` captured before any closure.

**Signals:**
- child island does not react to parent state changes
- state shared via DOM attributes instead of signals
- `use_context` called inside event handler closure

**Search Intent:**
leptos island shared state, provide_context island, tabs island state sync

**Keywords:**
leptos provide_context, island context, RwSignal shared, tabs island pattern

---

## Principle

Reactive state shared across island boundaries must flow through Leptos context. Any other mechanism — DOM mutation, eval, query_selector, prop serialization — is forbidden.

---

## Problem
```rust
// ❌ broken — DOM mutation as state
el.set_attribute("data-rs-state", "active").ok();

// ❌ broken — use_context inside closure
on:click=move |_| {
    if let Some(ctx) = use_context::<TabsContext>() { ... }
}
```

---

## Patterns

### Forbidden Pattern
- `set_attribute` to sync state between islands
- `query_selector` to find sibling islands
- `eval` or `Function::new_no_args` to run JS
- `use_context` called inside event handler or reactive closure

### Canonical Pattern
```rust
// Root island — publishes state
#[island]
pub fn TabsRootIsland(children: Children, #[prop(into)] initial: String) -> impl IntoView {
    let active = RwSignal::new(initial);
    provide_context(TabsContext(active));
    view! { <div data-rs-tabs="">{children()}</div> }
}

// Child island — consumes state, captures context before closure
#[island]
pub fn TabsTriggerIsland(children: Children, #[prop(into)] value: String) -> impl IntoView {
    let signal = use_context::<TabsContext>().map(|TabsContext(a)| a); // captured here
    let v2 = value.clone();
    let state = move || signal.map(|a| if a.get() == v2 { "active" } else { "inactive" }).unwrap_or("inactive");
    let on_click = {
        let signal = signal;
        move |_| { if let Some(a) = signal { a.set(value.clone()); } }
    };
    view! { <button data-rs-state=state on:click=on_click>{children()}</button> }
}
```

---

## Contract

### Enforcement
- shared island state must use `provide_context` + `use_context`
- `use_context` must be called at component root, never inside closures
- signal must be captured as `Option<RwSignal<T>>` before any closure

### Exceptions
None.

---

## Version History
- 1.0.0 - Initial definition
