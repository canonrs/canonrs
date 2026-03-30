# Canon Rule #47: Tree, Context & Selection

**Status:** ENFORCED

**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** component-architecture
**Tags:** tree, selection, context, commands
**Language:** EN

---

**Intro:**
Coupling navigation components with actions or business logic leads to rigid and non-scalable systems. Tree structures must expose only hierarchy and selection while delegating actions to command systems.

**Problem:**
tree components execute actions or contain business logic instead of exposing selection

**Solution:**
separate tree selection from actions using selection context and command registry

**Signals:**
- tree executes action
- button inside tree node
- navigation coupled with logic

**Search Intent:**
how to decouple tree navigation from

**Keywords:**
tree selection context pattern, frontend navigation decoupling, command registry contextual actions, hierarchical ui design pattern

---

## Principle
Tree components expose **structure and selection**, never actions. Context flows from Tree → SelectionContext → Command Registry, enabling contextual commands without coupling tree navigation to domain logic.

---

## The Core Pattern
```
┌──────────────┐
│ Tree         │ Exposes: structure + selection
│ (Navigator)  │ Never: executes actions
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Selection    │ Derived: from tree selection
│ Context      │ Contains: id, type, label, metadata
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Command      │ Filters: based on context
│ Registry     │ Returns: available commands
└──────┬───────┘
       │
       ▼
┌──────────────┐
│ Command      │ Displays: contextual commands
│ Palette      │ Executes: user intent
└──────────────┘
```

**Golden Rule:** Tree never knows commands exist. Commands always know what context they need.

---

## The Problem Tree Solves

### ❌ WRONG: Flat List Navigation
```rust
// Everything in one list - no hierarchy
view! {
    <div>
        <div on:click=select_workflow1>"Workflow 1"</div>
        <div on:click=select_step1>"  Step 1"</div>
        <div on:click=select_step2>"  Step 2"</div>
        <div on:click=select_workflow2>"Workflow 2"</div>
        <div on:click=select_step3>"  Step 3"</div>
    </div>
}
```

**Problems:**
- No visual hierarchy
- Hard to understand relationships
- Can't collapse/expand
- Doesn't scale
- No parent/child concept

### ✅ CORRECT: Tree Navigation
```rust
TreeNode::new("workflows", "Workflows", "root")
    .with_children(vec![
        TreeNode::new("wf-1", "User Onboarding", "workflow")
            .with_children(vec![
                TreeNode::new("step-1", "Review", "step"),
                TreeNode::new("step-2", "Approval", "step"),
            ]),
        TreeNode::new("wf-2", "Billing", "workflow")
            .with_children(vec![
                TreeNode::new("step-3", "Charge", "step"),
            ]),
    ])
```

**Benefits:**
- Visual hierarchy clear
- Relationships explicit
- Collapse/expand naturally
- Scales to any depth
- Domain structure visible

---

## TreeNode Structure

### Pure Data
```rust
pub struct TreeNode {
    /// Unique identifier
    pub id: String,
    
    /// Display label
    pub label: String,
    
    /// Node type (domain-specific)
    pub node_type: String,
    
    /// Optional icon
    pub icon: Option<String>,
    
    /// Child nodes (recursive)
    pub children: Vec<TreeNode>,
    
    /// UI state: expanded or collapsed
    pub expanded: bool,
    
    /// Optional metadata (serialized domain data)
    pub metadata: Option<String>,
}
```

**Key Points:**
- No callbacks
- No domain logic
- No action execution
- Pure hierarchical data
- UI state (expanded) separated from domain state

### Builder Pattern
```rust
TreeNode::new("id", "Label", "type")
    .with_icon("📋")
    .with_children(vec![...])
    .with_expanded(true)
    .with_metadata("status:active")
```

**Why Builder:**
- Optional fields clean
- Chainable construction
- Self-documenting
- Easy to extend

---

## Tree Component Contract

### What Tree DOES

✅ **Renders Hierarchy**
```rust
#[component]
pub fn Tree(
    nodes: Signal<Vec<TreeNode>>,
    selected_id: Signal<Option<String>>,
    on_select: Callback<String>,
    on_toggle: Callback<String>,
) -> impl IntoView
```

✅ **Manages Visual State**
- Expand/collapse arrows
- Indentation by depth
- Selected node highlight
- Icon display

