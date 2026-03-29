# Canon Rule #128: Layout Shell and Zone Contracts

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** layout
**Version:** 1.0.0
**Date:** 2025-01-22

---

## Principle

**Layouts define structural boundaries and positioning without business logic. Every Layout MUST be classified as Shell or Zone.**

---

## The Problem

Without formal layout types, developers create layouts that:

1. Contain business logic (authentication, data fetching)
2. Manage domain state (user preferences, cart data)
3. Render domain components directly (forms, dashboards)
4. Mix structural concerns with feature concerns
5. Create untestable, tightly-coupled page structures

**Real symptoms:**
- Layouts fetching user data
- Layouts with conditional rendering based on auth
- Layouts managing sidebar state globally
- Layouts that can't be reused across routes
- Page structures that break when refactored

**Without this rule:**
- Layout becomes dumping ground for "page-level stuff"
- Structural changes break feature code
- Testing requires full app context
- Reusability impossible

---

## Layout Types

### Type 1: ShellLayout

**Definition:** Application-level structure defining top-level regions (header, sidebar, main, footer).

**Use when:** Defining app-wide structure that persists across routes.

**Examples:** `AppShell`, `DashboardShell`, `AuthShell`, `MarketingShell`, `AdminShell`

#### CAN:
- Define top-level structure: `<header>`, `<aside>`, `<main>`, `<footer>`
- Manage global z-index stacking (modals above content, tooltips above modals)
- Control responsive layout breakpoints (sidebar collapse at mobile)
- Accept `ChildrenFn` slots for each region
- Use `<Outlet />` for router content in main area
- Provide layout-level context (sidebar collapsed state)
- Define CSS Grid or Flexbox for shell structure
- Accept layout configuration props (sidebar position: left/right)

#### CANNOT:

**ZoneLayouts are pure structural functions and MUST be signal-free. Any state implies misclassification as Shell or UI component.**
- Have business logic (validation, calculations, domain rules)
- Fetch data or call APIs
- Render domain components directly (forms, dashboards) — use slots/outlets
- Know about authentication or authorization state
- Manage user session or preferences
- Emit domain callbacks (`on_save`, `on_submit`)
- Know about routing paths ("/dashboard", "/settings")

#### Canonical Example:
```rust
// ✅ CORRECT: ShellLayout
#[component]
pub fn AppShell(
    #[prop(optional)] sidebar_content: Option<ChildrenFn>,
    #[prop(default = false)] sidebar_collapsed: bool,
) -> impl IntoView {
    let (collapsed, set_collapsed) = signal(sidebar_collapsed);
    
    // Provide layout context for children
    provide_context(LayoutContext {
        sidebar_collapsed: collapsed.read_only(),
        toggle_sidebar: Callback::new(move |_| {
            set_collapsed.update(|c| *c = !*c);
        }),
    });
    
    view! {
        <div data-shell="" data-sidebar-collapsed={collapsed.get()}>
            <header data-shell-header="">
                <slot name="header" />
            </header>
            
            {sidebar_content.map(|content| view! {
                <aside data-shell-sidebar="" data-collapsed={collapsed.get()}>
                    {content()}
                </aside>
            })}
            
            <main data-shell-main="">
                <Outlet />  // ✅ Router handles content
            </main>
            
            <footer data-shell-footer="">
                <slot name="footer" />
            </footer>
        </div>
    }
}
```

#### Forbidden Example 1: Business logic in Shell
```rust
// ❌ WRONG: Authentication logic in Shell
#[component]
pub fn AppShell() -> impl IntoView {
    let user = use_context::<User>().unwrap();  // ❌ Domain state
    
    let logout = move |_| {  // ❌ Business logic
        api::logout().await;
        navigate("/login");
    };
    
    view! {
        <div data-shell="">
            <header>
                <span>"Welcome, "{user.name}</span>  // ❌
                <Button on_click=logout>"Logout"</Button>
            </header>
            <main><Outlet /></main>
        </div>
    }
}
```

#### Forbidden Example 2: Direct component rendering
```rust
// ❌ WRONG: Rendering domain components in Shell
#[component]
pub fn AppShell() -> impl IntoView {
    view! {
        <div data-shell="">
            <header>
                <UserMenu />  // ❌ Domain component in shell
            </header>
            <aside>
                <NavigationMenu />  // ❌ Should be passed as slot
            </aside>
            <main><Outlet /></main>
        </div>
    }
}
```

---

### Type 2: ZoneLayout

**Definition:** Page-level layout defining content zones without knowing what fills them.

