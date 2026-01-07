# Canon Rule #45: Demo Components & Ephemeral State

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-02

## Principle
Demo components optimize for **clarity**, not realism. Ephemeral state with explicit signals is preferred over production patterns when the goal is documentation, learning, or visual showcase.

---

## The Core Insight

**Demos are NOT mini-production systems.**

They serve a different purpose:

| Production Components | Demo Components |
|----------------------|-----------------|
| Optimize for architecture | Optimize for understanding |
| Use real data sources | Use hardcoded state |
| Follow CQRS strictly | Use direct mutations |
| SSR-critical | SSR-optional |
| Shared state patterns | Individual signals |
| Enterprise patterns | Pedagogical patterns |

---

## When to Use Demo Patterns

### ✅ USE Demo Patterns When:

1. **Documentation & Learning**
   - Component library showcase
   - Tutorial examples
   - Interactive documentation
   - Onboarding materials

2. **Visual Testing**
   - Storybook-style stories
   - Visual regression testing
   - Design system showcase
   - UX prototyping

3. **Proof of Concept**
   - Exploring new patterns
   - Testing visual ideas
   - Rapid prototyping
   - Client presentations

### ❌ DON'T Use Demo Patterns When:

1. **Production Features**
   - User-facing features
   - Business logic
   - Data persistence needed
   - Audit requirements

2. **Reusable Infrastructure**
   - Shared components
   - Library components
   - Framework primitives

---

## Demo Pattern: Explicit Signals

### The Pattern
```rust
#[component]
pub fn WorkflowDemo() -> impl IntoView {
    // ✅ One signal per piece of state
    let (step1_status, set_step1) = signal(StepStatus::Active);
    let (step2_status, set_step2) = signal(StepStatus::Pending);
    
    // ✅ Direct mutation (no async, no server)
    let transition = move |new_status| {
        set_step1.set(new_status);
    };
    
    view! {
        <div>
            <StepItem status=step1_status />
            <button on:click=move |_| transition(StepStatus::Completed)>
                "Complete"
            </button>
        </div>
    }
}
```

**Why this works for demos:**
- Crystal clear reactivity (visual A → B)
- Zero infrastructure noise
- Easy to understand in 30 seconds
- No async complications
- No SSR edge cases

### Anti-Pattern: Simulating Production
```rust
// ❌ WRONG for a demo
#[component]
pub fn WorkflowDemo() -> impl IntoView {
    let steps = Resource::new(|| (), |_| async {
        fetch_from_fake_api().await  // Why?
    });
    
    let action = ServerMultiAction::new(...);  // Overkill
    
    Effect::new(move |_| {
        if action.version().get() > 0 {
            steps.refetch();  // Simulating refresh cycle
        }
    });
    
    // ... 100 lines later ...
}
```

**Why this fails:**
- Hides the actual component being demoed
- Adds infrastructure complexity
- Confuses learners ("why all this boilerplate?")
- Makes simple things look hard

---

## Production Pattern: CQRS

### The Pattern
```rust
// Domain Component (READ)
#[component]
pub fn WorkflowView(
    refresh_trigger: ReadSignal<u32>
) -> impl IntoView {
    let steps = Resource::new(
        move || refresh_trigger.get(),
        |_| async { fetch_workflow_steps().await }
    );
    
    view! {
        <Transition>
            {move || steps.get().map(|s| /* render */)}
        </Transition>
    }
}

// Command Component (WRITE)
#[component]
pub fn WorkflowActions(
    on_complete: Callback<()>
) -> impl IntoView {
    view! {
        <button on:click=move |_| {
            #[cfg(target_arch = "wasm32")]
            spawn_local(async move {
                transition_step().await;
                on_complete.run(());
            });
        }>"Complete"</button>
    }
}

// Orchestrator
#[component]
pub fn WorkflowClient() -> impl IntoView {
    let (refresh, set_refresh) = signal(0);
    
    view! {
        <WorkflowView refresh_trigger=refresh />
        <WorkflowActions on_complete=move |_| set_refresh.update(|n| *n += 1) />
    }
}
```

**Why this is production:**
- Separates concerns
- SSR-safe domain components
- Testable in isolation
- Scales to real requirements

---

## Decision Matrix

| Question | Demo | Production |
|----------|------|------------|
| Will this be deployed? | ❌ | ✅ |
| Needs database? | ❌ | ✅ |
| Needs SSR? | Maybe | Usually |
| Shared state across users? | ❌ | ✅ |
| Audit trail required? | ❌ | ✅ |
| Used in multiple features? | ❌ | ✅ |

**Rule:** If 3+ answers are "Production", don't use demo patterns.

---

## Demo State Patterns

### Pattern 1: Individual Signals (Recommended)
```rust
let (counter, set_counter) = signal(0);
let (name, set_name) = signal("Alice".to_string());
let (active, set_active) = signal(true);
```

**Use when:**
- < 10 pieces of state
- State is independent
- Clarity is priority

### Pattern 2: Struct Signal (For Related State)
```rust
#[derive(Clone)]
struct DemoState {
    counter: i32,
    name: String,
    active: bool,
}

let (state, set_state) = signal(DemoState {
    counter: 0,
    name: "Alice".into(),
    active: true,
});
```