✅ **Emits Events**
- `on_select(id)` - user clicked node
- `on_toggle(id)` - user expanded/collapsed

✅ **Recursive Rendering**
```rust
fn render_node_recursive(
    node: TreeNode,
    depth: usize,
    selected_id: Signal<Option<String>>,
    on_select: Callback<String>,
    on_toggle: Callback<String>,
) -> impl IntoView {
    view! {
        <TreeNodeItem node=node depth=depth ... />
        {if node.expanded {
            node.children.map(|child| {
                render_node_recursive(child, depth + 1, ...)
            })
        }}
    }
}
```

### What Tree NEVER DOES

❌ **Execute Domain Actions**
```rust
// ❌ WRONG
on:click=move |_| {
    complete_workflow_step();  // Tree shouldn't know this
}
```

❌ **Know About Commands**
```rust
// ❌ WRONG
if node.type == "step" {
    show_complete_button();  // Commands come from elsewhere
}
```

❌ **Contain Business Rules**
```rust
// ❌ WRONG
if step.status == "Active" && user.role == "Admin" {
    allow_transition();  // Server decides this
}
```

❌ **Direct Navigation**
```rust
// ❌ WRONG
on:click=move |_| navigate("/workflows/123");  // Commands handle this
```

---

## SelectionContext

### Structure
```rust
pub struct SelectionContext {
    /// Currently selected node ID
    pub selected_id: Option<String>,
    
    /// Selected node type
    pub node_type: Option<String>,
    
    /// Selected node label
    pub label: Option<String>,
    
    /// Optional metadata
    pub metadata: Option<String>,
}
```

### Derivation Pattern
```rust
let selection_context = Signal::derive(move || {
    if let Some(id) = selected_id.get() {
        let node = nodes.get()
            .iter()
            .find_map(|n| n.find(&id))
            .cloned();
        
        if let Some(node) = node {
            SelectionContext::with_selection(
                node.id,
                node.node_type,
                node.label,
                node.metadata,
            )
        } else {
            SelectionContext::new()
        }
    } else {
        SelectionContext::new()
    }
});
```

**Why Derived:**
- Always in sync with tree
- No manual updates
- Reactive propagation
- Single source of truth

### Helper Methods
```rust
impl SelectionContext {
    pub fn is_type(&self, type_name: &str) -> bool {
        self.node_type.as_deref() == Some(type_name)
    }
    
    pub fn has_selection(&self) -> bool {
        self.selected_id.is_some()
    }
}
```

---

## Contextual Commands

### Pattern: Commands Filter by Context
```rust
pub fn create_contextual_commands(
    context: Signal<SelectionContext>,
    on_action: Callback<String>,
) -> Signal<CommandRegistry> {
    Signal::derive(move || {
        let ctx = context.get();
        let mut registry = CommandRegistry::new();
        
        // Commands for "step" nodes
        if ctx.is_type("step") {
            registry = registry.add_group(
                CommandGroup::new("step-actions", "Step Actions")
                    .add_command(Command {
                        id: "step.complete".into(),
                        label: format!("Complete: {}", ctx.label.unwrap_or_default()),
                        icon: Some("✅".into()),
                        callback: CommandCallback::new(move || {
                            on_action.run(format!("complete:{}", ctx.selected_id.unwrap_or_default()))
                        }),
                        ..Default::default()
                    })
            );
        }
        
        // Commands for "workflow" nodes
        if ctx.is_type("workflow") {
            registry = registry.add_group(
                CommandGroup::new("workflow-actions", "Workflow Actions")
                    .add_command(Command {
                        id: "workflow.add_step".into(),
                        label: "Add Step".into(),
                        icon: Some("➕".into()),
                        callback: CommandCallback::new(move || {
                            on_action.run(format!("add_step:{}", ctx.selected_id.unwrap_or_default()))
                        }),
                        ..Default::default()
                    })
            );
        }
        
        // Global commands (always available)
        registry = registry.add_group(
            CommandGroup::new("navigation", "Navigation")
                .add_command(Command {
                    id: "nav.dashboard".into(),
                    label: "Go to Dashboard".into(),
                    icon: Some("🏠".into()),
                    callback: CommandCallback::new(move || {
                        on_action.run("nav:dashboard".into())
                    }),
                    ..Default::default()
                })
        );
        
        registry
    })
}
```

