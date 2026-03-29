# Canon Rule #121: StoredValue for Non-Copy Values in view! Closures

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** leptos, ssr, state
**Version:** 1.0.0
**Date:** 2025-01-22

---

## Principle

**Non-Copy values captured by closures inside `view!` MUST be wrapped in `StoredValue` — never moved directly.**

---

## The Problem

Leptos `view!` macro generates closures that must be `Fn` (callable multiple times). When non-Copy values are moved into these closures, Rust enforces `FnOnce` semantics, causing compile failures.

**The root cause chain:**

1. `view!` needs `Fn` for reactivity (re-render on signal changes)
2. Moving non-Copy values → closure becomes `FnOnce`
3. `FnOnce` cannot be called multiple times → compile error
4. `Clone` doesn't help — still consumes on first call

**Real symptoms from production:**
```rust
error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
  --> src/ui/component.rs:15:5
   |
15 | /     view! {
16 | |         <div>
17 | |             {registry.search()}  // ← moves `registry`
   | |              -------- closure is `FnOnce` because it moves the variable out
18 | |         </div>
19 | |     }
   | |_____^ this closure implements `FnOnce`, not `Fn`
```

**Additional symptoms:**
- `NonNull<()> cannot be shared between threads safely` (Fragment not Send+Sync)
- Closures capturing `String`, `Vec`, custom structs fail
- Works once, fails on re-render

---

## Forbidden Patterns

### ❌ Forbidden: Moving String into view! closure
```rust
#[component]
pub fn Component(title: String) -> impl IntoView {
    view! {
        <div>{title}</div>  // ❌ Moves `title` — FnOnce error
    }
}
```

**Compile error:**
```
error[E0525]: expected closure implementing `Fn`, found `FnOnce`
  because it moves the variable `title` out of its environment
```

### ❌ Forbidden: Moving registry/context into closure
```rust
#[component]
pub fn CommandPalette(registry: CommandRegistry) -> impl IntoView {
    view! {
        {move || {
            let results = registry.search("query");  // ❌ Moves registry
            results.into_iter().map(|r| view! { <div>{r}</div> }).collect_view()
        }}
    }
}
```

**Why this fails:**
- `registry` is not `Copy`
- Closure moves `registry` on first call
- Second render attempts to move again → already moved
- `FnOnce` violation

### ❌ Forbidden: Clone inside closure (still FnOnce)
```rust
#[component]
pub fn Component(data: Vec<String>) -> impl IntoView {
    view! {
        {move || {
            data.clone()  // ❌ Still moves `data` into closure environment
                .into_iter()
                .map(|s| view! { <div>{s}</div> })
                .collect_view()
        }}
    }
}
```

**Why this fails:**
- `.clone()` happens *inside* closure
- Closure still captures `data` by move
- `FnOnce` violation remains

### ❌ Forbidden: Invoking Children inside view!
```rust
#[component]
pub fn Wrapper(children: Children) -> impl IntoView {
    view! {
        <div>
            {children()}  // ❌ Moves `children` (Fragment)
        </div>
    }
}
```

**Why this fails:**
- `children()` returns `Fragment`
- `Fragment` is not `Send + Sync` (contains `NonNull<()>`)
- Cannot be captured in reactive closure

---

## Canonical Pattern

### ✅ Canonical: Wrap non-Copy values in StoredValue
```rust
#[component]
pub fn Component(title: String) -> impl IntoView {
    let title = StoredValue::new(title);  // ✅ Store before view!
    
    view! {
        <div>{move || title.get_value()}</div>  // ✅ Access via closure
    }
}
```

**Why this works:**
- `StoredValue` is `Copy`
- Closure captures copy of `StoredValue`, not the `String`
- `get_value()` retrieves value on each render
- Closure remains `Fn`

### ✅ Canonical: Registry in StoredValue
```rust
#[component]
pub fn CommandPalette(registry: CommandRegistry) -> impl IntoView {
    let registry = StoredValue::new(registry);  // ✅ Store once
    
    view! {
        {move || {
            let results = registry.with_value(|r| r.search("query"));
            results.into_iter().map(|r| view! { <div>{r}</div> }).collect_view()
        }}
    }
}
```

**Why this works:**
- `StoredValue<CommandRegistry>` is `Copy`
- `with_value()` provides safe access to inner value
- Closure can be called multiple times

### ✅ Canonical: Children with ChildrenFn + StoredValue
```rust
#[component]
pub fn Wrapper(children: ChildrenFn) -> impl IntoView {
    let children = StoredValue::new(children);  // ✅ Store ChildrenFn
    
    view! {
        <div>
            {move || children.with_value(|c| c())}  // ✅ Invoke via closure
        </div>
    }
}
```

