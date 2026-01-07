# Component Types (Canon Rule #1)

## Classification

| Type                     | Estado      | Effects | Browser APIs | Exemplo         |
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

**Characteristics:**
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

**Characteristics:**
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

**Characteristics:**
- Uses Effects with `#[cfg]` guards
- May use browser APIs (document, window)
- SSR placeholder when needed

## Decision Tree
```
Has internal state? 
  ├─ No  → Type 1
  └─ Yes → Uses Effects or Browser APIs?
            ├─ No  → Type 2
            └─ Yes → Type 3
```
