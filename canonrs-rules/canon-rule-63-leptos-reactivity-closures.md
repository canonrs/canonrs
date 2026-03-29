# Canon Rule #63: Leptos Reactivity - No .get() Outside Closures

**Status:** ENFORCED


**Severity:** HIGH
**Scope:** leptos, state
**Version:** 1.0.0
**Date:** 2025-01-16

---
  

---

## The Principle

**Reactive values MUST be accessed inside closures when used in reactive contexts.**

`Signal<T>` and `Memo<T>` are NOT the same type. Their usage patterns differ critically.

---

## The Problem

### ❌ Wrong Pattern (Breaks Reactivity)
```rust
let tokens = Memo::new(move |_| fetch_tokens());

// WRONG: .get() outside tracking context
let count = tokens.get().len();

view! {
    <p>{count}</p>  // 🚫 Never updates
}
```

**Error message:**
```
you access a reactive_graph::computed::arc_memo::ArcMemo<T> 
outside a reactive tracking context
```

**Why this fails:**
- `.get()` was called during component initialization
- Leptos didn't track the dependency
- Changes to `tokens` never trigger re-render
- UI becomes stale

---

## The Solution

### ✅ Correct Pattern (Reactive)
```rust
let tokens = Memo::new(move |_| fetch_tokens());

view! {
    <p>{move || tokens.get().len()}</p>  // ✅ Reactive
}
```

**Why this works:**
- `move ||` creates closure
- Leptos tracks `tokens` as dependency
- Changes trigger re-render automatically
- UI stays synchronized

---

## Signal vs Memo: Critical Difference

### Signal<T> (Callable)
```rust
let count = RwSignal::new(0);

// ✅ BOTH work for Signal<T>
view! {
    <p>{count}</p>           // ✅ Signal is Fn() -> T
    <p>{move || count.get()}</p>  // ✅ Also works
}
```

### Memo<T> (NOT Callable)
```rust
let doubled = Memo::new(move |_| count.get() * 2);

// ❌ WRONG
view! {
    <p>{doubled}</p>  // 🚫 Memo is NOT Fn()
}

// ✅ CORRECT
view! {
    <p>{move || doubled.get()}</p>  // ✅ Must use closure
}
```

---

## Common Scenarios

### 1. Component Props

#### ❌ Wrong: Passing Memo directly
```rust
#[component]
pub fn DataTable<T>(
    data: Vec<T>,  // Expects Vec, not Memo
) -> impl IntoView { ... }

// WRONG usage
let filtered = Memo::new(move |_| filter_items());
view! {
    <DataTable data=filtered/>  // 🚫 Type mismatch
}
```

#### ✅ Correct: Extract value in closure
```rust
view! {
    <DataTable data=move || filtered.get()/>  // ✅ Passes Vec<T>
}
```

Or better:
```rust
// Change DataTable to accept reactive prop
#[component]
pub fn DataTable<T>(
    #[prop(into)] data: Signal<Vec<T>>,  // Accepts reactive types
) -> impl IntoView { ... }
```

### 2. Show Component

#### ❌ Wrong: Condition without closure
```rust
let has_data = Memo::new(move |_| items.get().len() > 0);

// WRONG
view! {
    <Show when={has_data}>  // 🚫 Memo is not Fn() -> bool
        <Table/>
    </Show>
}
```

#### ✅ Correct: Wrap in closure
```rust
view! {
    <Show when={move || has_data.get()}>  // ✅ Closure returns bool
        <Table/>
    </Show>
}
```

### 3. Event Handlers

#### ❌ Wrong: Access outside handler
```rust
let selected = Memo::new(move |_| get_selected_items());

// WRONG
let count = selected.get().len();  // 🚫 Called at init time
let on_click = move |_| {
    log!("Selected: {}", count);  // Stale value
};
```

#### ✅ Correct: Access inside handler
```rust
let on_click = move |_| {
    let count = selected.get().len();  // ✅ Fresh value
    log!("Selected: {}", count);
};
```

### 4. Suspense Fallback

#### ❌ Wrong: Eager evaluation
```rust
<Suspense fallback=|| view! { <p>"Loading..."</p> }>
    {move || {
        if tokens.get().is_empty() {  // ✅ Inside closure (correct)
            view! { <p>"No data"</p> }
        } else {
            view! { <DataTable data=tokens/> }  // 🚫 Memo passed directly
        }
    }}
</Suspense>
```

