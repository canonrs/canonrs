# Canon Rule #43: Domain Components and Commands

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-02

## Principle
Domain components **NEVER** execute commands directly. They only **OBSERVE** state. Commands are **ALWAYS** emitted by an explicit "Command Component" layer. This rule enforces CQRS at the UI level.

---

## The Problem

### ❌ WRONG: Command Inside Domain Component
```rust
#[component]
pub fn Workflow() -> impl IntoView {
    let steps = Resource::new(|| (), |_| fetch_steps());

    view! {
        <Transition>
            {move || steps.get().map(|steps| view! {
                {steps.into_iter().map(|step| view! {
                    <div>
                        <span>{step.label}</span>
                        <button on:click=move |_| {
                            // ❌ Command inside domain component
                            transition_step(step.id, "Completed");
                        }>
                            "Complete"
                        </button>
                    </div>
                }).collect_view()}
            })}
        </Transition>
    }
}
```

**Why this is architecturally wrong (even if it works):**
- UI decides when and how to change state
- No separation between:
  - Visualization (what to show)
  - Intention (what to do)
  - Authorization (who can do it)
- Impossible to reuse command outside UI
- Breaks clean audit trail
- Breaks future automation
- Breaks CLI / API / background jobs
- Component becomes a "God component"

### ✅ CORRECT: Explicit Separation
```rust
// 1. Domain Component (READ)
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
            {move || steps.get().map(|steps| view! {
                {steps.into_iter().map(|step| view! {
                    <div>
                        <span>{step.label}</span>
                        <span class="status">{step.status}</span>
                    </div>
                }).collect_view()}
            })}
        </Transition>
    }
}

// 2. Command Component (WRITE)
#[component]
pub fn WorkflowActions(
    step_id: String,
    current_status: String,
    #[prop(into)] on_transition_complete: Callback<()>
) -> impl IntoView {
    view! {
        <button on:click=move |_| {
            #[cfg(target_arch = "wasm32")]
            {
                use leptos::task::spawn_local;
                let sid = step_id.clone();
                spawn_local(async move {
                    if transition_step(sid, "Completed".into()).await.is_ok() {
                        on_transition_complete.run(());
                    }
                });
            }
        }>
            "Complete"
        </button>
    }
}

// 3. Orchestrator (GLUE)
#[component]
pub fn WorkflowClient() -> impl IntoView {
    let (refresh, set_refresh) = signal(0);

    let on_done = Callback::new(move |_| {
        set_refresh.update(|n| *n += 1);
    });

    view! {
        <WorkflowView refresh_trigger=refresh />
        <WorkflowActions
            step_id="approval"
            current_status="Active"
            on_transition_complete=on_done
        />
    }
}
```

---

## Component Classification

### 1. Domain Component (READ MODEL)

**What it is:**
- Visualizes state
- Reflects current reality
- Completely passive

**Rules:**
- ❌ No `spawn_local`
- ❌ No mutations
- ❌ No business rules
- ✅ SSR-safe
- ✅ Deterministic

**Example:**
```rust
#[component]
pub fn WorkflowView(
    #[prop(optional)] refresh_trigger: Option<ReadSignal<u32>>
) -> impl IntoView {
    let steps = Resource::new(
        move || refresh_trigger.map(|r| r.get()).unwrap_or(0),
        |_| async { fetch_workflow_steps(1).await }
    );
    
    view! {
        <Transition fallback=|| view! { <div>"Loading..."</div> }>
            {move || steps.get().map(|result| match result {
                Ok(steps) => view! {
                    <div class="workflow-steps">
                        {steps.into_iter().map(|step| {
                            view! {
                                <div class="step">
                                    <span>{step.label}</span>
                                    <span class="status">{step.status}</span>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                }.into_any(),
                Err(_) => view! { <div>"Error"</div> }.into_any()
            })}
        </Transition>
    }
}
```

### 2. Command Component (WRITE MODEL)

**What it is:**
- Emits intentions
- Never renders state
- Never decides business rules