**Key Pattern:**
- Commands **react** to context changes
- Context **never** calls commands
- Tree **never** knows about commands
- One-way flow: Tree → Context → Commands

---

## Integration Pattern

### Full Integration Example
```rust
#[component]
pub fn AppWithTreeAndCommands() -> impl IntoView {
    let (nodes, set_nodes) = signal(create_tree_structure());
    let (selected_id, set_selected_id) = signal(None::<String>);
    let (palette_open, set_palette_open) = signal(false);
    
    // 1. Derive context from selection
    let selection_context = Signal::derive(move || {
        if let Some(id) = selected_id.get() {
            let node = nodes.get().iter()
                .find_map(|n| n.find(&id))
                .cloned();
            
            if let Some(node) = node {
                SelectionContext::with_selection(
                    node.id,
                    node.node_type,
                    node.label,
                    node.metadata,
                )
            } else {
                SelectionContext::new()
            }
        } else {
            SelectionContext::new()
        }
    });
    
    // 2. Create contextual commands
    let on_action = Callback::new(move |action: String| {
        leptos::logging::log!("Action: {}", action);
        // Execute action via server function, etc.
        set_palette_open.set(false);
    });
    
    let contextual_commands = create_contextual_commands(
        selection_context,
        on_action
    );
    
    // 3. Wire tree callbacks
    let on_select = Callback::new(move |id: String| {
        set_selected_id.set(Some(id));
    });
    
    let on_toggle = Callback::new(move |id: String| {
        set_nodes.update(|nodes| {
            for node in nodes.iter_mut() {
                if let Some(found) = node.find_mut(&id) {
                    found.expanded = !found.expanded;
                    break;
                }
            }
        });
    });
    
    view! {
        <div>
            <Tree
                nodes=nodes
                selected_id=selected_id
                on_select=on_select
                on_toggle=on_toggle
            />
            
            <CommandPalette
                registry=contextual_commands.get()
                open=palette_open
                on_close=Callback::new(move |_| set_palette_open.set(false))
            />
        </div>
    }
}
```

---

## Node Type Conventions

### Recommended Types

| Type | Represents | Commands Example |
|------|------------|------------------|
| `root` | Top-level container | Add child, Expand all |
| `collection` | Group of items | Add item, Filter |
| `workflow` | Workflow instance | Add step, Archive, Export |
| `step` | Workflow step | Complete, Fail, Reset |
| `user` | User entity | Edit, Disable, Reset password |
| `role` | Role entity | Edit permissions, Delete |
| `file` | File system item | Open, Rename, Delete |
| `folder` | Directory | Add file, Collapse all |

### Type Naming
```
lowercase
single_word_preferred
use_underscore_for_compound
```

**Examples:**
- ✅ `step`, `workflow`, `user`, `role`
- ✅ `workflow_step`, `user_group`
- ❌ `WorkflowStep`, `STEP`, `Workflow-Step`

---

## Tree State Management

### Expand/Collapse
```rust
let on_toggle = Callback::new(move |id: String| {
    set_nodes.update(|nodes: &mut Vec<TreeNode>| {
        for node in nodes.iter_mut() {
            if let Some(found) = node.find_mut(&id) {
                found.expanded = !found.expanded;
                break;
            }
        }
    });
});
```

### Expand All / Collapse All
```rust
fn expand_all(nodes: &mut Vec<TreeNode>) {
    for node in nodes.iter_mut() {
        node.expanded = true;
        expand_all(&mut node.children);
    }
}

fn collapse_all(nodes: &mut Vec<TreeNode>) {
    for node in nodes.iter_mut() {
        node.expanded = false;
        collapse_all(&mut node.children);
    }
}
```

### Persist Expand State
```rust
// Save to localStorage
#[cfg(target_arch = "wasm32")]
fn save_expand_state(nodes: &Vec<TreeNode>) {
    let expanded_ids: Vec<String> = nodes.iter()
        .flat_map(|n| get_expanded_ids(n))
        .collect();
    
    let json = serde_json::to_string(&expanded_ids).unwrap();
    window().local_storage()
        .unwrap()
        .unwrap()
        .set_item("tree_expanded", &json)
        .ok();
}

// Restore from localStorage
fn restore_expand_state(nodes: &mut Vec<TreeNode>, expanded_ids: &[String]) {
    for node in nodes.iter_mut() {
        if expanded_ids.contains(&node.id) {
            node.expanded = true;
        }
        restore_expand_state(&mut node.children, expanded_ids);
    }
}
```