#### ✅ Correct: Extract value properly
```rust
<Suspense fallback=|| view! { <p>"Loading..."</p> }>
    {move || {
        let items = tokens.get();  // Extract once
        if items.is_empty() {
            view! { <p>"No data"</p> }
        } else {
            view! { <DataTable data=items/>}  // ✅ Pass Vec<T>
        }
    }}
</Suspense>
```

---

## Implementation Checklist

When using reactive values:

- [ ] `Signal<T>` can be used directly in `view!`
- [ ] `Memo<T>` MUST be wrapped in `move ||` closure
- [ ] Props expecting `Vec<T>` get `memo.get()`, not `memo`
- [ ] `<Show when={}>` always receives `Fn() -> bool`
- [ ] Event handlers access reactive values inside closure
- [ ] Test: No "outside reactive tracking context" errors

---

## Debugging Guide

### Error: "accessed outside reactive tracking context"

**Step 1: Find the line**
```
At src/components/table.rs:42:21, you access a 
reactive_graph::computed::arc_memo::ArcMemo<Vec<Item>>
```

**Step 2: Check for naked .get()**
```rust
// Line 42
let items = filtered_items.get();  // 🚫 Found it
```

**Step 3: Wrap in closure**
```rust
// Fix
view! {
    <div>{move || filtered_items.get().len()}</div>
}
```

### Error: "Type mismatch: expected Vec<T>, found Memo"

**Step 1: Find component call**
```rust
<DataTable data=my_memo/>  // 🚫 Wrong type
```

**Step 2: Extract value**
```rust
<DataTable data=my_memo.get()/>  // ✅ Fixed
```

**Step 3: Or change prop type**
```rust
#[component]
fn DataTable(
    #[prop(into)] data: Signal<Vec<T>>  // Accepts Memo now
) { ... }
```

---

## Anti-Patterns to Avoid

### 🚫 Eager Computation
```rust
// WRONG: Computed at init time
let count = items.get().len();

view! {
    <p>{count}</p>  // Never updates
}
```

### 🚫 Memo as Callable
```rust
// WRONG: Memo is not Fn()
<Show when={is_visible}>  // 🚫 Memo<bool> is not Fn() -> bool
```

### 🚫 Untracked in Reactive Context
```rust
// WRONG: Breaks reactivity intentionally
view! {
    <p>{items.get_untracked().len()}</p>  // 🚫 Never updates
}
```

---

## Correct Patterns Reference

### Pattern 1: Display Reactive Value
```rust
view! {
    <p>{move || memo.get()}</p>
}
```

### Pattern 2: Conditional Rendering
```rust
<Show
    when={move || memo.get().len() > 0}
    fallback={|| view! { <p>"Empty"</p> }}
>
    <List/>
</Show>
```

### Pattern 3: Pass to Component
```rust
// Option A: Extract value
<MyComponent data=memo.get()/>

// Option B: Accept Signal
#[component]
fn MyComponent(#[prop(into)] data: Signal<Vec<T>>) { ... }
<MyComponent data=memo/>
```

### Pattern 4: Event Handler
```rust
let on_click = move |_| {
    let value = memo.get();  // Fresh value
    do_something(value);
};
```

---

## Testing
```rust
#[test]
fn test_reactivity() {
    let count = RwSignal::new(0);
    let doubled = Memo::new(move |_| count.get() * 2);
    
    // ✅ Test that Memo reacts to Signal changes
    assert_eq!(doubled.get(), 0);
    count.set(5);
    assert_eq!(doubled.get(), 10);
}
```

---

## Related Rules

- **Rule #49:** Drag & Drop as Intent (uses Callback pattern)
- **Rule #XX:** Component Props (pending)
- **Rule #XX:** Event Handlers (pending)

---

## Normative Status

- Violations **MUST** block PRs
- Compiler errors related to reactivity **MUST** be fixed before merge
- `Memo<T>` **MUST NOT** be used where `Fn() -> T` is expected
- All reactive values in `view!` **MUST** be in closures (except `Signal<T>`)

---

**Author:** Canon Working Group  
**Replaces:** None

---

## Economic Impact

**Time saved per incident:** ~1 hour  
**Frequency without rule:** Every component with Memo  
**Annual savings (50 components):** ~50 hours

**Root causes eliminated:**
- ❌ "outside reactive tracking context" errors
- ❌ Stale UI that doesn't update
- ❌ Type mismatches with component props
- ❌ Confusion between Signal and Memo
