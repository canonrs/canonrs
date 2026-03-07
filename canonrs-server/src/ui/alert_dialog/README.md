---
component: AlertDialog
layer: UI
status: Stable
since: v1.0
last_review: 2025-01-18
ownership: canonrs
keywords:
  - design system
  - dioxus
  - ssr
  - alert dialog
  - modal
  - enterprise ui
  - blocking modal
path_primitive: /opt/docker/monorepo/packages-rust/rs-design/src/primitives/alert_dialog.rs
path_ui: /opt/docker/monorepo/packages-rust/rs-design/src/ui/alert_dialog/
---

# AlertDialog

## 1. Conceptual Introduction

The AlertDialog is a **UI component** that provides a blocking modal interaction pattern for critical user decisions. It serves as the canonical enterprise solution for destructive actions, confirmations, warnings, and situations requiring explicit user acknowledgment before proceeding.

The AlertDialog exists in the **UI layer** because it provides:
- Blocking modal behavior (non-dismissible by default)
- Controlled state management via `RwSignal<bool>`
- Ergonomic API for critical decision flows
- Composition of primitives without business logic

**What it is NOT:**
- Not a primitive (it composes primitives)
- Not a dismissible dialog (use Dialog for that)
- Not a runtime behavior (no browser APIs)
- Not a notification (use Toast for non-blocking feedback)

---

## 2. Architectural Responsibility (Contract)

### Responsibility

The AlertDialog UI component:
- **Composes** `AlertDialogPrimitive`, `AlertDialogTrigger`, `AlertDialogPortal`, `AlertDialogOverlay`, `AlertDialogContent`, `AlertDialogHeader`, `AlertDialogFooter`, `AlertDialogTitle`, `AlertDialogDescription`, `AlertDialogAction`, and `AlertDialogCancel`
- **Declares** non-dismissible modal behavior via `data-modal="true"` and `data-dismissible="false"`
- **Exposes** controlled state API with `open: RwSignal<bool>` and `default_open: bool`
- **Emits** SSR-safe HTML with data-attributes for runtime hydration
- **Guarantees** blocking interaction pattern (ESC and overlay clicks disabled)

### Non-Responsibility

The AlertDialog UI component explicitly does NOT:
- ❌ Execute browser APIs (focus trap, click handlers, ESC key)
- ❌ Manage state imperatively (state lives in `RwSignal` or DOM)
- ❌ Apply CSS classes or inline styles
- ❌ Register global event listeners
- ❌ Perform side effects or mutations
- ❌ Allow dismissal without explicit Action/Cancel

**Side effects are PROHIBITED.**  
All runtime behavior is delegated to the Shell Runtime JS layer.

---

## 3. Position in CanonRS Ecosystem

The AlertDialog participates in the canonical CanonRS flow:
```text
Page/Block (usage)
  ↓
UI Component (AlertDialog) — controlled state, blocking behavior
  ↓
Primitives (AlertDialogPrimitive, AlertDialogOverlay, etc.) — HTML + data-attributes
  ↓
SSR Render — static HTML emitted
  ↓
Shell Runtime JS — focus trap, event delegation, controlled state sync
  ↓
Browser API — DOM manipulation, ARIA, keyboard navigation
  ↓
CSS — styling via [data-alert-dialog-*] selectors
```

**SSR Context:**
- AlertDialog renders complete HTML structure on server
- Initial state determined by `default_open` prop
- No client-side JS required for initial render
- Runtime JS hydrates interactive behavior on client

**Hydration:**
- Shell Runtime reads `data-modal`, `data-dismissible`, and `data-default-open`
- Sets initial `data-open="true|false"` based on `default_open` or controlled `open` signal
- Attaches focus trap to content region
- Disables ESC and overlay click (non-dismissible contract)
- Syncs `RwSignal<bool>` with DOM state when Action/Cancel clicked

---

## 4. Tokens Applied

The AlertDialog UI component does **not directly apply tokens**—it delegates to CSS.  
The CSS layer (`style/ui/alert_dialog.css`) consumes the following token families:

### Overlay & Layering Tokens
- `--z-modal` (overlay and content z-index)
- `--overlay-backdrop-opacity` (overlay transparency)

### Layout Tokens
- `--space-lg` (content padding, max-width offset)
- `--space-md` (content gap)
- `--space-sm` (footer gap)
- `--space-xs` (header gap)
- `--size-modal-md` (content width)

### Typography Tokens
- `--font-size-lg` (title size)
- `--font-size-sm` (description size)
- `--font-weight-semibold` (title weight)
- `--line-height-tight` (title line height)
- `--line-height-normal` (description line height)

### Color Tokens
- `--color-bg-surface` (content background)
- `--color-fg-default` (title text)
- `--color-fg-muted` (description text)
- `--color-border-muted` (content border)

