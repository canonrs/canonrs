# Canon Rule #50: Provider Singleton & Runtime Separation

**Status:** Normative  
**Severity:** High  
**Applies to:** All Provider-based reactive systems

---

## The Principle

### Definitions

**Interaction Scope:** A subtree responsible for a continuous unit of interaction (drag operation, editor session, workflow step). Each scope maintains its own reactive runtime.


**Runtime Providers must be unique per interaction scope.**

Providers create reactive runtime (signals, state, lifecycle). Duplicating them creates invisible parallel contexts → ghost bugs.

**Corollary:** Design systems NEVER create Providers. Only the Application layer controls Provider lifecycle.

---

## The Problem

### ❌ Wrong Pattern (Duplicated Provider)
```rust
// App layer
<App>
  <DragDropProvider>  // ✅ Provider 1
    <Dashboard>
      <DragDropProvider>  // 🚫 Provider 2 (DUPLICATE!)
        ...
      </DragDropProvider>
    </Dashboard>
  </DragDropProvider>
</App>
```

**Why this breaks:**
- Two separate `DragContext` instances created
- `DragHandle` uses Context 1
- `DropZone` uses Context 2
- Events emitted to Context 1, callbacks listening on Context 2
- **Result:** Drag & drop silently fails (no errors, just doesn't work)

### ❌ Wrong Pattern (Provider in Design System)
```rust
// rs-design/ui/dashboard/dashboard.rs
#[component]
pub fn Dashboard() -> impl IntoView {
    view! {
        <DragDropProvider>  // 🚫 FORBIDDEN
            <div>...</div>
        </DragDropProvider>
    }
}
```

**Why this breaks:**
- Design system component now controls runtime lifecycle
- Impossible to reuse `Dashboard` without drag & drop
- Violates separation of concerns
- Creates duplicate provider when used in app that already has one

---

## The Solution

### ✅ Correct Pattern (Singleton per Scope)
```rust
// App layer ONLY
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <DragDropProvider>  // ✅ ONE provider at app root
                <DragDropCallbacksProvider>  // ✅ Nested correctly
                    <Dashboard />  // ✅ NO provider here
                    <WorkflowEditor />  // ✅ NO provider here
                </DragDropCallbacksProvider>
            </DragDropProvider>
        </Router>
    }
}
```
```rust
// rs-design: Design system component (NO provider)
#[component]
pub fn Dashboard(
    widgets: Signal<Vec<WidgetDef>>,
    on_drop: Callback<DropEvent>,
) -> impl IntoView {
    // ✅ Consumes context (does NOT create it)
    let drag_ctx = use_drag_context();
    
    view! {
        <div>
            <DragHandle .../>  // ✅ Uses existing context
            <DropZone .../>    // ✅ Uses existing context
        </div>
    }
}
```

---

## Architecture Layers

### Layer Responsibilities

| Layer | Creates Providers? | Consumes Context? | Responsibility |
|-------|-------------------|-------------------|----------------|
| **App (frontend-leptos)** | ✅ YES | ✅ YES | Lifecycle, orchestration |
| **Design System (rs-design)** | ❌ NO | ✅ YES | UI primitives, emit events |
| **Domain/Core** | ❌ NO | ❌ NO | Business logic |

### Canonical Provider Hierarchy
```
App Root
└── DragDropProvider (runtime: creates signals, state)
    └── DragDropCallbacksProvider (interprets events → commands)
        └── LanguageProvider
            └── ThemeProvider
                └── UI Components (consume contexts)
```

**Critical Rule:** Providers that depend on runtime MUST be nested INSIDE the runtime provider.

**Why:** `DragDropCallbacksProvider` interprets events from `DragDropProvider`. If callbacks are outside, they have no runtime to listen to.

---

## Valid Scenarios

### ✅ Multiple Isolated Scopes
```rust
// Valid: Each scope has its own runtime
<App>
  <Route path="/dashboard">
    <DragDropProvider>  // ✅ Scope 1
      <Dashboard />
    </DragDropProvider>
  </Route>
  
  <Route path="/kanban">
    <DragDropProvider>  // ✅ Scope 2 (isolated)
      <KanbanBoard />
    </DragDropProvider>
  </Route>
</App>
```

**Rule:** Multiple providers are valid if they control **different interaction scopes**.

### ✅ Conditional Provider
```rust
// Valid: Feature flag
{move || {
    if feature_flags.drag_enabled.get() {
        view! {
            <DragDropProvider>
                <Dashboard />
            </DragDropProvider>
        }.into_any()
    } else {
        view! {
            <Dashboard />  // ✅ Works without drag
        }.into_any()
    }
}}
```

---

## Forbidden Patterns ❌

### 🚫 Provider in rs-design
```rust
// rs-design/ui/dashboard.rs
pub fn Dashboard() -> impl IntoView {
    view! {
        <DragDropProvider>  // 🚫 FORBIDDEN
            ...
        </DragDropProvider>
    }
}
```

### 🚫 Nested Provider in Same Scope
```rust
<DragDropProvider>
  <Tabs>
    <TabContent>
      <DragDropProvider>  // 🚫 DUPLICATE
        ...
      </DragDropProvider>
    </TabContent>
  </Tabs>
</DragDropProvider>
```

### 🚫 Callbacks Provider Outside Runtime
```rust
// WRONG ORDER
<DragDropCallbacksProvider>  // 🚫 No runtime to listen to!
  <DragDropProvider>
    ...
  </DragDropProvider>
</DragDropCallbacksProvider>
```

### 🚫 Hidden Provider in Reusable Component
```rust
// Some reusable component
pub fn ReusableWidget() -> impl IntoView {
    view! {
        <DragDropProvider>  // 🚫 Hidden coupling
            <div>...</div>
        </DragDropProvider>
    }
}
```

---

## SSR & WASM Rules

### ✅ SSR-Safe Provider
```rust
#[component]
pub fn DragDropProvider(children: Children) -> impl IntoView {
    #[cfg(target_arch = "wasm32")]
    {
        let context = DragContext::new();  // ✅ Only in WASM
        provide_context(context);
    }
    
    children()
}
```

### 🚫 Unsafe: Signal Creation in SSR
```rust
pub fn DragDropProvider() -> impl IntoView {
    // 🚫 Creates RwSignal in SSR → spawn_local panic
    let context = DragContext::new();
    provide_context(context);
}
```

**Rule:** Reactive signals MUST NOT be created during SSR unless guarded.

---

## Real-World Example: Dashboard Editor

### ❌ Before (Broken)
```rust
// App
<App>
  <DragDropProvider>
    <DashboardTab />
  </DragDropProvider>
</App>

// rs-design/dashboard.rs
pub fn Dashboard() -> impl IntoView {
    view! {
        <DragDropProvider>  // 🚫 DUPLICATE
            ...
        </DragDropProvider>
    }
}
```

**Symptom:** Drag events fire, but widgets don't appear. No errors logged.

**Cause:** Two contexts. `DragHandle` emits to Context A, `DropZone` listens on Context B.

### ✅ After (Fixed)
```rust
// App (ONE provider at root)
<App>
  <DragDropProvider>
    <DragDropCallbacksProvider>
      <DashboardTab />
    </DragDropCallbacksProvider>
  </DragDropProvider>
</App>

// rs-design/dashboard.rs (NO provider)
pub fn Dashboard() -> impl IntoView {
    let ctx = use_drag_context();  // ✅ Uses existing
    view! { ... }
}

// frontend-leptos/dashboard_tab.rs
pub fn DashboardTab() -> impl IntoView {
    let callbacks = use_context::<DragDropCallbacks>()
        .expect("DragDropCallbacks required");
    
    let on_drop = Callback::new(|event| { ... });
    callbacks.register_drop(on_drop);  // ✅ Registers to shared context
}
```

**Result:** Single context. All components share same runtime. Drag & drop works.

---

## Benefits

### ✅ Predictable Context Resolution
- One provider → one context
- No ambiguity about which context is active
- Leptos `use_context()` always finds the right one

### ✅ Reusable Components
```rust
// Dashboard works with OR without drag
<Dashboard widgets=.../>  // ✅ No drag (just displays)

<DragDropProvider>
  <Dashboard widgets=.../>  // ✅ With drag (interactive)
</DragDropProvider>
```

### ✅ Testable
```rust
#[test]
fn test_dashboard_no_drag() {
    // ✅ Can test without provider
    let dashboard = Dashboard::new(widgets);
    assert!(dashboard.renders());
}
```

### ✅ SSR-Safe
- Providers only create runtime in WASM
- No `spawn_local` panics
- Server can render static HTML

---

## Debug Checklist

When drag & drop doesn't work:

- [ ] **Check for duplicate providers**
```bash
  grep -r "DragDropProvider" src/
  # Should appear ONCE in app.rs
```

- [ ] **Verify provider order**
```
  Runtime BEFORE Callbacks
  DragDropProvider > DragDropCallbacksProvider
```

- [ ] **Check SSR guards**
```rust
  #[cfg(target_arch = "wasm32")]
```

- [ ] **Verify context consumption**
```rust
  // Should NOT panic
  let ctx = use_drag_context();
```

- [ ] **Check for hidden providers in components**
```bash
  grep -r "provide_context" rs-design/
  # Should be EMPTY
```

---

## Comparison: CanonRS vs Others

| Framework | Provider Pattern | Duplication Risk |
|-----------|------------------|------------------|
| **CanonRS** | Explicit singleton | ✅ Enforced by rule |
| React Context | Per-component | ⚠️ Easy to duplicate |
| Vue Provide/Inject | Hierarchical | ⚠️ Ambiguous scopes |
| Svelte Context | Module-level | ✅ Natural singleton |

**Veredito:** CanonRS is **more explicit** and **less error-prone** than React/Vue.

---

## Related Rules

- **Rule #49:** Drag & Drop as Intent (Event separation)
- **Rule #8:** Overlay Islands (Client-only architecture)
- **Rule #XX:** Separation of Concerns (pending)

---

## Testing Requirements

**E2E Tests:**
- E2E tests MUST mount the same providers as production
- Test environment MUST replicate provider hierarchy
- Mock providers MUST maintain same context structure


## Normative Requirements

**MUST:**
- Runtime providers MUST be unique per interaction scope
- Design system MUST NOT create providers
- Providers MUST be guarded for SSR when creating signals
- Provider order MUST respect dependencies (runtime before callbacks)

**MUST NOT:**
- Duplicate providers in same functional scope
- Hide providers inside reusable components
- Create reactive signals in SSR without guards

**SHOULD:**
- Place providers at app root or route level
- Document provider dependencies explicitly
- Test components with and without providers

---

**Author:** Canon Working Group  
**Date:** 2026-01-04  
**Version:** 1.0  
**Supersedes:** Implicit provider patterns  
**Related:** Canon Rule #49 (Drag & Drop as Intent)
