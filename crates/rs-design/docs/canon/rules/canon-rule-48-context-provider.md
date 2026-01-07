# Canon Rule #48: Context Provider Pattern

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-03

## Principle
Use Context Providers to eliminate prop drilling and enable global access to selection state. Components consume context via hooks instead of receiving props.

---

## The Problem: Prop Drilling

### ❌ WITHOUT Context Provider
```rust
#[component]
pub fn App() -> impl IntoView {
    let selection_context = Signal::derive(...);
    
    view! {
        <Tree selection_context=selection_context />
        <Breadcrumb selection_context=selection_context />
        <Inspector selection_context=selection_context />
        <StatusBar selection_context=selection_context />
        <ActionBar selection_context=selection_context />
        <CommandPalette selection_context=selection_context />
    }
}
```

**Problems:**
- ❌ Every component needs selection prop
- ❌ Adding new consumer = update all call sites
- ❌ Deep nesting = prop drilling hell
- ❌ Refactoring is painful
- ❌ Component APIs polluted with passthrough props

### ✅ WITH Context Provider
```rust
#[component]
pub fn App() -> impl IntoView {
    view! {
        <SelectionContextProvider>
            <Tree />
            <Breadcrumb />
            <Inspector />
            <StatusBar />
            <ActionBar />
            <CommandPalette />
        </SelectionContextProvider>
    }
}
```

**Benefits:**
- ✅ Zero prop drilling
- ✅ Components access context directly
- ✅ Easy to add new consumers
- ✅ Clean component APIs
- ✅ Single source of truth

---

## Context Provider Implementation

### Context Value Structure
```rust
use leptos::prelude::*;

/// SelectionContextValue - Shared selection state
#[derive(Clone, Copy)]
pub struct SelectionContextValue {
    pub selected_id: RwSignal<Option<String>>,
    pub node_type: RwSignal<Option<String>>,
    pub label: RwSignal<Option<String>>,
    pub metadata: RwSignal<Option<String>>,
}
```

**Key Points:**
- `Clone + Copy` - Can be copied cheaply
- All fields are `RwSignal` - Reactive
- `Option<String>` - Can be empty (no selection)

### Helper Methods
```rust
impl SelectionContextValue {
    /// Update entire context at once
    pub fn update_selection(
        &self,
        id: Option<String>,
        node_type: Option<String>,
        label: Option<String>,
        metadata: Option<String>,
    ) {
        self.selected_id.set(id);
        self.node_type.set(node_type);
        self.label.set(label);
        self.metadata.set(metadata);
    }
    
    /// Clear selection
    pub fn clear(&self) {
        self.selected_id.set(None);
        self.node_type.set(None);
        self.label.set(None);
        self.metadata.set(None);
    }
    
    /// Check if has selection
    pub fn has_selection(&self) -> bool {
        self.selected_id.get().is_some()
    }
    
    /// Check if node is of type
    pub fn is_type(&self, type_name: &str) -> bool {
        self.node_type.get().as_deref() == Some(type_name)
    }
}
```

### Provider Component
```rust
#[component]
pub fn SelectionContextProvider(children: Children) -> impl IntoView {
    // Create reactive signals
    let selected_id = RwSignal::new(None::<String>);
    let node_type = RwSignal::new(None::<String>);
    let label = RwSignal::new(None::<String>);
    let metadata = RwSignal::new(None::<String>);
    
    // Create context value
    let context = SelectionContextValue {
        selected_id,
        node_type,
        label,
        metadata,
    };
    
    // Provide to children
    provide_context(context);
    
    view! { {children()} }
}
```

**Pattern:**
1. Create `RwSignal` for each field
2. Bundle into context struct
3. `provide_context()`
4. Render children

### Consumer Hook
```rust
/// Hook to consume selection context
pub fn use_selection_context() -> SelectionContextValue {
    use_context::<SelectionContextValue>()
        .expect("SelectionContext not provided. Wrap in <SelectionContextProvider>")
}
```

**Usage:**
```rust
#[component]
pub fn Inspector() -> impl IntoView {
    let ctx = use_selection_context();
    
    view! {
        <div>
            {move || ctx.label.get().unwrap_or_default()}
        </div>
    }
}
```

---

## Producer Pattern

Components that **update** selection:
```rust
#[component]
pub fn Tree() -> impl IntoView {
    let ctx = use_selection_context();
    
    let on_select = Callback::new(move |id: String| {
        let node = find_node(&id);
        
        if let Some(node) = node {
            ctx.update_selection(
                Some(node.id),
                Some(node.node_type),
                Some(node.label),
                node.metadata,
            );
        }
    });
    
    // ... render tree
}
```

