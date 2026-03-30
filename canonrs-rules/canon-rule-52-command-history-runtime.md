# Canon Rule #52: Command History as First-Class Runtime

**Status:** ENFORCED


**Severity:** MEDIUM
**Scope:** interactive, state
**Version:** 1.0.0
**Date:** 2025-01-16

---
  

---

## The Principle

**Command History MUST be a first-class provider in the application runtime.**

Like DragDropProvider (Rule #50) and other runtime primitives, CommandHistory is provided once at the app root and consumed by all interactive components.

This enables:
- Global undo/redo (Ctrl+Z / Ctrl+Y)
- Audit trail across all user actions
- Time-travel debugging
- Command replay
- Consistent command execution

---

## The Problem

### Wrong Pattern Local Command History
```rust
// Each component creates its own history
#[component]
pub fn Dashboard() -> impl IntoView {
    let command_history = CommandHistory::new(); // 🚫 Local
    
    view! {
        <div on:click=move |_| {
            command_history.execute(SomeCommand {});
        }>
    }
}
```

**Why this breaks:**
- No global undo/redo
- Cannot undo across components
- Duplicate histories waste memory
- No centralized audit trail

---

## The Solution

### Correct Pattern Global Provider
```rust
// App layer (ONE provider at root)
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <DragDropProvider>
                <DragDropCallbacksProvider>
                    <CommandHistoryProvider>  // ✅ Global
                        <Dashboard />
                        <FormBuilder />
                        <DataGrid />
                    </CommandHistoryProvider>
                </DragDropCallbacksProvider>
            </DragDropProvider>
        </Router>
    }
}

// Component layer (consume context)
#[component]
pub fn Dashboard() -> impl IntoView {
    let history = use_command_history(); // ✅ Consume
    
    let on_drop = Callback::new(move |event| {
        history.execute(MoveWidgetCommand { ... });
    });
}
```

---

## Provider Implementation

### CommandHistory
```rust
#[derive(Clone, Copy)]
pub struct CommandHistory {
    undo_stack: RwSignal<Vec<Arc<dyn Command>>>,
    redo_stack: RwSignal<Vec<Arc<dyn Command>>>,
}

impl CommandHistory {
    pub fn execute(&self, command: impl Command + 'static);
    pub fn undo(&self) -> Option<String>;
    pub fn redo(&self) -> Option<String>;
    pub fn can_undo(&self) -> Signal<bool>;
    pub fn can_redo(&self) -> Signal<bool>;
    pub fn undo_history(&self) -> Signal<Vec<String>>;
}
```

### CommandHistoryProvider
```rust
#[component]
pub fn CommandHistoryProvider(children: Children) -> impl IntoView {
    let history = CommandHistory::new();
    provide_context(history);
    
    // ✅ Global keyboard shortcuts
    #[cfg(target_arch = "wasm32")]
    {
        // Ctrl+Z → undo
        // Ctrl+Y → redo
        // Ctrl+Shift+Z → redo
    }
    
    children()
}
```

---

## Keyboard Shortcuts

CommandHistoryProvider automatically binds:

| Shortcut | Action | Platform |
|----------|--------|----------|
| Ctrl+Z   | Undo   | Windows/Linux |
| Cmd+Z    | Undo   | macOS |
| Ctrl+Y   | Redo   | Windows/Linux |
| Cmd+Y    | Redo   | macOS |
| Ctrl+Shift+Z | Redo | Windows/Linux |
| Cmd+Shift+Z | Redo | macOS |

**Implementation:**
```rust
let closure = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
    if event.ctrl_key() || event.meta_key() {
        match event.key().as_str() {
            "z" if !event.shift_key() => {
                event.prevent_default();
                history.undo();
            }
            "y" | "Z" => {
                event.prevent_default();
                history.redo();
            }
            _ => {}
        }
    }
}));
```

---

## Provider Order Canon Rule 50
```
Router
└── DragDropProvider (runtime)
    └── DragDropCallbacksProvider (interprets drag events)
        └── CommandHistoryProvider (executes commands)
            └── LanguageProvider
                └── ThemeProvider
                    └── Components
```

**Order matters:**
1. DragDropProvider creates reactive state
2. DragDropCallbacksProvider interprets events → commands
3. CommandHistoryProvider executes commands with undo/redo

---

## Real World Examples

### Dashboard Widget Movement
```rust
// dashboard_tab.rs
let history = use_command_history();

let on_drop = Callback::new(move |event: DropEvent| {
    history.execute(MoveWidgetCommand {
        widgets: placed_widgets,
        widget_id: event.item_id.0,
        from_position: /* ... */,
        to_position: /* ... */,
    });
});
```

### Form Field Edit
```rust
let history = use_command_history();

let on_change = Callback::new(move |new_value: String| {
    history.execute(UpdateFieldCommand {
        field_signal: field_value,
        old_value: field_value.get(),
        new_value,
    });
});
```

### Bulk Delete
```rust
let history = use_command_history();

let delete_selected = move || {
    let commands: Vec<_> = selected_items.get()
        .into_iter()
        .map(|id| RemoveItemCommand { id })
        .collect();
    
    history.execute(MacroCommand {
        commands,
        description: format!("Delete {} items", commands.len()),
    });
};
```

---

## Audit Trail UI
```rust
#[component]
pub fn AuditTrail() -> impl IntoView {
    let history = use_command_history();
    
    view! {
        <div class="audit-trail">
            <h3>"Command History"</h3>
            <For
                each=move || history.undo_history().get()
                key=|cmd| cmd.clone()
                children=move |cmd| view! {
                    <div class="audit-entry">{cmd}</div>
                }
            />
        </div>
    }
}
```

---

## Testing

### ✅ Command Execution
```rust
#[test]
fn test_command_history() {
    let history = CommandHistory::new();
    let state = RwSignal::new(0);
    
    history.execute(IncrementCommand { state });
    assert_eq!(state.get(), 1);
    
    history.undo();
    assert_eq!(state.get(), 0);
    
    history.redo();
    assert_eq!(state.get(), 1);
}
```

### ✅ Keyboard Shortcuts
```rust
#[wasm_bindgen_test]
fn test_ctrl_z_undo() {
    // Mount CommandHistoryProvider
    // Simulate Ctrl+Z keydown
    // Assert undo was called
}
```

---

## Benefits

### ✅ Global Undo/Redo
- Works across all components
- Single keyboard shortcut system
- Consistent behavior

### ✅ Audit Trail
- All commands logged
- Export command history
- Replay bug scenarios

### ✅ Memory Efficient
- One history for entire app
- Shared command storage
- Automatic cleanup

### ✅ Testable
- Test commands independently
- Mock command history
- Replay sequences

---

## Forbidden Patterns

### 🚫 Local Command History
```rust
#[component]
pub fn MyComponent() -> impl IntoView {
    let history = CommandHistory::new(); // 🚫
}
```

### 🚫 Multiple Providers
```rust
<CommandHistoryProvider>
    <Dashboard>
        <CommandHistoryProvider> // 🚫 Duplicate
            ...
        </CommandHistoryProvider>
    </Dashboard>
</CommandHistoryProvider>
```

### 🚫 Provider in Design System
```rust
// rs-design/ui/dashboard.rs
pub fn Dashboard() -> impl IntoView {
    view! {
        <CommandHistoryProvider> // 🚫 FORBIDDEN
            ...
        </CommandHistoryProvider>
    }
}
```

---

## Integration With Canon Rules

### Rule #49: Drag & Drop as Intent
Event → Callback → Command
```rust
<DragHandle on_drop=on_drop /> // Rule #49
    ↓
Callback creates command // Rule #51
    ↓
history.execute(command) // Rule #52
```

### Rule #50: Provider Singleton
ONE CommandHistoryProvider per app
```rust
<CommandHistoryProvider> // ✅ Once at root
    <Dashboard />
    <FormBuilder />
</CommandHistoryProvider>
```

### Rule #51: Callbacks as Commands
Callbacks emit commands executed via history
```rust
let on_event = Callback::new(|e| {
    history.execute(SomeCommand { ... }); // Rule #51 + #52
});
```

---

## Normative Requirements

**MUST:**
- CommandHistoryProvider MUST be provided once at app root
- Components MUST use `use_command_history()` to access
- Keyboard shortcuts MUST be global (Ctrl+Z / Ctrl+Y)

**MUST NOT:**
- Components MUST NOT create local CommandHistory
- Design system MUST NOT provide CommandHistoryProvider
- Multiple CommandHistoryProviders MUST NOT exist in same scope

**SHOULD:**
- Undo/redo SHOULD show toast notifications
- Command history SHOULD be exportable
- Audit trail UI SHOULD be available in dev mode

---

**Author:** Canon Working Group  
**Related:** Rule #50 (Provider Singleton), Rule #51 (Callbacks as Commands)
