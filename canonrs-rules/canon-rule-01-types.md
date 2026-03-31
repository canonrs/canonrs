# Canon Rule #01: Component Types

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** component-architecture
**Tags:** components, ssr, state, effects
**Language:** EN

---

**Intro:**
Incorrect classification of components in Leptos leads to SSR issues, unsafe browser API usage, and inconsistent state handling. Mixing state, effects, and DOM APIs without clear boundaries breaks hydration and component predictability.

**Problem:**
components mix state, effects, and browser APIs without clear classification

**Solution:**
classify components into pure, stateful, or interactive with strict rules

**Signals:**
- hydration errors
- unsafe effects
- browser api panic

**Search Intent:**
how to classify leptos components

**Keywords:**
leptos component types, ssr component classification, leptos state vs effects, interactive component leptos

---

# Classification

| Type                     | State      | Effects | APIs Browser | Exemple         |
| ------------------------ | ----------- | ------- | ------------ | --------------- |
| **Type 1 - Pure**        | ❌          | ❌      | ❌           | Label, Badge    |
| **Type 2 - Stateful**    | ✅ RwSignal | ❌      | ❌           | Toggle, Tabs    |
| **Type 3 - Interactive** | ✅          | ✅      | ✅           | Popover, Dialog |

## Type 1 - Pure Components
```rust
#[component]
pub fn Label(children: Children) -> impl IntoView {
    view! { <label>{children()}</label> }
}
```

### Characteristics:
- No internal state
- No side effects
- SSR-safe by default

## Type 2 - Stateful Components
```rust
#[component]
pub fn Toggle(checked: RwSignal<bool>) -> impl IntoView {
    view! {
        <button on:click=move |_| checked.update(|v| *v = !*v)>
            {move || if checked.get() { "On" } else { "Off" }}
        </button>
    }
}
```

### Characteristics:
- Uses `RwSignal` for state
- No browser APIs
- No Effects

## Type 3 - Interactive Components
```rust
#[component]
pub fn Dialog(open: RwSignal<bool>) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if open.get() {
            // Focus trap, scroll lock, etc.
        }
    });
    
    view! { ... }
}
```

### Characteristics:
- Uses Effects with `#[cfg]` guards
- May use browser APIs (document, window)
- SSR placeholder when needed

###  Decision Tree
```
Has internal state? 
  ├─ No  → Type 1
  └─ Yes → Uses Effects or Browser APIs?
            ├─ No  → Type 2
            └─ Yes → Type 3
```