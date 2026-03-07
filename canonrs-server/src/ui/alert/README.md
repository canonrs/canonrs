---
component: Alert
layer: UI
status: Stable
since: v1.0
last_review: 2025-01-18
ownership: canonrs
keywords:
  - design system
  - dioxus
  - ssr
  - alert
  - notification
  - feedback
path_primitive: /opt/docker/monorepo/packages-rust/rs-design/src/primitives/alert.rs
path_ui: /opt/docker/monorepo/packages-rust/rs-design/src/ui/alert/
---

# Alert

## 1. Conceptual Introduction

The Alert is a **UI component** that provides non-blocking, inline feedback messages to users. It serves as the canonical enterprise solution for informational messages, warnings, error states, and success confirmations within page content.

The Alert exists in the **UI layer** because it provides:
- Variant system (default, destructive) for semantic feedback
- Ergonomic composition of title, description, and optional icon
- Inline feedback pattern (not modal, not toast)
- Composition of primitives without business logic

**What it is NOT:**
- Not a primitive (it composes primitives)
- Not a blocking modal (use AlertDialog for critical decisions)
- Not a toast notification (use Toast for temporary feedback)
- Not a form validation message (use Field/FieldError)

---

## 2. Architectural Responsibility (Contract)

### Responsibility

The Alert UI component:
- **Composes** `AlertPrimitive`, `AlertTitle`, and `AlertDescription`
- **Declares** semantic variant via `data-variant` attribute (default, destructive)
- **Exposes** ergonomic API with `variant` prop
- **Emits** SSR-safe HTML with data-attributes for styling
- **Guarantees** accessible `role="alert"` for screen readers

### Non-Responsibility

The Alert UI component explicitly does NOT:
- ❌ Execute browser APIs (focus, animations, timers)
- ❌ Manage dismissible state (Alert is static; use Toast for dismissible)
- ❌ Apply CSS classes or inline styles
- ❌ Register event listeners
- ❌ Perform side effects or mutations

**Side effects are PROHIBITED.**  
Alert is purely presentational—state management happens elsewhere.

---

## 3. Position in CanonRS Ecosystem

The Alert participates in the canonical CanonRS flow:
```text
Page/Block (usage)
  ↓
UI Component (Alert) — semantic variants, composition
  ↓
Primitives (AlertPrimitive, AlertTitle, AlertDescription) — HTML + data-attributes
  ↓
SSR Render — static HTML emitted
  ↓
CSS — styling via [data-alert-*] selectors
  ↓
Browser — renders styled alert (no JS required)
```

**SSR Context:**
- Alert renders complete HTML structure on server
- No client-side JS required for display
- Accessible via `role="alert"` immediately

**Hydration:**
- Alert requires no hydration (no interactive behavior)
- CSS applies styling via data-attributes
- Works perfectly without JavaScript

---

## 4. Tokens Applied

The Alert UI component does **not directly apply tokens**—it delegates to CSS.  
The CSS layer (`style/ui/alert.css`) consumes the following token families:

### Layout Tokens
- `--space-md` (padding)
- `--space-sm` (gap between icon and content)
- `--space-xs` (gap between title and description)

### Typography Tokens
- `--font-size-sm` (base text size)
- `--font-weight-medium` (title weight)
- `--line-height-tight` (title line height)
- `--line-height-normal` (description line height)
- `--line-height-relaxed` (paragraph line height)

### Color Tokens
- `--color-bg-surface` (background)
- `--color-fg-default` (default text)
- `--color-fg-muted` (description text)
- `--color-border-muted` (border)
- `--color-danger-fg` (destructive text)
- `--color-danger-border` (destructive border)

### Border & Radius Tokens
- `--border-width-hairline` (border)
- `--radius-md` (border radius)

**Token Resolution:**  
UI component emits data-attributes → CSS applies tokens → Browser renders.

---

## 5. Technical Structure (How It Works)

### SSR Render Phase

The Alert component renders to static HTML:
```html
<div data-alert="" data-variant="destructive" role="alert">
  <svg>...</svg>
  <div data-alert-title="">Error</div>
  <div data-alert-description="">
    <p>Your session has expired. Please log in again.</p>
  </div>
</div>
```

**Key contracts:**
- `data-alert` marks root container
- `data-variant="default|destructive"` determines visual style
- `role="alert"` ensures screen reader accessibility
- Optional `<svg>` child triggers icon layout via `:has(> svg)` selector
- `data-alert-title` marks heading
- `data-alert-description` marks body content

### CSS Styling Phase

CSS selectors target data-attributes:
```css
[data-alert] {
  display: grid;
  grid-template-columns: 0 1fr; /* No icon by default */
}

[data-alert]:has(> svg) {
  grid-template-columns: calc(var(--space-md) * 1.5) 1fr; /* Icon column */
}

[data-alert][data-variant="destructive"] {
  color: var(--color-danger-fg);
  border-color: var(--color-danger-border);
}
```

**No classes. No inline styles. Only data-attributes.**

---

## 6. Execution Flow
```text
1. SSR Render (Server)
   ↓
   Alert component executes
   ↓
   Composes primitives with data-attributes
   ↓
   Emits static HTML with role="alert"

2. HTML Delivery (Network)
   ↓
   Browser receives HTML
   ↓
   Screen reader announces alert immediately

3. CSS Application (Client)
   ↓
   Browser applies alert.css styles
   ↓
   Grid layout adjusts based on icon presence
   ↓
   Variant colors applied

4. Final Render
   ↓
   User sees styled alert
   ↓
   No JavaScript required
```

**Critical:** Alert is fully functional without JavaScript.  
It's a pure presentational component relying on SSR + CSS.

---

## 7. Canonical Use Cases