**Use when:**
- State is tightly coupled
- Need to reset all at once
- < 5 fields in struct

### Pattern 3: RwSignal for Simple Mutation
```rust
let state = RwSignal::new(vec![1, 2, 3]);

let add_item = move |item| {
    state.update(|s| s.push(item));
};
```

**Use when:**
- Simple list/vec manipulation
- No complex reactivity needed
- Mutations are atomic

---

## Common Demo Mistakes

### Mistake 1: Over-Engineering
```rust
// ❌ Too much for a demo
#[component]
pub fn ButtonDemo() -> impl IntoView {
    let db = use_context::<Database>();  // Why?
    let auth = use_context::<Auth>();    // Why?
    
    view! {
        <Button>"Click me"</Button>
    }
}
```

**Fix:** Just render the button.

### Mistake 2: Hiding the Subject
```rust
// ❌ Demo about Button, but wrapped in noise
#[component]
pub fn ButtonDemo() -> impl IntoView {
    view! {
        <ThemeProvider>
            <AuthGuard>
                <Layout>
                    <Sidebar />
                    <Main>
                        <Button>"Click"</Button>  // Lost in noise
                    </Main>
                </Layout>
            </AuthGuard>
        </ThemeProvider>
    }
}
```

**Fix:** Minimal wrapper.

### Mistake 3: Fake Async
```rust
// ❌ Simulating async for no reason
let data = Resource::new(|| (), |_| async {
    sleep(Duration::from_millis(500)).await;  // Fake delay
    Ok("Hello")
});
```

**Fix:** Just use a signal.

---

## Documentation Guidelines

### Good Demo Documentation
```rust
/// WorkflowDemo - Interactive showcase
/// 
/// **Purpose:** Visual documentation of workflow transitions
/// **Pattern:** Individual signals (clarity over realism)
/// **NOT for:** Production use, real workflows, SSR-critical paths
/// 
/// Each step has independent state for pedagogical clarity.
#[component]
pub fn WorkflowDemo() -> impl IntoView { ... }
```

### Bad Demo Documentation
```rust
/// Workflow component
#[component]
pub fn WorkflowDemo() -> impl IntoView { ... }
```

**Always include:**
- Purpose statement
- Pattern used
- What it's NOT for

---

## Testing Demos

### Unit Test Pattern
```rust
#[test]
fn demo_transitions_work() {
    let runtime = create_runtime();
    
    let rendered = mount_to_body(|| view! {
        <WorkflowDemo />
    });
    
    // Test the demo itself works
    rendered.find("button.start").click();
    assert!(rendered.find(".active").exists());
    
    runtime.dispose();
}
```

**Focus:** Does the demo demonstrate the concept correctly?

---

## Migration Path

### From Demo to Production
```
Week 1: Prototype with demo patterns
  ✅ Validate UX
  ✅ Test visual design
  ✅ Get stakeholder approval

Week 2: Design production architecture
  ✅ Identify data sources
  ✅ Plan CQRS separation
  ✅ Design audit strategy

Week 3: Implement production version
  ✅ Domain components (READ)
  ✅ Command components (WRITE)
  ✅ Orchestrators (GLUE)

Week 4: Replace demo
  ✅ Swap demo with production
  ✅ Keep demo in storybook
  ✅ Update documentation
```

**Key:** Demo stays in docs, production goes to app.

---

## Component Location

| Type | Location | Reason |
|------|----------|--------|
| Demo | `rs-design/components/*/demo.rs` | Documentation purposes |
| Production Domain | `rs-design/components/*/` | Reusable across apps |
| Production Commands | `apps/*/components/` | App-specific logic |
| Production Orchestrators | `apps/*/components/` | App-specific coordination |

---

## Related Rules

- [Canon Rule #43: Domain Components and Commands](./canon-rule-43-domain-components-and-commands.md) - CQRS pattern
- [Canon Rule #44: Orchestrators](./canon-rule-44-orchestrators.md) - Coordination layer
- [Canon Rule #41: Leptos Resource Consumption](./canon-rule-41-leptos-resource-consumption.md) - Async patterns
- [Canon Rule #1: Component Types](./canon-rule-01-types.md) - Component classification

---

## Summary

**Golden Rule:** Demos teach, production ships.

**Demo Principles:**
- Clarity over correctness
- Explicit over implicit
- Simple over scalable
- Visual over architectural

**When to use demos:**
- Documentation
- Learning
- Prototyping
- Visual testing

**When NOT to use demos:**
- Production features
- Real user data
- Business requirements
- Shared infrastructure

**Remember:**
> "A demo that looks like production code teaches the wrong lesson."

The best demos make complex things look simple. Production code makes simple things reliable.

---

**Enforcement:** Documentation review  
**Violation:** Using production patterns in demos (or vice versa)  
**Severity:** Low (clarity issue, not correctness)

---

**Last Updated:** 2025-01-02  
**Series Complete:** Rules #41-45 (Leptos CQRS Architecture)
