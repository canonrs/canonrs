# Canon Rule #07: Theme and Token Governance

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** design-system, tokens, theming
**Version:** 1.0.0
**Date:** 2025-01-16

---



## Critical Principle
**Tokens are not owned by Tailwind. Theme is not a UI concern. Apps never import design system internals.**

Violation = Coupling / Build breaks / Design drift / Maintenance hell

---

## The Common Monorepo Mistake

### What Developers Try (Breaks At Scale)
```css
/* App imports design system directly */
@import "../../packages-rust/rs-design/style/tokens.css";
```

**Why this fails:**
- Path fragility (breaks on refactor)
- Couples app to internal structure
- Multiple apps = multiple import points
- Build cache invalidation issues
- Can't version tokens independently

---

## Canon Architecture

### Three-Layer Token Pipeline
```
┌─────────────────────────────────────┐
│   rs-design (Source of Truth)      │
│   ├── style/tokens.css              │  ← AUTHORITATIVE
│   └── src/ui/*.rs                   │  ← Consumes tokens
└─────────────────────────────────────┘
              ↓ SYNC (build-time)
┌─────────────────────────────────────┐
│   rs-tailwind (Distribution Bridge) │
│   └── tokens/theme.css              │  ← COPY of tokens
└─────────────────────────────────────┘
              ↓ IMPORT (npm package)
┌─────────────────────────────────────┐
│   Apps (Consumers)                  │
│   @import "@monorepo/rs-tailwind"   │  ← Clean dependency
└─────────────────────────────────────┘
```

**Golden Rule:**
> Apps import from `rs-tailwind`, never from `rs-design`.

---

## Responsibility Matrix

| Layer | Decides Design? | Applies Styling? | Distributes Tokens? |
|-------|----------------|------------------|---------------------|
| **rs-design** | ✅ YES | ❌ NO | ❌ NO |
| **rs-tailwind** | ❌ NO | ❌ NO | ✅ YES |
| **App** | ❌ NO | ✅ YES | ❌ NO |

### rs-design (Design System)
**Purpose:** Define design tokens, export UI components
```css
/* style/tokens.css - SOURCE OF TRUTH */
@theme inline {
  --color-background: 0 0% 100%;
  --color-primary: 38 92% 50%;
}

.dark {
  --color-background: 0 0% 9%;
}
```

**Rules:**
- ✅ Defines all semantic tokens
- ✅ Exports UI components
- ❌ Does NOT distribute tokens to apps
- ❌ Does NOT know about app builds

### rs-tailwind (Distribution Bridge)
**Purpose:** Package tokens for Tailwind consumption
```
rs-tailwind/
├── package.json          ← npm package
├── tokens/
│   └── theme.css         ← SYNCED from rs-design
└── preset/
    └── index.js          ← Tailwind config
```

**Rules:**
- ✅ Copies tokens from rs-design (build step)
- ✅ Exposes as npm package
- ❌ Does NOT create design decisions
- ❌ Does NOT modify token values

### Apps (Consumers)
**Purpose:** Consume tokens via Tailwind
```css
/* app/style/tailwind.css */
@import "tailwindcss";
@import "@monorepo/rs-tailwind/tokens";

@layer base {
  body {
    @apply bg-background text-foreground;
  }
}
```

**Rules:**
- ✅ Imports from `@monorepo/rs-tailwind`
- ✅ Uses utility classes (`bg-background`)
- ❌ Does NOT import from `rs-design`
- ❌ Does NOT hardcode colors (`bg-slate-100`)

---

## Theme System (Dark/Light)

### Architecture

**State Management:**
```rust
// providers/theme_provider.rs
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let theme = RwSignal::new(Theme::Light);
    
    // Apply class to <html>
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        if let Some(html) = document().document_element() {
            match theme.get() {
                Theme::Light => { html.class_list().remove_1("dark"); }
                Theme::Dark => { html.class_list().add_1("dark"); }
            }
        }
    });
    
    provide_context(theme);
    view! { {children()} }
}
```

