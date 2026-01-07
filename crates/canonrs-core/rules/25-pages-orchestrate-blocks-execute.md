# Canon Rule #25: Pages Orchestrate, Blocks Execute

**Status:** `ACTIVE`  
**Enforcement:** `MANDATORY`  
**Scope:** All Leptos frontend architecture

---

## 🎯 Objective

Establish clear architectural boundaries between Pages (orchestration layer) and Blocks (execution layer) to prevent tight coupling, context proliferation, and unpredictable component behavior.

---

## 📋 The Rule

**Pages coordinate context, data fetching, and composition. Blocks consume context and execute presentation logic. Never mix responsibilities.**

Responsibilities:

- ✅ Pages: provide context, fetch data, compose layout
- ✅ Blocks: use context, render UI, handle local interactions
- ❌ Blocks: provide context, fetch global data, make assumptions about parents

---

## 🧠 Architecture Layers

### Page Layer (Orchestration)

```
App
└── Router
    └── Routes
        └── HomePage ← PAGE LAYER
            ├── provide_context(user)
            ├── provide_context(theme)
            ├── fetch_dashboard_data()
            └── compose layout:
                ├── <Header />
                ├── <DashboardBlock />
                └── <Footer />
```

**Page responsibilities:**

- Dependency injection (provide_context)
- Data orchestration (fetch, transform, coordinate)
- Layout composition (which blocks, in what order)
- Route-specific logic
- Error boundaries

### Block Layer (Execution)

```
DashboardBlock ← BLOCK LAYER
├── use_context(user) ← consume, never provide
├── use_context(theme)
└── render pure UI based on props + context
```

**Block responsibilities:**

- Consume context (read-only)
- Render UI based on inputs
- Handle local interactions (click, hover)
- Emit events to parent
- Zero assumptions about where it's used

---

## ✅ Correct Pattern: Page Orchestrates

```rust
#[component]
pub fn DashboardPage() -> impl IntoView {
    // ✅ PAGE: Provides all contexts
    provide_context(create_rw_signal(DashboardState::default()));
    provide_context(create_signal(WidgetConfig::default()));

    // ✅ PAGE: Fetches data
    let dashboard_data = create_resource(
        || (),
        |_| async { fetch_dashboard_data().await }
    );

    // ✅ PAGE: Composes layout
    view! {
        <div class="dashboard-layout">
            <Suspense fallback=|| view! { <Spinner /> }>
                {move || dashboard_data.get().map(|data| view! {
                    <DashboardHeader stats=data.stats />
                    <MetricsBlock />
                    <ChartsBlock />
                    <ActivityBlock />
                })}
            </Suspense>
        </div>
    }
}

#[component]
fn MetricsBlock() -> impl IntoView {
    // ✅ BLOCK: Consumes context
    let state = use_context::<RwSignal<DashboardState>>()
        .expect("DashboardState must be provided by page");

    let config = use_context::<ReadSignal<WidgetConfig>>()
        .expect("WidgetConfig must be provided by page");

    // ✅ BLOCK: Pure rendering
    view! {
        <div class="metrics">
            <h2>"Metrics"</h2>
            <For
                each=move || state.get().metrics.clone()
                key=|m| m.id
                children=move |metric| view! {
                    <MetricCard metric config=config.get() />
                }
            />
        </div>
    }
}
```

---

## ❌ Anti-Pattern: Block Provides Context

```rust
#[component]
fn DashboardPage() -> impl IntoView {
    view! {
        <MetricsBlock />  // ❌ Expects block to provide its own context
        <ChartsBlock />
    }
}

#[component]
fn MetricsBlock() -> impl IntoView {
    // ❌ WRONG: Block providing context
    provide_context(create_rw_signal(MetricsState::default()));

    view! {
        <MetricsList />
    }
}

#[component]
fn MetricsList() -> impl IntoView {
    // ✅ Can access, but...
    let state = use_context::<RwSignal<MetricsState>>().unwrap();

    // ❌ Problem: MetricsBlock is now tightly coupled
    // ❌ Problem: Can't reuse MetricsList in different context
    // ❌ Problem: Page lost control of state
    view! { /* ... */ }
}
```

**Why this is wrong:**

- Block became coordinator (not its job)
- State management hidden inside block
- Page can't control or observe block state
- Other components can't access MetricsState
- Testing requires rendering entire block

**Fix:** Move context to page:

```rust
#[component]
fn DashboardPage() -> impl IntoView {
    // ✅ PAGE controls all state
    provide_context(create_rw_signal(MetricsState::default()));

    view! {
        <MetricsBlock />
        <ChartsBlock />
    }
}

#[component]
fn MetricsBlock() -> impl IntoView {
    // ✅ BLOCK just consumes
    let state = use_context::<RwSignal<MetricsState>>().unwrap();
    view! { <MetricsList /> }
}
```

---

## ❌ Anti-Pattern: Block Fetches Global Data

```rust
#[component]
fn UserProfileBlock() -> impl IntoView {
    // ❌ WRONG: Block fetching its own data
    let user = create_resource(
        || (),
        |_| async { fetch_current_user().await }
    );

    view! {
        <Suspense fallback=|| view! { <Spinner /> }>
            {move || user.get().map(|u| view! {
                <div class="profile">
                    <Avatar src=u.avatar />
                    <Name>{u.name}</Name>
                </div>
            })}
        </Suspense>
    }
}
```

**Problems:**

