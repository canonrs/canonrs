# Canon Rule #44: Orchestrators

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-02

## Principle
Orchestrators are the **GLUE** between Domain Components (READ) and Command Components (WRITE). They coordinate the CQRS cycle at the UI level without implementing business logic or rendering complexity.

---

## The Problem

### ❌ WRONG: Tight Coupling
```rust
// Domain component knows about commands
#[component]
pub fn WorkflowView() -> impl IntoView {
    let steps = Resource::new(|| (), |_| fetch_steps());
    
    // ❌ Domain component emitting commands
    let on_click = move |_| {
        spawn_local(async { transition_step().await });
    };
    
    view! {
        <div>
            {/* rendering */}
            <button on:click=on_click>"Complete"</button>  // ❌
        </div>
    }
}
```

**Problems:**
- Domain component has side effects
- Cannot SSR safely
- Cannot test READ independently
- Cannot reuse commands outside this view

### ✅ CORRECT: Orchestrator Pattern
```rust
// Domain Component (READ)
#[component]
pub fn WorkflowView(
    #[prop(optional)] refresh_trigger: Option<ReadSignal<u32>>
) -> impl IntoView {
    let steps = Resource::new(
        move || refresh_trigger.map(|r| r.get()).unwrap_or(0),
        |_| async { fetch_steps().await }
    );
    
    view! {
        <Transition>
            {move || steps.get().map(|s| {
                // Pure rendering, no commands
            })}
        </Transition>
    }
}

// Command Component (WRITE)
#[component]
pub fn WorkflowActions(
    step_id: String,
    #[prop(into)] on_complete: Callback<()>
) -> impl IntoView {
    view! {
        <button on:click=move |_| {
            #[cfg(target_arch = "wasm32")]
            {
                use leptos::task::spawn_local;
                let sid = step_id.clone();
                spawn_local(async move {
                    if transition_step(sid).await.is_ok() {
                        on_complete.run(());
                    }
                });
            }
        }>
            "Complete"
        </button>
    }
}

// Orchestrator (GLUE)
#[component]
pub fn WorkflowClient() -> impl IntoView {
    let (refresh, set_refresh) = signal(0);
    
    let on_complete = Callback::new(move |_| {
        set_refresh.update(|n| *n += 1);
    });
    
    view! {
        <div>
            <WorkflowView refresh_trigger=refresh />
            <WorkflowActions
                step_id="approval"
                on_complete=on_complete
            />
        </div>
    }
}
```

---

## Orchestrator Contract

### What Orchestrators DO

✅ **Coordinate Refresh Cycle**
```rust
let (refresh, set_refresh) = signal(0);

let on_command_complete = Callback::new(move |_| {
    set_refresh.update(|n| *n += 1);
});
```

✅ **Pass Callbacks Between Layers**
```rust
// Orchestrator creates callback
let on_done = Callback::new(move |_| {
    // Trigger refresh
    // Log analytics
    // Show notification
});

// Passes to command component
<WorkflowActions on_complete=on_done />
```

✅ **Manage Layout**
```rust
view! {
    <div class="grid grid-cols-2 gap-6">
        <WorkflowView refresh_trigger=refresh />
        <WorkflowActions on_complete=on_done />
    </div>
}
```

✅ **Handle Optimistic Updates**
```rust
let (optimistic_state, set_optimistic) = signal(None);

let on_command = Callback::new(move |cmd| {
    // Optimistic update
    set_optimistic.set(Some(cmd.clone()));
    
    // Actual command
    spawn_local(async move {
        match execute_command(cmd).await {
            Ok(_) => {
                // Server confirmed
                set_optimistic.set(None);
                refresh.update(|n| *n += 1);
            }
            Err(_) => {
                // Rollback optimistic
                set_optimistic.set(None);
            }
        }
    });
});
```

### What Orchestrators NEVER DO

❌ **Implement Business Rules**
```rust
// ❌ WRONG
if step.status == "Active" && user.role == "Admin" {
    // Business logic in orchestrator
}
```
**Rule:** Business rules live in server/core

❌ **Complex Rendering**
```rust
// ❌ WRONG
view! {
    <div class="complex-layout">
        <div class="sidebar">
            <div class="nested">
                <div class="deep">
                    // 100 lines of rendering
                </div>
            </div>
        </div>
    </div>
}
```
**Rule:** Delegate to domain components