**Use when:** Structuring page content areas (two-column, grid, split-view).

**Examples:** `TwoColumnLayout`, `SidebarLayout`, `GridLayout`, `SplitLayout`, `MasterDetailLayout`

**ZoneLayouts are pure structural functions and MUST be signal-free. Any state implies misclassification as Shell or UI component.**

#### CAN:
- Define CSS Grid or Flexbox structure for page zones
- Accept multiple `ChildrenFn` for different zones (left, right, main, aside)
- Provide responsive breakpoints (stack on mobile, side-by-side on desktop)
- Define zone-level spacing and gaps
- Use data attributes for zone identification (`data-zone="sidebar"`)
- Accept layout configuration (ratios, alignments)

#### CANNOT:

**ZoneLayouts are pure structural functions and MUST be signal-free. Any state implies misclassification as Shell or UI component.**
- Know what components are in zones (content-agnostic)
- Have state management (no `RwSignal`)
- Fetch data or call APIs
- Emit callbacks or events
- Manage z-index or stacking context (that's Shell's job)
- Render specific components (pass everything as children)

#### Canonical Example:
```rust
// ✅ CORRECT: ZoneLayout
#[component]
pub fn TwoColumnLayout(
    left: ChildrenFn,
    right: ChildrenFn,
    #[prop(default = "1fr 2fr")] columns: &'static str,
) -> impl IntoView {
    view! {
        <div
            data-layout="two-column"
            style:grid-template-columns={columns}
        >
            <div data-zone="left">
                {left()}
            </div>
            <div data-zone="right">
                {right()}
            </div>
        </div>
    }
}
```

#### Forbidden Example 1: State in ZoneLayout
```rust
// ❌ WRONG: State management in Zone
#[component]
pub fn TwoColumnLayout(left: ChildrenFn, right: ChildrenFn) -> impl IntoView {
    let (expanded, set_expanded) = signal(false);  // ❌ State
    
    view! {
        <div data-layout="two-column">
            <div data-zone="left" data-expanded={expanded.get()}>
                {left()}
                <Button on_click=move |_| set_expanded.update(|e| *e = !*e)>  // ❌
                    "Toggle"
                </Button>
            </div>
            <div data-zone="right">{right()}</div>
        </div>
    }
}
```

#### Forbidden Example 2: Knowing content
```rust
// ❌ WRONG: Layout knows about content
#[component]
pub fn DashboardLayout() -> impl IntoView {
    view! {
        <div data-layout="dashboard">
            <div data-zone="sidebar">
                <DashboardNav />  // ❌ Should be passed as ChildrenFn
            </div>
            <div data-zone="main">
                <DashboardContent />  // ❌
            </div>
        </div>
    }
}
```

---

## Universal Layout Contracts

**ALL Layouts MUST follow these contracts:**

### 1. Structure Only
**Define regions and positioning — nothing else.**
```rust
// ✅ CORRECT: Pure structure
<div data-shell="">
    <header data-shell-header="">
        {header_content()}
    </header>
    <main data-shell-main="">
        <Outlet />
    </main>
</div>

// ❌ WRONG: Mixed with logic
<div data-shell="">
    <header>
        {if user.is_admin() { /* admin header */ }}  // ❌
    </header>
</div>
```

### 2. Content Agnostic
**Accept content via slots — never render specific components.**
```rust
// ✅ CORRECT: Generic slots
#[component]
pub fn Layout(
    header: ChildrenFn,
    main: ChildrenFn,
) -> impl IntoView { /* ... */ }

// ❌ WRONG: Specific components
#[component]
pub fn Layout() -> impl IntoView {
    view! {
        <header><NavBar /></header>  // ❌ Knows about NavBar
        <main><Dashboard /></main>   // ❌ Knows about Dashboard
    }
}
```

### 3. No Business Logic
**Zero domain logic, validation, or API calls.**
```rust
// ✅ CORRECT: Pure layout
provide_context(LayoutContext {
    sidebar_collapsed: collapsed.read_only(),
});

// ❌ WRONG: Business logic
let user = fetch_user().await;  // ❌
if user.is_authenticated() { /* ... */ }  // ❌
```

### 4. No Routing Knowledge
**Use `<Outlet />` for router content — don't know about paths.**
```rust
// ✅ CORRECT: Generic outlet
<main><Outlet /></main>

// ❌ WRONG: Path knowledge
<main>
    {if location == "/dashboard" { /* ... */ }}  // ❌
</main>
```

