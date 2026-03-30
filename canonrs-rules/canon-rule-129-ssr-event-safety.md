# Canon Rule #129: SSR Event Safety

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2025-01-22

**Category:** core-runtime
**Tags:** ssr, hydration, events, render
**Language:** EN

---

**Intro:**
Event handlers attached to dynamically generated nodes during SSR cause hydration mismatches due to non-deterministic listener reconstruction. This leads to runtime panics.

**Problem:**
dynamic event handlers in ssr lists break hydration determinism

**Solution:**
avoid event handlers in iterators and use delegation or csr only patterns

**Signals:**
- unreachable panic
- hydration error
- event binding failure

**Search Intent:**
why ssr events break in lists

**Keywords:**
leptos ssr event handler issue, hydration mismatch dynamic list, event handler iterator problem, ssr closure mismatch leptos

---

## Principle

**Blocks MUST be treated as SSR-static structures. Interactivity in blocks MUST be delegated or CSR-only.**

**Event handlers `on:*`) MUST NOT be attached to nodes generated dynamically by `map()`, `<For>`, or any iterator within SSR-rendered components.**

---

## The Problem

When event handlers are attached to dynamically generated nodes during SSR:

1. The server renders static HTML with no event listeners
2. During hydration, the client tries to attach listeners to DOM nodes
3. The DOM walking algorithm expects identical structure between SSR and CSR
4. Dynamic closures create non-deterministic listener IDs
5. Hydration fails with `RuntimeError: unreachable` panic

**Observable symptoms:**
- `RuntimeError: unreachable` in browser console
- Panic occurs only after page load (during hydration)
- Removing `on:click` makes the error disappear
- Error happens even with empty handler: `on:click=|_| {}`

---

## Forbidden Patterns

### Forbidden
```rust
// Event handler in map()
{pillars.into_iter().map(|pillar| {
    view! {
        <button on:click=move |_| {
            log!("Clicked: {}", pillar.label);
        }>
            {pillar.label}
        </button>
    }
}).collect_view()}
```

### Forbidden
```rust
// Event handler in <For>
<For
    each=move || items.get()
    key=|item| item.id
    children=|item| {
        view! {
            <button on:click=move |_| handle(item)>
                "Click"
            </button>
        }
    }
/>
```

### Forbidden
```rust
// Event handler in component instantiated by iterator
{cards.map(|card| {
    view! {
        <Card on_click=move |_| select(card.id) />
    }
})}
```

---

## Canonical Pattern

### Canonical Data Attributes For Event Delegation
```rust
// Use data-* attributes for identification
{pillars.into_iter().map(|pillar| {
    view! {
        <button 
            class="pillar-item"
            attr:data-pillar-id={pillar.id.clone()}
            attr:data-action="select"
        >
            {pillar.label}
        </button>
    }
}).collect_view()}

// Event delegation handled externally (JavaScript or CSR-only wrapper)
```

### Canonical Child Component Pattern
```rust
// Move handler to stable child component
{pillars.into_iter().map(|pillar| {
    view! {
        <PillarItem pillar=pillar />
    }
}).collect_view()}

// Child component with CSR-only interactivity
#[component]
fn PillarItem(pillar: Pillar) -> impl IntoView {
    #[cfg(not(feature = "ssr"))]
    let handler = move |_| {
        log!("Clicked: {}", pillar.label);
    };
    
    view! {
        <button attr:data-pillar-id={pillar.id}>
            {pillar.label}
        </button>
    }
}
```

### Canonical CSR Only Wrapper
```rust
// Render static in SSR, add handlers in CSR
#[cfg(not(feature = "ssr"))]
fn attach_handlers() {
    // JavaScript or WASM event delegation
}
```

---

## Rationale

Leptos 0.7+ uses deterministic DOM walking for hydration. During hydration:

1. The framework walks the DOM tree from SSR
2. For each node, it expects a specific event listener at a specific index
3. Dynamic closures in iterators generate non-deterministic listener IDs
4. The mismatch causes an assertion failure: `unreachable`

This is a known limitation of Leptos SSR hydration and DOM walking. The server cannot serialize JavaScript closures. The client must reconstruct them identically.

**Why this is CRITICAL:**
- Breaks user interactivity silently
- Only appears in production (SSR mode)
- No helpful error message
- Affects all list-based UIs

---

## Enforcement

**Static Analysis:**
- Lint rule: detect `on:*` inside `map()`, `<For>`, or iterators
- Pattern match: `view! { ... on:click ... }` within iterator context

**Runtime:**
- Hydration panic in browser console
- `console_error_panic_hook` shows no Rust stack trace (WASM assertion)

**Code Review:**
- Check for `on:*` in any dynamic list rendering
- Verify CSR-only annotation or data-attributes pattern

---

## Exceptions

**Exception 1: CSR-Only Components**
```rust
#[cfg(not(feature = "ssr"))]
{items.map(|item| {
    view! {
        <button on:click=move |_| handle(item)>"Click"</button>
    }
})}
```
This is acceptable because no SSR hydration occurs.

**Exception 2: Single Static Instance**
If the component is NOT rendered by an iterator, handlers are safe:
```rust
view! {
    <button on:click=move |_| handle()>"Single button"</button>
}
```

---

## Version History

- **1.0.0** — Initial canonical version (2025-01-22)