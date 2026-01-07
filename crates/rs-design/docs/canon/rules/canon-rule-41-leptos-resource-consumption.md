# Canon Rule #41: Leptos Resource Consumption Contract

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-02

## Principle
`Resource` in Leptos is a **reactive signal**, not a `Future`. It must be consumed via reactive tracking (`.get()`, `.read()`, `.with()`), never via `.await`. This rule prevents silent render failures and ensures proper hydration.

## The Problem

### ❌ WRONG: Treating Resource as Future
```rust
#[component]
pub fn MyComponent() -> impl IntoView {
    let data = Resource::new(|| (), |_| async {
        fetch_data().await
    });
    
    view! {
        <Suspense fallback=|| view! { <div>"Loading..."</div> }>
            {move || Suspend::new(async move {
                // ❌ FATAL ERROR: Resource is not a Future
                match data.await {
                    Ok(result) => view! { <div>{result}</div> },
                    Err(_) => view! { <div>"Error"</div> }
                }
            })}
        </Suspense>
    }
}
```

**Why this fails:**
- `Resource` does NOT implement `Future`
- `.await` breaks reactive tracking
- `Suspense` never resolves
- UI stays frozen on fallback
- No error, no panic—just silence

### ✅ CORRECT: Reactive Consumption
```rust
#[component]
pub fn MyComponent() -> impl IntoView {
    let data = Resource::new(|| (), |_| async {
        fetch_data().await
    });
    
    view! {
        <Transition fallback=|| view! { <div>"Loading..."</div> }>
            {move || data.get().map(|result| match result {
                Ok(value) => view! { <div>{value}</div> }.into_any(),
                Err(_) => view! { <div>"Error"</div> }.into_any()
            })}
        </Transition>
    }
}
```

**Why this works:**
- `data.get()` → reactive signal read
- `Transition` knows to wait for `None → Some`
- Leptos controls re-render timing
- Proper hydration
- SSR-safe

## Rule Contract

### Resource API Matrix

| Method | Use Case | Works in SSR | Works in Client | Triggers Re-render |
|--------|----------|--------------|-----------------|-------------------|
| `.get()` | Read current value | ✅ | ✅ | ✅ |
| `.read()` | Read with callback | ✅ | ✅ | ✅ |
| `.with()` | Transform value | ✅ | ✅ | ✅ |
| `.with_untracked()` | Read without tracking | ✅ | ✅ | ❌ |
| `.await` | **NEVER USE** | ❌ | ❌ | ❌ |
| `.refetch()` | Trigger reload | ❌ SSR | ✅ | ✅ |

> ⚠️ `.refetch()` must never be triggered during SSR render phase. It is safe only after hydration, typically via `Action + Effect`.

### Wrapper Components Matrix

| Wrapper | For | Resource Method | Notes |
|---------|-----|-----------------|-------|
| `<Transition>` | `Resource` | `.get()` | SSR + Client |
| `<Suspense>` | Direct `Future` | N/A | Client-only |
| `<Show>` | Conditional | `.get()` | SSR + Client |
| `<For>` | Lists | `.get()` | SSR + Client |

## Common Violations

### Violation 1: Using Suspend with Resource
```rust
// ❌ WRONG
view! {
    <Suspense>
        {move || Suspend::new(async move {
            data_resource.await  // Resource is not Future
        })}
    </Suspense>
}

// ✅ CORRECT
view! {
    <Transition>
        {move || data_resource.get().map(|r| ...)}
    </Transition>
}
```

### Violation 2: Mixing Resource and Action
```rust
// ❌ WRONG (causes spawn_local panic in SSR)
let action = Action::new(|input| async move {
    resource.await  // Double error: Resource not Future + spawn_local
});

// ✅ CORRECT
let action = Action::new(|input| async move {
    call_server_fn().await
});

Effect::new(move |_| {
    if action.version().get() > 0 {
        resource.refetch();
    }
});
```

### Violation 3: Server Function as Resource
```rust
// ❌ WRONG
let data = Resource::new(|| (), |_| async {
    // Server function already returns Future
    my_server_fn().await
});

view! {
    <Suspense>
        {move || Suspend::new(async move {
            data.await  // Nested futures don't work
        })}
    </Suspense>
}

// ✅ CORRECT
let data = Resource::new(|| (), |_| async {
    my_server_fn().await
});

view! {
    <Transition>
        {move || data.get().map(|result| ...)}
    </Transition>
}
```

## Decision Tree
```
Need async data?
├─ SSR required?
│  ├─ YES → Use Resource + Transition
│  └─ NO → Use Resource + Transition (still recommended)
│
├─ User interaction triggers fetch?
│  ├─ YES → Use Action + Effect + Resource.refetch()
│  └─ NO → Use Resource
│
└─ Rendering?
   ├─ Resource → .get() + Transition
   ├─ Direct Future → Suspend (client-only)
   └─ Static data → No wrapper needed
```

