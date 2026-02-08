---
component: Dialog
layer: UI
status: Stable
since: v0.8
last_review: 2026-01-18
ownership: canonrs
keywords:
  - dialog
  - modal
  - overlay
  - design system
  - leptos
  - ssr
  - hydration
path_primitive: /opt/docker/monorepo/packages-rust/rs-design/src/primitives/dialog.rs
path_ui: /opt/docker/monorepo/packages-rust/rs-design/src/ui/dialog.rs
---

# Dialog Component

## 1. Conceptual Introduction

The Dialog component provides a structured overlay interface for focused user interactions that require full attention. It exists in the CanonRS UI layer as an ergonomic composition of Dialog primitives, offering sensible defaults for common modal use cases.

Dialog solves the architectural challenge of creating accessible, SSR-safe modal interfaces without coupling to browser APIs or imperative DOM manipulation. It lives in the **UI layer**, sitting between application Blocks and the underlying Primitives.

**What Dialog does NOT do:**
- Does not execute browser APIs (focus trap, escape handling, etc.)
- Does not apply animations or transitions
- Does not manage global state or routing
- Does not enforce specific accessibility implementations

---

## 2. Architectural Responsibility (Contract)

### Responsibility

- Composes Dialog primitives into coherent overlay structure
- Provides semantic slots (Header, Title, Body, Close)
- Exposes open/closed state contract via RwSignal
- Defines intent (overlay, dismissible content)
- Establishes data-attribute surface for runtime and CSS

### Non-Responsibility

- Does NOT execute focus trapping (runtime layer handles this)
- Does NOT bind keyboard events (runtime layer handles this)
- Does NOT create animations (CSS layer handles this)
- Does NOT enforce ARIA relationships (runtime layer handles this)
- Does NOT mutate document structure imperatively

This separation ensures SSR safety and hydration correctness.

---

## 3. Position in CanonRS Ecosystem
```text
Application (ButtonPage)
    ↓
Block (PillarsStrip) 
    ↓
UI (Dialog) ← YOU ARE HERE
    ↓
Primitive (DialogPrimitive, DialogBackdropPrimitive, etc.)
    ↓
data-* attributes
    ↓
Shell Runtime JS (focus trap, escape, outside click)
    ↓
Browser API (focus(), addEventListener, etc.)
```

**SSR Flow:**
- Dialog renders complete HTML structure during SSR
- Primitives emit data-* attributes
- Runtime JS hydrates and attaches behavior
- CSS controls visibility via `[hidden]` attribute

**Hydration:**
- Structure must be identical between SSR and client
- State changes only affect attributes, never structure
- No conditional rendering of Dialog root

---

## 4. Tokens Applied

### Layout
- `--space-md` — Header, body padding
- `--space-lg` — Viewport margins
- `--radius-lg` — Dialog border radius

### Typography
- `--font-family-sans` — All text content
- `--font-size-lg` — Title size
- `--font-size-md` — Body content
- `--font-weight-semibold` — Title weight
- `--font-weight-regular` — Body weight
- `--line-height-tight` — Title line height
- `--line-height-normal` — Body line height

### Color
- `--color-bg-elevated` — Dialog background
- `--color-fg-default` — Primary text
- `--color-fg-muted` — Close button
- `--color-border-muted` — Header border, dialog outline
- `--border-width-hairline` — Border thickness

### Layering & Overlay
- `--z-modal` — Backdrop and popup z-index
- `--overlay-backdrop-opacity` — Backdrop opacity

### Shadow
- `--shadow-md` — Dialog elevation

### Size
- `--size-modal-md` — Max width constraint

All tokens follow CanonRS token governance. No hardcoded values exist in CSS.

---

## 5. Technical Structure

### SSR HTML Output
```html
<div data-dialog="">
  <div data-dialog-backdrop=""></div>
  <div data-dialog-popup="">
    <div data-dialog-header="">
      <div data-dialog-title="">
        <h2>Canon Documentation</h2>
      </div>
      <button data-dialog-close="">×</button>
    </div>
    <div data-dialog-body="">
      <!-- content -->
    </div>
  </div>
</div>
```

**Key architectural decisions:**
- Dialog ALWAYS renders complete structure (SSR and client identical)
- Visibility controlled via `[hidden]` attribute, never conditional rendering
- Primitives provide data-* contract, UI provides semantic composition
- No inline styles, no dynamic classes

### Runtime Contract

The Shell runtime (JavaScript) looks for:
- `[data-dialog-backdrop]` — Outside click dismiss
- `[data-dialog-close]` — Close button behavior
- `[data-dialog-popup]` — Focus trap container
- Escape key handling attached to document

CSS looks for:
- `[hidden]` attribute to control visibility
- `data-*` attributes for styling hooks

---

## 6. Execution Flow
```text
1. SSR Phase
   ├─ Dialog renders with open=false
   ├─ Complete HTML structure generated
   └─ [hidden] attributes applied

2. Hydration Phase
   ├─ WASM mounts, walking existing DOM
   ├─ Signal<bool> created matching SSR state
   ├─ Event handlers attached
   └─ NO DOM mutations occur

3. User Interaction
   ├─ Button click sets open.set(true)
   ├─ Signal update triggers attribute change
   ├─ CSS removes [hidden] attribute
   └─ Runtime JS activates focus trap

4. Runtime Enhancement
   ├─ Escape key listener (Shell)
   ├─ Outside click listener (Shell)
   ├─ Focus trap activation (Shell)
   └─ These ONLY work post-hydration
```

**Critical:** Dialog never conditionally renders. Structure is fixed, attributes change.

---

## 7. Canonical Use Cases

