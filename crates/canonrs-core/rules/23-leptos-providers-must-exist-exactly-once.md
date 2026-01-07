# Canon Rule #23: Providers Must Exist Exactly Once

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All Leptos applications using context

---

## 🎯 Objective

Prevent runtime failures and silent bugs caused by duplicate or missing context providers in the component tree.

---

## 📋 The Rule

**A provider for any given type must appear exactly once in the reactive scope where it's consumed.**

Violations:
- ❌ Provider missing → `use_context()` panics
- ❌ Provider duplicated → wrong instance accessed
- ❌ Provider in wrong scope → context not found

---

## 🧠 What "Exactly Once" Means

### Correct: Single Provider
```rust
#[component]
pub fn App() -> impl IntoView {
    let theme = create_signal(Theme::Dark);
    provide_context(theme);  // ✅ Provided once
    
    view! {
        <Layout>
            <Page />
        </Layout>
    }
}

#[component]
fn Page() -> impl IntoView {
    let theme = use_context::<ReadSignal<Theme>>()
        .expect("theme context missing");  // ✅ Found
    
    view! { <div class=theme.get().css_class()></div> }
}
```

### Wrong: Duplicate Provider
```rust
#[component]
pub fn App() -> impl IntoView {
    provide_context(create_signal(Theme::Dark));  // ❌ Provider #1
    
    view! {
        <Layout>
            <ThemeProvider>  // ❌ Provider #2 (duplicate)
                <Page />
            </ThemeProvider>
        </Layout>
    }
}

#[component]
fn ThemeProvider(children: Children) -> impl IntoView {
    provide_context(create_signal(Theme::Light));  // ❌ Shadows parent
    children()
}

#[component]
fn Page() -> impl IntoView {
    let theme = use_context::<ReadSignal<Theme>>().unwrap();
    // ⚠️ Gets Theme::Light, not Theme::Dark
    // Parent theme is shadowed
    view! { <div>{format!("{:?}", theme.get())}</div> }
}
```

**Problem:** Leptos doesn't warn about shadowing. Child gets nearest provider, silently ignoring parent.

---

## 🚨 Common Failure Modes

### Failure 1: Provider in Wrong Component
```rust
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/" view=HomePage />
            </Routes>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    provide_context(create_signal(AppState::default()));  // ❌ Too late
    
    view! { <Dashboard /> }
}

#[component]
fn Dashboard() -> impl IntoView {
    // ✅ Works here
    let state = use_context::<ReadSignal<AppState>>().unwrap();
    view! { /* ... */ }
}

#[component]
fn OtherPage() -> impl IntoView {
    // ❌ PANIC: context not found
    // Provider only exists in HomePage branch
    let state = use_context::<ReadSignal<AppState>>().unwrap();
    view! { /* ... */ }
}
```

**Fix:** Provide at App level, above Router.

### Failure 2: Conditional Provider
```rust
#[component]
pub fn App() -> impl IntoView {
    let show_debug = create_signal(true);
    
    view! {
        <Show
            when=move || show_debug.get()
            fallback=|| view! { <div>"Release mode"</div> }
        >
            {move || {
                provide_context(create_signal(DebugInfo::default()));  // ❌ Conditional
                view! { <DebugPanel /> }
            }}
        </Show>
    }
}
```

**Problem:** Provider only exists when `show_debug` is true. If condition changes, components lose context.

**Fix:** Always provide, conditionally render UI:
```rust
#[component]
pub fn App() -> impl IntoView {
    provide_context(create_signal(DebugInfo::default()));  // ✅ Always present
    let show_debug = create_signal(true);
    
    view! {
        <Show
            when=move || show_debug.get()
            fallback=|| view! { <div>"Release mode"</div> }
        >
            <DebugPanel />
        </Show>
    }
}
```

### Failure 3: Provider Per Route
```rust
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <Routes>
                <Route path="/admin" view=|| view! {
                    {provide_context(create_signal(AdminContext::default()))}  // ❌
                    <AdminPanel />
                } />
                <Route path="/user" view=|| view! {
                    <UserPanel />  // ❌ No AdminContext here
                } />
            </Routes>
        </Router>
    }
}
```

**Problem:** Context scope limited to route. Shared components break.

---

## ✅ Correct Patterns

### Pattern 1: Top-Level Providers
```rust
#[component]
pub fn App() -> impl IntoView {
    // Provide ALL global contexts here
    provide_context(create_rw_signal(AppState::default()));
    provide_context(create_signal(Theme::Dark));
    provide_context(create_signal(UserSession::None));
    
    view! {
        <Router>
            <Routes>
                <Route path="/" view=HomePage />
                <Route path="/admin" view=AdminPage />
            </Routes>
        </Router>
    }
}
```

