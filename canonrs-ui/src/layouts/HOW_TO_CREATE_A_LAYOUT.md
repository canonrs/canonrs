# How to Create a CanonRS Layout

## What is a Layout?

A **Layout** defines the **macro structure** of a page/application. It:

1. **Arranges regions** (header, sidebar, main, footer)
2. **Consumes blocks** (uses Header, Footer, etc.)
3. **Exposes slots** for content injection
4. **Defines data-layout attributes** for CSS targeting

### ✅ Layout Characteristics

- ✅ Structural, not visual
- ✅ Region-based (header/main/footer positions)
- ✅ Block-agnostic (uses generic Header, not "MarketingHeader")
- ✅ Token-driven (no hardcoded dimensions)

### ❌ What Layouts Don't Do

- ❌ Define branding or logos
- ❌ Contain business logic
- ❌ Make visual decisions (colors, fonts)
- ❌ Know about specific products

---

## Official Layout Types

CanonRS defines 7 canonical layouts:

1. **MarketingLayout** - Institutional sites, landing pages
2. **AppShellLayout** - Apps with sidebar navigation
3. **DashboardLayout** - Data-dense dashboards
4. **AuthLayout** - Login/signup flows
5. **FullscreenLayout** - Editors, canvas apps
6. **WizardLayout** - Multi-step flows
7. **SplitViewLayout** - Comparison/settings views

---

## Complete Flow: Token → Layout → CSS

### Step 1: Define Layout Tokens

**Location:** `/opt/docker/monorepo/packages-rust/rs-design/src/tokens/layout.rs`

**Example:** Adding `AppShellLayout` tokens
```rust
// In /opt/docker/monorepo/packages-rust/rs-design/src/tokens/layout.rs

// AppShell Layout tokens
pub const LAYOUT_APP_SIDEBAR_WIDTH: &str = "layout.app.sidebar.width";
pub const LAYOUT_APP_SIDEBAR_WIDTH_COLLAPSED: &str = "layout.app.sidebar.width.collapsed";
pub const LAYOUT_APP_HEADER_HEIGHT: &str = "layout.app.header.height";
pub const LAYOUT_APP_CONTENT_PADDING: &str = "layout.app.content.padding";
pub const LAYOUT_APP_GAP: &str = "layout.app.gap";
```

---

### Step 2: Create Layout Component

**Location:** `/opt/docker/monorepo/packages-rust/rs-design/src/layouts/`

**Example:** Creating `AppShellLayout`
```bash
cat > /opt/docker/monorepo/packages-rust/rs-design/src/layouts/app_shell_layout.rs << 'LAYOUT'
//! # AppShellLayout
//! 
//! **Purpose:** Applications with persistent sidebar navigation
//! 
//! **Structure:**
//! ```
//! Header (full width)
//! ├─ Sidebar (collapsible)
//! └─ Main Content
//! ```
//! 
//! **Token Family:** D (Navigation)
//! 
//! **Blocks consumed:**
//! - Header (with logo, nav, actions)

use leptos::prelude::*;
use crate::blocks::Header;

#[component]
pub fn AppShellLayout(
    /// Header logo slot
    header_logo: Children,
    /// Header actions slot
    header_actions: Children,
    /// Sidebar content (navigation)
    sidebar: Children,
    /// Whether sidebar starts collapsed
    #[prop(default = false)]
    sidebar_collapsed: bool,
    /// Main content
    children: Children,
) -> impl IntoView {
    let collapsed = RwSignal::new(sidebar_collapsed);

    view! {
        <div
            class="layout-app-shell"
            data-layout="app-shell"
            data-sidebar-collapsed=move || collapsed.get()
        >
            <Header
                logo=header_logo
                actions=header_actions
            />

            <div class="layout-app-shell-body">
                <aside
                    class="layout-app-shell-sidebar"
                    data-layout-region="sidebar"
                >
                    {sidebar()}
                </aside>

                <main
                    class="layout-app-shell-main"
                    data-layout-region="main"
                >
                    {children()}
                </main>
            </div>
        </div>
    }
}
LAYOUT
```

**Register in layouts/mod.rs:**
```rust
// In /opt/docker/monorepo/packages-rust/rs-design/src/layouts/mod.rs
pub mod app_shell_layout;
pub use app_shell_layout::AppShellLayout;
```

---

### Step 3: Create Layout CSS

**Location:** `/opt/docker/monorepo/packages-rust/rs-design/style/layouts/`

**Example:** Creating `app_shell.css`
```bash
cat > /opt/docker/monorepo/packages-rust/rs-design/style/layouts/app_shell.css << 'CSS'
/**
 * AppShellLayout Styles
 * 
 * Token Family: D (Navigation)
 * 
 * Consumes tokens:
 * - layout.app.sidebar.width
 * - layout.app.sidebar.width.collapsed
 * - layout.app.header.height
 * - layout.app.content.padding
 * - layout.app.gap
 */

