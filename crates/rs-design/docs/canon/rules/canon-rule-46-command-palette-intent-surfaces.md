# Canon Rule #46: Command Palette & Intent Surfaces

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-02

## Principle
Command Palette is not a "nice-to-have widget" — it's the **command bus of the UI**. It decouples user intent from visual affordances, enabling discovery, accessibility, automation, and scaling without UI bloat.

---

## The Problem

### ❌ WRONG: Button Proliferation
```rust
// Every action becomes a button
view! {
    <div>
        <button on:click=complete>"Complete Step"</button>
        <button on:click=fail>"Fail Step"</button>
        <button on:click=reset>"Reset Step"</button>
        <button on:click=start>"Start Step"</button>
        <button on:click=unblock>"Unblock Step"</button>
        <button on:click=audit>"View Audit"</button>
        <button on:click=export>"Export Report"</button>
        // ... 50 more buttons
    </div>
}
```

**Problems:**
- UI becomes cluttered
- Actions hard to discover
- Cognitive overload
- No keyboard navigation
- Doesn't scale
- Context-switching required
- No command history
- Can't automate

### ✅ CORRECT: Intent Surface
```rust
// Visual affordances for common actions
view! {
    <div>
        <button on:click=complete>"Complete"</button>
        <button on:click=fail>"Fail"</button>
    </div>
}

// Command Palette for ALL actions
CommandPalette {
    registry: CommandRegistry::new()
        .add_group("workflow", [
            complete_command,
            fail_command,
            reset_command,
            start_command,
            unblock_command,
            audit_command,
            export_command,
            // ... all 50 commands discoverable
        ])
}
```

**Benefits:**
- UI stays clean
- All actions discoverable
- Keyboard-first
- Scales infinitely
- Command history possible
- Automation-ready
- Consistent UX

---

## What is an Intent Surface?

**Intent Surface** = Interface for declaring "what user wants to do"

It's **NOT:**
- A menu
- A shortcut list
- A search box
- A modal dialog

It's a **command registry + execution layer**
```
User Intent → Command Registry → Command Execution → State Change
```

---

## Command Palette Architecture

### Components
```
┌─────────────────────────────────────┐
│ CommandPalette (Orchestrator)       │
│ - Opens/closes                      │
│ - Keyboard shortcuts (Ctrl+K)       │
│ - Filters registry                  │
└────────────┬────────────────────────┘
             │
┌────────────▼────────────────────────┐
│ CommandRegistry (State)             │
│ - Available commands                │
│ - Search/filter logic               │
│ - Command groups                    │
└────────────┬────────────────────────┘
             │
┌────────────▼────────────────────────┐
│ Command (Data)                      │
│ - id, label, group                  │
│ - shortcut, icon                    │
│ - callback (execution)              │
└─────────────────────────────────────┘
```

### Separation of Concerns

| Layer | Responsibility | Never Does |
|-------|---------------|------------|
| **CommandPalette** (UI) | Rendering, keyboard, focus | Business logic, command execution |
| **CommandRegistry** (State) | Search, filter, organize | UI rendering, execution |
| **Command** (Data) | Metadata + callback | State management, UI |
| **Domain** (Integration) | Register domain commands | Generic UI logic |

---

## Command Pattern

### Command Structure
```rust
pub struct Command {
    // Identity
    pub id: String,              // "workflow.complete_step"
    pub label: String,           // "Complete Active Step"
    pub group: Option<String>,   // "Workflow"
    
    // UX
    pub icon: Option<String>,    // "✅"
    pub shortcut: Option<String>,// "Ctrl+Enter"
    
    // Execution
    pub callback: CommandCallback,
}
```

### Naming Convention
```
{domain}.{action}_{object}

Examples:
- workflow.complete_step
- workflow.fail_step
- user.create_account
- billing.export_invoice
- navigation.goto_dashboard
- system.toggle_theme
```

**Rules:**
- Lowercase
- Dots separate domain from action
- Underscores separate words
- Verb-first naming (action-oriented)

### Command Groups
```rust
CommandGroup::new("workflow", "Workflow")
    .add_command(complete_cmd)
    .add_command(fail_cmd)
    .add_command(reset_cmd)

CommandGroup::new("navigation", "Navigation")
    .add_command(goto_dashboard)
    .add_command(goto_settings)

CommandGroup::new("system", "System")
    .add_command(toggle_theme)
    .add_command(logout)
```

**Purpose:**
- Organize commands logically
- Visual separation in palette
- Better discoverability
- Domain boundaries clear

---

## Integration Patterns

### Pattern 1: Domain Commands