### Modal Documentation Viewer (PillarsStrip)
```rust
let is_open = RwSignal::new(false);

view! {
    <button on:click=move |_| is_open.set(true)>
        "Open Docs"
    </button>

    <Dialog open=is_open>
        <DialogBackdrop />
        <DialogContent>
            <DialogHeader>
                <DialogTitle>"Documentation"</DialogTitle>
                <DialogClose><span>"×"</span></DialogClose>
            </DialogHeader>
            <DialogBody>
                <CanonMarkdown content=markdown_signal />
            </DialogBody>
        </DialogContent>
    </Dialog>
}
```

### Confirmation Dialog
```rust
<Dialog open=confirm_open>
    <DialogBackdrop />
    <DialogContent>
        <DialogHeader>
            <DialogTitle>"Confirm Action"</DialogTitle>
        </DialogHeader>
        <DialogBody>
            <p>"Are you sure?"</p>
            <button on:click=move |_| handle_confirm()>"Yes"</button>
        </DialogBody>
    </DialogContent>
</Dialog>
```

### Form Dialog

Dialog works naturally with forms since structure is always present:
```rust
<Dialog open=form_open>
    <DialogBackdrop />
    <DialogContent>
        <DialogHeader>
            <DialogTitle>"New Entry"</DialogTitle>
            <DialogClose><span>"×"</span></DialogClose>
        </DialogHeader>
        <DialogBody>
            <form on:submit=handle_submit>
                <Input />
                <Button type_="submit">"Save"</Button>
            </form>
        </DialogBody>
    </DialogContent>
</Dialog>
```

---

## 8. Anti-Patterns (PROHIBITED)

### ❌ Conditional Dialog Rendering
```rust
// WRONG - Causes hydration mismatch
{move || if show_dialog.get() {
    Some(view! { <Dialog>...</Dialog> })
} else {
    None
}}
```

**Why it breaks:** SSR renders one structure, client another → unreachable panic.

**Correct approach:** Always render, control via `open` signal.

### ❌ Nested Dialogs Without Proper Z-Index
```rust
// WRONG - Z-index conflicts
<Dialog open=outer>
    <Dialog open=inner>  // Both use same z-modal
    </Dialog>
</Dialog>
```

**Why it breaks:** Both dialogs share `--z-modal` token, causing stacking issues.

**Correct approach:** Use separate overlay layers or portal pattern.

### ❌ Accessing Dialog DOM Imperatively
```rust
// WRONG - Breaks SSR contract
Effect::new(move |_| {
    let dialog = document().query_selector("[data-dialog]");
    dialog.unwrap().set_attribute("aria-hidden", "false");
});
```

**Why it breaks:** Violates SSR/hydration contract, causes panics.

**Correct approach:** Let signal drive state, runtime handles attributes.

### ❌ Using Display:None for Visibility
```css
/* WRONG */
[data-dialog]:not([data-open="true"]) {
  display: none;
}
```

**Why it breaks:**
- Breaks animations
- Breaks focus trap cleanup
- Prevents transition effects

**Correct approach:** Use `[hidden]` attribute with opacity/pointer-events.

---

## 9. SSR, Hydration and Runtime

### SSR Behavior

- Dialog renders **complete structure** regardless of `open` state
- `[hidden]` attribute applied when `open=false`
- All data-* attributes present in initial HTML
- No JavaScript required for initial render

### Hydration Requirements

**Critical:** Dialog structure must be **identical** between SSR and client initial state.
```rust
// ✅ CORRECT - Signal matches SSR state
let is_open = RwSignal::new(false);  // SSR also renders closed

// ❌ WRONG - Client starts different from SSR
let is_open = RwSignal::new(true);  // SSR rendered closed!
```

### Runtime Enhancement

The Shell runtime provides:
- Focus trap activation when dialog opens
- Escape key → close dialog
- Outside click → close dialog
- Restore focus on close

**These behaviors only work post-hydration.** During SSR, they don't exist.

### AutoReload Constraints

With `cargo-leptos --hot-reload`, script order is NOT guaranteed. Dialog runtime must:
- Be idempotent
- Use event delegation
- Never assume initialization order

---

## 10. Conformance Checklist

- [x] SSR-safe — Renders complete structure server-side
- [x] No imperative JS — All behavior via signals and data-attributes
- [x] Uses tokens — All styling via canonical tokens
- [x] Tokens listed — Section 4 documents all tokens
- [x] Anti-patterns documented — Section 8 provides prohibited patterns
- [x] Rules cited — Section 11 lists applicable Canon Rules
- [x] Hydration-safe — Structure identical SSR/client
- [x] No conditional root — Dialog always exists in DOM

---

## 11. Canon Rules Applied

### Canon Rules Applied

- **Canon Rule #4** — Hydration Safety First: Dialog structure must be identical between SSR and client to prevent `unreachable` panics during DOM walking.

- **Canon Rule #6** — Visual State Must Be Predictable: Dialog visibility controlled exclusively via `open` signal, never via imperative DOM manipulation.

- **Canon Rule #7** — Token Governance Over Ad-Hoc Styling: All Dialog styling uses canonical tokens (`--space-*`, `--color-*`, `--z-modal`, etc.), zero hardcoded values.

- **Canon Rule #50** — Provider Singleton Pattern: DialogContext provided once per Dialog instance, consumed by child primitives via `expect_context`.

- **Canon Rule #102** — Runtime JS Is Shell Infrastructure: Focus trap, escape handling, outside-click dismiss live in Shell runtime, not in component code.

- **Canon Rule #103** — Critical Runtime JS Must Be Inline in SSR: Dialog behavior scripts must be inline or preloaded to ensure hydration correctness.

- **Canon Rule #104** — AutoReload Breaks Script Order Guarantees: Dialog runtime must use event delegation and be idempotent, never assume script load order.