- Multiple blocks fetch same user (waste)
- Page can't coordinate loading states
- Block can't be rendered without network
- Testing requires mocking network
- SSR hydration mismatch risk

**Fix:** Page fetches, block receives:

```rust
#[component]
fn ProfilePage() -> impl IntoView {
    // ✅ PAGE: Fetches once
    let user = create_resource(
        || (),
        |_| async { fetch_current_user().await }
    );

    provide_context(user);

    view! {
        <Suspense fallback=|| view! { <PageSpinner /> }>
            {move || user.get().map(|_| view! {
                <UserProfileBlock />
                <UserActivityBlock />
                <UserSettingsBlock />
            })}
        </Suspense>
    }
}

#[component]
fn UserProfileBlock() -> impl IntoView {
    // ✅ BLOCK: Consumes resource
    let user = use_context::<Resource<(), User>>()
        .expect("User resource must be provided");

    view! {
        <div class="profile">
            <Avatar src=user.get().unwrap().avatar />
            <Name>{user.get().unwrap().name}</Name>
        </div>
    }
}
```

---

## ✅ Correct Pattern: Props for Explicit Dependencies

```rust
#[component]
fn DashboardPage() -> impl IntoView {
    let selected_period = create_rw_signal(Period::LastWeek);

    view! {
        <PeriodSelector selected=selected_period />
        <MetricsBlock period=selected_period.get() />
    }
}

#[component]
fn MetricsBlock(
    #[prop(into)] period: Period,
) -> impl IntoView {
    // ✅ Explicit dependency via prop
    // ✅ Easy to test
    // ✅ Clear contract

    let metrics = create_resource(
        move || period.clone(),
        |p| async move { fetch_metrics(p).await }
    );

    view! {
        <Suspense fallback=|| view! { <Spinner /> }>
            {move || metrics.get().map(|m| view! {
                <MetricsList metrics=m />
            })}
        </Suspense>
    }
}
```

**Benefits:**

- Block contract is visible (props)
- Easy to test with different periods
- No hidden context dependencies
- Reusable in any context

---

## 🔍 Decision Matrix

| Scenario                      | Page | Block |
| ----------------------------- | ---- | ----- |
| Provide app-wide context      | ✅   | ❌    |
| Provide page-specific context | ✅   | ❌    |
| Fetch user session            | ✅   | ❌    |
| Fetch page data               | ✅   | ❌    |
| Consume context               | ✅   | ✅    |
| Render UI                     | ✅   | ✅    |
| Handle button click           | ⚠️   | ✅    |
| Compose layout                | ✅   | ❌    |
| Error boundaries              | ✅   | ❌    |

⚠️ = Delegate to block via callback prop

---

## 🧪 Testing Benefits

### Page Testing (Integration)

```rust
#[test]
fn test_dashboard_page_composition() {
    // Test orchestration, not implementation
    let page = render!(DashboardPage);

    assert!(page.contains("MetricsBlock"));
    assert!(page.contains("ChartsBlock"));
    assert!(page.context_provided::<DashboardState>());
}
```

### Block Testing (Unit)

```rust
#[test]
fn test_metrics_block_rendering() {
    // Test pure logic with mocked context
    let mock_state = create_rw_signal(DashboardState::mock());
    provide_context(mock_state);

    let block = render!(MetricsBlock);
    assert_eq!(block.metric_count(), 5);
}
```

**Key advantage:** Blocks are testable in isolation because they don't create their own dependencies.

---

## 📝 Validation Checklist

- [ ] All `provide_context()` calls are in Page components
- [ ] No Block creates resources for global/page-level data
- [ ] Block props explicitly declare dependencies
- [ ] Blocks can be rendered in Storybook without page context
- [ ] Page tests validate composition, Block tests validate behavior

---

## 🚫 Red Flags

**If you see this in a Block component:**

- `provide_context()` → Move to page
- `create_resource()` for API calls → Move to page
- `Router` or `Route` → Blocks don't route
- Assumptions about sibling blocks → Use page to coordinate

**If you see this in a Page component:**

- Inline complex UI logic → Extract to block
- Direct DOM manipulation → Extract to block
- Heavy computation in render → Extract to block or memo

---

## 🔗 Related Rules

- **Canon Rule #23** (Providers Exactly Once): Pages provide once
- **Canon Rule #24** (Context Is Lexical): Pages provide at root of subtree
- **Canon Rule #26** (Effects per Runtime): Pages coordinate effects

---

## 🧠 Key Insight

**Pages are conductors. Blocks are musicians.**

The conductor (Page) decides:

- What music to play (which blocks)
- What tempo (loading states)
- What key (theme, config)

The musicians (Blocks) execute their part perfectly, given:

- Sheet music (props)
- Conductor's signals (context)

If musicians start conducting, chaos ensues.

---

**Architectural Principle:**  
_"High cohesion within, loose coupling between. Pages couple to blocks via context and props. Blocks never couple to pages."_

---

**Last Updated:** 2025-01-07  
**Version:** 1.0
EOF

echo "✅ Canon Rule #25 criada"

echo -e "\n📊 PROGRESS:"
echo "✅ Rule #22: Three Runtimes"
echo "✅ Rule #23: Providers Exactly Once"
echo "✅ Rule #24: Context Is Lexical"
echo "✅ Rule #25: Pages Orchestrate, Blocks Execute"
echo "⏳ Rule #26: Effects per Runtime (próxima)"
