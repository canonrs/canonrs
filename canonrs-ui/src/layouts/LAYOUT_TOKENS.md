# Layout Tokens Reference

## Purpose

Layout tokens define **page structure** and **macro organization**. They control:
- Maximum widths and containers
- Spacing between major regions
- Header/footer positioning
- Responsive breakpoints
- Grid systems for page-level organization

## What Layouts DO NOT Control

❌ Component appearance (colors, typography, borders)
❌ Interactive states (hover, focus, disabled)
❌ Animation/motion
❌ Form validation styling
❌ Button or input styling

> **Rule:** If it's about a component's internal look, it's a Block token, not a Layout token.

---

## Token Families Used by Layouts

**Primary:** D (Navigation & Structure)
**Secondary:** Core (spacing)

**Never used:** Color, State, Motion, Typography (those are Block-level)

---

## 1. Layout — Global Structure

Controls page-level container and spacing:
```rust
// Location: /opt/docker/monorepo/packages-rust/rs-design/src/tokens/layout.rs

pub const LAYOUT_MAX_WIDTH: &str = "layout.max-width";
pub const LAYOUT_PADDING_X: &str = "layout.padding.x";
pub const LAYOUT_PADDING_Y: &str = "layout.padding.y";
pub const LAYOUT_GAP: &str = "layout.gap";
pub const LAYOUT_SECTION_GAP: &str = "layout.section.gap";
pub const LAYOUT_STACK_ORDER: &str = "layout.stack.order";
```

**Usage:**
```css
[data-layout] {
  max-width: var(--layout-max-width);
  padding: var(--layout-padding-y) var(--layout-padding-x);
  gap: var(--layout-gap);
}
```

---

## 2. Header/Footer as Layout Slots

Layouts decide **where** header/footer go, not **what** they look like:
```rust
pub const LAYOUT_HEADER_HEIGHT: &str = "layout.header.height";
pub const LAYOUT_HEADER_OFFSET: &str = "layout.header.offset";
pub const LAYOUT_HEADER_STICKY: &str = "layout.header.sticky";
pub const LAYOUT_FOOTER_HEIGHT: &str = "layout.footer.height";
pub const LAYOUT_FOOTER_OFFSET: &str = "layout.footer.offset";
```

**Usage:**
```css
[data-layout-region="header"] {
  height: var(--layout-header-height);
  top: var(--layout-header-offset);
}
```

---

## 3. Marketing Layout — Hero Section

Specific to `MarketingLayout`:
```rust
pub const LAYOUT_MARKETING_HERO_HEIGHT: &str = "layout.marketing.hero.height";
pub const LAYOUT_MARKETING_HERO_MIN_HEIGHT: &str = "layout.marketing.hero.min-height";
pub const LAYOUT_MARKETING_HERO_ALIGN: &str = "layout.marketing.hero.align";
pub const LAYOUT_MARKETING_HERO_PADDING: &str = "layout.marketing.hero.padding";
```

**Usage:**
```css
.layout-marketing-hero {
  min-height: var(--layout-marketing-hero-height);
  padding: var(--layout-marketing-hero-padding);
}
```

---

## 4. Content Area

Controls main content region:
```rust
pub const LAYOUT_CONTENT_MAX_WIDTH: &str = "layout.content.max-width";
pub const LAYOUT_CONTENT_PADDING: &str = "layout.content.padding";
pub const LAYOUT_CONTENT_GAP: &str = "layout.content.gap";
pub const LAYOUT_CONTENT_ALIGN: &str = "layout.content.align";
```

**Usage:**
```css
[data-layout-slot="content"] {
  max-width: var(--layout-content-max-width);
  padding: var(--layout-content-padding);
  gap: var(--layout-content-gap);
}
```

---

## 5. Grid System (Macro)