---

## Keyboard Navigation

### Basic Pattern
```rust
#[component]
pub fn TreeWithKeyboard() -> impl IntoView {
    let (selected_index, set_selected_index) = signal(0usize);
    
    let all_ids = Signal::derive(move || {
        flatten_tree_ids(&nodes.get())
    });
    
    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        match ev.key().as_str() {
            "ArrowDown" => {
                ev.prevent_default();
                set_selected_index.update(|idx| {
                    *idx = (*idx + 1).min(all_ids.get().len() - 1);
                });
            }
            "ArrowUp" => {
                ev.prevent_default();
                set_selected_index.update(|idx| {
                    *idx = idx.saturating_sub(1);
                });
            }
            "ArrowRight" => {
                // Expand if collapsed
                ev.prevent_default();
                expand_selected();
            }
            "ArrowLeft" => {
                // Collapse if expanded
                ev.prevent_default();
                collapse_selected();
            }
            "Enter" => {
                // Select/activate
                ev.prevent_default();
                activate_selected();
            }
            _ => {}
        }
    };
    
    view! {
        <div on:keydown=on_keydown tabindex="0">
            <Tree ... />
        </div>
    }
}
```

---

## Testing

### Test TreeNode Utilities
```rust
#[test]
fn test_find() {
    let tree = TreeNode::new("root", "Root", "root")
        .with_children(vec![
            TreeNode::new("child", "Child", "node"),
        ]);
    
    assert!(tree.find("root").is_some());
    assert!(tree.find("child").is_some());
    assert!(tree.find("missing").is_none());
}

#[test]
fn test_depth() {
    let tree = TreeNode::new("root", "Root", "root")
        .with_children(vec![
            TreeNode::new("l1", "Level 1", "node")
                .with_children(vec![
                    TreeNode::new("l2", "Level 2", "node"),
                ]),
        ]);
    
    assert_eq!(tree.depth(), 2);
}
```

### Test Selection Context
```rust
#[test]
fn test_context_is_type() {
    let ctx = SelectionContext::with_selection(
        "step-1".into(),
        "step".into(),
        "Review".into(),
        None,
    );
    
    assert!(ctx.is_type("step"));
    assert!(!ctx.is_type("workflow"));
}
```

### Test Contextual Commands
```rust
#[test]
fn test_commands_change_with_context() {
    let (context, set_context) = signal(SelectionContext::new());
    let on_action = Callback::new(|_| {});
    
    let commands = create_contextual_commands(context.into(), on_action);
    
    // No selection → only global commands
    assert_eq!(commands.get().get_all_commands().len(), 2);  // Dashboard, Settings
    
    // Select step → step commands appear
    set_context.set(SelectionContext::with_selection(
        "step-1".into(),
        "step".into(),
        "Review".into(),
        None,
    ));
    
    assert_eq!(commands.get().get_all_commands().len(), 5);  // 3 step + 2 global
}
```

---

## Anti Patterns

### Anti-Pattern 1: Tree with Actions
```rust
// ❌ WRONG
#[component]
pub fn TreeNodeItem(node: TreeNode) -> impl IntoView {
    view! {
        <div>
            {node.label}
            {if node.type == "step" {
                view! { <button on:click=complete>"Complete"</button> }  // ❌
            }}
        </div>
    }
}
```

**Fix:** Commands come from CommandPalette based on context

### Anti-Pattern 2: Context with Logic
```rust
// ❌ WRONG
impl SelectionContext {
    pub fn can_complete(&self) -> bool {
        self.metadata.as_ref()
            .map(|m| m.contains("active"))
            .unwrap_or(false)
    }
}
```

**Fix:** Business logic lives on server, not in context

### Anti-Pattern 3: Mixed Responsibilities
```rust
// ❌ WRONG
#[component]
pub fn Tree() -> impl IntoView {
    let on_select = move |id| {
        set_selected(id);
        fetch_details(id);      // ❌ Side effect
        update_breadcrumbs(id); // ❌ Side effect
    };
}
```

**Fix:** Tree only updates selection, consumers react

---

## Related Rules

