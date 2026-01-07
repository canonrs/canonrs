# Canon Rule #6 - # Visual State Declaration (Canon Rule #6)

## Critical Rule
**UI components expose semantic state via data-attributes. CSS + tokens decide visual representation.**

Violation = Theme breaks / Dark mode fails / Design debt / IA generates brittle code

---

## Philosophy

State ≠ Style

Components declare **what they are** (selected, disabled, active).
CSS decides **how they look** (colors, spacing, effects).

This separation enables:
- Theme switching without component changes
- Dark mode via CSS variables only
- Design system evolution without refactoring
- AI-friendly predictable patterns

---

## ❌ WRONG Patterns

### Anti-Pattern #1: Hardcoded Colors
```rust
// ❌ FORBIDDEN - Concrete colors in UI
view! {
    <tr
        class:bg-slate-100=move || selected.get()
        class:hover:bg-slate-200=move || selected.get()
    >
}
```

**Why wrong:**
- Breaks themes (can't override slate)
- Dark mode requires component rewrite
- No semantic meaning
- Design system can't evolve

### Anti-Pattern #2: Inline Style State
```rust
// ❌ FORBIDDEN - JS-controlled styling
view! {
    <tr
        style=move || if selected.get() {
            "background: #f0f0f0"
        } else {
            ""
        }
    >
}
```

**Why wrong:**
- CSS variables ignored
- Theme system bypassed
- Imperative, not declarative
- SSR mismatch risk

### Anti-Pattern #3: Dynamic Class Strings
```rust
// ❌ FORBIDDEN - String concatenation
view! {
    <tr
        class=move || format!(
            "bg-{} hover:bg-{}",
            if selected.get() { "blue-100" } else { "white" },
            if selected.get() { "blue-200" } else { "gray-50" }
        )
    >
}
```

**Why wrong:**
- Tailwind may not generate classes
- Untrackable in build
- String-based logic is fragile
- No type safety

---

## ✅ CORRECT Pattern

### Canonical Implementation
```rust
use leptos::prelude::*;

#[component]
pub fn TableRowSelectable(
    #[prop(into)] selected: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <tr
            data-selected=move || selected.get()
            class="
                border-b
                transition-colors duration-150
                bg-[hsl(var(--color-muted)/0.4)]
                hover:bg-[hsl(var(--color-muted)/0.6)]
                data-[selected=true]:bg-[hsl(var(--color-muted)/0.8)]
                data-[selected=true]:hover:bg-[hsl(var(--color-muted)/1)]
            "
        >
            {children()}
        </tr>
    }
}
```

### Why Correct:
✅ `data-selected` = semantic state (not visual)
✅ `var(--color-muted)` = theme-controlled token
✅ CSS decides all visuals
✅ Works with any theme/density/mode
✅ Declarative, SSR-safe
✅ AI can generate this pattern

---

## State → Attribute Mapping

| Component State | data-attribute | CSS Selector |
|----------------|----------------|--------------|
| `selected: bool` | `data-selected="true"` | `data-[selected=true]:` |
| `disabled: bool` | `data-disabled="true"` | `data-[disabled=true]:` |
| `active: bool` | `data-active="true"` | `data-[active=true]:` |
| `variant: enum` | `data-variant="primary"` | `data-[variant=primary]:` |
| `size: enum` | `data-size="md"` | `data-[size=md]:` |

---

## Token Usage Patterns

### ✅ Semantic Tokens (ALWAYS)
```rust
// State variations via opacity
bg-[hsl(var(--color-muted)/0.4)]          // Base
hover:bg-[hsl(var(--color-muted)/0.6)]    // Hover
data-[selected=true]:bg-[hsl(var(--color-muted)/0.8)]  // Selected
data-[selected=true]:hover:bg-[hsl(var(--color-muted)/1)]  // Selected + Hover

// Border states
border-[hsl(var(--color-border))]
data-[error=true]:border-[hsl(var(--color-danger-border))]

// Text states
text-[hsl(var(--color-fg-default))]
data-[disabled=true]:text-[hsl(var(--color-fg-muted))]
data-[disabled=true]:opacity-[var(--state-disabled-opacity)]
```

### ❌ Concrete Colors (NEVER)
```rust
// ❌ FORBIDDEN
bg-slate-100
text-blue-600
border-red-500
hover:bg-gray-200
```

---

## Common Use Cases

### Interactive Lists
```rust
#[component]
pub fn ListItem(
    #[prop(into)] selected: Signal<bool>,
    #[prop(into)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <li
            data-selected=move || selected.get()
            data-disabled=move || disabled.get()
            class="
                p-[var(--space-md)]
                transition-colors
                hover:bg-[hsl(var(--color-muted)/0.5)]
                data-[selected=true]:bg-[hsl(var(--color-primary-bg)/0.1)]
                data-[disabled=true]:opacity-[var(--state-disabled-opacity)]
                data-[disabled=true]:cursor-not-allowed
            "
        >
}
```

### Tab Navigation
```rust
#[component]
pub fn TabButton(
    #[prop(into)] active: Signal<bool>,
) -> impl IntoView {
    view! {
        <button
            data-active=move || active.get()
            class="
                px-[var(--space-md)]
                py-[var(--space-sm)]
                border-b-2
                border-transparent
                transition-colors
                hover:border-[hsl(var(--color-border))]
                data-[active=true]:border-[hsl(var(--color-primary))]
                data-[active=true]:text-[hsl(var(--color-primary))]
            "
        >
}
```

### Form Controls
```rust
#[component]
pub fn InputField(
    #[prop(into)] error: Signal<bool>,
    #[prop(into)] disabled: Signal<bool>,
) -> impl IntoView {
    view! {
        <input
            data-error=move || error.get()
            data-disabled=move || disabled.get()
            disabled=move || disabled.get()
            class="
                h-[var(--size-control-md)]
                px-[var(--space-control-x)]
                border
                border-[hsl(var(--color-border))]
                rounded-[var(--radius-md)]
                focus:ring-1
                focus:ring-[hsl(var(--state-focus-ring))]
                data-[error=true]:border-[hsl(var(--color-danger-border))]
                data-[disabled=true]:opacity-[var(--state-disabled-opacity)]
            "
        />
}
```

---

## Decision Matrix

| Pattern | Allowed? | Use Case | Canonical Layer |
|---------|----------|----------|-----------------|
| `data-selected=signal` | ✅ YES | State exposure | UI Component |
| `var(--color-*)` | ✅ YES | Visual styling | UI Component |
| `bg-slate-100` | ❌ NO | Never | None |
| `style="background: ..."` | ❌ NO | Never | None |
| `class:bg-*=signal` | ⚠️ AVOID | Couples state+style | Use data-attr instead |
| Tailwind arbitrary values `[hsl(...)]` | ✅ YES | Token consumption | UI Component |

---

## Real Production Benefits

### Before Canon (Brittle)
```rust
// Component locked to light theme
view! {
    <tr class:bg-blue-50=selected class:hover:bg-blue-100=selected>
}
```

**Problems:**
- Dark mode: rewrite component
- Rebrand: find/replace all colors
- Density change: touch every component

### After Canon (Flexible)
```rust
// Component theme-agnostic
view! {
    <tr
        data-selected=selected
        class="bg-[hsl(var(--color-muted)/0.4)]
               data-[selected=true]:bg-[hsl(var(--color-muted)/0.8)]"
    >
}
```

**Benefits:**
- Dark mode: update `tokens.css` only
- Rebrand: change tokens, zero component edits
- Density: tokens adjust, components unaware

---

## 🧪 Testing Canon Compliance

### Audit Command
```bash
# Find violations in codebase
grep -r "class:bg-" src/ --include="*.rs"
grep -r "bg-slate\|bg-blue\|bg-red" src/ --include="*.rs"
grep -r 'style="' src/ --include="*.rs"
```

### Manual Check
```rust
// ❌ If you see this pattern, it's wrong
class:bg-{color}=signal
style=move || format!(...)
bg-{concrete-color}

// ✅ If you see this pattern, it's correct
data-{state}=signal
bg-[hsl(var(--color-{token}))]
```

---

## 📚 Related Canon Rules

- **Rule #1 (Types):** State belongs in Type 2 components
- **Rule #2 (Ownership):** `Signal<bool>` for reactive state
- **Rule #5 (SSR):** data-attributes are SSR-safe
- **ARCHITECTURE:** UI layer consumes tokens, never defines them

---

## 🔒 Canonical Checklist

Before merging any UI component:

- [ ] No hardcoded colors (`slate`, `blue`, `red`, etc.)
- [ ] State exposed via `data-*` attributes
- [ ] Visual styling uses `var(--color-*)` tokens only
- [ ] No `style=` with inline values
- [ ] No string concatenation for classes
- [ ] All hover/focus states via CSS selectors
- [ ] Works in light + dark mode (test both)

---

## ⚡ Quick Reference Card
```rust
// ✅ CANONICAL PATTERN (copy-paste ready)

use leptos::prelude::*;

#[component]
pub fn InteractiveElement(
    #[prop(into)] selected: Signal<bool>,
    #[prop(into)] disabled: Signal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <div
            data-selected=move || selected.get()
            data-disabled=move || disabled.get()
            class="
                transition-colors
                bg-[hsl(var(--color-background))]
                hover:bg-[hsl(var(--color-muted)/0.5)]
                data-[selected=true]:bg-[hsl(var(--color-muted)/0.8)]
                data-[disabled=true]:opacity-[var(--state-disabled-opacity)]
            "
        >
            {children()}
        </div>
    }
}

// ❌ ANTI-PATTERNS (never do this)

// Hardcoded colors
class:bg-slate-100=selected  // ❌

// Inline styles
style="background: #f0f0f0"  // ❌

// String concat
class=format!("bg-{}", color)  // ❌

// Concrete Tailwind
bg-blue-500 text-red-600  // ❌
```

---

**Status:** Production-Critical | Foundation Rule
**Last Updated:** 2025-12-28 | Canon v1.3
**Dependencies:** tokens.css, ARCHITECTURE.md, Rule #2 (Ownership)

---

## Summary

**One Rule to Remember:**
> Components expose **what** (state).
> CSS + tokens decide **how** (appearance).

Never mix them.
