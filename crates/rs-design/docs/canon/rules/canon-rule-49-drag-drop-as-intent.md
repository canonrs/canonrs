# Canon Rule #49: Drag & Drop as Intent (Not Action)

**Status:** Normative  
**Applies to:** All drag & drop implementations

---

## The Principle

**Drag & Drop is an input gesture, not business logic.**

It emits **semantic intent**, which the application layer converts into **commands**.

---

## The Problem

### ❌ Wrong Pattern (Coupled)
```rust
// DragHandle executes business logic directly
fn on_drop(item_id: String, target_id: String) {
    // WRONG: Component knows about database
    database::move_item(item_id, target_id).await;
    // WRONG: Component knows about workflow
    workflow::update_step_order(item_id).await;
}
```

**Why this is wrong:**
- Design system **polluted** with domain logic
- Impossible to reuse `DragHandle` across contexts
- Cannot undo/redo (no command history)
- Cannot test drag without database
- Violates separation of concerns

---

## The Solution

### ✅ Correct Pattern (Decoupled)
```rust
// 1. Design System: Emits events (NO business logic)
use rs_design::ui::drag_drop::{DragDropProvider, DragHandle, DropEvent};

#[component]
pub fn MyApp() -> impl IntoView {
    let on_drop = Callback::new(|event: DropEvent| {
        // 2. Application Layer: Converts event → command
        let command = MoveItemCommand {
            item_id: event.item_id,
            target_id: event.target_id,
        };
        
        // 3. Execute command (can be undone)
        command_history.execute(command);
    });

    view! {
        <DragDropProvider on_drop=on_drop>
            <DragHandle item_id="task-1">
                <div>"Drag me"</div>
            </DragHandle>
        </DragDropProvider>
    }
}
```

---

## Architecture Layers

### Layer 1: Design System (rs-design)
**Responsibility:** Input gesture detection  
**Output:** `DropEvent { item_id, target_id }`
```rust
// rs-design/ui/drag_drop/
pub struct DropEvent {
    pub item_id: DragItemId,
    pub target_id: DropTargetId,
    pub data: Option<String>,
}

// Component ONLY emits events
on_drop.run(DropEvent { ... });
```

### Layer 2: Application (frontend-leptos)
**Responsibility:** Convert events → commands
```rust
let on_drop = Callback::new(|event: DropEvent| {
    match current_context {
        Context::Workflow => execute_workflow_reorder(event),
        Context::Tree => execute_tree_move(event),
        Context::Kanban => execute_kanban_move(event),
    }
});
```

### Layer 3: Command History
**Responsibility:** Undo/Redo/Audit
```rust
struct MoveItemCommand {
    item_id: String,
    from: Position,
    to: Position,
}

impl Command for MoveItemCommand {
    fn execute(&self) { /* move item */ }
    fn undo(&self) { /* restore position */ }
}

command_history.push(command);
```

---

## Real-World Example: Workflow Steps

### ❌ Wrong (Coupled)
```rust
// WRONG: DragHandle knows about workflows
<DragHandle 
    item_id="step-1"
    on_drop=move |_| {
        database::update_workflow_step_order().await; // 🚫
    }
>
```

### ✅ Correct (Decoupled)
```rust
// Design System: Pure gesture
<DragDropProvider on_drop=on_workflow_drop>
    <SortableList items=steps .../>
</DragDropProvider>

// Application: Business logic
let on_workflow_drop = Callback::new(|event: DropEvent| {
    let command = ReorderStepsCommand {
        workflow_id: current_workflow,
        step_id: event.item_id,
        new_position: event.target_id,
    };
    
    history.execute(command); // ✅ Undoable
});
```

---

## Benefits

### ✅ Reusability
Same `DragHandle` works for:
- Workflow steps
- Tree nodes
- Kanban cards
- File managers
- Task lists

### ✅ Testability
```rust
#[test]
fn test_drag_drop() {
    let event = DropEvent { 
        item_id: "task-1", 
        target_id: "zone-b" 
    };
    
    // Test WITHOUT rendering component
    assert_eq!(handle_drop(event), ExpectedState);
}
```

### ✅ Undo/Redo
```rust
// Free undo/redo if using Command pattern
user_presses_ctrl_z();
command_history.undo(); // ✅ Drag is undone
```

### ✅ Audit Trail
```rust
// All drags are logged as commands
for command in history.undo_stack {
    audit_log.push(command.description);
}
// "Moved task-1 from zone-a to zone-b"
```

---

## Implementation Checklist

When implementing drag & drop:

- [ ] Design system components emit **events only**
- [ ] Zero database/API calls in drag components
- [ ] Application layer handles events
- [ ] Events convertible to Commands
- [ ] Commands are undoable
- [ ] Drag works across multiple contexts (workflow, tree, kanban)

---

## Anti-Patterns to Avoid

### 🚫 API Calls in DragHandle
```rust
// WRONG
<DragHandle on_drop=move |_| {
    api::update_position().await; // 🚫
}>
```

### 🚫 Database Updates in DropZone
```rust
// WRONG
<DropZone on_drop=move |_| {
    db.update_item_position(); // 🚫
}>
```

### 🚫 Workflow Logic in Design System
```rust
// WRONG - in rs-design crate
pub fn DragHandle() {
    if workflow.is_blocked() { ... } // 🚫
}
```

---

## Comparison: CanonRS vs Others

| Framework | Approach | Coupling |
|-----------|----------|----------|
| **CanonRS** | Event → Command | ✅ Zero coupling |
| react-dnd | Imperative handlers | ⚠️ Often coupled |
| @dnd-kit | Context-based | ⚠️ Moderate coupling |
| SortableJS | DOM manipulation | 🚫 Tightly coupled |

**Veredito:** CanonRS é **mais rigoroso** e **testável**.

---

## Related Rules

- **Rule #8:** Overlay Islands (Client-Only Architecture)
- **Rule #XX:** Command Pattern (pending)
- **Rule #XX:** Separation of Concerns (pending)

---

## Normative Status

- Violations **MUST** block PRs
- Design system components **MUST NOT** contain business logic
- All drag & drop **MUST** emit events, not execute actions
- Events **SHOULD** be convertible to Commands for undo/redo

---

**Author:** Canon Working Group  
**Date:** 2026-01-03  
**Version:** 1.0  
**Replaces:** None (primeira definição formal)

---

## Update 2026-01-04: SortableList Pattern

### Best Practice for Reorderable Lists

**Use `SortableList` component instead of manual `DragHandle` + `DropZone`:**
```rust
use rs_design::ui::drag_drop::SortableList;

#[component]
pub fn MyComponent() -> impl IntoView {
    let (items, set_items) = signal(vec![...]);
    
    let on_reorder = Callback::new(move |new_items: Vec<Item>| {
        set_items.set(new_items);
        // Convert to Command if needed
    });
    
    view! {
        <SortableList
            items=items
            on_reorder=on_reorder
            item_id=|item: &Item| item.id.clone()
            render=move |item: Item| {
                view! { <div>{item.label}</div> }
            }
        />
    }
}
```

**Why this is better:**

✅ Built-in drop indicators (before/after)  
✅ Automatic drag handle rendering  
✅ Proper event handling (already tested)  
✅ SSR-safe implementation  
✅ Less boilerplate code  

**Working examples:**
- `drag_drop_tab.rs` - Multi-zone drag & drop
- `workflow_sortable_tab.rs` - Sortable workflow steps

**When NOT to use SortableList:**
- Custom drop zones (use `DropZone` directly)
- Cross-component dragging (use `DragHandle` + callbacks)
- Non-list layouts (grids, trees)