**Why this works:**
- `ChildrenFn` is `Arc<dyn Fn() -> View>`, not `Copy`
- `StoredValue` makes it capturable
- Closure invokes function on each render

### ✅ Canonical: Multiple non-Copy values
```rust
#[component]
pub fn Complex(
    title: String,
    items: Vec<Item>,
    config: AppConfig,
) -> impl IntoView {
    let title = StoredValue::new(title);
    let items = StoredValue::new(items);
    let config = StoredValue::new(config);
    
    view! {
        <div>
            <h1>{move || title.get_value()}</h1>
            {move || items.with_value(|i| {
                i.iter().map(|item| view! { <div>{item.name}</div> }).collect_view()
            })}
        </div>
    }
}
```

---

## Rationale

### Why This Rule Exists

1. **Leptos reactivity model:**
   - `view!` generates `Fn` closures for re-rendering
   - Non-Copy values violate `Fn` contract
   - `StoredValue` is the official escape hatch

2. **SSR + Hydration safety:**
   - `StoredValue` works in both SSR and client contexts
   - Provides stable identity across renders
   - Thread-safe (when inner value is `Send + Sync`)

3. **Architectural boundary:**
   - Signals → reactive primitives (Copy)
   - StoredValue → non-reactive storage (Copy wrapper)
   - Raw values → must be wrapped or cloned outside `view!`

### What This Rule Protects

- **Closure trait correctness** — `Fn` vs `FnOnce` invariants
- **Re-render safety** — closures callable multiple times
- **SSR compatibility** — no client-only assumptions
- **Type system consistency** — explicit ownership semantics

---

## Enforcement

### Static Analysis
```rust
// Linter pseudocode
for component in all_components {
    for value in captured_values_in_view_macro(component) {
        if !value.is_copy() && !value.is_signal() && !value.is_stored_value() {
            emit_error!(
                "Canon Rule #121: Non-Copy value '{}' must be wrapped in StoredValue",
                value.name
            );
        }
    }
}
```

### CI Check
```bash
# Check for common FnOnce errors
cargo check 2>&1 | grep "expected.*Fn.*found.*FnOnce" && \
    echo "❌ Canon Rule #121: Use StoredValue for non-Copy values" && exit 1
```

### Code Review Checklist

- [ ] All non-Copy props wrapped in `StoredValue` before `view!`
- [ ] No `String`, `Vec`, custom structs captured directly
- [ ] `ChildrenFn` always in `StoredValue` if passed to child components
- [ ] No `Fragment` captured in reactive closures

---

## Exceptions

### Exception 1: Values cloned OUTSIDE view!
```rust
#[component]
pub fn Component(title: String) -> impl IntoView {
    let title_clone = title.clone();  // ✅ Clone outside
    
    view! {
        <div>{title_clone}</div>  // ✅ Moved once, not re-captured
    }
}
```

**When allowed:**
- Clone happens before `view!`
- Value used exactly once in template
- No reactive closure needed

### Exception 2: Copy types (never need StoredValue)
```rust
#[component]
pub fn Component(count: i32, enabled: bool) -> impl IntoView {
    view! {
        <div>
            {move || count}  // ✅ i32 is Copy
            {move || enabled}  // ✅ bool is Copy
        </div>
    }
}
```

### Exception 3: Signals (already reactive)
```rust
#[component]
pub fn Component(count: Signal<i32>) -> impl IntoView {
    view! {
        <div>{move || count.get()}</div>  // ✅ Signal is Copy
    }
}
```

**All other cases require `StoredValue`.**

---

## Quick Reference

| Type | Copy? | Needs StoredValue? |
|------|-------|-------------------|
| `i32`, `bool`, `f64` | ✅ | ❌ No |
| `Signal<T>` | ✅ | ❌ No |
| `String`, `Vec<T>` | ❌ | ✅ Yes |
| `ChildrenFn` | ❌ | ✅ Yes |
| `Fragment` | ❌ | ✅ Yes (or invoke before `view!`) |
| Custom structs | ❌ | ✅ Yes (unless #[derive(Copy)]) |

---

## Related Rules

- **Canon Rule #63:** Leptos Reactivity Closures (Fn vs FnOnce)
- **Canon Rule #86:** Children vs ChildrenFn Contract
- **Canon Rule #98:** Axum-Leptos SSR Closures Must Own State

---

## Version History

- **1.0.0** — Initial canonical version (2025-01-22)
