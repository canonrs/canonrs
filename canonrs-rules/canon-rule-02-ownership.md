# Canon Rule #02: Ownership Rules

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** state-reactivity
**Tags:** ownership, signals, storedvalue, closures
**Language:** EN

---

**Intro:**
Incorrect ownership handling in Leptos causes move errors, broken reactivity, and invalid closures. Passing non-reactive data or using improper children types leads to rendering inconsistencies and runtime failures.

**Problem:**
improper ownership and reactive patterns break closures and rendering

**Solution:**
use StoredValue, ChildrenFn, and move closures for correct ownership and reactivity

**Signals:**
- move error
- fnonce error
- stale data

**Search Intent:**
how to fix leptos ownership errors

**Keywords:**
leptos ownership rules, storedvalue leptos usage, childrenfn leptos reactivity, leptos move closure pattern

---

## Rule #1: StoredValue is DEFAULT

### Wrong
```rust
let label = "Users".to_string();
view! { <span>{label}</span> } // Move error
```

### Correct
```rust
let label = StoredValue::new("Users".to_string());
view! { <span>{label.get_value()}</span> }
```

**When to use StoredValue:**
- Any non-reactive data passed to closures
- Props that don't need reactivity
- Callbacks stored for later use

## Rule #2: ChildrenFn for Re-render

### Wrong
```rust
#[component]
pub fn Show(children: Children) -> impl IntoView {
    // FnOnce - can only render once
}
```

### Correct
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

### Wrong
```rust
view! { <div hidden=is_open.get() /> }
```

### Correct
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