### Border & Radius Tokens
- `--border-width-hairline` (content border)
- `--radius-lg` (content border radius)

### Shadow Tokens
- `--shadow-lg` (content shadow)

**Token Resolution:**  
UI component emits data-attributes → CSS applies tokens → Browser renders.

---

## 5. Technical Structure (How It Works)

### SSR Render Phase

The AlertDialog component renders to static HTML:
```html
<div data-alert-dialog="" data-modal="true" data-dismissible="false" data-default-open="false">
  <button data-alert-dialog-trigger="" type="button">
    Delete Account
  </button>
  
  <div data-alert-dialog-portal="">
    <div data-alert-dialog-overlay=""></div>
    <div data-alert-dialog-content="">
      <div data-alert-dialog-header="">
        <div data-alert-dialog-title="">Are you absolutely sure?</div>
        <p data-alert-dialog-description="">
          This action cannot be undone. This will permanently delete your account.
        </p>
      </div>
      <div data-alert-dialog-footer="">
        <button data-alert-dialog-cancel="">Cancel</button>
        <button data-alert-dialog-action="">Delete</button>
      </div>
    </div>
  </div>
</div>
```

**Key contracts:**
- `data-alert-dialog` marks root container
- `data-modal="true"` enforces blocking behavior
- `data-dismissible="false"` disables ESC/overlay close
- `data-default-open="true|false"` specifies initial state
- `data-open="true|false"` reflects current open state (set by runtime or controlled signal)
- `data-alert-dialog-action` marks confirm button (closes dialog on click)
- `data-alert-dialog-cancel` marks cancel button (closes dialog on click)

### Runtime Hydration Phase

Shell Runtime JS:
1. Queries `[data-alert-dialog]` elements
2. Reads `data-modal`, `data-dismissible`, and `data-default-open` attributes
3. Sets initial `data-open="true|false"` based on `default_open` or controlled `open` signal
4. Attaches focus trap to `[data-alert-dialog-content]` when open
5. Disables ESC key handler (non-dismissible)
6. Disables overlay click handler (non-dismissible)
7. Attaches click handlers to `[data-alert-dialog-action]` and `[data-alert-dialog-cancel]`
8. On Action/Cancel click: sets `data-open="false"` and syncs controlled `RwSignal` if present

### CSS Styling Phase

CSS selectors target data-attributes:
```css
[data-alert-dialog-overlay] {
  opacity: 0;
  transition: opacity 200ms ease;
}

[data-alert-dialog][data-open="true"] [data-alert-dialog-overlay] {
  opacity: 1;
}

[data-alert-dialog-content] {
  opacity: 0;
  transform: translate(-50%, -50%) scale(0.95);
  transition: opacity 200ms ease, transform 200ms ease;
}

[data-alert-dialog][data-open="true"] [data-alert-dialog-content] {
  opacity: 1;
  transform: translate(-50%, -50%) scale(1);
}
```

**No classes. No inline styles. Only data-attributes.**

---

## 6. Execution Flow
```text
1. SSR Render (Server)
   ↓
   AlertDialog component executes
   ↓
   Composes primitives with data-attributes
   ↓
   Emits static HTML with data-modal="true", data-dismissible="false"

2. HTML Delivery (Network)
   ↓
   Browser receives HTML
   ↓
   Renders static structure (overlay/content hidden via opacity: 0)

3. Shell Runtime Hydration (Client)
   ↓
   Runtime JS executes
   ↓
   Reads data-alert-dialog attributes
   ↓
   Sets initial data-open based on default_open or controlled signal
   ↓
   Attaches focus trap and event delegation
   ↓
   Disables ESC and overlay click (non-dismissible contract)

4. User Interaction (Trigger Click)
   ↓
   User clicks [data-alert-dialog-trigger]
   ↓
   Runtime sets data-open="true" on container
   ↓
   CSS animates overlay (opacity 0→1) and content (scale 0.95→1)
   ↓
   Focus trap activates, moves focus to content

5. User Decision (Action or Cancel)
   ↓
   User clicks [data-alert-dialog-action] or [data-alert-dialog-cancel]
   ↓
   Runtime sets data-open="false"
   ↓
   If controlled: syncs RwSignal.set(false)
   ↓
   CSS animates close (opacity 1→0, scale 1→0.95)
   ↓
   Focus returns to trigger element
```

**Critical:** State lives in DOM (`data-open`) and optionally in `RwSignal<bool>`.  
Runtime synchronizes both, ensuring SSR compatibility and controlled state management.

---

## 7. Canonical Use Cases