**Rules:**
- ❌ Does not read `Resource`
- ❌ Does not render domain
- ✅ Only knows about commands
- ✅ Can be client-only
- ✅ Can exist without UI (CLI / API)

**Example:**
```rust
#[component]
pub fn WorkflowActions(
    step_id: String,
    current_status: String,
    #[prop(into)] on_transition_complete: Callback<()>
) -> impl IntoView {
    let actions = match current_status.as_str() {
        "Pending" => vec![("Start", "Active")],
        "Active" => vec![("Complete", "Completed"), ("Fail", "Failed")],
        "Completed" => vec![("Reset", "Pending")],
        _ => vec![],
    };
    
    view! {
        <div class="actions">
            {actions.into_iter().map(|(label, new_status)| {
                let sid = step_id.clone();
                let ns = new_status.to_string();
                let on_complete = on_transition_complete;
                
                view! {
                    <button on:click=move |_| {
                        #[cfg(target_arch = "wasm32")]
                        {
                            use leptos::task::spawn_local;
                            let sid = sid.clone();
                            let ns = ns.clone();
                            spawn_local(async move {
                                if transition_step(sid, ns).await.is_ok() {
                                    on_complete.run(());
                                }
                            });
                        }
                    }>
                        {label}
                    </button>
                }
            }).collect_view()}
        </div>
    }
}
```

### 3. Orchestrator (GLUE)

**What it does:**
- Coordinates READ ↔ WRITE
- Updates refresh triggers
- Maintains consistent UX

**Example:**
```rust
#[component]
pub fn WorkflowClient() -> impl IntoView {
    let (refresh, set_refresh) = signal(0u32);
    
    let steps_for_actions = Resource::new(
        move || refresh.get(),
        |_| async { fetch_workflow_steps(1).await }
    );
    
    let on_done = Callback::new(move |_| {
        set_refresh.update(|n| *n += 1);
    });

    view! {
        <div>
            <WorkflowView refresh_trigger=refresh />
            
            <Transition>
                {move || steps_for_actions.get().map(|result| match result {
                    Ok(steps) => view! {
                        {steps.into_iter().map(|step| view! {
                            <WorkflowActions
                                step_id=step.step_id
                                current_status=step.status
                                on_transition_complete=on_done
                            />
                        }).collect_view()}
                    }.into_any(),
                    Err(_) => view! { <div>"Error"</div> }.into_any()
                })}
            </Transition>
        </div>
    }
}
```

---

## Component Capability Matrix

| Type | Can Read State | Can Mutate State | SSR-Safe | Example |
|------|----------------|------------------|----------|---------|
| **Domain Component** | ✅ | ❌ | ✅ | `WorkflowView` |
| **Command Component** | ❌ | ✅ | ❌ (usually) | `WorkflowActions` |
| **Orchestrator** | ⚠️ | ⚠️ | ⚠️ | `WorkflowClient` |

---

## Common Violations

### Violation 1: Button Inside Domain Component
```rust
// ❌ WRONG
#[component]
pub fn WorkflowView() -> impl IntoView {
    view! {
        <div>
            <span>"Step 1"</span>
            <button on:click=move |_| transition_step()>  // ❌
                "Complete"
            </button>
        </div>
    }
}
```

**Fix:**
```rust
// ✅ CORRECT
#[component]
pub fn WorkflowView() -> impl IntoView {
    view! {
        <div>
            <span>"Step 1"</span>
            // No button - pure display
        </div>
    }
}

#[component]
pub fn WorkflowActions() -> impl IntoView {
    view! {
        <button on:click=move |_| transition_step()>
            "Complete"
        </button>
    }
}
```

### Violation 2: Resource Inside Command Component
```rust
// ❌ WRONG
#[component]
pub fn WorkflowActions() -> impl IntoView {
    let steps = Resource::new(|| (), |_| fetch_steps());  // ❌
    
    view! {
        <button>"Complete"</button>
    }
}
```

