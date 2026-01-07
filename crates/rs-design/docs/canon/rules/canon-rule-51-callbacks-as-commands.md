# Canon Rule #51: Callbacks as Commands

**Status:** Normative  
**Severity:** High  
**Applies to:** All event-driven UI interactions

---

## The Principle

**UI callbacks MUST emit commands, not execute mutations directly.**

Event → Command → Execution

This enables:
- Undo/Redo
- Audit trails
- Testing
- Replay
- Command composition

---

## The Problem

### ❌ Wrong Pattern (Direct Mutation)
```rust
// Dashboard callback executes mutation directly
let on_widget_drag = Callback::new(move |event: WidgetDragEvent| {
    // 🚫 WRONG: Direct state mutation
    set_widgets.update(|ws| {
        if let Some(w) = ws.iter_mut().find(|w| w.id == event.widget_id) {
            w.position = event.to_position;
        }
    });
    
    // 🚫 WRONG: Direct API call
    api::update_widget_position(event.widget_id, event.to_position).await;
});
```

**Why this breaks:**
- Cannot undo
- Cannot replay
- Cannot audit
- Cannot test without side effects
- Violates CQRS

---

## The Solution

### ✅ Correct Pattern (Command-based)
```rust
// 1. Define command
#[derive(Clone, Debug)]
struct MoveWidgetCommand {
    widget_id: String,
    from_position: WidgetPosition,
    to_position: WidgetPosition,
}

impl Command for MoveWidgetCommand {
    fn execute(&self, state: &mut AppState) {
        if let Some(w) = state.widgets.iter_mut().find(|w| w.id == self.widget_id) {
            w.position = self.to_position.clone();
        }
    }
    
    fn undo(&self, state: &mut AppState) {
        if let Some(w) = state.widgets.iter_mut().find(|w| w.id == self.widget_id) {
            w.position = self.from_position.clone();
        }
    }
    
    fn description(&self) -> String {
        format!("Move widget {} to ({}, {})", 
            self.widget_id, self.to_position.x, self.to_position.y)
    }
}

// 2. Callback creates command
let on_widget_drag = Callback::new(move |event: WidgetDragEvent| {
    let command = MoveWidgetCommand {
        widget_id: event.widget_id,
        from_position: event.from_position,
        to_position: event.to_position,
    };
    
    // 3. Execute via command history
    command_history.execute(command);
});
```

---

## Architecture

### Command Flow
```
UI Event (WidgetDragEvent)
    ↓
Callback (on_widget_drag)
    ↓
Command (MoveWidgetCommand)
    ↓
CommandHistory.execute()
    ↓
State Mutation + Persistence
```

### Benefits
1. **Undo/Redo**: Free via command history
2. **Audit Trail**: All commands logged
3. **Replay**: Re-execute command sequence
4. **Testing**: Test commands without UI
5. **Composition**: Combine commands (MacroCommand)

---

## Command Trait
```rust
pub trait Command: Clone + Debug {
    /// Execute the command (mutate state)
    fn execute(&self, state: &mut AppState);
    
    /// Undo the command (restore previous state)
    fn undo(&self, state: &mut AppState);
    
    /// Human-readable description
    fn description(&self) -> String;
    
    /// Optional: Can this command be undone?
    fn is_undoable(&self) -> bool {
        true
    }
}
```

---

## Command History
```rust
pub struct CommandHistory {
    undo_stack: Vec<Box<dyn Command>>,
    redo_stack: Vec<Box<dyn Command>>,
}

impl CommandHistory {
    pub fn execute(&mut self, command: impl Command + 'static) {
        command.execute(&mut app_state);
        self.undo_stack.push(Box::new(command));
        self.redo_stack.clear(); // Clear redo on new command
    }
    
    pub fn undo(&mut self) -> Option<String> {
        if let Some(command) = self.undo_stack.pop() {
            command.undo(&mut app_state);
            let desc = command.description();
            self.redo_stack.push(command);
            Some(desc)
        } else {
            None
        }
    }
    
    pub fn redo(&mut self) -> Option<String> {
        if let Some(command) = self.redo_stack.pop() {
            command.execute(&mut app_state);
            let desc = command.description();
            self.undo_stack.push(command);
            Some(desc)
        } else {
            None
        }
    }
}
```

---

## Real-World Examples

### Dashboard Widget Movement
```rust
// Command
struct MoveWidgetCommand {
    widget_id: String,
    from: WidgetPosition,
    to: WidgetPosition,
}

// Callback
let on_drop = Callback::new(move |event: DropEvent| {
    command_history.execute(MoveWidgetCommand {
        widget_id: event.item_id.0,
        from: event.from_position,
        to: event.to_position,
    });
});
```