**Pattern:**
1. Get context via `use_selection_context()`
2. On user action (select, click)
3. Call `ctx.update_selection()`
4. All consumers react automatically

---

## Consumer Pattern

Components that **read** selection:
```rust
#[component]
pub fn Inspector() -> impl IntoView {
    let ctx = use_selection_context();
    
    view! {
        <div>
            {move || {
                if ctx.has_selection() {
                    view! {
                        <div>
                            <h3>{move || ctx.label.get().unwrap_or_default()}</h3>
                            <code>{move || ctx.selected_id.get().unwrap_or_default()}</code>
                        </div>
                    }
                } else {
                    view! { <p>"No selection"</p> }
                }
            }}
        </div>
    }
}
```

**Pattern:**
1. Get context via `use_selection_context()`
2. Use signals reactively (`ctx.label.get()`)
3. No props needed!
4. Automatically updates when context changes

---

## Contextual Actions Pattern

Components that show actions based on selection:
```rust
#[component]
pub fn InspectorPanel(
    #[prop(optional)] on_action: Option<Callback<String>>,
) -> impl IntoView {
    let ctx = use_selection_context();
    
    view! {
        <div>
            {move || {
                if ctx.is_type("step") {
                    view! {
                        <button on:click=move |_| {
                            if let Some(id) = ctx.selected_id.get() {
                                on_action.unwrap().run(format!("complete:{}", id));
                            }
                        }>
                            "Complete Step"
                        </button>
                    }
                } else if ctx.is_type("workflow") {
                    view! {
                        <button on:click=move |_| {
                            if let Some(id) = ctx.selected_id.get() {
                                on_action.unwrap().run(format!("add_step:{}", id));
                            }
                        }>
                            "Add Step"
                        </button>
                    }
                } else {
                    view! { <p>"No actions"</p> }
                }
            }}
        </div>
    }
}
```

**Pattern:**
1. Read `ctx.is_type()` to check selection type
2. Render type-specific actions
3. Use `ctx.selected_id.get()` to get ID
4. Call action callback with `action:id` format

---

## Integration with Existing Patterns

### With Tree (Rule #47)

Tree produces selection → Context → Consumers react
```rust
<SelectionContextProvider>
    <Tree />  // Updates ctx.update_selection()
    <Inspector />  // Reads ctx.label.get()
</SelectionContextProvider>
```

### With Breadcrumb (Rule #47)

Breadcrumb can both read and update context:
```rust
#[component]
pub fn Breadcrumb() -> impl IntoView {
    let ctx = use_selection_context();
    
    let items = Signal::derive(move || {
        if let Some(id) = ctx.selected_id.get() {
            build_breadcrumb_path(&id)
        } else {
            vec![]
        }
    });
    
    // Click breadcrumb → update context
    let on_click = move |id: String| {
        ctx.update_selection(Some(id), ...);
    };
}
```

### With Command Palette (Rule #46)

Commands filter based on context:
```rust
let contextual_commands = Signal::derive(move || {
    let ctx = use_selection_context();
    
    let mut registry = CommandRegistry::new();
    
    if ctx.is_type("step") {
        registry.add_command(Command::new("Complete Step"));
    }
    
    registry
});
```

---

## Testing Context
```rust
#[test]
fn test_context_update() {
    let (selected_id, set_selected_id) = signal(None);
    let (node_type, set_node_type) = signal(None);
    let (label, set_label) = signal(None);
    let (metadata, set_metadata) = signal(None);
    
    let ctx = SelectionContextValue {
        selected_id,
        node_type,
        label,
        metadata,
    };
    
    // Update
    ctx.update_selection(
        Some("step-1".into()),
        Some("step".into()),
        Some("Review".into()),
        Some("Active".into()),
    );
    
    // Assert
    assert_eq!(ctx.selected_id.get(), Some("step-1".to_string()));
    assert_eq!(ctx.node_type.get(), Some("step".to_string()));
    assert!(ctx.has_selection());
    assert!(ctx.is_type("step"));
    
    // Clear
    ctx.clear();
    assert!(!ctx.has_selection());
}
```

---

## Anti-Patterns

### Anti-Pattern 1: Prop Drilling with Context Available
```rust
// ❌ WRONG: Passing context as prop when provider exists
#[component]
pub fn App() -> impl IntoView {
    let ctx = use_selection_context();
    
    view! {
        <SelectionContextProvider>
            <Inspector ctx=ctx />  // ❌ Don't pass as prop!
        </SelectionContextProvider>
    }
}
```