❌ **Direct Database Access**
```rust
// ❌ WRONG
let conn = Connection::open("db.sqlite");
```
**Rule:** Use server functions

❌ **Heavy State Logic**
```rust
// ❌ WRONG
let complex_derived = Memo::new(move |_| {
    // 50 lines of computation
});
```
**Rule:** Derive in domain components or server

---

## Orchestrator Patterns

### Pattern 1: Simple Refresh
```rust
#[component]
pub fn SimpleOrchestrator() -> impl IntoView {
    let (refresh, set_refresh) = signal(0);
    
    view! {
        <DomainView refresh=refresh />
        <Commands on_done=move |_| set_refresh.update(|n| *n += 1) />
    }
}
```

**Use when:**
- Single domain view
- Single command source
- Simple refresh cycle

### Pattern 2: Multiple Resources
```rust
#[component]
pub fn MultiResourceOrchestrator() -> impl IntoView {
    let (steps_refresh, set_steps) = signal(0);
    let (audit_refresh, set_audit) = signal(0);
    
    let on_transition = Callback::new(move |_| {
        set_steps.update(|n| *n += 1);
        set_audit.update(|n| *n += 1);
    });
    
    view! {
        <StepsView refresh=steps_refresh />
        <AuditView refresh=audit_refresh />
        <Actions on_complete=on_transition />
    }
}
```

**Use when:**
- Multiple related resources
- All need refresh on command
- Coordinated updates

### Pattern 3: Conditional Commands
```rust
#[component]
pub fn ConditionalOrchestrator() -> impl IntoView {
    let (refresh, set_refresh) = signal(0);
    
    let steps_for_actions = Resource::new(
        move || refresh.get(),
        |_| async { fetch_steps().await }
    );
    
    view! {
        <DomainView refresh=refresh />
        
        <Transition>
            {move || steps_for_actions.get().map(|result| match result {
                Ok(steps) => view! {
                    {steps.into_iter().map(|step| view! {
                        <Actions
                            step_id=step.id
                            status=step.status
                            on_done=move |_| set_refresh.update(|n| *n += 1)
                        />
                    }).collect_view()}
                }.into_any(),
                Err(_) => view! { <div>"No actions"</div> }.into_any()
            })}
        </Transition>
    }
}
```

**Use when:**
- Available commands depend on state
- Dynamic action buttons
- Server determines what's allowed

### Pattern 4: Cross-Component Coordination
```rust
#[component]
pub fn CoordinatedOrchestrator() -> impl IntoView {
    let (workflow_refresh, set_workflow) = signal(0);
    let (notifications_refresh, set_notifications) = signal(0);
    
    let on_workflow_change = Callback::new(move |_| {
        set_workflow.update(|n| *n += 1);
        set_notifications.update(|n| *n += 1);
    });
    
    view! {
        <WorkflowView refresh=workflow_refresh />
        <NotificationsView refresh=notifications_refresh />
        <WorkflowActions on_complete=on_workflow_change />
    }
}
```

**Use when:**
- Command affects multiple domains
- Side effects needed (notifications, analytics)
- Coordinated UI updates

---

## Orchestrator Lifecycle
```
┌─────────────────────────────────────┐
│ 1. User Intent                      │
│    (button click)                   │
└────────────┬────────────────────────┘
             │
┌────────────▼────────────────────────┐
│ 2. Command Component                │
│    - Validates input                │
│    - Calls server function          │
└────────────┬────────────────────────┘
             │
┌────────────▼────────────────────────┐
│ 3. Command Callback                 │
│    - on_complete.run(())            │
└────────────┬────────────────────────┘
             │
┌────────────▼────────────────────────┐
│ 4. Orchestrator                     │
│    - set_refresh.update(|n| *n+1)   │
│    - Triggers resource refetch      │
└────────────┬────────────────────────┘
             │
┌────────────▼────────────────────────┐
│ 5. Domain Component                 │
│    - Resource detects change        │
│    - Refetches data                 │
│    - Re-renders                     │
└─────────────────────────────────────┘
```

---

## Testing Strategy

### Test Domain Components Independently
```rust
#[test]
fn domain_view_renders_state() {
    let (refresh, _) = signal(0);
    
    let rendered = mount_to_body(|| view! {
        <WorkflowView refresh_trigger=refresh />
    });
    
    assert!(rendered.inner_html().contains("Code Review"));
}
```