**CSS Resolution:**
```css
/* tokens.css */
:root {
  --color-background: 0 0% 100%;  /* Light */
}

.dark {
  --color-background: 0 0% 9%;    /* Dark */
}
```

### Separation of Concerns

| Concern | Owner | Mechanism |
|---------|-------|-----------|
| **State** (light/dark) | ThemeProvider | `RwSignal<Theme>` + context |
| **DOM** (`.dark` class) | ThemeProvider | Effect + `html.classList` |
| **Appearance** | CSS | `.dark { --color-*: ... }` |
| **Consumption** | Components | `bg-[hsl(var(--color-background))]` |

**Critical Rule:**
> ThemeProvider manipulates ONLY `html.classList`. Never touches CSS/tokens.

---

## Tailwind's Correct Role

### ✅ Tailwind SHOULD:
- Consume semantic tokens (`bg-background`)
- Provide utility ergonomics
- Generate responsive variants
- Optimize CSS output

### ❌ Tailwind SHOULD NOT:
- Define semantic tokens
- Own color palette
- Decide dark mode values
- Be source of truth

### Example (Correct)
```css
/* App uses semantic tokens */
<div class="bg-background text-foreground" />
```
```css
/* Tailwind resolves to */
.bg-background { background-color: hsl(var(--color-background)); }
```

**Why correct:**
- Token value lives in CSS variables
- Tailwind is just utility generator
- Theme change = CSS update, zero build
- Components unaware of theme

---

## Anti-Patterns (Real Production Issues)

### Anti-Pattern 1: Direct rs-design Import
```css
/* ❌ WRONG */
@import "../../packages-rust/rs-design/style/tokens.css";
```

**Problems:**
- Build fails on path changes
- Couples to internal structure
- Can't version independently

**Fix:**
```css
/* ✅ CORRECT */
@import "@monorepo/rs-tailwind/tokens";
```

### Anti-Pattern 2: Hardcoded Colors
```rust
// ❌ WRONG
class="bg-slate-100 text-blue-600"
```

**Problems:**
- Theme ignored
- Dark mode broken
- Design drift

**Fix:**
```rust
// ✅ CORRECT
class="bg-background text-primary"
```

### Anti-Pattern 3: ThemeProvider with CSS
```rust
// ❌ WRONG
#[component]
pub fn ThemeProvider() -> impl IntoView {
    Effect::new(|| {
        document().body().style().set_property(
            "background", 
            if theme == Dark { "#000" } else { "#fff" }
        );
    });
}
```

**Problems:**
- Bypasses CSS variables
- JS-controlled styling
- SSR mismatch

**Fix:**
```rust
// ✅ CORRECT
Effect::new(|| {
    html.class_list().toggle_class("dark", theme == Dark);
});
```

---

## Token Sync Workflow

### Manual (Current)
```bash
# When tokens change in rs-design:
cp packages-rust/rs-design/style/tokens.css \
   packages-rust/rs-tailwind/tokens/theme.css
```

### Automated (Recommended)
```bash
# Add to package.json scripts
{
  "scripts": {
    "sync-tokens": "cp ../rs-design/style/tokens.css ./tokens/theme.css"
  }
}
```

### Build Integration (Production)
```bash
# In CI/CD before build
npm run sync-tokens
cargo leptos build --release
```

---

## Compliance Checklist

Before merging theme/token changes:

- [ ] Tokens defined in `rs-design/style/tokens.css`
- [ ] Synced to `rs-tailwind/tokens/theme.css`
- [ ] App imports `@monorepo/rs-tailwind/tokens`
- [ ] No hardcoded colors (`bg-slate-*`, `text-blue-*`)
- [ ] ThemeProvider only touches `html.classList`
- [ ] Dark mode works without component changes
- [ ] No relative imports from rs-design
- [ ] Tailwind uses semantic tokens only

---

## Testing Theme Governance

