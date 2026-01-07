# Lists & Iteration (Canon Rule #3)

## The FnOnce Hell Problem

### ❌ NEVER DO THIS
```rust
items.iter().map(|item| {
    view! { <Item on_click=callback /> } // FnOnce error
}).collect_view()
```

### ❌ ALSO WRONG
```rust
items.iter().map(|item| {
    let val = StoredValue::new(item.value); // Hydration mismatch
    view! { <Item /> }
}).collect_view()
```

## ✅ CANONICAL SOLUTION

### Step 1: Isolated Component
```rust
#[component]
fn ItemComponent(
    value: String,
    on_select: Callback<String>,
) -> impl IntoView {
    let val = StoredValue::new(value); // ✅ Inside component
    
    view! {
        <div on:click=move |_| on_select.run(val.get_value())>
            {val.get_value()}
        </div>
    }
}
```

### Step 2: Use <For>
```rust
<For
    each=move || items.get_value()
    key=|item| item.id.clone()
    children=move |item| {
        view! {
            <ItemComponent
                value=item.value
                on_select=callback
            />
        }
    }
/>
```

## Pattern Rules

1. **Component Isolation:** Every list item = separate component
2. **StoredValue Placement:** Inside the component (not in `.map()`)
3. **<For> Only:** Never use `.iter().map().collect_view()`
4. **Key Function:** Must be unique and stable

## Advanced: Nested Lists
```rust
#[component]
fn CategoryRow(category: Category, on_select: Callback<String>) -> impl IntoView {
    let category = StoredValue::new(category);
    
    view! {
        <div>
            <h3>{category.get_value().name}</h3>
            <For
                each=move || category.get_value().items
                key=|item| item.id
                children=move |item| {
                    view! {
                        <ItemComponent value=item.value on_select=on_select />
                    }
                }
            />
        </div>
    }
}
```

## Common Mistakes

### Callback in closure
```rust
// ❌ WRONG
<For
    children=move |item| {
        view! { <div on:click=callback /> } // Moved
    }
/>

// ✅ CORRECT - Component receives callback as prop
<For
    children=move |item| {
        view! { <ItemComponent on_click=callback /> }
    }
/>
```
