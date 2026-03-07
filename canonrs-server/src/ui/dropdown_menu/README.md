---
component: DropdownMenu
layer: UI
status: Stable
since: v0.8
last_review: 2026-01-18
ownership: canonrs
keywords:
  - dropdown
  - menu
  - overlay
  - selection
  - design system
  - leptos
  - ssr
  - hydration
path_primitive: /opt/docker/monorepo/packages-rust/rs-design/src/primitives/dropdown_menu.rs
path_ui: /opt/docker/monorepo/packages-rust/rs-design/src/ui/dropdown_menu/dropdown_menu.rs
---

# DropdownMenu Component

## 1. Conceptual Introduction

The DropdownMenu component provides a contextual selection interface triggered by user interaction. It exists in the CanonRS UI layer as an ergonomic composition of DropdownMenu primitives, offering sensible defaults for common menu use cases including action menus, selection lists, and multi-select checkboxes.

DropdownMenu solves the architectural challenge of creating accessible, SSR-safe contextual menus without coupling to browser APIs, positioning logic, or imperative DOM manipulation. It lives in the **UI layer**, sitting between application Components and the underlying Primitives.

**What DropdownMenu does NOT do:**
- Does not execute browser APIs (positioning, outside click detection, etc.)
- Does not apply animations or transitions
- Does not manage selection state globally
- Does not enforce positioning logic (top, bottom, left, right)
- Does not handle keyboard navigation (runtime layer handles this)

---

## 2. Architectural Responsibility (Contract)

### Responsibility

- Composes DropdownMenu primitives into coherent menu structure
- Provides semantic slots (Trigger, Content, Group, Item, Label, Separator)
- Exposes open/closed state contract via RwSignal
- Defines intent (contextual overlay menu)
- Establishes data-attribute surface for runtime and CSS
- Provides ergonomic items with hover/destructive states

### Non-Responsibility

- Does NOT execute positioning logic (runtime layer handles this)
- Does NOT bind outside click listeners (runtime layer handles this)
- Does NOT create animations (CSS layer handles this)
- Does NOT enforce keyboard navigation (runtime layer handles this)
- Does NOT mutate document structure imperatively
- Does NOT manage checkbox state mutations (primitives only reflect state)

This separation ensures SSR safety and hydration correctness.

---

## 3. Position in CanonRS Ecosystem
```text
Application (DataTable, UserMenu)
    ↓
Component (Custom menus)
    ↓
UI (DropdownMenu) ← YOU ARE HERE
    ↓
Primitive (DropdownMenuPrimitive, DropdownMenuTriggerPrimitive, etc.)
    ↓
data-* attributes
    ↓
Shell Runtime JS (outside click, escape, positioning, focus)
    ↓
Browser API (addEventListener, getBoundingClientRect, focus(), etc.)
```

**SSR Flow:**
- DropdownMenu renders complete HTML structure during SSR
- Primitives emit data-* attributes
- Runtime JS hydrates and attaches behavior
- CSS controls visibility via `[hidden]` attribute

**Hydration:**
- Structure must be identical between SSR and client
- State changes only affect attributes, never structure
- No conditional rendering of DropdownMenu root

---

## 4. Tokens Applied

### Layout
- `--space-xs` — Content padding, group spacing, item gaps
- `--space-sm` — Item padding, label padding
- `--space-md` — (reserved for future use)
- `--radius-sm` — Item border radius
- `--radius-md` — Content border radius

### Typography
- `--font-family-sans` — All text content
- `--font-size-sm` — Items, labels
- `--font-weight-regular` — Default items
- `--font-weight-medium` — Labels
- `--line-height-normal` — Text line height

### Color
- `--color-bg-elevated` — Menu background
- `--color-bg-muted` — Hover state backgrounds
- `--color-fg-default` — Primary text
- `--color-fg-muted` — Labels, secondary text
- `--color-border-muted` — Menu border, separators
- `--color-danger-fg` — Destructive item text
- `--color-danger-bg` — Destructive item hover

### Border & Shadow
- `--border-width-hairline` — Menu border, separator height
- `--shadow-md` — Menu elevation

### State
- `--state-hover-opacity` — Item hover intensity

### Layering
- `--z-dropdown` — Menu z-index

### Lists (Família B)
- `--list-item-height` — Item minimum height
- `--list-item-padding` — Item padding (with fallback)

All tokens follow CanonRS token governance. No hardcoded values exist in CSS.

---

## 5. Technical Structure