**Benefits:**
- All routes have access
- Single source of truth
- No duplication possible
- Easy to audit

### Pattern 2: Scoped Providers (Advanced)
```rust
#[component]
pub fn App() -> impl IntoView {
    provide_context(create_signal(GlobalTheme::Dark));
    
    view! {
        <Router>
            <Routes>
                <Route path="/editor" view=|| view! {
                    <EditorProvider>
                        <CodeEditor />
                    </EditorProvider>
                } />
            </Routes>
        </Router>
    }
}

#[component]
fn EditorProvider(children: Children) -> impl IntoView {
    // ✅ Different type, no conflict with GlobalTheme
    provide_context(create_signal(EditorState::default()));
    children()
}
```

**Key:** Use different types for different scopes. Never shadow same type.

### Pattern 3: Optional Context
```rust
#[component]
fn OptionalFeature() -> impl IntoView {
    // ✅ Gracefully handle missing context
    let debug_info = use_context::<ReadSignal<DebugInfo>>();
    
    view! {
        <Show
            when=move || debug_info.is_some()
            fallback=|| view! { <div>"Debug disabled"</div> }
        >
            {move || {
                let info = debug_info.unwrap();
                view! { <div>{format!("{:?}", info.get())}</div> }
            }}
        </Show>
    }
}
```

---

## 🔍 Detection and Debugging

### Symptom: Context Not Found
```
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value'
note: run with `RUST_BACKTRACE=1` for a backtrace
```

**Diagnosis:**
```rust
#[component]
fn DebugContext() -> impl IntoView {
    // Check if context exists
    if use_context::<ReadSignal<AppState>>().is_none() {
        logging::error!("AppState context missing");
    }
    
    view! { <div>"Check console"</div> }
}
```

### Symptom: Wrong Context Instance
```rust
#[component]
pub fn App() -> impl IntoView {
    let count = create_rw_signal(0);
    provide_context(count);
    
    view! {
        <Child1 />
        <Child2 />
    }
}

#[component]
fn Child1() -> impl IntoView {
    let count = use_context::<RwSignal<i32>>().unwrap();
    count.set(42);
    
    view! { <div>"Child1 set count to 42"</div> }
}

#[component]
fn Child2() -> impl IntoView {
    let count = use_context::<RwSignal<i32>>().unwrap();
    
    view! {
        // Should show 42
        <div>"Count: " {count.get()}</div>
    }
}
```

**If Child2 shows wrong value:** Check for duplicate provider shadowing.

---

## 📝 Validation Checklist

- [ ] All `provide_context()` calls are in `App` or stable parent components
- [ ] No `provide_context()` inside `<Show>`, `<For>`, or conditional blocks
- [ ] Each context type is provided exactly once per scope
- [ ] All `use_context()` calls have `.expect()` with meaningful error message
- [ ] No provider shadowing (same type provided twice in hierarchy)

---

## 🚫 Anti-Patterns

### Anti-Pattern 1: Provider Per Component
```rust
// ❌ WRONG: Each component provides its own context
#[component]
fn ComponentA() -> impl IntoView {
    provide_context(create_signal(DataA::default()));
    view! { /* ... */ }
}

#[component]
fn ComponentB() -> impl IntoView {
    provide_context(create_signal(DataB::default()));
    view! { /* ... */ }
}
```

**Problem:** Context proliferation, unclear dependencies.

**Fix:** Provide all at App level, document in one place.

### Anti-Pattern 2: Runtime Provider Creation
```rust
// ❌ WRONG: Provider created on button click
#[component]
fn DynamicProvider() -> impl IntoView {
    let (enabled, set_enabled) = create_signal(false);
    
    view! {
        <button on:click=move |_| {
            set_enabled.set(true);
            provide_context(create_signal(Feature::default()));  // ❌ Too late
        }>
            "Enable Feature"
        </button>
    }
}
```

**Problem:** Components rendered before click can't access context.

**Fix:** Provide immediately, toggle visibility instead.

---

## 🔗 Related Rules

- **Canon Rule #22** (Three Runtimes): Provider behavior differs per runtime
- **Canon Rule #24** (Context Lifetime): When context is accessible
- **Canon Rule #25** (Effect Execution): Context changes trigger effects

---

**Key Insight:** Context is Leptos's dependency injection. Like any DI system, duplicate or missing providers cause cascading failures. Enforce single provider at app root.

---

**Last Updated:** 2025-01-07  
**Version:** 1.0
