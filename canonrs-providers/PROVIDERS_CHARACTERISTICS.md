# CanonRS Providers - Characteristics & Guidelines

## What is a Provider?

A **Provider** is a component that:
- Manages **global application state**
- Provides **context** to the component tree
- Handles **side effects** (DOM manipulation, storage, etc.)
- **Bifurcates logic** between SSR and hydrate
- Lives **above the UI layer**, not within it

Providers are **runtime code**, not UI code.

---

## ✅ Canonical Criteria for a Provider

A code belongs in `canonrs-providers` if it meets **ANY** of these criteria:

### 1. Uses `provide_context`
```rust
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let theme = create_rw_signal(Theme::Dark);
    provide_context(ThemeContext { theme }); // ✅ Provider
    children()
}
```

### 2. Touches browser APIs
- `window`, `document`, `html`, `body`
- `classList`, `dataset`, `localStorage`, `sessionStorage`
```rust
#[cfg(feature = "hydrate")]
{
    let html = document().document_element().unwrap();
    html.class_list().add_1("dark"); // ✅ Provider behavior
}
```

### 3. Bifurcates SSR/Hydrate logic
```rust
#[cfg(not(feature = "hydrate"))]
{
    // SSR path
}

#[cfg(feature = "hydrate")]
{
    // Hydrate path with DOM access
}
```

### 4. Represents global application state
- Theme mode (Dark/Light)
- Language/Locale
- Authentication status
- Feature flags
- Environment variables

---

## ✅ Examples of Providers

### `ThemeProvider`
- **Why**: Provides global theme context
- **SSR/Hydrate**: Applies CSS classes only in hydrate
- **Global**: Theme affects entire application

### `CanonRSRoot`
- **Why**: Orchestrates multiple providers
- **Bootstrap**: Sets up application context
- **Global**: Root-level provider composition

### `LanguageProvider`
- **Why**: Sets `<html lang="...">` attribute
- **DOM**: Touches HTML element
- **Global**: Language affects entire app

### `DensityProvider`
- **Why**: Provides global density context (Compact/Normal/Comfortable)
- **Global**: UI density affects all components

### `HydrationBootstrap` (future)
- **Why**: Manages hydration lifecycle
- **SSR/Hydrate**: Detects and handles hydration state
- **Side effects**: Initializes client-side runtime

### `ClientEnvProvider` (future)
- **Why**: Exposes client environment variables
- **Global**: Environment config affects app behavior

---

## ❌ What is NOT a Provider

### UI Components
```rust
// ❌ NOT a provider - this is UI
#[component]
pub fn Button(children: Children) -> impl IntoView {
    view! { <button>{children()}</button> }
}
```
**Belongs in**: `canonrs-ui`

### Interactive Wrappers
```rust
// ❌ NOT a provider - this is interactive UI
#[component]
pub fn ButtonInteractive(
    on_click: Callback<()>,
    children: Children,
) -> impl IntoView {
    view! {
        <Button on:click=move |_| on_click.run(())>
            {children()}
        </Button>
    }
}
```
**Belongs in**: `canonrs-ui-interactive`

### Page Controllers
```rust
// ❌ NOT a provider - this is page logic
#[component]
pub fn DashboardPage() -> impl IntoView {
    let data = create_resource(|| (), |_| fetch_dashboard_data());
    view! { <div>{/* ... */}</div> }
}
```
**Belongs in**: Application code

### Layout Components
```rust
// ❌ NOT a provider - this is visual layout
#[component]
pub fn PageLayout(children: Children) -> impl IntoView {
    view! {
        <div class="page-layout">
            <Header />
            <main>{children()}</main>
            <Footer />
        </div>
    }
}
```
**Belongs in**: `canonrs-ui/layouts`

### Local State Management
```rust
// ❌ NOT a provider - this is local state
#[component]
pub fn Counter() -> impl IntoView {
    let count = create_rw_signal(0); // Local, not global
    view! {
        <button on:click=move |_| count.update(|n| *n += 1)>
            {count}
        </button>
    }
}
```
**Belongs in**: Component itself

---

## 🧩 Provider Architecture Rules

### Rule 1: Providers Live Above UI
```
Application
 ├── CanonRSRoot (Provider)
 │   ├── ThemeProvider (Provider)
 │   ├── LanguageProvider (Provider)
 │   └── Router
 │       └── Pages
 │           └── UI Components ← Never provide context
```

### Rule 2: No UI in Providers
Providers return `children()`, never complex UI:
```rust
// ✅ CORRECT
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    provide_context(/* ... */);
    children()
}

// ❌ WRONG - Provider rendering UI
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    view! {
        <div class="theme-wrapper">
            <button>"Toggle"</button>
            {children()}
        </div>
    }
}
```

### Rule 3: SSR-Safe by Default
All providers must work in SSR without panicking:
```rust
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    // ✅ Works in SSR
    let mode = create_rw_signal(ThemeMode::Dark);
    provide_context(ThemeContext { mode });
    
    // ✅ DOM access guarded
    #[cfg(feature = "hydrate")]
    {
        Effect::new(move |_| {
            // DOM manipulation
        });
    }
    
    children()
}
```

### Rule 4: One Concern per Provider
Don't create monolithic providers:
```rust
// ❌ WRONG - too many concerns
pub struct AppContext {
    theme: RwSignal<Theme>,
    language: RwSignal<Language>,
    user: RwSignal<Option<User>>,
    router: RwSignal<Router>,
}

// ✅ CORRECT - separate providers
ThemeProvider    → ThemeContext
LanguageProvider → LanguageContext
AuthProvider     → AuthContext
RouterProvider   → RouterContext
```

---

## 📁 Directory Structure
```
canonrs-providers/
├── src/
│   ├── theme/
│   │   ├── mod.rs
│   │   ├── theme_provider.rs
│   │   └── theme_types.rs
│   ├── root/
│   │   ├── mod.rs
│   │   └── canonrs_root.rs
│   ├── language/
│   │   ├── mod.rs
│   │   └── language_provider.rs
│   ├── density/
│   │   ├── mod.rs
│   │   ├── density_provider.rs
│   │   └── density_types.rs
│   ├── hydration/
│   │   └── mod.rs
│   ├── prelude.rs
│   └── lib.rs
└── PROVIDERS_CHARACTERISTICS.md
```

---

## 🎯 Decision Tree
```
Does it use provide_context?
├─ YES → Provider
└─ NO
    ├─ Does it touch window/document?
    │   ├─ YES → Provider
    │   └─ NO
    │       ├─ Does it need SSR/Hydrate bifurcation?
    │       │   ├─ YES → Provider
    │       │   └─ NO
    │       │       ├─ Is it global app state?
    │       │       │   ├─ YES → Provider
    │       │       │   └─ NO → NOT a Provider
```

---

## Summary

**Providers are for**:
- Global context
- Side effects
- Runtime orchestration
- SSR/Hydrate coordination

**Providers are NOT for**:
- UI rendering
- Local component state
- Visual layouts
- Page logic

When in doubt: **If it provides context or touches global state/DOM, it's a provider. Everything else is not.**
