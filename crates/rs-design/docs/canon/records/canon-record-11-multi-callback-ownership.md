# Multi-Callback Ownership (Canon Rule #11)

**Status:** Normative  
**Applies to:** Actions, dropdowns, any component with multiple closures  
**Date:** 2025-12-30

---

## The Problem

**Multiple callbacks = moved value errors:**
```rust
// ❌ WRONG
DataTableColumn::actions(|t: &TokenRow| {
    let name = t.name.clone();
    
    vec![
        RowAction {
            on_click: Callback::new(move |_| {
                copy_to_clipboard(&name);  // ← name moved here
            }),
        },
        RowAction {
            on_click: Callback::new(move |_| {
                edit_signal.set(name.clone());  // ← ERROR: name already moved
            }),
        },
    ]
})
```

**Error:**
```
error[E0382]: use of moved value: `name`
help: consider cloning the value before moving it
```

---

## ✅ The Correct Solution: Clone Before Each Callback

### Pattern
```rust
DataTableColumn::actions(move |t: &TokenRow| {  // ← move here
    // Extract all values
    let name = t.name.clone();
    let value = t.value.clone();
    let id = t.id.clone();
    
    // Clone for EACH callback
    let name_copy = name.clone();
    let value_copy = value.clone();
    let name_edit = name.clone();
    let value_edit = value.clone();
    let id_edit = id.clone();
    
    vec![
        RowAction {
            label: "Copy name".to_string(),
            on_click: Callback::new(move |_| {
                copy_to_clipboard(&name_copy);  // ← uses clone
            }),
        },
        RowAction {
            label: "Copy value".to_string(),
            on_click: Callback::new(move |_| {
                copy_to_clipboard(&value_copy);  // ← uses clone
            }),
        },
        RowAction {
            label: "Edit".to_string(),
            on_click: Callback::new(move |_| {
                edit_name.set(name_edit.clone());    // ← uses clone
                edit_value.set(value_edit.clone());
                edit_id.set(id_edit.clone());
                edit_open.set(true);
            }),
        },
    ]
})
```

---

## Why This Works

1. **`move` on outer closure** = captures parent signals
2. **Clone per callback** = each closure owns its data
3. **No shared ownership** = no borrow checker issues
4. **Clear intent** = `name_copy`, `name_edit` naming

---

## Common Patterns

### Two Callbacks (Same Data)
```rust
let value = t.value.clone();

let value_copy = value.clone();
let value_edit = value.clone();

vec![
    RowAction {
        on_click: Callback::new(move |_| {
            copy(&value_copy);
        }),
    },
    RowAction {
        on_click: Callback::new(move |_| {
            edit(&value_edit);
        }),
    },
]
```

### Three+ Callbacks
```rust
let name = t.name.clone();

let name_1 = name.clone();
let name_2 = name.clone();
let name_3 = name.clone();

vec![
    RowAction { on_click: Callback::new(move |_| use_1(&name_1)) },
    RowAction { on_click: Callback::new(move |_| use_2(&name_2)) },
    RowAction { on_click: Callback::new(move |_| use_3(&name_3)) },
]
```

### Capturing External Signals
```rust
DataTableColumn::actions(move |t: &TokenRow| {  // ← MUST use move
    // Now can capture external signals
    let value = t.value.clone();
    let value_copy = value.clone();
    
    vec![
        RowAction {
            on_click: Callback::new(move |_| {
                external_signal.set(value_copy.clone());  // ← captures signal
            }),
        },
    ]
})
```

---

## Critical Requirements

### ✅ DO
```rust
// Outer move
DataTableColumn::actions(move |t| { ... })

// Clone per callback
let value_copy = value.clone();
let value_edit = value.clone();

// Use in closures
on_click: Callback::new(move |_| {
    use_it(&value_copy);
})
```

### ❌ DON'T
```rust
// No outer move (if capturing external)
DataTableColumn::actions(|t| { ... })  // ← Missing move

// Reuse same value
on_click: Callback::new(move |_| {
    use_it(&value);  // ← Moved
})
// ...
on_click: Callback::new(move |_| {
    use_it(&value);  // ← ERROR
})
```

---

## Real Production Example
```rust
#[component]
pub fn TokensTable() -> impl IntoView {
    // External signals
    let edit_open = RwSignal::new(false);
    let edit_name = RwSignal::new(String::new());
    let edit_value = RwSignal::new(String::new());
    
    // ...
    
    DataTableColumn::actions(move |t: &TokenRow| {  // ← move
        // Extract
        let token_name = t.full_name.clone();
        let token_value = t.value.clone();
        let token_id = t.id.clone();
        let token_scope = t.scope.clone();
        let token_category = t.category.clone();
        let token_status = t.status.clone();
        
        // Clone for each callback
        let token_name_copy = token_name.clone();
        let token_value_copy = token_value.clone();
        let token_name_edit = token_name.clone();
        let token_value_edit = token_value.clone();
        let token_id_edit = token_id.clone();
        let token_scope_edit = token_scope.clone();
        let token_category_edit = token_category.clone();
        let token_status_edit = token_status.clone();
        
        vec![
            RowAction {
                label: "Copy token name".to_string(),
                on_click: Callback::new(move |_| {
                    crate::utils::copy_to_clipboard(&token_name_copy);
                }),
                variant: RowActionVariant::Default,
            },
            RowAction {
                label: "Copy token value".to_string(),
                on_click: Callback::new(move |_| {
                    crate::utils::copy_to_clipboard(&token_value_copy);
                }),
                variant: RowActionVariant::Default,
            },
            RowAction {
                label: "Edit line".to_string(),
                on_click: Callback::new(move |_| {
                    edit_name.set(token_name_edit.clone());
                    edit_value.set(token_value_edit.clone());
                    edit_id.set(token_id_edit.clone());
                    edit_scope.set(token_scope_edit.clone());
                    edit_category.set(token_category_edit.clone());
                    edit_status.set(token_status_edit.clone());
                    edit_open.set(true);
                }),
                variant: RowActionVariant::Default,
            },
        ]
    })
}
```

---

## Decision Matrix

| Scenario | Pattern | Why |
|----------|---------|-----|
| 1 callback | Clone once | Simple |
| 2+ callbacks (same data) | Clone per callback | Ownership |
| Callbacks + external signals | `move` + clones | Captures |
| Callbacks + no external | No `move` needed | Local only |

---

## Compiler Hints

When you see:
```
error[E0382]: use of moved value: `name`
help: consider cloning the value before moving it
```

**Fix:**
1. Add clone BEFORE first callback: `let name_copy = name.clone();`
2. Use `name_copy` in first callback
3. Keep `name` for other clones

---

## Normative Status

- Violations **MUST** block PRs
- Multiple callbacks **MUST** use dedicated clones
- Outer closure **MUST** use `move` if capturing external signals
- Exceptions require explicit justification

---

**Author:** Canon Working Group  
**Date:** 2025-12-30  
**Version:** 1.0  
**Status:** Production-Critical