### SSR HTML Output
```html
<div data-dropdown-menu-content="">
  <div data-dropdown-menu-group="">
    <div data-dropdown-menu-label="">Actions</div>
    <button data-dropdown-menu-item="" data-variant="default">
      Edit
    </button>
    <button data-dropdown-menu-item="" data-variant="destructive">
      Delete
    </button>
  </div>
  <div data-dropdown-menu-separator=""></div>
  <button data-dropdown-menu-checkbox-item="" data-checked="false">
    Show Grid
  </button>
</div>
```

**Key architectural decisions:**
- DropdownMenu ALWAYS renders complete structure (SSR and client identical)
- Visibility controlled via `[hidden]` attribute, never conditional rendering
- Primitives provide data-* contract, UI provides semantic composition
- No inline styles, no dynamic classes
- Neutral `<div>` and `<button>` tags for maximum flexibility

### Runtime Contract

The Shell runtime (JavaScript) looks for:
- `[data-dropdown-menu-trigger]` — Toggle open/closed
- `[data-dropdown-menu-action="toggle"]` — Trigger click behavior
- `[data-dropdown-menu-action="select-item"]` — Item selection
- `[data-dropdown-menu-action="toggle-checkbox"]` — Checkbox toggle
- Outside click → close menu
- Escape key → close menu

CSS looks for:
- `[hidden]` attribute for visibility
- `[data-variant]` for destructive items
- `[data-checked]` for checkbox states
- `data-*` attributes for styling hooks

---

## 6. Execution Flow
```text
1. SSR Phase
   ├─ DropdownMenu renders with default_open state
   ├─ Complete HTML structure generated
   └─ [hidden] attribute applied if closed

2. Hydration Phase
   ├─ WASM mounts, walking existing DOM
   ├─ Signal<bool> created matching SSR state
   ├─ Event handlers attached to primitives
   └─ NO DOM mutations occur

3. User Interaction
   ├─ Trigger click emits data-action="toggle"
   ├─ Runtime toggles open signal
   ├─ CSS removes [hidden] attribute
   └─ Runtime activates positioning & focus

4. Runtime Enhancement
   ├─ Position calculation (Shell)
   ├─ Outside click dismiss (Shell)
   ├─ Escape key handling (Shell)
   ├─ Keyboard navigation (Shell)
   └─ These ONLY work post-hydration
```

**Critical:** DropdownMenu never conditionally renders. Structure is fixed, attributes change.

---

## 7. Canonical Use Cases

### Basic Action Menu
```rust
<DropdownMenu>
    <DropdownMenuTrigger>
        <Button>"Actions"</Button>
    </DropdownMenuTrigger>
    
    <DropdownMenuContent>
        <DropdownMenuItem>"Edit"</DropdownMenuItem>
        <DropdownMenuItem>"Duplicate"</DropdownMenuItem>
        <DropdownMenuSeparator />
        <DropdownMenuItem destructive=true>"Delete"</DropdownMenuItem>
    </DropdownMenuContent>
</DropdownMenu>
```

### Grouped Menu
```rust
<DropdownMenu>
    <DropdownMenuTrigger>
        <Button>"File"</Button>
    </DropdownMenuTrigger>
    
    <DropdownMenuContent>
        <DropdownMenuGroup>
            <DropdownMenuLabel>"File"</DropdownMenuLabel>
            <DropdownMenuItem>"New"</DropdownMenuItem>
            <DropdownMenuItem>"Open"</DropdownMenuItem>
        </DropdownMenuGroup>
        
        <DropdownMenuSeparator />
        
        <DropdownMenuGroup>
            <DropdownMenuLabel>"Edit"</DropdownMenuLabel>
            <DropdownMenuItem>"Undo"</DropdownMenuItem>
            <DropdownMenuItem>"Redo"</DropdownMenuItem>
        </DropdownMenuGroup>
    </DropdownMenuContent>
</DropdownMenu>
```

### Checkbox Menu (View Options)
```rust
let show_grid = RwSignal::new(true);
let show_rulers = RwSignal::new(false);

view! {
    <DropdownMenu>
        <DropdownMenuTrigger>
            <Button>"View"</Button>
        </DropdownMenuTrigger>
        
        <DropdownMenuContent>
            <DropdownMenuLabel>"View Options"</DropdownMenuLabel>
            <DropdownMenuCheckboxItem checked=show_grid>
                "Show Grid"
            </DropdownMenuCheckboxItem>
            <DropdownMenuCheckboxItem checked=show_rulers>
                "Show Rulers"
            </DropdownMenuCheckboxItem>
        </DropdownMenuContent>
    </DropdownMenu>
}
```

---

## 8. Anti-Patterns (PROHIBITED)

### ❌ Conditional DropdownMenu Rendering
```rust
// WRONG - Causes hydration mismatch
{move || if show_menu.get() {
    Some(view! { <DropdownMenu>...</DropdownMenu> })
} else {
    None
}}
```