**Where:** `apps/*/components/domain_commands.rs`
```rust
// workflow_commands.rs
pub fn create_workflow_commands(
    on_transition: Callback<()>
) -> CommandRegistry {
    CommandRegistry::new()
        .add_group(
            CommandGroup::new("workflow", "Workflow")
                .add_command(Command {
                    id: "workflow.complete_step".into(),
                    label: "Complete Active Step".into(),
                    group: Some("Workflow".into()),
                    shortcut: Some("Ctrl+Enter".into()),
                    icon: Some("✅".into()),
                    callback: CommandCallback::new(move || {
                        #[cfg(target_arch = "wasm32")]
                        {
                            use leptos::task::spawn_local;
                            spawn_local(async move {
                                transition_workflow_step(...).await;
                                on_transition.run(());
                            });
                        }
                    }),
                })
        )
}
```

**Key points:**
- Domain-specific
- Uses domain server functions
- Triggers domain callbacks (refresh)
- Client-only execution

### Pattern 2: System Commands

**Where:** `apps/*/components/system_commands.rs`
```rust
pub fn create_system_commands(
    set_theme: WriteSignal<Theme>,
    set_lang: WriteSignal<Language>,
) -> CommandRegistry {
    CommandRegistry::new()
        .add_group(
            CommandGroup::new("system", "System")
                .add_command(Command {
                    id: "system.toggle_theme".into(),
                    label: "Toggle Theme".into(),
                    shortcut: Some("Ctrl+Shift+T".into()),
                    callback: CommandCallback::new(move || {
                        set_theme.update(|t| *t = t.toggle());
                    }),
                })
                .add_command(Command {
                    id: "system.change_language".into(),
                    label: "Change Language".into(),
                    shortcut: Some("Ctrl+Shift+L".into()),
                    callback: CommandCallback::new(move || {
                        set_lang.update(|l| *l = l.next());
                    }),
                })
        )
}
```

### Pattern 3: Navigation Commands
```rust
pub fn create_navigation_commands(
    navigate: impl Fn(&str) + 'static
) -> CommandRegistry {
    CommandRegistry::new()
        .add_group(
            CommandGroup::new("navigation", "Go to...")
                .add_command(Command {
                    id: "nav.dashboard".into(),
                    label: "Dashboard".into(),
                    shortcut: Some("Ctrl+Shift+D".into()),
                    callback: CommandCallback::new(move || navigate("/dashboard")),
                })
        )
}
```

### Pattern 4: Composite Registry
```rust
// App-level aggregation
pub fn create_app_commands(
    workflow_on_transition: Callback<()>,
    set_theme: WriteSignal<Theme>,
    navigate: impl Fn(&str) + 'static,
) -> CommandRegistry {
    let mut registry = CommandRegistry::new();
    
    // Merge domain registries
    for group in create_workflow_commands(workflow_on_transition).groups {
        registry = registry.add_group(group);
    }
    
    for group in create_system_commands(set_theme).groups {
        registry = registry.add_group(group);
    }
    
    for group in create_navigation_commands(navigate).groups {
        registry = registry.add_group(group);
    }
    
    registry
}
```

---

## Keyboard Shortcuts

### Global Shortcut
```rust
// Ctrl+K or Cmd+K to open palette
Effect::new(move |_| {
    let handler = move |ev: web_sys::KeyboardEvent| {
        if (ev.ctrl_key() || ev.meta_key()) && ev.key() == "k" {
            ev.prevent_default();
            set_palette_open.update(|open| *open = !*open);
        }
    };
    
    // Register listener
    window.add_event_listener("keydown", handler);
});
```

### Command Shortcuts
```rust
Command {
    id: "workflow.complete".into(),
    label: "Complete Step".into(),
    shortcut: Some("Ctrl+Enter".into()),  // Displayed in UI
    callback: CommandCallback::new(move || { ... }),
}
```

**Note:** Shortcuts in commands are **visual hints**, not automatic bindings. Implement actual key handlers separately if needed.

---

## SSR & Hydration

### The Hydration Problem
```rust
// ❌ WRONG: Renders different trees in SSR vs client
view! {
    {
        #[cfg(target_arch = "wasm32")]
        {
            view! { <CommandPalette /> }
        }
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            view! { <div></div> }
        }
    }
}
```

**Problem:** SSR renders `<div>`, client expects `<CommandPalette>` → hydration panic

### ✅ CORRECT: Mount After Hydration
```rust
let (palette_mounted, set_palette_mounted) = signal(false);

Effect::new(move |_| {
    // Register keyboard shortcuts
    // ...
    set_palette_mounted.set(true);
});

view! {
    <div>
        <button on:click=open_palette>"Commands"</button>
        
        <Show when=move || palette_mounted.get()>
            <CommandPalette />
        </Show>
    </div>
}
```

**Why this works:**
- SSR renders button only
- Client hydrates button (matches)
- Effect runs AFTER hydration
- CommandPalette mounts dynamically (no mismatch)

---

## Command Execution

