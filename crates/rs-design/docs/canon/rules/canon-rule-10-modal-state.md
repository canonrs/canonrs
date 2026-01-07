# Modal Reactive State Management (Canon Rule #10)

**Status:** Normative  
**Applies to:** Dialog, Modal, Drawer, Sheet - any component with reactive props  
**Date:** 2025-12-30

---

## The Problem

Passing `String` props to modals = **values never update**:
```rust
// ❌ WRONG
#[component]
fn EditDialog(
    open: RwSignal<bool>,
    name: String,  // ← Static! Won't react
) -> impl IntoView {
    view! {
        <Input value=name />  // ← Never updates when parent changes
    }
}
```

**Why it fails:**
- Props are **captured at creation time**
- Parent signal changes don't propagate
- Modal always shows stale data

---

## ✅ The Correct Solution: Signal Props

### Pattern
```rust
#[component]
pub fn EditDialog(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into)] name: Signal<String>,      // ← Signal, not String
    #[prop(into)] value: Signal<String>,
    // ... other fields
) -> impl IntoView {
    // Local editable state
    let local_name = RwSignal::new(String::new());
    let local_value = RwSignal::new(String::new());
    
    // Sync when modal opens
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if open.get() {
            local_name.set(name.get());
            local_value.set(value.get());
        }
    });
    
    let on_save = move |_| {
        leptos::logging::log!("Saving: {}, {}", local_name.get(), local_value.get());
        open.set(false);
    };
    
    view! {
        <Dialog open=open>
            <DialogContent>
                <Input value=local_name />
                <Input value=local_value />
                <Button on:click=on_save>"Save"</Button>
            </DialogContent>
        </Dialog>
    }
}
```

### Parent Component
```rust
#[component]
pub fn ParentComponent() -> impl IntoView {
    let edit_open = RwSignal::new(false);
    let edit_name = RwSignal::new(String::new());
    let edit_value = RwSignal::new(String::new());
    
    view! {
        <Button on:click=move |_| {
            edit_name.set("New Name".to_string());
            edit_value.set("New Value".to_string());
            edit_open.set(true);
        }>
            "Edit"
        </Button>
        
        <EditDialog
            open=edit_open
            name=Signal::derive(move || edit_name.get())     // ← Signal::derive
            value=Signal::derive(move || edit_value.get())
        />
    }
}
```

---

## Why This Works

1. **`Signal<T>` props** = reactive by design
2. **`Signal::derive`** = creates reactive binding
3. **Effect with guard** = syncs safely (SSR-safe)
4. **Local state** = user edits don't affect parent until save

---

## Critical Requirements

### ✅ DO
```rust
// Props
#[prop(into)] data: Signal<String>

// Parent call
<Modal data=Signal::derive(move || state.get()) />

// Effect guard
#[cfg(target_arch = "wasm32")]
Effect::new(move |_| { ... });
```

### ❌ DON'T
```rust
// Props
data: String  // ← Static

// Parent call
<Modal data=state.get() />  // ← No reactivity

// Effect without guard
Effect::new(move |_| { ... });  // ← SSR panic
```

---

## Common Patterns

### Multiple Fields
```rust
#[component]
pub fn EditTokenDialog(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into)] token_id: Signal<String>,
    #[prop(into)] token_name: Signal<String>,
    #[prop(into)] token_value: Signal<String>,
    #[prop(into)] token_scope: Signal<String>,
    #[prop(into)] token_category: Signal<String>,
    #[prop(into)] token_status: Signal<String>,
) -> impl IntoView {
    let name = RwSignal::new(String::new());
    let value = RwSignal::new(String::new());
    let scope = RwSignal::new(String::new());
    let category = RwSignal::new(String::new());
    let status = RwSignal::new(String::new());

    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if open.get() {
            name.set(token_name.get());
            value.set(token_value.get());
            scope.set(token_scope.get());
            category.set(token_category.get());
            status.set(token_status.get());
        }
    });
    
    // ... rest
}
```

### Optional Fields
```rust
#[component]
pub fn EditDialog(
    #[prop(into)] open: RwSignal<bool>,
    #[prop(into, optional)] description: Option<Signal<String>>,
) -> impl IntoView {
    let local_desc = RwSignal::new(String::new());
    
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if open.get() {
            if let Some(desc) = description {
                local_desc.set(desc.get());
            }
        }
    });
    
    // ...
}
```

---

## Decision Matrix

| Scenario | Prop Type | Pattern |
|----------|-----------|---------|
| Modal shows data | `Signal<T>` | ✅ This rule |
| Modal edits data | `Signal<T>` + local state | ✅ This rule |
| Static label | `String` | ✅ OK (never changes) |
| Computed value | `Signal::derive` | ✅ This rule |
| Optional field | `Option<Signal<T>>` | ✅ This rule |

---

## Real Production Example
```rust
// Parent: TokensTable
DataTableColumn::actions(move |t: &TokenRow| {
    let token_id = t.id.clone();
    let token_name = t.full_name.clone();
    let token_value = t.value.clone();
    // ...
    
    vec![
        RowAction {
            label: "Edit line".to_string(),
            on_click: Callback::new(move |_| {
                edit_token_id.set(token_id.clone());
                edit_token_name.set(token_name.clone());
                edit_token_value.set(token_value.clone());
                // ...
                edit_dialog_open.set(true);
            }),
            variant: RowActionVariant::Default,
        },
    ]
})

// Later in view!
<EditTokenDialog
    open=edit_dialog_open
    token_id=Signal::derive(move || edit_token_id.get())
    token_name=Signal::derive(move || edit_token_name.get())
    token_value=Signal::derive(move || edit_token_value.get())
    // ...
/>
```

---

## Normative Status

- Violations **MUST** block PRs
- Modal props **MUST** use `Signal<T>` for reactive data
- Effect sync **MUST** have `#[cfg(target_arch = "wasm32")]` guard
- Exceptions require explicit justification

---

**Author:** Canon Working Group  
**Date:** 2025-12-30  
**Version:** 1.0  
**Status:** Production-Critical