**Fix:** Let component access context directly:
```rust
// ✅ CORRECT
<SelectionContextProvider>
    <Inspector />  // Gets context via use_selection_context()
</SelectionContextProvider>
```

### Anti-Pattern 2: Multiple Sources of Truth
```rust
// ❌ WRONG: Both context and local state
let ctx = use_selection_context();
let (local_selection, set_local_selection) = signal(None);

// Which is correct?
```

**Fix:** Context is ALWAYS source of truth:
```rust
// ✅ CORRECT: Only use context
let ctx = use_selection_context();
// No local state needed
```

### Anti-Pattern 3: Forgetting Provider
```rust
// ❌ WRONG: Using hook without provider
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Inspector />  // ❌ Will panic: "SelectionContext not provided"
    }
}
```

**Fix:** Wrap in provider:
```rust
// ✅ CORRECT
<SelectionContextProvider>
    <Inspector />
</SelectionContextProvider>
```

### Anti-Pattern 4: Creating Multiple Providers
```rust
// ❌ WRONG: Multiple providers = multiple contexts
<SelectionContextProvider>
    <Tree />
</SelectionContextProvider>

<SelectionContextProvider>
    <Inspector />  // Different context!
</SelectionContextProvider>
```

**Fix:** Single provider at top level:
```rust
// ✅ CORRECT
<SelectionContextProvider>
    <Tree />
    <Inspector />  // Same context
</SelectionContextProvider>
```

---

## When to Use Context Provider

### ✅ USE Context Provider When:

1. **Multiple consumers** - 3+ components need same state
2. **Deep nesting** - Components are nested 2+ levels deep
3. **Frequent additions** - New consumers added regularly
4. **Global state** - State is truly application-wide (selection, theme, user)

### ❌ DON'T Use Context Provider When:

1. **Single consumer** - Only one component needs the data
2. **Parent-child only** - Direct prop is clearer
3. **Performance critical** - Context updates ALL consumers (use signals instead)
4. **Temporary state** - Local form state, modals, etc.

---

## Comparison with Other Frameworks

| Framework | Pattern | API |
|-----------|---------|-----|
| **React** | Context API | `createContext`, `useContext` |
| **Vue** | Provide/Inject | `provide()`, `inject()` |
| **Solid** | Context | `createContext`, `useContext` |
| **Leptos** | Context | `provide_context`, `use_context` |

**Leptos Advantage:** Signals + Context = Fine-grained reactivity without extra wrappers.

---

## Migration from Prop Drilling

### Before (Prop Drilling)
```rust
#[component]
pub fn App() -> impl IntoView {
    let selection = signal(...);
    
    view! {
        <Tree selection=selection />
        <Inspector selection=selection />
    }
}

#[component]
pub fn Inspector(selection: Signal<Selection>) -> impl IntoView {
    // ...
}
```

### After (Context Provider)
```rust
#[component]
pub fn App() -> impl IntoView {
    view! {
        <SelectionContextProvider>
            <Tree />
            <Inspector />
        </SelectionContextProvider>
    }
}

#[component]
pub fn Inspector() -> impl IntoView {
    let ctx = use_selection_context();
    // ...
}
```

**Migration Steps:**
1. Create provider component
2. Wrap app in provider
3. Replace props with `use_selection_context()`
4. Remove prop from component signatures
5. Update call sites (remove prop passing)

---

## Summary

**Golden Rule:** Context Provider eliminates prop drilling for global state.

**Pattern:**
```
Provider (top) → provide_context()
    ↓
Consumer (any depth) → use_context()
```

**When to Use:**
- ✅ Multiple consumers
- ✅ Deep nesting
- ✅ Global state (selection, theme, user)

**Benefits:**
- ✅ Zero prop drilling
- ✅ Clean component APIs
- ✅ Easy to add consumers
- ✅ Single source of truth

**Related Rules:**
- [Rule #47: Tree, Context & Selection](./canon-rule-47-tree-context-selection.md) - Tree produces selection
- [Rule #46: Command Palette](./canon-rule-46-command-palette-intent-surfaces.md) - Commands consume selection

---

**Enforcement:** Architecture review  
**Violation:** Prop drilling when context available  
**Severity:** Medium (code smell, not breakage)

---

**Last Updated:** 2025-01-03  
**Series:** Rules #41-48 (Enterprise UI Architecture)
