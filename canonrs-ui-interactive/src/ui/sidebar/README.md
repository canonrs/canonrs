---
component: Sidebar
layer: UI
status: Stable
since: v0.8
last_review: 2026-01-18
ownership: canonrs
keywords:
  - sidebar
  - navigation
  - layout
  - design system
  - leptos
  - ssr
  - hydration
path_primitive: /opt/docker/monorepo/packages-rust/rs-design/src/primitives/sidebar.rs
path_ui: /opt/docker/monorepo/packages-rust/rs-design/src/ui/sidebar/sidebar.rs
---

# Sidebar Component

## 1. Conceptual Introduction

The Sidebar component provides a structured navigation and content organization interface for application layouts. It exists in the CanonRS UI layer as an ergonomic composition of Sidebar primitives, offering sensible defaults for common sidebar use cases including navigation menus, collapsible panels, and content organization.

Sidebar solves the architectural challenge of creating accessible, SSR-safe navigation interfaces without coupling to browser APIs, viewport-specific logic, or imperative DOM manipulation. It lives in the **UI layer**, sitting between application Layouts and the underlying Primitives.

**What Sidebar does NOT do:**
- Does not execute browser APIs (resize observers, media queries, etc.)
- Does not apply animations or transitions
- Does not manage routing or active states
- Does not enforce specific layout modes (off-canvas, floating, etc.)
- Does not handle mobile/desktop breakpoints

---

## 2. Architectural Responsibility (Contract)

### Responsibility

- Composes Sidebar primitives into coherent navigation structure
- Provides semantic slots (Content, Group, Menu, MenuItem, MenuButton)
- Exposes open/closed state contract via RwSignal
- Defines intent (collapsible navigation panel)
- Establishes data-attribute surface for runtime and CSS
- Provides ergonomic MenuButton with hover/active states

### Non-Responsibility

- Does NOT execute responsive behavior (runtime layer handles this)
- Does NOT bind viewport observers (runtime layer handles this)
- Does NOT create animations (CSS layer handles this)
- Does NOT enforce off-canvas/overlay modes (CSS + runtime handle this)
- Does NOT mutate document structure imperatively
- Does NOT choose semantic tags (`<nav>`, `<aside>`) — app layer decides

This separation ensures SSR safety and hydration correctness.

---

## 3. Position in CanonRS Ecosystem
```text
Application (AppLayout)
    ↓
Layout (SidebarLayout)
    ↓
UI (Sidebar) ← YOU ARE HERE
    ↓
Primitive (SidebarPrimitive, SidebarTriggerPrimitive, etc.)
    ↓
data-* attributes
    ↓
Shell Runtime JS (collapse triggers, overlay dismiss, etc.)
    ↓
Browser API (focus management, resize observers, etc.)
```

**SSR Flow:**
- Sidebar renders complete HTML structure during SSR
- Primitives emit data-* attributes
- Runtime JS hydrates and attaches behavior
- CSS controls visibility via `[hidden]` attribute

**Hydration:**
- Structure must be identical between SSR and client
- State changes only affect attributes, never structure
- No conditional rendering of Sidebar root

---

## 4. Tokens Applied

### Layout
- `--sidebar-width` — Default sidebar width (Família D)
- `--space-xs` — Minimal spacing
- `--space-sm` — Group label padding
- `--space-md` — Content padding, group gaps
- `--space-lg` — (reserved for future use)
- `--radius-sm` — Menu button border radius

### Typography
- `--font-family-sans` — All text content
- `--font-size-sm` — Group labels
- `--font-size-md` — Menu items
- `--font-weight-regular` — Default menu items
- `--font-weight-medium` — Active menu items
- `--font-weight-semibold` — Group labels

### Color
- `--color-bg-surface` — Sidebar background
- `--color-bg-muted` — Hover/active state backgrounds
- `--color-fg-default` — Primary text, menu items
- `--color-fg-muted` — Group labels, secondary text
- `--color-border-muted` — Sidebar border

### Border
- `--border-width-hairline` — Sidebar edge border

### State
- `--state-hover-opacity` — Menu button hover intensity
- `--state-active-opacity` — Active menu item intensity

### Navigation (Família D)
- `--nav-item-height` — Menu button minimum height
- `--nav-item-padding` — Menu button padding (with fallback)

All tokens follow CanonRS token governance. No hardcoded values exist in CSS.

---

## 5. Technical Structure