### Sync Commands
```rust
CommandCallback::new(move || {
    set_theme.update(|t| *t = t.toggle());
})
```

### Async Commands (Client-Only)
```rust
CommandCallback::new(move || {
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::task::spawn_local;
        spawn_local(async move {
            transition_step().await;
            on_complete.run(());
        });
    }
})
```

### With Confirmation
```rust
CommandCallback::new(move || {
    if window.confirm("Are you sure?") {
        delete_item();
    }
})
```

---

## Scaling Commands

### Contextual Commands
```rust
// Commands change based on context
let commands = Signal::derive(move || {
    let selected = selected_item.get();
    
    if let Some(item) = selected {
        create_item_commands(item)  // Edit, Delete, Share
    } else {
        create_global_commands()    // New, Import
    }
});

view! {
    <CommandPalette registry=commands.get() />
}
```

### Dynamic Registration
```rust
// Add commands at runtime
let (registry, set_registry) = signal(CommandRegistry::new());

// Add workflow commands when workflow loads
Effect::new(move |_| {
    if workflow_loaded.get() {
        set_registry.update(|r| {
            *r = r.clone().add_group(create_workflow_commands());
        });
    }
});
```

---

## Testing

### Unit Test Commands
```rust
#[test]
fn command_execution() {
    let (called, set_called) = signal(false);
    
    let cmd = Command {
        id: "test".into(),
        label: "Test".into(),
        callback: CommandCallback::new(move || set_called.set(true)),
        ..Default::default()
    };
    
    cmd.callback.call();
    assert!(called.get());
}
```

### Test Registry
```rust
#[test]
fn registry_search() {
    let registry = CommandRegistry::new()
        .add_group(
            CommandGroup::new("test", "Test")
                .add_command(Command {
                    id: "test.foo".into(),
                    label: "Foo Action".into(),
                    ..Default::default()
                })
        );
    
    let results = registry.search("foo");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].id, "test.foo");
}
```

---

## Enterprise Benefits

### 1. Automation

Commands are **data**, not just UI:
```rust
// CLI can execute same commands
fn execute_command(id: &str) {
    let registry = create_app_commands(...);
    if let Some(cmd) = registry.find_command(id) {
        cmd.callback.call();
    }
}

// $ app-cli exec workflow.complete_step
```

### 2. Analytics
```rust
CommandCallback::new(move || {
    analytics::track("command_executed", json!({
        "command_id": "workflow.complete_step",
        "user_id": user.id,
    }));
    
    execute_transition();
})
```

### 3. A11y

- Keyboard-first by design
- No mouse required
- Screen reader friendly (ARIA labels)
- Discoverable without visual hunt

### 4. Onboarding
```rust
// Show "getting started" commands on first login
if is_first_login {
    registry.add_group(
        CommandGroup::new("tutorial", "Getting Started")
            .add_command(create_account_cmd)
            .add_command(import_data_cmd)
    );
}
```

---

## Command Palette vs Other Patterns

| Pattern | Use Case | Command Palette Replaces? |
|---------|----------|--------------------------|
| Toolbar | Common actions | Partially (keep 1-2 most common) |
| Context Menu | Right-click actions | ✅ Yes |
| Dropdown Menu | Grouped actions | ✅ Yes |
| Keyboard Shortcuts | Power users | ✅ Complements (shows shortcuts) |
| Search | Find items | ❌ Different (find vs execute) |

---

## Related Rules

- [Canon Rule #43: Domain Components and Commands](./canon-rule-43-domain-components-and-commands.md) - CQRS separation
- [Canon Rule #44: Orchestrators](./canon-rule-44-orchestrators.md) - Coordination patterns
- [Canon Rule #45: Demo Components](./canon-rule-45-demo-components.md) - When to simplify

---

## Summary

**Golden Rule:** Command Palette is infrastructure, not decoration.

**Architecture:**
```
Intent Surface → Command Registry → Execution → State Change
```

**Benefits:**
- ✅ Scalable (100s of commands without UI bloat)
- ✅ Discoverable (search > memory)
- ✅ Accessible (keyboard-first)
- ✅ Automatable (CLI, scripts, AI)
- ✅ Consistent (same commands everywhere)
- ✅ Testable (commands are data)

**Never:**
- Replace ALL buttons with palette
- Hide critical actions behind palette
- Make palette the only way to act

**Always:**
- Keep 1-2 most common actions as buttons
- Make palette a **complement**, not replacement
- Register ALL actions as commands (even if buttons exist)

**Remember:**
> "Buttons are shortcuts to commands, not the commands themselves."

---

**Enforcement:** Architecture review  
**Violation:** Creating 10+ buttons without command palette  
**Severity:** Medium (UX degradation, doesn't scale)

---

**Last Updated:** 2025-01-02  
**Series:** Rules #41-46 (Leptos CQRS + Command Architecture)