### Destructive Action Confirmation
```rust
use dioxus::prelude::*;
use rs_design::ui::alert_dialog::*;

fn DeleteAccountDialog() -> Element {
    rsx! {
        AlertDialog {
            default_open: false,
            
            AlertDialogTrigger { "Delete Account" }
            
            AlertDialogPortal {
                AlertDialogOverlay {}
                AlertDialogContent {
                    AlertDialogHeader {
                        AlertDialogTitle { "Are you absolutely sure?" }
                        AlertDialogDescription {
                            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        }
                    }
                    AlertDialogFooter {
                        AlertDialogCancel { "Cancel" }
                        AlertDialogAction { "Delete Account" }
                    }
                }
            }
        }
    }
}
```

### Controlled State (External Trigger)
```rust
fn ControlledAlertDialog() -> Element {
    let mut is_open = use_signal(|| false);
    
    rsx! {
        button { onclick: move |_| is_open.set(true), "Open Alert" }
        
        AlertDialog {
            open: is_open,
            
            AlertDialogPortal {
                AlertDialogOverlay {}
                AlertDialogContent {
                    AlertDialogHeader {
                        AlertDialogTitle { "Confirm Logout" }
                        AlertDialogDescription { "Are you sure you want to log out?" }
                    }
                    AlertDialogFooter {
                        AlertDialogCancel { 
                            onclick: move |_| is_open.set(false),
                            "Cancel" 
                        }
                        AlertDialogAction { 
                            onclick: move |_| {
                                is_open.set(false);
                                // perform logout
                            },
                            "Logout" 
                        }
                    }
                }
            }
        }
    }
}
```

### Warning Dialog
```rust
AlertDialog {
    default_open: false,
    
    AlertDialogTrigger { "Submit Form" }
    
    AlertDialogPortal {
        AlertDialogOverlay {}
        AlertDialogContent {
            AlertDialogHeader {
                AlertDialogTitle { "Unsaved Changes" }
                AlertDialogDescription {
                    "You have unsaved changes. Submitting will discard them."
                }
            }
            AlertDialogFooter {
                AlertDialogCancel { "Go Back" }
                AlertDialogAction { "Submit Anyway" }
            }
        }
    }
}
```

---

## 8. Anti-Patterns (PROHIBITED)

### ❌ Anti-Pattern 1: Using Signal<bool> Instead of RwSignal<bool>
```rust
// WRONG — Signal is read-only, AlertDialog needs write access
let open = use_signal(|| false);

AlertDialog {
    open: open, // ❌ FORBIDDEN — cannot close dialog
}
```

**Why it breaks:**
- `Signal<bool>` is read-only
- Runtime cannot call `.set(false)` on Action/Cancel
- Dialog becomes unclosable without external hack
- Violates enterprise controllability contract

**Correct approach:**
```rust
let mut open = use_signal(|| false); // RwSignal via use_signal

AlertDialog {
    open: open, // ✅ Runtime can write
}
```

---

### ❌ Anti-Pattern 2: Making AlertDialog Dismissible
```rust
// WRONG — AlertDialog is non-dismissible by design
rsx! {
    div {
        "data-alert-dialog": "",
        "data-dismissible": "true", // ❌ FORBIDDEN
    }
}
```

**Why it breaks:**
- AlertDialog is for critical decisions
- Accidental dismissal (ESC, overlay click) is dangerous
- Violates enterprise UX pattern for blocking modals
- Use Dialog component if dismissible behavior needed

**Correct approach:**  
Use `Dialog` for dismissible modals. AlertDialog is always blocking.

---

### ❌ Anti-Pattern 3: Direct Browser API Usage
```rust
// WRONG — accessing window in UI component
#[component]
pub fn AlertDialog(...) -> Element {
    use_effect(move |_| {
        window().document().query_selector(...); // FORBIDDEN
    });
}
```

**Why it breaks:**
- UI components must be SSR-safe
- No `window()`, `document()`, or `web_sys` allowed
- Violates layer separation

**Correct approach:**  
Shell Runtime handles all browser APIs (focus trap, event delegation).

---

### ❌ Anti-Pattern 4: Inline Styles or Classes
```rust
// WRONG — applying classes or styles directly
rsx! {
    AlertDialogContent {
        class: "bg-white p-6 rounded-lg", // FORBIDDEN
        style: "z-index: 9999", // FORBIDDEN
    }
}
```

**Why it breaks:**
- Primitives emit data-attributes only
- CSS layer owns all styling
- Breaks token system and theming

**Correct approach:**  
Emit `data-alert-dialog-content` and let CSS handle styling.

---

### ❌ Anti-Pattern 5: Nesting AlertDialogs
```rust
// WRONG — nesting alert dialogs
AlertDialog {
    AlertDialogContent {
        AlertDialog { // ❌ FORBIDDEN — cognitive overload
            AlertDialogContent { ... }
        }
    }
}
```

**Why it breaks:**
- Cognitive overload for users
- Confusing focus trap stacking
- Violates enterprise UX patterns

**Correct approach:**  
Resolve to single decision point or use sequential dialogs.

---

