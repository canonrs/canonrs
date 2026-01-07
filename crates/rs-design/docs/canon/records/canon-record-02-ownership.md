# Ownership Rules (Canon Rule #2)

## Rule #1: StoredValue is DEFAULT

### ❌ WRONG
```rust
let label = "Users".to_string();
view! { <span>{label}</span> } // Move error
```

### ✅ CORRECT
```rust
let label = StoredValue::new("Users".to_string());
view! { <span>{label.get_value()}</span> }
```

**When to use StoredValue:**
- Any non-reactive data passed to closures
- Props that don't need reactivity
- Callbacks stored for later use

## Rule #2: ChildrenFn for Re-render

### ❌ WRONG
```rust
#[component]
pub fn Show(children: Children) -> impl IntoView {
    // FnOnce - can only render once
}
```

### ✅ CORRECT
```rust
#[component]
pub fn Show(children: ChildrenFn) -> impl IntoView {
    // Can re-render when reactive dependencies change
}
```

**Use ChildrenFn in:**
- `Show`, `Suspense`, `Transition`
- Any component that conditionally renders children
- Components with reactive visibility

## Rule #3: move || for Reactive Access

### ❌ WRONG
```rust
view! { <div hidden=is_open.get() /> }
```

### ✅ CORRECT
```rust
view! { <div hidden=move || is_open.get() /> }
```

**Pattern:**
- Always wrap signal access in `move ||`
- Leptos tracks reactive dependencies automatically

## Common Pitfalls

### Callback Cloning
```rust
// ❌ WRONG
let cb = callback.clone();
view! { <button on:click=move |_| cb.run() /> }

// ✅ CORRECT
let cb = StoredValue::new(callback);
view! { <button on:click=move |_| cb.get_value().run() /> }
```

### Arc Sharing
```rust
// ✅ CORRECT for shared immutable data
let columns = Arc::new(columns);
let columns_clone = Arc::clone(&columns);
```