**Fix:**
```rust
// ✅ CORRECT
#[component]
pub fn WorkflowActions(
    step_id: String,  // Receive data as props
    current_status: String,
) -> impl IntoView {
    view! {
        <button>"Complete"</button>
    }
}
```

### Violation 3: Domain Deciding Business Rules
```rust
// ❌ WRONG
#[component]
pub fn WorkflowView() -> impl IntoView {
    view! {
        {if status == "Active" {  // ❌ UI deciding rules
            view! { <button>"Complete"</button> }
        } else {
            view! { <span>"Not available"</span> }
        }}
    }
}
```

**Fix:**
```rust
// ✅ CORRECT - Server decides rules
#[server]
pub async fn get_available_actions(step_id: String) -> Result<Vec<Action>, ServerFnError> {
    // Business rules live here
    match get_step_status(step_id).await? {
        "Active" => Ok(vec![Action::Complete, Action::Fail]),
        "Pending" => Ok(vec![Action::Start]),
        _ => Ok(vec![]),
    }
}

#[component]
pub fn WorkflowActions() -> impl IntoView {
    let actions = Resource::new(|| (), |_| get_available_actions("step1".into()));
    
    view! {
        <Transition>
            {move || actions.get().map(|acts| {
                acts.into_iter().map(|action| view! {
                    <button>{action.label}</button>
                }).collect_view()
            })}
        </Transition>
    }
}
```

---

## Real Benefits

### ✅ What You Gain

1. **Deterministic UI**
   - Domain components always render the same for same state
   - No hidden side effects
   - Easy to test

2. **Reliable SSR**
   - Domain components work in SSR
   - Commands are client-only by design
   - No hydration mismatches

3. **Clean Audit Trail**
   - Every command is explicit
   - Easy to log
   - Easy to replay

4. **Event Replay**
   - Commands can be stored as events
   - Replayable for debugging
   - Time-travel debugging possible

5. **Shared Core**
   - CLI, API, UI share same commands
   - No UI-specific business logic
   - Single source of truth

6. **Automation Without Refactor**
   - Background jobs use same commands
   - Scheduled actions use same code
   - No duplicate logic

7. **ISO / SOC2 Justifiable**
   - Clear separation of concerns
   - Auditable command trail
   - Provable compliance

---

## Architecture Diagram
```
┌─────────────────────────────────────────┐
│         User Interface Layer            │
├─────────────────────────────────────────┤
│                                         │
│  ┌───────────────┐   ┌──────────────┐  │
│  │ Domain        │   │ Command      │  │
│  │ Component     │   │ Component    │  │
│  │ (READ)        │   │ (WRITE)      │  │
│  │               │   │              │  │
│  │ - WorkflowView│   │ - WorkflowActions│
│  │ - AuditView   │   │ - TransitionButtons│
│  └───────┬───────┘   └──────┬───────┘  │
│          │                  │          │
│          │  ┌───────────────┘          │
│          │  │                          │
│  ┌───────▼──▼───────┐                 │
│  │   Orchestrator   │                 │
│  │   (GLUE)         │                 │
│  │                  │                 │
│  │ - WorkflowClient │                 │
│  └────────┬─────────┘                 │
│           │                           │
└───────────┼───────────────────────────┘
            │
┌───────────▼───────────────────────────┐
│      Application Core / Server        │
├───────────────────────────────────────┤
│                                       │
│  ┌─────────────┐   ┌──────────────┐  │
│  │ Queries     │   │ Commands     │  │
│  │ (READ)      │   │ (WRITE)      │  │
│  │             │   │              │  │
│  │ fetch_steps │   │ transition_  │  │
│  │ fetch_audit │   │ step         │  │
│  └──────┬──────┘   └──────┬───────┘  │
│         │                 │          │
│         └────────┬────────┘          │
│                  │                   │
│         ┌────────▼────────┐          │
│         │   Database      │          │
│         │   - workflows   │          │
│         │   - audit       │          │
│         └─────────────────┘          │
└───────────────────────────────────────┘
```