### Error Message (Destructive Variant)
```rust
use dioxus::prelude::*;
use rs_design::ui::alert::*;

fn ErrorAlert() -> Element {
    rsx! {
        Alert {
            variant: AlertVariant::Destructive,
            
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                width: "16",
                height: "16",
                "viewBox": "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                "stroke-width": "2",
                path { d: "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z" }
                line { x1: "12", y1: "9", x2: "12", y2: "13" }
                line { x1: "12", y1: "17", x2: "12.01", y2: "17" }
            }
            
            AlertTitle { "Error" }
            AlertDescription {
                p { "Your session has expired. Please log in again." }
            }
        }
    }
}
```

### Informational Message (Default Variant)
```rust
Alert {
    variant: AlertVariant::Default,
    
    AlertTitle { "Heads up!" }
    AlertDescription {
        p { "You can add components to your app using the CLI." }
    }
}
```

### Alert Without Icon
```rust
Alert {
    variant: AlertVariant::Default,
    
    AlertTitle { "Update Available" }
    AlertDescription {
        p { "A new version of this application is available." }
        p { "Refresh the page to update." }
    }
}
```

---

## 8. Anti-Patterns (PROHIBITED)

### ❌ Anti-Pattern 1: Using Alert for Critical Blocking Actions
```rust
// WRONG — Alert is non-blocking, use AlertDialog
Alert {
    variant: AlertVariant::Destructive,
    AlertTitle { "Delete Account?" }
    AlertDescription { "This action cannot be undone." }
    // Missing: Action/Cancel buttons (Alert has no interactive behavior)
}
```

**Why it breaks:**
- Alert is presentational only
- Cannot block user workflow
- No built-in action buttons

**Correct approach:**  
Use `AlertDialog` for critical decisions requiring user action.

---

### ❌ Anti-Pattern 2: Making Alert Dismissible
```rust
// WRONG — Alert has no dismissible state
let mut visible = use_signal(|| true);

if visible() {
    Alert {
        // No close button supported
    }
}
```

**Why it breaks:**
- Alert is static content
- No state management built-in
- Dismissible feedback belongs in Toast

**Correct approach:**  
Use `Toast` for temporary, dismissible notifications.

---

### ❌ Anti-Pattern 3: Inline Styles or Classes
```rust
// WRONG — applying classes directly
rsx! {
    Alert {
        class: "bg-red-500 text-white", // FORBIDDEN
        style: "padding: 2rem", // FORBIDDEN
    }
}
```

**Why it breaks:**
- Primitives emit data-attributes only
- CSS layer owns all styling
- Breaks token system

**Correct approach:**  
Emit `data-variant` and let CSS handle styling.

---

### ❌ Anti-Pattern 4: Using Alert for Form Validation
```rust
// WRONG — Alert is not form-scoped
form {
    Alert {
        variant: AlertVariant::Destructive,
        AlertDescription { "Email is required" }
    }
    Input { /* email field */ }
}
```

**Why it breaks:**
- Alert is page-level feedback
- Not associated with specific field
- Confusing for screen readers

**Correct approach:**  
Use `FieldError` component for inline field validation.

---

## 9. SSR, Hydration, and Runtime

### SSR Impact

**Server-Side:**
- Alert renders complete HTML structure
- All `data-*` attributes present in initial HTML
- `role="alert"` ensures immediate screen reader announcement
- No JavaScript required for functionality

**Benefits:**
- SEO-friendly (content visible to crawlers)
- Instant accessibility (no hydration wait)
- Zero JavaScript bundle cost
- Works with JavaScript disabled

**Constraints:**
- Cannot use browser APIs during SSR
- Must emit pure HTML

---

### Hydration Process

Alert **does not require hydration** because it has no interactive behavior.

1. **HTML Delivery:** Browser receives static HTML
2. **CSS Application:** Browser applies `alert.css` styles
3. **Final Render:** Alert is fully functional

No JavaScript execution needed.

---

### Runtime Global Constraints

**No Runtime Constraints:**
- Alert has no runtime behavior
- No event listeners
- No state mutations
- No browser API usage

**AutoReload/Hot Reload:**
- Alert preserves across reloads (static HTML)
- No state to lose

---

## 10. Conformance Checklist

- [x] SSR-safe (no browser APIs)
- [x] No imperative JS (pure presentational)
- [x] Uses tokens (indirectly via CSS)
- [x] All tokens documented in section 4
- [x] Anti-patterns documented with explanations
- [x] Canon Rules cited in section 11
- [x] Execution flow documented
- [x] Use cases provided
- [x] Hydration contract explicit (none required)
- [x] Runtime constraints documented (none)
- [x] Accessible via `role="alert"`

---

## 11. Canon Rules Applied

### Canon Rules Applied

- **Canon Rule #107 — Primitives Are SSR-Safe Structural Components**  
  Alert composes SSR-safe primitives that emit only HTML and data-attributes, never browser-dependent logic.

- **Canon Rule #108 — UI Components Provide Ergonomic Composition**  
  Alert exists in the UI layer to provide semantic variants (`default`, `destructive`) and reduce boilerplate in application code.

- **Canon Rule #109 — State Lives in DOM, Not Memory**  
  Alert has no state—it's purely presentational. Visual state (variant) is declared via `data-variant` attribute.

- **Canon Rule #110 — CSS Targets Data-Attributes, Never Classes**  
  Alert styling uses `[data-alert-*]` selectors exclusively, ensuring token-based theming and avoiding utility class pollution.

- **Canon Rule #112 — Presentational Components Require No Hydration**  
  Alert is fully functional without JavaScript, relying on SSR and CSS alone. No hydration step needed.

---

**End of Documentation**