## 9. SSR, Hydration, and Runtime

### SSR Impact

**Server-Side:**
- AlertDialog renders complete HTML structure
- All `data-*` attributes present in initial HTML
- Initial state determined by `default_open` prop
- No JavaScript required for structure
- Content hidden via CSS (`opacity: 0`)

**Benefits:**
- SEO-friendly (content visible to crawlers)
- Fast First Contentful Paint
- Works without JavaScript (graceful degradation)
- No layout shift during hydration

**Constraints:**
- Cannot use browser APIs during SSR
- Cannot access `window`, `document`, `localStorage`
- Must emit pure HTML

---

### Hydration Process

1. **HTML Delivery:** Browser receives static HTML with `data-alert-dialog` attributes
2. **CSS Application:** Browser applies `alert_dialog.css` styles (content hidden)
3. **Runtime Load:** Shell Runtime JS executes
4. **State Initialization:** Runtime reads `data-default-open` or controlled `open` signal, sets `data-open="true|false"`
5. **Focus Trap Setup:** Runtime prepares focus trap (inactive until opened)
6. **Event Delegation:** Runtime attaches delegated handlers for trigger, action, cancel
7. **Interactive:** Component is now fully interactive

**Hydration Contract:**
- Runtime never re-renders HTML
- Runtime only mutates `data-open` attribute
- Runtime syncs `RwSignal<bool>` if controlled
- CSS reacts to `data-open` state changes

---

### Runtime Global Constraints

**Focus Trap Behavior:**
- Focus trap activates when `data-open="true"`
- Traps focus within `[data-alert-dialog-content]`
- Returns focus to trigger on close
- Respects ARIA best practices

**Non-Dismissible Contract:**
- ESC key handler disabled
- Overlay click handler disabled
- Only Action/Cancel buttons close dialog
- Ensures critical decision completion

**AutoReload Behavior:**
- AutoReload during dev can break script order
- Shell Runtime must be inline in SSR (Canon Rule #103)
- External scripts may load out of order

**Mitigation:**
- Critical runtime JS inlined in `<head>`
- Non-critical behavior degrades gracefully
- Never depend on external script load timing

**Hot Reload:**
- AlertDialog preserves state across hot reloads
- `data-open` persists in DOM
- Controlled `RwSignal` re-syncs on component re-render
- No state loss during development

---

## 10. Conformance Checklist

- [x] SSR-safe (no browser APIs in component)
- [x] No imperative JS (no `use_effect`, `window()`, `document()`)
- [x] Uses tokens (indirectly via CSS)
- [x] All tokens documented in section 4
- [x] Anti-patterns documented with explanations
- [x] Canon Rules cited in section 11
- [x] Execution flow documented
- [x] Use cases provided
- [x] Hydration contract explicit
- [x] Runtime constraints documented
- [x] Controlled state uses `RwSignal<bool>`
- [x] Non-dismissible behavior enforced

---

## 11. Canon Rules Applied

### Canon Rules Applied

- **Canon Rule #102 — Runtime JS Is Shell Infrastructure**  
  AlertDialog delegates all browser interaction (focus trap, event delegation, ESC handling) to Shell Runtime, never executing imperative JS in the component layer.

- **Canon Rule #103 — Critical Runtime JS Must Be Inline in SSR**  
  AlertDialog hydration depends on Shell Runtime being available immediately; inline runtime ensures no race conditions for focus trap initialization.

- **Canon Rule #104 — AutoReload Breaks Script Order Guarantees**  
  AlertDialog state persists in DOM (`data-open`) and optionally in `RwSignal` to survive AutoReload script re-execution during development.

- **Canon Rule #107 — Primitives Are SSR-Safe Structural Components**  
  AlertDialog composes SSR-safe primitives that emit only HTML and data-attributes, never browser-dependent logic.

- **Canon Rule #108 — UI Components Provide Ergonomic Composition**  
  AlertDialog exists in the UI layer to provide blocking modal behavior (`data-modal`, `data-dismissible`) and controlled state API (`RwSignal<bool>`), reducing boilerplate in application code.

- **Canon Rule #109 — State Lives in DOM, Not Memory**  
  AlertDialog state is declared via `data-open` attribute and optionally controlled via `RwSignal<bool>`, making it inspectable, debuggable, and SSR-compatible.

- **Canon Rule #110 — CSS Targets Data-Attributes, Never Classes**  
  AlertDialog styling uses `[data-alert-dialog-*]` selectors exclusively, ensuring token-based theming and avoiding Tailwind/utility class pollution.

- **Canon Rule #111 — Controlled Components Use RwSignal, Not Signal**  
  AlertDialog enforces `RwSignal<bool>` for controlled state to enable runtime write access (Action/Cancel buttons must close dialog), rejecting read-only `Signal<bool>`.

---

**End of Documentation**
