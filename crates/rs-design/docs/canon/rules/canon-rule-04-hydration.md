# Anti-Hydration Rules (Canon Rule #4)

## Golden Rule
**SSR HTML MUST BE IDENTICAL to first WASM render**

## Common Violations

### ❌ Violation #1: Conditional with Async State
```rust
{if data.is_empty() { 
    view! { "Loading" } 
} else { 
    view! { <Table /> } 
}}
```

**Problem:** SSR renders one thing, client renders another.

### ✅ Solution: <Suspense>
```rust
<Suspense fallback=|| "Loading">
    <Resource ... />
</Suspense>
```

### ❌ Violation #2: <For> in Dropdown
```rust
view! { 
    <DropdownMenuContent>
        <For each=items ... />
    </DropdownMenuContent> 
}
```

**Problem:** <For> renders differently SSR vs client.

### ✅ Solution: Component Isolation
```rust
#[component]
fn DropdownItem(value: String) -> impl IntoView {
    let value = StoredValue::new(value);
    view! { <div>{value.get_value()}</div> }
}

<For
    each=items
    children=|item| view! { <DropdownItem value=item /> }
/>
```

### ❌ Violation #3: RwSignal Props
```rust
#[component]
fn DataTable(data: RwSignal<Vec<T>>) -> impl IntoView {
    // SSR gets empty Vec, client gets populated Vec
}
```

### ✅ Solution: Vec Props
```rust
#[component]
fn DataTable(data: Vec<T>) -> impl IntoView {
    // SSR and client get same snapshot
}
```

## Checklist

- [ ] `<For>` only with isolated components
- [ ] `StoredValue` inside component (not in `.map()`)
- [ ] Use `<Suspense>` for async data
- [ ] Props: `Vec<T>` not `RwSignal<Vec<T>>`
- [ ] No `if/else` with async state
- [ ] No conditional `#[cfg]` inside `view!`

## Debug Hydration Mismatch
```
Error: failed_to_cast_text_node
at tachys::hydration
```

**Steps:**
1. Find the text node in error (e.g., "Columns")
2. Check if SSR and client render same HTML
3. Look for:
   - `<For>` without component
   - `StoredValue` in `.map()`
   - Conditional rendering
   - Effects without `#[cfg]` guard

## SSR Placeholders (Last Resort)
```rust
if leptos::is_server() {
    return view! { <div>"Server placeholder"</div> }.into_any();
}

// Client-only code
view! { <ComplexInteractive /> }.into_any()
```

**Use only when:**
- Component fundamentally incompatible with SSR
- Performance critical (avoid server work)
- Third-party JS library integration