[data-layout="app-shell"] {
  display: flex;
  flex-direction: column;
  min-height: 100vh;
}

.layout-app-shell-body {
  display: flex;
  flex: 1;
  gap: var(--layout-app-gap, 0);
}

.layout-app-shell-sidebar {
  width: var(--layout-app-sidebar-width, 240px);
  flex-shrink: 0;
  transition: width 200ms ease;
}

[data-sidebar-collapsed="true"] .layout-app-shell-sidebar {
  width: var(--layout-app-sidebar-width-collapsed, 60px);
}

.layout-app-shell-main {
  flex: 1;
  padding: var(--layout-app-content-padding, 2rem);
  overflow-x: auto;
}

/* Responsive: collapse sidebar on mobile */
@media (max-width: 768px) {
  .layout-app-shell-sidebar {
    position: fixed;
    left: 0;
    top: var(--layout-app-header-height, 64px);
    bottom: 0;
    z-index: 100;
    transform: translateX(-100%);
    transition: transform 200ms ease;
  }

  [data-sidebar-collapsed="false"] .layout-app-shell-sidebar {
    transform: translateX(0);
  }
}
CSS
```

**Register in layouts.css:**
```css
/* In /opt/docker/monorepo/packages-rust/rs-design/style/layouts.css */
@import './layouts/app_shell.css';
```

---

## File Structure Summary

After creating a layout:
```
/opt/docker/monorepo/packages-rust/rs-design/
├── src/
│   ├── layouts/
│   │   ├── app_shell_layout.rs  ← Layout implementation
│   │   └── mod.rs               ← Register layout here
│   └── tokens/
│       └── layout.rs            ← Add layout tokens here
└── style/
    ├── layouts/
    │   └── app_shell.css        ← Layout CSS
    └── layouts.css              ← Import layout here
```

---

## Usage Example

Using the layout in a product:
```rust
use rs_design::layouts::AppShellLayout;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <AppShellLayout
            header_logo=|| view! { <a href="/">"MyApp"</a> }
            header_actions=|| view! { <ThemeToggle /> }
            sidebar=|| view! {
                <nav>
                    <a href="/dashboard">"Dashboard"</a>
                    <a href="/settings">"Settings"</a>
                </nav>
            }
        >
            <Routes>
                <Route path="/" view=HomePage />
            </Routes>
        </AppShellLayout>
    }
}
```

---

## Layout vs Block

| Aspect | Block | Layout |
|--------|-------|--------|
| **Purpose** | Component structure | Page structure |
| **Scope** | Small (header, card) | Large (entire page) |
| **Consumes** | Tokens | Blocks + tokens |
| **Examples** | Header, Footer, Alert | MarketingLayout, AppShell |
| **Location** | `src/blocks/` | `src/layouts/` |

---

## Checklist

Before considering a layout "complete":

- [ ] Tokens added to `tokens/layout.rs`
- [ ] Layout component created with `data-layout` attribute
- [ ] Layout registered in `layouts/mod.rs`
- [ ] CSS file created consuming ONLY tokens
- [ ] CSS registered in `layouts.css`
- [ ] Layout uses blocks (Header/Footer), not custom components
- [ ] Layout tested in at least one product
- [ ] Documentation includes structure diagram and token list

---

## Canon Rules Applied

This workflow enforces:
- **Rule #96**: SSR requires explicit provider tree (layouts wrap correctly)
- **Separation of concerns**: Layout = structure, Block = component, Token = contract
- **Data-driven styling**: `data-layout` and `data-layout-region` attributes
- **Token consumption**: No hardcoded visual values
