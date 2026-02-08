---
component: Badge
layer: UI
status: Stable
since: v1.0
last_review: 2025-01-18
ownership: canonrs
keywords:
  - design system
  - dioxus
  - ssr
  - badge
  - status
  - label
  - pill
path_primitive: /opt/docker/monorepo/packages-rust/rs-design/src/primitives/badge.rs
path_ui: /opt/docker/monorepo/packages-rust/rs-design/src/ui/badge/
---

# Badge

## 1. Conceptual Introduction

The Badge is a **UI component** that displays compact status indicators, labels, or counts. It serves as the canonical enterprise solution for status pills, notification counters, category tags, and inline metadata that requires visual prominence without interaction.

The Badge exists in the **UI layer** because it provides:
- Semantic variants (default, secondary, destructive, outline, warning, success)
- Interactive state support for clickable badges
- Compact pill-shaped feedback
- Composition of primitives without business logic

**What it is NOT:**
- Not a primitive (it composes primitives)
- Not a button (use Button for primary actions; Badge can be interactive via `data-interactive`)
- Not a chip with close action (use Chip for removable items)
- Not a full alert (use Alert for expanded feedback)

---

## 2. Architectural Responsibility (Contract)

### Responsibility

The Badge UI component:
- **Composes** `BadgePrimitive`
- **Declares** semantic variant via `data-variant` attribute
- **Declares** interactive state via `data-interactive` attribute
- **Exposes** ergonomic API with `variant` and `interactive` props
- **Emits** SSR-safe HTML with data-attributes for styling
- **Guarantees** compact, pill-shaped container

### Non-Responsibility

The Badge UI component explicitly does NOT:
- ❌ Execute browser APIs (click handlers, animations)
- ❌ Manage dismissible state (Badge is static; use Chip for removable)
- ❌ Apply CSS classes or inline styles
- ❌ Register event listeners (interactivity declared via `data-interactive`)
- ❌ Perform side effects or mutations

**Side effects are PROHIBITED.**  
Badge is presentational—interactivity is declared, not executed.

---

## 3. Position in CanonRS Ecosystem

The Badge participates in the canonical CanonRS flow:
```text
Page/Block (usage)
  ↓
UI Component (Badge) — semantic variants, interactive state
  ↓
Primitive (BadgePrimitive) — HTML + data-attributes
  ↓
SSR Render — static HTML emitted
  ↓
CSS — styling via [data-badge] selectors
  ↓
Browser — renders styled badge
```

**SSR Context:**
- Badge renders complete HTML structure on server
- No client-side JS required for display
- Interactive state declared via `data-interactive` (Shell Runtime handles clicks if needed)

**Hydration:**
- Badge requires no hydration for static display
- If `interactive="true"`, Shell Runtime may attach click handlers
- Works perfectly without JavaScript for non-interactive badges

---

## 4. Tokens Applied

The Badge UI component does **not directly apply tokens**—it delegates to CSS.  
The CSS layer (`style/ui/badge.css`) consumes the following token families:

### Status & Feedback Family (E)
Badge belongs to **Family E — Status & Feedback**.

### Layout Tokens
- `--space-xs` (internal gap for icon + text)
- `--space-sm` (horizontal padding)

### Typography Tokens
- `--font-family-sans`
- `--font-size-xs`
- `--font-weight-medium`
- `--line-height-tight`

### Color Tokens
- `--color-primary-bg`, `--color-primary-fg` (default variant)
- `--color-bg-muted`, `--color-fg-default` (secondary variant)
- `--color-danger-bg`, `--color-danger-fg` (destructive variant)
- `--color-border-default` (outline variant)
- `--color-warning-bg`, `--color-warning-fg` (warning variant)
- `--color-success-bg`, `--color-success-fg` (success variant)

### Border & Radius Tokens
- `--border-width-hairline`
- `--radius-sm` (pill shape)