## Complete Working Example
```rust
use leptos::prelude::*;

#[server]
pub async fn fetch_workflow_steps(workflow_id: i64) -> Result<Vec<Step>, ServerFnError> {
    // Database query
    Ok(query_db(workflow_id).await?)
}

#[server]
pub async fn transition_step(workflow_id: i64, step_id: String, new_status: String) 
    -> Result<(), ServerFnError> 
{
    update_db(workflow_id, step_id, new_status).await?;
    Ok(())
}

#[component]
pub fn WorkflowView() -> impl IntoView {
    // ✅ CORRECT: Resource for initial data
    let steps = Resource::new(
        || (), 
        |_| async { fetch_workflow_steps(1).await }
    );
    
    // ✅ CORRECT: Action for mutations
    let transition = Action::new(
        move |(wid, sid, status): &(i64, String, String)| {
            let wid = *wid;
            let sid = sid.clone();
            let status = status.clone();
            async move {
                transition_step(wid, sid, status).await
            }
        }
    );
    
    // ✅ CORRECT: Effect to refetch after mutation
    Effect::new(move |_| {
        if transition.version().get() > 0 {
            steps.refetch();
        }
    });
    
    view! {
        // ✅ CORRECT: Transition wraps Resource.get()
        <Transition fallback=|| view! { <div>"Loading..."</div> }>
            {move || steps.get().map(|result| match result {
                Ok(data) => view! {
                    <div>
                        {data.into_iter().map(|step| {
                            let step_id = step.id.clone();
                            view! {
                                <div>
                                    <span>{step.label}</span>
                                    <button on:click=move |_| {
                                        transition.dispatch((1, step_id.clone(), "Active".into()));
                                    }>
                                        "Activate"
                                    </button>
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

## SSR Implications

### Server-Side Rendering Flow
```
1. SSR renders component
   └─ Resource.get() returns None (async not started)
   └─ Transition shows fallback

2. HTML sent to client with fallback

3. Client hydrates
   └─ Resource starts async fetch
   └─ Resource.get() returns Some(result)
   └─ Transition switches to content
```

### Hydration Safety
```rust
// ✅ SAFE: SSR renders same HTML as client expects
view! {
    <Transition fallback=|| view! { <div>"Loading"</div> }>
        {move || resource.get().map(|r| view! { <div>{r}</div> })}
    </Transition>
}

// SSR output: <div>Loading</div>
// Client expects: <div>Loading</div> (initially)
// ✅ Hydration matches

// ❌ UNSAFE: SSR and client render different trees
view! {
    {move || if cfg!(target_arch = "wasm32") {
        view! { <ClientComponent /> }
    } else {
        view! { <div>"SSR placeholder"</div> }
    }}
}

// SSR output: <div>SSR placeholder</div>
// Client expects: <ClientComponent />
// ❌ Hydration mismatch panic
```

## Testing

### Unit Test Template
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use leptos::*;
    
    #[test]
    fn resource_consumption_correct() {
        let runtime = create_runtime();
        
        let data = Resource::new(|| (), |_| async {
            Ok::<_, ()>("test data".to_string())
        });
        
        // ✅ CORRECT: Read via .get()
        let result = data.get();
        assert!(result.is_none()); // Not loaded yet
        
        // ❌ WRONG: Cannot .await Resource
        // let result = data.await; // Compile error
        
        runtime.dispose();
    }
}
```

## Linter Rules
```rust
// tools/leptos-linter/src/resource_rules.rs

pub fn check_resource_await(code: &str) -> Vec<Violation> {
    let mut violations = Vec::new();
    
    // Detect: resource_name.await
    let re = Regex::new(r"(\w+)\.await").unwrap();
    for cap in re.captures_iter(code) {
        let var_name = &cap[1];
        
        // Check if variable is a Resource
        if is_resource_type(var_name, code) {
            violations.push(Violation {
                rule: "CANON-041",
                message: format!(
                    "Resource `{}` consumed via .await. Use .get() instead.",
                    var_name
                ),
                severity: Severity::Error,
            });
        }
    }
    
    violations
}

pub fn check_suspend_with_resource(code: &str) -> Vec<Violation> {
    // Detect: Suspend::new(async move { resource.await })
    if code.contains("Suspend::new") && code.contains(".await") {
        return vec![Violation {
            rule: "CANON-041",
            message: "Suspend used with Resource. Use Transition + .get() instead.".into(),
            severity: Severity::Error,
        }];
    }
    vec![]
}
```

## Migration Guide

### From Suspend to Transition
```rust
// BEFORE (broken)
view! {
    <Suspense fallback=|| view! { <div>"Loading"</div> }>
        {move || Suspend::new(async move {
            match my_resource.await {  // ❌
                Ok(data) => view! { <div>{data}</div> },
                Err(_) => view! { <div>"Error"</div> }
            }
        })}
    </Suspense>
}

// AFTER (working)
view! {
    <Transition fallback=|| view! { <div>"Loading"</div> }>
        {move || my_resource.get().map(|result| match result {  // ✅
            Ok(data) => view! { <div>{data}</div> }.into_any(),
            Err(_) => view! { <div>"Error"</div> }.into_any()
        })}
    </Transition>
}
```

## Related Rules

- [Canon Rule #5: SSR Effects](./canon-rule-05-ssr-effects.md) - Effect execution in SSR
- [Canon Rule #42: Leptos Island Architecture](./canon-rule-42-leptos-island-architecture.md) - Client-only components
- [Canon Rule #42: Leptos Island Architecture](./canon-rule-42-leptos-island-architecture.md) - Client-only components
- [Canon Rule #43: spawn_local Prohibition](./canon-rule-43-spawn-local-prohibition.md) - Manual async spawning
- [Canon Rule #41A: SSR Components Are Read-Only](./canon-rule-41a-ssr-read-only.md) - Derived architectural principle(./canon-rule-43-spawn-local-prohibition.md) - Manual async spawning

## Summary

**Golden Rule:** `Resource` is reactive, not awaitable.

**API:**
- ✅ `.get()` → read current value
- ✅ `Transition` → wait for None → Some
- ❌ `.await` → compile error (not a Future)
- ❌ `Suspend` → wrong tool (for direct Futures)

**Pattern:**
```rust
Resource::new() → .get() → Transition → SSR-safe
```

---

**Enforcement:** Compile-time (Resource not Future) + Linter  
**Severity:** Critical (silent UI freeze)  
**Applies to:** All Leptos components using async data

---

**Last Updated:** 2025-01-02  
**Next Review:** 2025-Q2
