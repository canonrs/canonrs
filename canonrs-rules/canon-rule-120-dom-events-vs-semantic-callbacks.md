# Canon Rule #120: DOM Events vs Semantic Callbacks Boundary

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** primitives, ui, behavior
**Version:** 1.0.0
**Date:** 2025-01-22

---

## Principle

**Primitives use `on:event` (DOM events). UI components use `on_event` (semantic callbacks). Never wrap callbacks in `Some()` at UI layer.**

---

## The Problem

Mixing DOM event syntax with callback props creates three structural failures:

1. **Type confusion:** `on:click` vs `on_click` vs `onclick` — unclear boundaries
2. **Unnecessary wrapping:** `Some(Callback::new(...))` patterns proliferate
3. **Primitive leakage:** DOM events appear in UI layer public APIs

**Real symptoms from production:**
```rust
error[E0308]: mismatched types
  --> src/ui/button.rs:15:21
   |
15 |         on_click=Some(Callback::new(move |_| handle_click()))
   |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |                 expected `Callback<MouseEvent>`, found `Option<Callback<_>>`
```

This error appears when developers confuse primitive-level optional props with UI-level semantic handlers.

---

## Forbidden Patterns

### ❌ Forbidden: DOM events in UI layer public API
```rust
// UI Component (WRONG)
#[component]
pub fn Button(
    children: Children,
    on:click: Callback<MouseEvent>,  // ❌ DOM event in UI API
) -> impl IntoView {
    view! {
        <ButtonPrimitive on:click=on:click>
            {children()}
        </ButtonPrimitive>
    }
}
```

**Why this fails:**
- Exposes DOM implementation detail
- Forces users to think about `MouseEvent`
- Breaks abstraction boundary

### ❌ Forbidden: Wrapping callbacks in Some()
```rust
// UI Component (WRONG)
#[component]
pub fn TreeNode(
    on_select: Callback<String>,
) -> impl IntoView {
    view! {
        <div on:click=Some(Callback::new(move |_| {  // ❌ NEVER DO THIS
            on_select.run("id".to_string());
        }))>
            "Node"
        </div>
    }
}
```

**Why this fails:**
- `Some()` wrapper is redundant
- Should pass callback directly or use `on:click` syntax
- Creates type confusion

### ❌ Forbidden: Optional callbacks as Option<Callback<T>>
```rust
// UI Component (WRONG)
#[component]
pub fn CommandItem(
    #[prop(optional)] on_click: Option<Callback<MouseEvent>>,  // ❌ WRONG TYPE
) -> impl IntoView {
    view! {
        <button on:click=move |ev| {
            if let Some(ref handler) = on_click {  // ❌ Awkward pattern
                handler.run(ev);
            }
        }>
            "Item"
        </button>
    }
}
```

**Why this fails:**
- Makes optional callbacks awkward to use
- Should use `#[prop(optional)]` with non-Option type
- Or provide no-op default

---

## Canonical Pattern

### ✅ Canonical: Primitives use on:event
```rust
// Primitive (CORRECT)
#[component]
pub fn ButtonPrimitive(
    children: ChildrenFn,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    view! {
        <button 
            data-button=""
            class={class}
            // DOM events available implicitly via Leptos
        >
            {children()}
        </button>
    }
}
```

**Usage in UI layer:**
```rust
view! {
    <ButtonPrimitive on:click=move |_| handle_click()>  // ✅ Direct DOM event
        "Click me"
    </ButtonPrimitive>
}
```

### ✅ Canonical: UI components use semantic callbacks
```rust
// UI Component (CORRECT)
#[component]
pub fn Button(
    children: Children,
    #[prop(optional)] on_click: Option<Callback<()>>,  // ✅ Semantic, unit type
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let on_click_handler = on_click.unwrap_or_else(|| Callback::new(|_| {}));
    
    view! {
        <ButtonPrimitive on:click=move |_| {
            if !disabled {
                on_click_handler.run(());  // ✅ Semantic unit event
            }
        }>
            {children()}
        </ButtonPrimitive>
    }
}
```

**Usage in application:**
```rust
view! {
    <Button on_click=Callback::new(|_| save_form())>  // ✅ Clean semantic API
        "Save"
    </Button>
}
```

### ✅ Canonical: Optional semantic callbacks with default
```rust
// UI Component (CORRECT)
#[component]
pub fn TreeNode(
    node_id: String,
    #[prop(default = Callback::new(|_| {}))] on_select: Callback<String>,  // ✅ Default no-op
) -> impl IntoView {
    view! {
        <div on:click=move |_| on_select.run(node_id.clone())>
            "Node"
        </div>
    }
}
```

**Why this works:**
- No `Option<>` wrapping needed
- No `if let Some` patterns
- Clean call site: `<TreeNode on_select=my_handler />`

---

## Rationale

### Architectural Clarity

1. **Primitives = DOM boundary**
   - Only primitives touch raw HTML elements
   - Only primitives use `on:event` syntax
   - Primitives are SSR-safe, stateless

2. **UI = Semantic boundary**
   - UI components expose business logic events
   - Use `on_event` (underscore) for semantic callbacks
   - Type: `Callback<T>` where `T` is domain-specific

3. **Separation of concerns**
   - DOM events (`MouseEvent`, `KeyboardEvent`) stay in primitives
   - Semantic events (`String`, `()`, domain types) in UI layer
   - Application code never imports `web_sys` types

### What This Rule Protects

- **API clarity:** Users know what layer they're in by syntax
- **Type safety:** No `Option<Callback<T>>` awkwardness
- **SSR compatibility:** DOM events never leak to server-side
- **Maintenance:** Clear refactoring boundaries

---

## Enforcement

### Pattern Matching (Linter)
```rust
// Check 1: No on:event in UI component props
if in_ui_layer(component) {
    for prop in component.props {
        if prop.name.starts_with("on:") {
            emit_error!("Canon Rule #120: Use on_event, not on:event");
        }
    }
}

// Check 2: No Some(Callback::new(...))
if code.contains("Some(Callback::new(") {
    emit_warning!("Canon Rule #120: Pass callback directly, remove Some()");
}
```

### CI Check
```bash
# Grep for forbidden patterns
rg 'on:.* = .*Callback' packages-rust/rs-design/src/ui/ && exit 1
rg 'Some\(Callback::new' packages-rust/rs-design/src/ui/ && exit 1
```

---

## Exceptions

**No exceptions. This rule is absolute.**

If a UI component needs DOM-level control, it's misclassified — it should be a primitive.

---

## Examples Summary

| Layer | Event Type | Syntax | Example |
|-------|-----------|---------|---------|
| **Primitive** | DOM | `on:click` | `<button on:click=handler>` |
| **UI** | Semantic | `on_click` | `<Button on_click=handler>` |
| **Application** | Business | `on_save` | `<Form on_save=save_fn>` |

---

## Related Rules

- **Canon Rule #75:** Primitives have zero styling
- **Canon Rule #89:** Primitives are SSR-safe
- **Canon Rule #119:** No `#[prop(optional, into)]` in UI layer

---

## Version History

- **1.0.0** — Initial canonical version (2025-01-22)