### State Tokens
- `--state-hover-opacity` (interactive hover)
- `--state-active-opacity` (interactive active)

**Token Resolution:**  
UI component emits `data-variant` and `data-interactive` → CSS applies tokens → Browser renders.

---

## 5. Technical Structure (How It Works)

### SSR Render Phase

The Badge component renders to static HTML:
```html
<span data-badge="" data-variant="success" data-interactive="false">
  Active
</span>
```

**Key contracts:**
- `data-badge` marks badge container
- `data-variant="default|secondary|destructive|outline|warning|success"` determines visual style
- `data-interactive="true|false"` enables hover/active states

### CSS Styling Phase

CSS selectors target data-attributes:
```css
[data-badge] {
  display: inline-flex;
  align-items: center;
  padding: calc(var(--space-xs) / 2) var(--space-sm);
  border-radius: var(--radius-sm);
  font-size: var(--font-size-xs);
}

[data-badge][data-variant="success"] {
  background: var(--color-success-bg);
  color: var(--color-success-fg);
}

[data-badge][data-interactive="true"]:hover {
  opacity: calc(1 - var(--state-hover-opacity));
  cursor: pointer;
}
```

**No classes. No inline styles. Only data-attributes.**

---

## 6. Execution Flow
```text
1. SSR Render (Server)
   ↓
   Badge component executes
   ↓
   Emits span with data-badge, data-variant, data-interactive
   ↓
   Static HTML

2. HTML Delivery (Network)
   ↓
   Browser receives HTML
   ↓
   CSS applies immediately

3. CSS Application (Client)
   ↓
   Browser applies badge.css
   ↓
   Variant colors applied
   ↓
   Interactive states enabled if data-interactive="true"

4. Final Render
   ↓
   User sees styled badge
   ↓
   Hover/active states work if interactive
   ↓
   No JavaScript required for display
```

**Critical:** Badge is fully functional without JavaScript for static display.  
Interactive badges may have Shell Runtime event handlers attached.

---

## 7. Canonical Use Cases

### Status Indicator (Success)
```rust
use dioxus::prelude::*;
use rs_design::ui::badge::*;

fn StatusBadge() -> Element {
    rsx! {
        Badge {
            variant: BadgeVariant::Success,
            "Active"
        }
    }
}
```

### Notification Count (Destructive)
```rust
Badge {
    variant: BadgeVariant::Destructive,
    "3"
}
```

### Category Tag (Outline)
```rust
Badge {
    variant: BadgeVariant::Outline,
    "Enterprise"
}
```

### Interactive Badge (Clickable)
```rust
Badge {
    variant: BadgeVariant::Default,
    interactive: true,
    onclick: move |_| { /* handle click */ },
    "Filter"
}
```

### Warning Indicator
```rust
Badge {
    variant: BadgeVariant::Warning,
    "Pending"
}
```

### Secondary Label
```rust
Badge {
    variant: BadgeVariant::Secondary,
    "Beta"
}
```

---

## 8. Anti-Patterns (PROHIBITED)

### ❌ Anti-Pattern 1: Using Badge for Primary Actions
```rust
// WRONG — Badge is for status/labels, not primary CTAs
Badge {
    variant: BadgeVariant::Default,
    onclick: |_| { save_form(); }, // Should be Button
    "Save"
}
```

**Why it breaks:**
- Badge is for metadata, not actions
- Primary actions need Button prominence
- Confusing UX (badges are typically non-interactive)

**Correct approach:**  
Use `Button` for primary actions. Badge is for status/labels.

---

### ❌ Anti-Pattern 2: Long Text in Badge
```rust
// WRONG — Badge is for compact labels
Badge {
    variant: BadgeVariant::Default,
    "This is a very long description that should not be in a badge"
}
```

**Why it breaks:**
- Badge is designed for 1-3 words max
- Long text breaks pill shape
- Use Alert or inline text instead

**Correct approach:**  
Keep badge text under 15 characters. Use Alert for longer feedback.