- [Canon Rule #46: Command Palette & Intent Surfaces](./canon-rule-46-command-palette-intent-surfaces.md) - Intent layer
- [Canon Rule #43: Domain Components and Commands](./canon-rule-43-domain-components-and-commands.md) - CQRS separation
- [Canon Rule #44: Orchestrators](./canon-rule-44-orchestrators.md) - Coordination

---

## Summary

**Golden Rule:** Tree exposes context, never executes intent.

**Flow:**
```
Tree → SelectionContext → CommandRegistry → CommandPalette
```

**Tree Responsibilities:**
- ✅ Render hierarchy
- ✅ Expand/collapse
- ✅ Visual selection
- ✅ Emit selection events

**Tree NEVER:**
- ❌ Execute actions
- ❌ Know about commands
- ❌ Contain business logic
- ❌ Trigger side effects

**SelectionContext:**
- Derived from tree selection
- Pure data (id, type, label, metadata)
- No logic, no actions

**Contextual Commands:**
- Filter based on context
- One-way dependency (commands → context)
- Reactive updates

**Result:**
- Decoupled navigation from actions
- Scalable command system
- Testable in isolation
- Keyboard-first UX
- Enterprise-grade

---


---


---

## Breadcrumb Pattern

### What is Breadcrumb?

**Breadcrumb** = Visual representation of the path from root to current selection

It's **NOT:**
- A standalone navigation component
- Independent from tree structure
- Manually maintained

It's **DERIVED FROM:**
- Tree selection
- Tree hierarchy
- Navigation context
```
Tree Selection → build_path() → BreadcrumbItemData[] → Breadcrumb Component
```

---



---



---



---





---










---


---




---






---



---

## Summary with Breadcrumb

**Updated Golden Rule:** Tree exposes context, Breadcrumb visualizes path, Commands execute intent.

**Complete Flow:**
```
Tree Selection → build_path() → Breadcrumb (render path)
                                      ↓
                              User clicks item
                                      ↓
                           Update tree selection
                                      ↓
                           Selection Context
                                      ↓
                           Command Registry (filtered)
                                      ↓
                           Command Palette (display + execute)
```

**Separation of Concerns:**

| Component | Responsibility | Never Does |
|-----------|---------------|------------|
| **Tree** | Structure + Selection | Execute actions, render path |
| **Breadcrumb** | Path visualization | Know tree structure, execute actions |
| **SelectionContext** | Current state | Render UI, execute actions |
| **CommandPalette** | Intent execution | Know tree structure, build paths |

**Result:**
- Tree for navigation (what exists)
- Breadcrumb for orientation (where am I)
- Commands for action (what can I do)
- All reactive, all decoupled

---


---

## Route Synchronization Pattern

### The Problem: Ephemeral State

**Without Route Sync:**
- Tree selection lost on refresh
- Can't share "open at step X" link
- Browser back/forward doesn't work
- SSR renders collapsed tree (bad UX)

**With Route Sync:**
- ✅ URL is source of truth
- ✅ Shareable deep links
- ✅ Browser navigation works
- ✅ SSR renders expanded state
- ✅ Refresh preserves context

---



---





---









---





---









---




---



---












---

## Complete Navigation System Summary

**Three Pillars:**

1. **Tree** - Structure + Selection
2. **Breadcrumb** - Path Visualization
3. **Command Palette** - Contextual Actions

**With Route Sync:**

4. **URL** - Persistent State (source of truth)
```
┌─────────────────────────────────────────────┐
│ URL (Query Param)                           │
│ /components?path=/workflows/w1/step-1-2     │
└────────┬────────────────────────────────────┘
         │ (source of truth)
         ↓
┌────────────────────────────────────────────┐
│ Tree (renders expanded + selected)         │
└────────┬───────────────────────────────────┘
         │
         ↓
┌────────────────────────────────────────────┐
│ Breadcrumb (shows path: Home › W1 › Step)  │
└────────┬───────────────────────────────────┘
         │
         ↓
┌────────────────────────────────────────────┐
│ Selection Context (type=step, id=step-1-2) │
└────────┬───────────────────────────────────┘
         │
         ↓
┌────────────────────────────────────────────┐
│ Commands (Complete, Fail, Reset)           │
└────────────────────────────────────────────┘
```

**Benefits:**
- ✅ Deep linking
- ✅ Browser navigation
- ✅ Shareable state
- ✅ SSR optimization
- ✅ No state duplication
- ✅ Single source of truth

---