### Audit Commands
```bash
# Find violations: hardcoded colors
rg "bg-(slate|blue|red|green)" apps/ --type rust

# Find violations: direct rs-design imports
rg "packages-rust/rs-design" apps/ --type css

# Find violations: inline styles
rg 'style="' packages-rust/rs-design/src --type rust
```

### Runtime Check
```javascript
// Console test after theme toggle
document.documentElement.classList.contains('dark') // true
getComputedStyle(document.body).getPropertyValue('--color-background') // "0 0% 9%"
```

---

## Migration Guide

### From Anti-Pattern to Canon

**Before (broken):**
```css
/* app/style/tailwind.css */
@import "../../packages-rust/rs-design/style/tokens.css";
```
```rust
// app/src/component.rs
class="bg-slate-100 hover:bg-slate-200"
```

**After (Canon):**
```css
/* app/style/tailwind.css */
@import "@monorepo/rs-tailwind/tokens";
```
```rust
// app/src/component.rs
class="bg-background hover:bg-muted"
```

**Steps:**
1. Sync tokens to rs-tailwind
2. Change app import
3. Replace hardcoded colors with semantic tokens
4. Verify dark mode works
5. Remove old imports

---

## Why This Matters (Business Case)

### Without Governance
- 3 apps = 3 token copies
- Theme update = 3 manual syncs
- Drift between products
- Designer can't enforce consistency
- Dark mode = per-app implementation

### With Governance
- 1 source of truth
- Theme update = 1 file change
- Guaranteed consistency
- Designer controls all products
- Dark mode = CSS toggle

**ROI:**
- Design iteration: 10x faster
- Maintenance cost: 5x lower
- Brand consistency: enforced
- Developer onboarding: 2x faster

---

## Related Canon Rules

- **Rule #6 (Visual State):** Components expose state, CSS decides appearance
- **Rule #5 (SSR Effects):** ThemeProvider uses `#[cfg(target_arch = "wasm32")]`
- **ARCHITECTURE:** UI layer consumes tokens, never defines

---

## Quick Reference
```rust
// ✅ CANONICAL PATTERN

// 1. ThemeProvider (providers layer)
#[component]
pub fn ThemeProvider(children: Children) -> impl IntoView {
    let theme = RwSignal::new(Theme::Light);
    
    #[cfg(target_arch = "wasm32")]
    Effect::new(move |_| {
        html.class_list().toggle_class("dark", theme.get() == Theme::Dark);
    });
    
    provide_context(theme);
    view! { {children()} }
}

// 2. ThemeToggle (UI layer)
#[component]
pub fn ThemeToggle() -> impl IntoView {
    let theme = expect_context::<RwSignal<Theme>>();
    view! {
        <button on:click=move |_| theme.update(|t| t.toggle())>
            {move || if theme.get() == Theme::Dark { "🌙" } else { "☀️" }}
        </button>
    }
}

// 3. Component (uses semantic tokens)
#[component]
pub fn Card() -> impl IntoView {
    view! {
        <div class="bg-card text-card-foreground">
            // Never knows about dark/light
        </div>
    }
}
```

---

**Last Updated:** 2025-12-28 | Canon v1.3
**Dependencies:** ARCHITECTURE.md, Rule #6 (Visual State)

---

## Summary

**Three Laws of Token Governance:**

1. **Single Source:** Tokens live ONLY in `rs-design/style/tokens.css`
2. **Clean Distribution:** Apps import from `rs-tailwind`, never `rs-design`
3. **CSS Decides:** Theme is CSS classes, not JS logic

Violate these = technical debt.
Follow these = enterprise scale.

---

## Normative Status

**This is a normative Canon document.**

- Violations MAY block PRs
- Design decisions MUST reference this document
- Token changes MUST follow sync workflow
- Apps MUST NOT bypass rs-tailwind bridge

Exceptions require explicit approval from design system maintainers.

---

**Canon Rule #7** | Enterprise Foundation | Normative Document
