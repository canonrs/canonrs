# Canon Rule #24: Context Resolution Is Lexical, Not Global

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All Leptos context usage

---

## 🎯 Objective

Prevent "provider exists but context not found" bugs by understanding that Leptos context resolution follows lexical scope (component tree), not global registration.

---

## 📋 The Rule

**Context is resolved by walking UP the component tree from the point of `use_context()`. It does NOT search globally.**

Violations:
- ❌ Assuming "provider exists somewhere" means "accessible here"
- ❌ Provider in parallel branch → sibling can't access
- ❌ Provider below consumer → never found
- ❌ Router/Show/Suspense boundaries not considered

---

## 🧠 How Lexical Resolution Works

### Mental Model: Tree Walking
```rust
App                        // provide_context(Theme)
├── Router
│   ├── Route("/")
│   │   └── HomePage      // ✅ use_context<Theme>() → walks up, finds App
│   └── Route("/admin")
│       └── AdminPage     // ✅ use_context<Theme>() → walks up, finds App
└── Footer                // ✅ use_context<Theme>() → walks up, finds App
```

**Resolution path:**
1. Check current component
2. Check parent component
3. Check parent's parent
4. ... continue up
5. Reach `App` → found or not found

---

## ❌ Anti-Pattern: Sibling Provider
```rust
#[component]
pub fn App() -> impl IntoView {
    view! {
        <LeftPanel />
        <RightPanel />
    }
}

#[component]
fn LeftPanel() -> impl IntoView {
    provide_context(create_signal(SharedState::default()));  // ❌ Only for descendants
    view! { <div>"Left"</div> }
}

#[component]
fn RightPanel() -> impl IntoView {
    // ❌ PANIC: RightPanel is sibling, not descendant
    let state = use_context::<ReadSignal<SharedState>>()
        .expect("state context");
    
    view! { <div>{state.get().value}</div> }
}
```

**Why it fails:**
```
App
├── LeftPanel (provides context)
└── RightPanel (sibling, can't access)
```

**Fix:** Provide in `App`:
```rust
#[component]
pub fn App() -> impl IntoView {
    provide_context(create_signal(SharedState::default()));  // ✅ Parent of both
    
    view! {
        <LeftPanel />
        <RightPanel />
    }
}
```

---

## ❌ Anti-Pattern: Provider Below Consumer
```rust
#[component]
pub fn App() -> impl IntoView {
    view! {
        <Header />
        <AppProvider />
    }
}

#[component]
fn Header() -> impl IntoView {
    // ❌ PANIC: AppProvider is below Header in tree
    let config = use_context::<ReadSignal<Config>>()
        .expect("config");
    
    view! { <div>{config.get().title}</div> }
}

#[component]
fn AppProvider() -> impl IntoView {
    provide_context(create_signal(Config::default()));
    view! { <MainContent /> }
}
```

**Why it fails:**
```
App
├── Header (tries to use context)
└── AppProvider (provides context) ❌ wrong order
```

**Fix:** Provide before consuming:
```rust
#[component]
pub fn App() -> impl IntoView {
    provide_context(create_signal(Config::default()));  // ✅ Before Header
    
    view! {
        <Header />
        <MainContent />
    }
}
```

---

## 🔍 Scope Boundaries

### Router Creates New Scope
```rust
#[component]
pub fn App() -> impl IntoView {
    provide_context(create_signal(GlobalState::default()));
    
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
    // ✅ Router preserves parent context
    let state = use_context::<ReadSignal<GlobalState>>().unwrap();
    view! { <div>{state.get().value}</div> }
}
```

**Router does NOT block context** - it forwards parent scope.

### Show/Suspense Preserve Scope
```rust
#[component]
fn ConditionalContent() -> impl IntoView {
    provide_context(create_signal(Data::default()));
    let show = create_signal(true);
    
    view! {
        <Show
            when=move || show.get()
            fallback=|| view! { <div>"Hidden"</div> }
        >
            // ✅ Can access parent context
            <ChildThatUsesData />
        </Show>
    }
}
```

### For Preserves Scope Per Item
```rust
#[component]
fn List() -> impl IntoView {
    provide_context(create_signal(ListConfig::default()));
    let items = vec![1, 2, 3];
    
    view! {
        <For
            each=move || items.clone()
            key=|i| *i
            children=move |item| view! {
                // ✅ Each iteration can access parent context
                <ItemRow item />
            }
        />
    }
}
```

---

## ✅ Correct Pattern: Top-Down Provision
```rust
#[component]
pub fn App() -> impl IntoView {
    // ✅ All contexts at root
    provide_context(create_rw_signal(AppState::default()));
    provide_context(create_signal(Theme::Dark));
    provide_context(create_signal(UserSession::None));
    
    view! {
        <Router>
            <Routes>
                <Route path="/" view=HomePage />
                <Route path="/admin" view=AdminPage />
                <Route path="/settings" view=SettingsPage />
            </Routes>
        </Router>
        <GlobalFooter />
    }
}

// ✅ All routes and components can access all contexts
```

---

## 🧪 Debugging Context Issues

### Symptom: "Context not found" but provider exists

**Debug checklist:**
```rust
#[component]
fn DebugContext() -> impl IntoView {
    let theme = use_context::<ReadSignal<Theme>>();
    
    match theme {
        Some(t) => {
            logging::log!("✅ Theme found: {:?}", t.get());
        }
        None => {
            logging::error!("❌ Theme context missing");
            logging::error!("Component tree path:");
            // Add temporary wrapper to trace path
        }
    }
    
    view! { <div>"Check console"</div> }
}
```

### Technique: Wrapper Components for Tracing
```rust
#[component]
fn TraceContext(children: Children) -> impl IntoView {
    let theme = use_context::<ReadSignal<Theme>>();
    logging::log!("Context at this level: {:?}", theme.is_some());
    children()
}

// Wrap suspicious components:
view! {
    <TraceContext>
        <ProblemComponent />
    </TraceContext>
}
```

---

## 📝 Validation Checklist

- [ ] All `provide_context()` calls are above their consumers in the tree
- [ ] No providers in sibling branches expecting cross-access
- [ ] No conditional providers (use conditional rendering instead)
- [ ] All route-specific contexts provided at Router level or above
- [ ] Context types documented with "Provided by: X component"

---

## 🔗 Related Rules

- **Canon Rule #22** (Three Runtimes): Context behaves differently per runtime
- **Canon Rule #23** (Providers Exactly Once): One provider per type
- **Canon Rule #25** (Pages vs Blocks): Where to provide context

---

## 🧠 Key Insight

**"Provider exists" ≠ "Context accessible"**

Context is not a global registry. It's a lexical scope chain. The component tree IS the scope chain. If your mental model is "I registered this somewhere," you will hit bugs. The correct model is "I provided this above the consumer in the tree."

---

**Last Updated:** 2025-01-07  
**Version:** 1.0