### Form Field Edit
```rust
// Command
struct UpdateFieldCommand {
    field_id: String,
    old_value: String,
    new_value: String,
}

// Callback
let on_change = Callback::new(move |new_value: String| {
    command_history.execute(UpdateFieldCommand {
        field_id: field_id.clone(),
        old_value: current_value.get(),
        new_value,
    });
});
```

### Bulk Operations (MacroCommand)
```rust
struct MacroCommand {
    commands: Vec<Box<dyn Command>>,
    description: String,
}

impl Command for MacroCommand {
    fn execute(&self, state: &mut AppState) {
        for cmd in &self.commands {
            cmd.execute(state);
        }
    }
    
    fn undo(&self, state: &mut AppState) {
        for cmd in self.commands.iter().rev() {
            cmd.undo(state);
        }
    }
}

// Usage
let delete_widgets = MacroCommand {
    commands: vec![
        Box::new(RemoveWidgetCommand { id: "w1" }),
        Box::new(RemoveWidgetCommand { id: "w2" }),
    ],
    description: "Delete 2 widgets".to_string(),
};
```

---

## Integration with Canon Rules

### Rule #49: Drag & Drop as Intent
```rust
// ✅ Drag emits event (Rule #49)
<DragHandle on_drop=on_drop>

// ✅ Callback creates command (Rule #51)
let on_drop = Callback::new(|event| {
    command_history.execute(MoveWidgetCommand { ... });
});
```

### Rule #50: Provider Singleton
```rust
// ✅ CommandHistory provided globally (Rule #50)
<CommandHistoryProvider>
    <Dashboard />
</CommandHistoryProvider>

// ✅ Callbacks use shared history (Rule #51)
let history = use_command_history();
history.execute(command);
```

---

## Forbidden Patterns ❌

### 🚫 Direct Mutation in Callback
```rust
Callback::new(|event| {
    state.update(|s| s.value = event.value); // 🚫
});
```

### 🚫 API Call in Callback
```rust
Callback::new(async |event| {
    api::save(event.data).await; // 🚫
});
```

### 🚫 Side Effects in Callback
```rust
Callback::new(|event| {
    log::info!("Changed"); // 🚫 (acceptable for debugging)
    analytics::track(event); // 🚫
    toast::show("Saved"); // 🚫
});
```

**Rule:** Callbacks create commands. Commands execute effects.

---

## Testing

### ✅ Command Testing (Easy)
```rust
#[test]
fn test_move_widget() {
    let mut state = AppState::new();
    let cmd = MoveWidgetCommand { ... };
    
    cmd.execute(&mut state);
    assert_eq!(state.widget("w1").position.x, 5);
    
    cmd.undo(&mut state);
    assert_eq!(state.widget("w1").position.x, 0);
}
```

### ❌ Callback Testing (Hard)
```rust
#[test]
fn test_callback() {
    // 🚫 Requires full UI, events, state
    let callback = create_callback();
    callback.run(WidgetDragEvent { ... });
    // How to assert?
}
```

---

## Persistence

Commands can be serialized for:
- **Save/Load**: Persist command history
- **Network**: Sync commands across clients
- **Replay**: Reproduce bug scenarios
```rust
#[derive(Serialize, Deserialize)]
struct MoveWidgetCommand { ... }

// Save
let json = serde_json::to_string(&command_history)?;
storage::save("history.json", json);

// Load & Replay
let commands: Vec<Box<dyn Command>> = storage::load("history.json")?;
for cmd in commands {
    cmd.execute(&mut state);
}
```

---

## Normative Requirements

**MUST:**
- Callbacks MUST create commands, not mutate state
- Commands MUST implement `execute()` and `undo()`
- CommandHistory MUST be provided via context (Rule #50)

**MUST NOT:**
- Callbacks MUST NOT call APIs directly
- Callbacks MUST NOT mutate shared state
- Commands MUST NOT have side effects beyond state mutation

**SHOULD:**
- Commands SHOULD be serializable
- Command descriptions SHOULD be user-friendly
- Undo/Redo SHOULD be available via keyboard shortcuts

---

**Author:** Canon Working Group  
**Date:** 2026-01-04  
**Version:** 1.0  
**Related:** Canon Rule #49 (Drag & Drop as Intent), Canon Rule #50 (Provider Singleton)