---

### ❌ Anti-Pattern 3: Nesting Interactive Elements
```rust
// WRONG — nesting button inside badge
Badge {
    variant: BadgeVariant::Default,
    button { onclick: |_| {}, "X" } // FORBIDDEN
}
```

**Why it breaks:**
- Nested interactive elements are inaccessible
- Confusing focus behavior
- Invalid HTML

**Correct approach:**  
If badge needs close action, use `Chip` component instead.

---

### ❌ Anti-Pattern 4: Using Badge as Notification Toast
```rust
// WRONG — Badge is inline, not overlay
div { style: "position: fixed; top: 1rem; right: 1rem",
    Badge {
        variant: BadgeVariant::Success,
        "Saved!"
    }
}
```

**Why it breaks:**
- Badge is inline content, not overlay
- No animation/dismissal support
- Use Toast for temporary notifications

**Correct approach:**  
Use `Toast` component for temporary overlay notifications.

---

## 9. SSR, Hydration, and Runtime

### SSR Impact

**Server-Side:**
- Badge renders complete HTML structure
- All `data-*` attributes present in initial HTML
- No JavaScript required for display

**Benefits:**
- SEO-friendly (content visible to crawlers)
- Instant display (no layout shift)
- Zero JavaScript bundle cost for static badges
- Works with JavaScript disabled

**Constraints:**
- Cannot use browser APIs during SSR
- Must emit pure HTML

---

### Hydration Process

Badge **does not require hydration** for static display.

1. **HTML Delivery:** Browser receives static HTML
2. **CSS Application:** Browser applies `badge.css`
3. **Interactive State:** If `data-interactive="true"`, Shell Runtime may attach handlers
4. **Final Render:** Badge is fully functional

For non-interactive badges, no JavaScript execution needed.

---

### Runtime Global Constraints

**Interactive Badges:**
- If `data-interactive="true"`, Shell Runtime may attach click handlers
- Hover/active states work via CSS (no JS needed)

**Non-Interactive Badges:**
- No runtime behavior
- No event listeners
- Pure CSS presentation

**AutoReload/Hot Reload:**
- Badge preserves across reloads (static HTML)
- No state to lose

---

## 10. Conformance Checklist

- [x] SSR-safe (no browser APIs in component)
- [x] No imperative JS (pure presentational)
- [x] Uses tokens (indirectly via CSS)
- [x] All tokens documented in section 4
- [x] Anti-patterns documented with explanations
- [x] Canon Rules cited in section 11
- [x] Execution flow documented
- [x] Use cases provided
- [x] Hydration contract explicit (minimal/optional)
- [x] Runtime constraints documented
- [x] Belongs to Family E (Status & Feedback)

---

## 11. Canon Rules Applied

### Canon Rules Applied

- **Canon Rule #107 — Primitives Are SSR-Safe Structural Components**  
  Badge composes SSR-safe primitives that emit only HTML and data-attributes, never browser-dependent logic.

- **Canon Rule #108 — UI Components Provide Ergonomic Composition**  
  Badge exists in the UI layer to provide semantic variants (`default`, `secondary`, `destructive`, `outline`, `warning`, `success`) and interactive state support.

- **Canon Rule #109 — State Lives in DOM, Not Memory**  
  Badge has minimal state—variant and interactivity are declared via `data-variant` and `data-interactive` attributes.

- **Canon Rule #110 — CSS Targets Data-Attributes, Never Classes**  
  Badge styling uses `[data-badge]` selectors exclusively, ensuring token-based theming and avoiding utility class pollution.

- **Canon Rule #112 — Presentational Components Require No Hydration**  
  Badge is fully functional without JavaScript for static display. Interactive badges may have optional runtime handlers.

- **Canon Rule #114 — Badge Belongs to Family E (Status & Feedback)**  
  Badge provides compact status indicators and labels, never primary actions or layout.

---

**End of Documentation**