### SSR HTML Output
```html
<div data-sidebar="">
  <div data-sidebar-content="">
    <div data-sidebar-group="">
      <div data-sidebar-group-label="">Navigation</div>
      <div data-sidebar-menu="">
        <div data-sidebar-menu-item="">
          <button data-sidebar-menu-button="" data-active="false">
            Home
          </button>
        </div>
      </div>
    </div>
  </div>
</div>
```

**Key architectural decisions:**
- Sidebar ALWAYS renders complete structure (SSR and client identical)
- Visibility controlled via `[hidden]` attribute, never conditional rendering
- Primitives provide data-* contract, UI provides semantic composition
- No inline styles, no dynamic classes
- Neutral `<div>` tags — semantic choice (`<nav>`, `<aside>`) is app responsibility

### Runtime Contract

The Shell runtime (JavaScript) looks for:
- `[data-sidebar-trigger]` — Toggle open/closed
- `[data-sidebar]` — Apply responsive behavior
- `[hidden]` attribute to control visibility

CSS looks for:
- `[hidden]` attribute for visibility
- `[data-active]` for menu button states
- `data-*` attributes for styling hooks

---

## 6. Execution Flow
```text
1. SSR Phase
   ├─ Sidebar renders with default_open state
   ├─ Complete HTML structure generated
   └─ [hidden] attribute applied if closed

2. Hydration Phase
   ├─ WASM mounts, walking existing DOM
   ├─ Signal<bool> created matching SSR state
   ├─ Event handlers attached to trigger
   └─ NO DOM mutations occur

3. User Interaction
   ├─ Trigger click toggles open signal
   ├─ Signal update triggers attribute change
   ├─ CSS adds/removes [hidden] attribute
   └─ Runtime JS may add overlay/focus trap

4. Runtime Enhancement
   ├─ Responsive collapse (Shell)
   ├─ Off-canvas overlay (Shell)
   ├─ Focus management (Shell)
   └─ These ONLY work post-hydration
```

**Critical:** Sidebar never conditionally renders. Structure is fixed, attributes change.

---

## 7. Canonical Use Cases

### Basic Navigation Sidebar
```rust
<SidebarProvider default_open=true>
    <Sidebar>
        <SidebarContent>
            <SidebarGroup>
                <SidebarGroupLabel>"Main"</SidebarGroupLabel>
                <SidebarMenu>
                    <SidebarMenuItem>
                        <SidebarMenuButton is_active=true>
                            "Dashboard"
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                    <SidebarMenuItem>
                        <SidebarMenuButton>
                            "Settings"
                        </SidebarMenuButton>
                    </SidebarMenuItem>
                </SidebarMenu>
            </SidebarGroup>
        </SidebarContent>
    </Sidebar>
    
    <SidebarInset>
        // Main content
    </SidebarInset>
</SidebarProvider>
```

### Collapsible Sidebar with Trigger
```rust
<SidebarProvider default_open=false>
    <header>
        <SidebarTrigger>
            <span>"☰"</span>
        </SidebarTrigger>
    </header>
    
    <Sidebar>
        <SidebarContent>
            <SidebarMenu>
                <SidebarMenuItem>
                    <SidebarMenuButton>"Home"</SidebarMenuButton>
                </SidebarMenuItem>
            </SidebarMenu>
        </SidebarContent>
    </Sidebar>
    
    <SidebarInset>
        // Content
    </SidebarInset>
</SidebarProvider>
```

### Multi-Group Sidebar
```rust
<Sidebar>
    <SidebarContent>
        <SidebarGroup>
            <SidebarGroupLabel>"Navigation"</SidebarGroupLabel>
            <SidebarMenu>
                <SidebarMenuItem>
                    <SidebarMenuButton>"Dashboard"</SidebarMenuButton>
                </SidebarMenuItem>
            </SidebarMenu>
        </SidebarGroup>
        
        <SidebarGroup>
            <SidebarGroupLabel>"Settings"</SidebarGroupLabel>
            <SidebarMenu>
                <SidebarMenuItem>
                    <SidebarMenuButton>"Profile"</SidebarMenuButton>
                </SidebarMenuItem>
            </SidebarMenu>
        </SidebarGroup>
    </SidebarContent>
</Sidebar>
```

---

## 8. Anti-Patterns (PROHIBITED)

### ❌ Conditional Sidebar Rendering
```rust
// WRONG - Causes hydration mismatch
{move || if show_sidebar.get() {
    Some(view! { <Sidebar>...</Sidebar> })
} else {
    None
}}
```