### Test Command Components Independently
```rust
#[test]
fn command_emits_callback() {
    let (called, set_called) = signal(false);
    let on_done = Callback::new(move |_| set_called.set(true));
    
    let rendered = mount_to_body(|| view! {
        <WorkflowActions
            step_id="test"
            on_complete=on_done
        />
    });
    
    rendered.find("button").click();
    assert!(called.get());
}
```

### Test Orchestrator Coordination
```rust
#[test]
fn orchestrator_coordinates_refresh() {
    let rendered = mount_to_body(|| view! {
        <WorkflowClient />
    });
    
    // Simulate command
    rendered.find(".action-button").click();
    
    // Wait for refresh
    wait_for(|| {
        rendered.find(".workflow-view").text().contains("Updated")
    });
}
```

---

## Anti-Patterns

### Anti-Pattern 1: God Orchestrator
```rust
// ❌ WRONG: Orchestrator doing everything
#[component]
pub fn GodOrchestrator() -> impl IntoView {
    let (state1, set1) = signal(...);
    let (state2, set2) = signal(...);
    let (state3, set3) = signal(...);
    // ... 20 more signals
    
    let complex_logic = Memo::new(move |_| {
        // 100 lines of business logic
    });
    
    view! {
        <div>
            // 500 lines of rendering
        </div>
    }
}
```

**Fix:** Break into smaller orchestrators + domain components

### Anti-Pattern 2: Callback Hell
```rust
// ❌ WRONG: Too many callback layers
let on_a = Callback::new(move |_| {
    let on_b = Callback::new(move |_| {
        let on_c = Callback::new(move |_| {
            // ...
        });
    });
});
```

**Fix:** Flatten with single orchestrator callback

### Anti-Pattern 3: Orchestrator as Data Source
```rust
// ❌ WRONG: Orchestrator fetching data
#[component]
pub fn BadOrchestrator() -> impl IntoView {
    let data = Resource::new(|| (), |_| fetch_data());  // ❌
    
    view! {
        <div>{move || data.get()}</div>  // ❌
    }
}
```

**Fix:** Move Resource to domain component

---

## Location Rules

| Component Type | Location | Reason |
|---------------|----------|--------|
| Domain (READ) | `rs-design/components/` | Reusable across apps |
| Commands (WRITE) | `apps/*/components/` | App-specific logic |
| Orchestrators | `apps/*/components/` | App-specific coordination |

**Exception:** Demo orchestrators (like `WorkflowDemo`) can live in `rs-design` for documentation purposes, clearly marked as ephemeral.

---

## Orchestrator Checklist

Before creating an orchestrator, ask:

- [ ] Does it coordinate READ + WRITE?
- [ ] Does it manage refresh triggers?
- [ ] Does it stay under 100 lines?
- [ ] Does it avoid business logic?
- [ ] Does it avoid complex rendering?
- [ ] Can domain/command components work independently?

If any answer is NO, reconsider the design.

---

## Related Rules

- [Canon Rule #43: Domain Components and Commands](./canon-rule-43-domain-components-and-commands.md) - CQRS in UI
- [Canon Rule #41: Leptos Resource Consumption](./canon-rule-41-leptos-resource-consumption.md) - Resource patterns
- [Canon Rule #5: SSR Effects](./canon-rule-05-ssr-effects.md) - SSR safety

---

## Summary

**Golden Rule:** Orchestrators coordinate, never implement.

**Pattern:**
```
Domain (READ) ← Orchestrator → Commands (WRITE)
```

**Orchestrator responsibilities:**
- ✅ Refresh triggers
- ✅ Callbacks
- ✅ Layout
- ✅ Optimistic updates

**Never:**
- ❌ Business logic
- ❌ Complex rendering
- ❌ Database access
- ❌ Heavy computation

**Keep orchestrators:**
- Small (< 100 lines)
- Simple (coordination only)
- Testable (mock both sides)

---

**Enforcement:** Architecture review + Size limit  
**Violation:** Business logic or rendering in orchestrator  
**Severity:** Medium (breaks separation of concerns)

---

**Last Updated:** 2025-01-02  
**Next Review:** 2025-Q2