Page-level grid, not component grid:
```rust
pub const LAYOUT_GRID_COLUMNS: &str = "layout.grid.columns";
pub const LAYOUT_GRID_GAP: &str = "layout.grid.gap";
pub const LAYOUT_GRID_BREAKPOINT: &str = "layout.grid.breakpoint";
pub const LAYOUT_FLOW_DIRECTION: &str = "layout.flow.direction";
```

**Usage:**
```css
.layout-grid {
  display: grid;
  grid-template-columns: repeat(var(--layout-grid-columns), 1fr);
  gap: var(--layout-grid-gap);
}
```

---

## 6. AppShell Layout (Sidebar)

Specific to `AppShellLayout`:
```rust
pub const LAYOUT_APP_SIDEBAR_WIDTH: &str = "layout.app.sidebar.width";
pub const LAYOUT_APP_SIDEBAR_WIDTH_COLLAPSED: &str = "layout.app.sidebar.width.collapsed";
pub const LAYOUT_APP_HEADER_HEIGHT: &str = "layout.app.header.height";
pub const LAYOUT_APP_CONTENT_PADDING: &str = "layout.app.content.padding";
pub const LAYOUT_APP_GAP: &str = "layout.app.gap";
```

**Usage:**
```css
[data-layout="app-shell"] .layout-sidebar {
  width: var(--layout-app-sidebar-width);
}

[data-sidebar-collapsed="true"] .layout-sidebar {
  width: var(--layout-app-sidebar-width-collapsed);
}
```

---

## 7. Dashboard Layout

Specific to `DashboardLayout`:
```rust
pub const LAYOUT_DASHBOARD_GRID_GAP: &str = "layout.dashboard.grid.gap";
pub const LAYOUT_DASHBOARD_PANEL_RADIUS: &str = "layout.dashboard.panel.radius";
pub const LAYOUT_DASHBOARD_HEADER_HEIGHT: &str = "layout.dashboard.header.height";
```

---

## 8. Auth Layout

Specific to `AuthLayout`:
```rust
pub const LAYOUT_AUTH_SPLIT_RATIO: &str = "layout.auth.split_ratio";
pub const LAYOUT_AUTH_FORM_WIDTH: &str = "layout.auth.form.width";
pub const LAYOUT_AUTH_BRAND_BG: &str = "layout.auth.brand.bg";
```

---

## Complete Token List by Layout

### MarketingLayout
```
layout.marketing.max-width
layout.marketing.hero.height
layout.marketing.section.gap
layout.marketing.content.padding
```

### AppShellLayout
```
layout.app.sidebar.width
layout.app.sidebar.width.collapsed
layout.app.header.height
layout.app.content.padding
layout.app.gap
```

### DashboardLayout
```
layout.dashboard.grid.gap
layout.dashboard.panel.radius
layout.dashboard.density
```

### AuthLayout
```
layout.auth.split_ratio
layout.auth.form.width
layout.auth.brand.bg
```

---

## Absolute File Locations

**Token definitions:**
```
/opt/docker/monorepo/packages-rust/rs-design/src/tokens/layout.rs
```

**CSS consumption:**
```
/opt/docker/monorepo/packages-rust/rs-design/style/layouts/marketing.css
/opt/docker/monorepo/packages-rust/rs-design/style/layouts/app_shell.css
/opt/docker/monorepo/packages-rust/rs-design/style/layouts/dashboard.css
```

**Component usage:**
```
/opt/docker/monorepo/packages-rust/rs-design/src/layouts/marketing_layout.rs
/opt/docker/monorepo/packages-rust/rs-design/src/layouts/app_shell_layout.rs
```

---

## Quick Reference Table

| Scope | Tokens | Never Uses |
|-------|--------|------------|
| **Layout** | layout.*, spacing.* | color.*, state.*, motion.*, validation.* |
| **Block** | All families | layout.grid.*, layout.max-width |
| **Primitive** | ❌ None | All tokens |

---

## Canon Rules Applied

- **Separation of concerns:** Layout = structure, not appearance
- **Token scoping:** Layout tokens for layout, block tokens for blocks
- **No visual decisions:** Layouts consume spacing/structure tokens only