**Why it breaks:** SSR renders one structure, client another → unreachable panic.

**Correct approach:** Always render, control via `open` signal in provider.

### ❌ Mutating Checkbox State Imperatively
```rust
// WRONG - Breaks primitive contract
Effect::new(move |_| {
    let checkbox = document().query_selector("[data-dropdown-menu-checkbox-item]");
    checkbox.unwrap().set_attribute("data-checked", "true");
});
```

**Why it breaks:** Violates SSR/hydration contract, bypasses reactive system.

**Correct approach:** Let signal drive state, primitives reflect state via data-attributes.

### ❌ Positioning with Inline Styles
```rust
// WRONG - Violates token governance
<DropdownMenuContent style="top: 100px; left: 50px">
```

**Why it breaks:** Positioning must be handled by runtime, not hardcoded.

**Correct approach:** Runtime calculates position based on trigger bounds and viewport.

### ❌ Using <Show> for Content Visibility
```rust
// WRONG - Breaks hydration
<Show when=move || is_open.get()>
    <DropdownMenuContent>...</DropdownMenuContent>
</Show>
```

**Why it breaks:** Creates/destroys DOM nodes, violating structural determinism.

**Correct approach:** Always render content, use `[hidden]` attribute for visibility.

### ❌ Executing Selection Logic in UI
```rust
// WRONG - UI should not execute business logic
<DropdownMenuItem on:click=move |_| {
    delete_user(user_id);
    show_notification();
}>
```

**Why it breaks:** UI only emits intent, application handles logic.

**Correct approach:** Pass callbacks from parent component, let app handle side effects.

---

## 9. SSR, Hydration and Runtime

### SSR Behavior

- DropdownMenu renders **complete structure** regardless of `open` state
- `[hidden]` attribute applied when `open=false`
- All data-* attributes present in initial HTML
- No JavaScript required for initial render

### Hydration Requirements

**Critical:** DropdownMenu structure must be **identical** between SSR and client initial state.
```rust
// ✅ CORRECT - Signal matches SSR state
let default_open = false;  // SSR also renders closed

// ❌ WRONG - Client starts different from SSR
// SSR rendered closed, client expects open
```

### Runtime Enhancement

The Shell runtime provides:
- Position calculation based on trigger bounds
- Outside click detection → close menu
- Escape key → close menu
- Keyboard navigation (Arrow keys, Enter, Space)
- Focus management

**These behaviors only work post-hydration.** During SSR, they don't exist.

### Checkbox State Management

Checkbox items receive `checked` as a `Signal<bool>`, making them **read-only** from the primitive's perspective. State mutations happen in the parent component, primitives only reflect current state.

---

## 10. Conformance Checklist

- [x] SSR-safe — Renders complete structure server-side
- [x] No imperative JS — All behavior via signals and data-attributes
- [x] Uses tokens — All styling via canonical tokens (Core + Typography + Color + Lists + Overlay)
- [x] Tokens listed — Section 4 documents all tokens
- [x] Anti-patterns documented — Section 8 provides prohibited patterns
- [x] Rules cited — Section 11 lists applicable Canon Rules
- [x] Hydration-safe — Structure identical SSR/client
- [x] No conditional root — Content always exists in DOM
- [x] Primitives emit only — No behavior execution in primitives
- [x] Runtime controls visibility — UI does not read context for [hidden]

---

## 11. Canon Rules Applied

### Canon Rules Applied

- **Canon Rule #4** — Hydration Safety First: DropdownMenu structure must be identical between SSR and client to prevent `unreachable` panics during DOM walking.

- **Canon Rule #6** — Visual State Must Be Predictable: DropdownMenu visibility controlled exclusively via `open` signal in provider, never via imperative DOM manipulation.

- **Canon Rule #7** — Token Governance Over Ad-Hoc Styling: All DropdownMenu styling uses canonical tokens (`--z-dropdown`, `--color-*`, `--list-*`, etc.), zero hardcoded values.

- **Canon Rule #50** — Provider Singleton Pattern: DropdownMenuContext provided once per DropdownMenu instance, consumed by child primitives via `expect_context`.

- **Canon Rule #102** — Runtime JS Is Shell Infrastructure: Positioning, outside click, escape handling, keyboard navigation live in Shell runtime, not in component code.

- **Canon Rule #103** — Critical Runtime JS Must Be Inline in SSR: DropdownMenu behavior scripts must be inline or preloaded to ensure hydration correctness.

- **Canon Rule #104** — AutoReload Breaks Script Order Guarantees: DropdownMenu runtime must use event delegation and be idempotent, never assume script load order.