**Why it breaks:** SSR renders one structure, client another → unreachable panic.

**Correct approach:** Always render, control via `open` signal in Provider.

### ❌ Using Sidebar as Semantic `<nav>`
```rust
// WRONG - UI should not enforce semantics
<Sidebar>  // Renders <nav> internally
    ...
</Sidebar>
```

**Why it breaks:** Sidebar might contain filters, commands, or non-navigation content.

**Correct approach:** Sidebar renders neutral `<div>`, app wraps in `<nav>` if needed.

### ❌ Accessing Sidebar DOM Imperatively
```rust
// WRONG - Breaks SSR contract
Effect::new(move |_| {
    let sidebar = document().query_selector("[data-sidebar]");
    sidebar.unwrap().class_list().add_1("collapsed");
});
```

**Why it breaks:** Violates SSR/hydration contract, causes panics.

**Correct approach:** Let signal drive state, CSS reacts to attributes.

### ❌ Hardcoding Width in Component
```rust
// WRONG - Violates token governance
<div style="width: 250px" data-sidebar="">
```

**Why it breaks:** Width must come from `--sidebar-width` token for theme consistency.

**Correct approach:** Use CSS with tokens, never inline styles.

### ❌ Managing Active State in Sidebar
```rust
// WRONG - Sidebar should not know routing
<SidebarMenuButton is_active={current_route == "/home"}>
```

**Why it breaks:** Sidebar is presentation, routing is app logic.

**Correct approach:** App passes `is_active` prop based on router state.

---

## 9. SSR, Hydration and Runtime

### SSR Behavior

- Sidebar renders **complete structure** regardless of `open` state
- `[hidden]` attribute applied when `open=false`
- All data-* attributes present in initial HTML
- No JavaScript required for initial render
- Neutral `<div>` tags preserve semantic flexibility

### Hydration Requirements

**Critical:** Sidebar structure must be **identical** between SSR and client initial state.
```rust
// ✅ CORRECT - Signal matches SSR state
let default_open = true;  // SSR also renders open

// ❌ WRONG - Client starts different from SSR
// SSR rendered closed, client expects open
```

### Runtime Enhancement

The Shell runtime provides:
- Responsive collapse behavior
- Off-canvas overlay mode
- Focus management
- Viewport observers

**These behaviors only work post-hydration.** During SSR, they don't exist.

### RTL Support

Sidebar CSS uses `border-inline-end` instead of `border-right`, ensuring proper RTL layout without JavaScript.

---

## 10. Conformance Checklist

- [x] SSR-safe — Renders complete structure server-side
- [x] No imperative JS — All behavior via signals and data-attributes
- [x] Uses tokens — All styling via canonical tokens (Core + Typography + Color + Nav)
- [x] Tokens listed — Section 4 documents all tokens
- [x] Anti-patterns documented — Section 8 provides prohibited patterns
- [x] Rules cited — Section 11 lists applicable Canon Rules
- [x] Hydration-safe — Structure identical SSR/client
- [x] No conditional root — Sidebar always exists in DOM
- [x] Neutral semantics — Uses `<div>`, not `<nav>` or `<aside>`
- [x] RTL-compatible — Uses logical properties (`border-inline-end`)

---

## 11. Canon Rules Applied

### Canon Rules Applied

- **Canon Rule #4** — Hydration Safety First: Sidebar structure must be identical between SSR and client to prevent `unreachable` panics during DOM walking.

- **Canon Rule #6** — Visual State Must Be Predictable: Sidebar visibility controlled exclusively via `open` signal in Provider, never via imperative DOM manipulation.

- **Canon Rule #7** — Token Governance Over Ad-Hoc Styling: All Sidebar styling uses canonical tokens (`--sidebar-width`, `--color-*`, `--nav-*`, etc.), zero hardcoded values.

- **Canon Rule #50** — Provider Singleton Pattern: SidebarContext provided once per Sidebar instance, consumed by child primitives via `expect_context`.

- **Canon Rule #102** — Runtime JS Is Shell Infrastructure: Responsive behavior, off-canvas mode, focus management live in Shell runtime, not in component code.

- **Canon Rule #103** — Critical Runtime JS Must Be Inline in SSR: Sidebar behavior scripts must be inline or preloaded to ensure hydration correctness.

- **Canon Rule #104** — AutoReload Breaks Script Order Guarantees: Sidebar runtime must use event delegation and be idempotent, never assume script load order.