---

## Testing Strategy

### Test Domain Components
```rust
#[test]
fn workflow_view_renders_state() {
    let runtime = create_runtime();
    
    let steps = vec![
        Step { id: 1, label: "Review".into(), status: "Active".into() },
    ];
    
    let rendered = mount_to_body(|| view! {
        <WorkflowView steps=steps />
    });
    
    assert!(rendered.inner_html().contains("Review"));
    assert!(rendered.inner_html().contains("Active"));
    
    runtime.dispose();
}
```

### Test Command Components
```rust
#[test]
fn workflow_actions_emit_commands() {
    let runtime = create_runtime();
    let (called, set_called) = signal(false);
    
    let on_complete = Callback::new(move |_| set_called.set(true));
    
    let rendered = mount_to_body(|| view! {
        <WorkflowActions
            step_id="test"
            current_status="Active"
            on_transition_complete=on_complete
        />
    });
    
    // Simulate click
    rendered.find("button").click();
    
    assert!(called.get());
    
    runtime.dispose();
}
```

### Test Orchestrator
```rust
#[test]
fn workflow_client_coordinates() {
    let runtime = create_runtime();
    
    let rendered = mount_to_body(|| view! {
        <WorkflowClient />
    });
    
    // Verify both View and Actions are present
    assert!(rendered.find(".workflow-view").exists());
    assert!(rendered.find(".workflow-actions").exists());
    
    runtime.dispose();
}
```

---

## Migration Path

### From Monolithic to CQRS

**Before:**
```rust
#[component]
pub fn Workflow() -> impl IntoView {
    let steps = Resource::new(|| (), |_| fetch_steps());
    
    view! {
        {move || steps.get().map(|s| view! {
            <div>
                <span>{s.label}</span>
                <button on:click=move |_| transition()>
                    "Complete"
                </button>
            </div>
        })}
    }
}
```

**After:**
```rust
// 1. Extract READ
#[component]
pub fn WorkflowView(refresh: ReadSignal<u32>) -> impl IntoView {
    let steps = Resource::new(move || refresh.get(), |_| fetch_steps());
    
    view! {
        {move || steps.get().map(|s| view! {
            <div><span>{s.label}</span></div>
        })}
    }
}

// 2. Extract WRITE
#[component]
pub fn WorkflowActions(on_done: Callback<()>) -> impl IntoView {
    view! {
        <button on:click=move |_| {
            spawn_local(async move {
                transition().await;
                on_done.run(());
            });
        }>
            "Complete"
        </button>
    }
}

// 3. Create Orchestrator
#[component]
pub fn WorkflowClient() -> impl IntoView {
    let (refresh, set_refresh) = signal(0);
    let on_done = Callback::new(move |_| set_refresh.update(|n| *n += 1));
    
    view! {
        <WorkflowView refresh=refresh />
        <WorkflowActions on_done=on_done />
    }
}
```

---

## Related Rules

- [Canon Rule #41: Leptos Resource Consumption](./canon-rule-41-leptos-resource-consumption.md) - Resource vs Future
- [Canon Rule #42: UI Data Surfaces](./canon-rule-42-ui-data-surfaces.md) - Component classification
- [Canon Rule #5: SSR Effects](./canon-rule-05-ssr-effects.md) - SSR safety

---

## Summary

**Golden Rule:** Domain components OBSERVE. Command components ACT. Never mix.

**Pattern:**
```
READ  (Domain)   → Display state
WRITE (Command)  → Emit intention
GLUE  (Orchestrator) → Coordinate both
```

**Never:**
- Put buttons in domain components
- Put Resource in command components
- Decide business rules in UI

**Always:**
- Separate read from write
- Keep domain components passive
- Make commands explicit
- Coordinate via orchestrator

---

**Enforcement:** Architecture review + Linter  
**Violation:** Command inside domain component  
**Severity:** High (breaks CQRS, audit, automation)

---

**Last Updated:** 2025-01-02  
**Next Review:** 2025-Q2