### 5. Responsive Structure
**Handle responsive layout changes structurally, not logically.**
```rust
// ✅ CORRECT: CSS-based responsive
<div data-shell="" class="shell-responsive">
    // CSS handles breakpoints
</div>

// ❌ WRONG: Logic-based responsive
{if is_mobile.get() {
    view! { <MobileLayout /> }
} else {
    view! { <DesktopLayout /> }
}}
```

---

## Rationale

### Why Layout Layer Exists

1. **Separation of structure and content** — layout changes don't break features
2. **Reusability** — same layout across multiple routes/pages
3. **Testing simplicity** — layouts testable without domain context
4. **Responsive design** — centralized breakpoint management
5. **Z-index management** — global stacking context control

### What This Rule Protects

- **Structural stability** — layout changes isolated from business logic
- **Feature independence** — features work in any layout
- **Maintainability** — clear responsibility boundaries
- **Testability** — layouts testable in isolation

---

## Enforcement

### Static Analysis
```rust
// Check 1: No business logic
if layout.contains("api::") || layout.contains("fetch_") {
    emit_error!("Canon Rule #128: Layouts cannot call APIs");
}

// Check 2: No domain state
if layout.contains("use_context::<User>") || layout.contains("use_context::<Cart>") {
    emit_error!("Canon Rule #128: Layouts cannot use domain context");
}

// Check 3: No specific component imports
for import in layout.imports {
    if import.path.contains("components/") {
        emit_error!("Canon Rule #128: Layouts cannot import domain components");
    }
}
```

### CI Checks
```bash
# No API calls in layouts
rg 'api::|fetch_|async fn' packages-rust/rs-design/src/layouts/ && exit 1

# No domain context in layouts
rg 'use_context::<User>|use_context::<Cart>' packages-rust/rs-design/src/layouts/ && exit 1

# No routing paths in layouts
rg '"/dashboard"|"/settings"|"/profile"' packages-rust/rs-design/src/layouts/ && exit 1
```

---

## Exceptions

### Exception 1: Layout-Specific Context (Allowed)
```rust
// ✅ ALLOWED: Layout state context
provide_context(LayoutContext {
    sidebar_collapsed: collapsed.read_only(),
    toggle_sidebar: callback,
});
```

**When allowed:**
- Context is purely structural (collapsed, expanded, visible)
- Context does not contain domain data
- Context controls layout behavior only

### Exception 2: Router Outlet (Required)
```rust
// ✅ REQUIRED: Router integration
<main data-shell-main="">
    <Outlet />  // Router outlet is mandatory in Shells
</main>
```

**No other exceptions exist.**

---

## Quick Reference

| Type | Structure | Content | State | Routing | Example |
|------|-----------|---------|-------|---------|---------|
| **Shell** | App-level | Slots + `<Outlet />` | ✅ Layout only | `<Outlet />` | `AppShell`, `DashboardShell` |
| **Zone** | Page-level | `ChildrenFn` slots | ❌ None | ❌ None | `TwoColumnLayout`, `GridLayout` |

---

## Common Patterns

### Pattern 1: Shell with Optional Regions
```rust
#[component]
pub fn AppShell(
    #[prop(optional)] header: Option<ChildrenFn>,
    #[prop(optional)] sidebar: Option<ChildrenFn>,
    #[prop(optional)] footer: Option<ChildrenFn>,
) -> impl IntoView {
    view! {
        <div data-shell="">
            {header.map(|h| view! { <header data-shell-header="">{h()}</header> })}
            {sidebar.map(|s| view! { <aside data-shell-sidebar="">{s()}</aside> })}
            <main data-shell-main=""><Outlet /></main>
            {footer.map(|f| view! { <footer data-shell-footer="">{f()}</footer> })}
        </div>
    }
}
```

### Pattern 2: Responsive Zone
```rust
#[component]
pub fn ResponsiveLayout(
    sidebar: ChildrenFn,
    main: ChildrenFn,
) -> impl IntoView {
    view! {
        <div
            data-layout="responsive"
            class="layout-responsive"  // CSS handles breakpoints
        >
            <div data-zone="sidebar">{sidebar()}</div>
            <div data-zone="main">{main()}</div>
        </div>
    }
}
```

---

## Related Rules

- **Canon Rule #124:** Primitive Contract Types
- **Canon Rule #125:** UI Component Contracts
- **Canon Rule #126:** Component Domain Contracts
- **Canon Rule #127:** Block Composition Contracts (previous rule)

---

## Version History

- **1.0.0** — Initial canonical version (2025-01-22)
