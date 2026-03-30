# Canon Rule #78: Token Definition Completeness

**Status:** ENFORCED


**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** design-system
**Tags:** tokens, theming, css
**Language:** EN

---

**Intro:**
Partial use of design tokens creates inconsistent UI and breaks theming. Components must fully adopt all applicable tokens without hardcoded values.

**Problem:**
components use partial tokens and hardcoded values breaking consistency

**Solution:**
enforce 100 percent token usage for all applicable properties

**Signals:**
- hardcoded values
- theme inconsistencies
- partial token usage

**Search Intent:**
how to enforce design token usage in components

**Keywords:**
design token completeness, no hardcoded css values, token based theming, css variables system design

---

---

## Principle

Every UI component MUST use **100% of applicable canonical and contextual tokens**.

Using only a subset of tokens creates **inconsistent visual systems**, breaks **theme inheritance**, and violates **Canon's deterministic design contract**.

If a token exists and applies to your component, it MUST be used.

---

## Token Categories

### Canonical Tokens (Global)

**Always applicable to all components:**
```css
/* Core */
space.xs/sm/md/lg/xl
radius.sm/md/lg
shadow.sm/md/lg
border.width.hairline/thin
z.base/popover/dropdown/modal

/* Color */
color.bg.surface/muted/elevated
color.fg.default/muted/inverted
color.border.default/muted
color.primary.bg/fg/border
color.danger/success/warning.bg/fg

/* Typography */
font.family.sans/mono
font.size.xs/sm/md/lg/xl
font.weight.regular/medium/semibold
line.height.tight/normal/relaxed

/* Motion */
motion.duration.fast/normal
motion.ease.standard/emphasized

/* State */
state.hover.opacity
state.active.opacity
state.disabled.opacity
state.focus.ring
```

### Contextual Tokens (Family-Specific)

**Family D (Navigation & Structure):**
```css
nav.item.height
nav.indicator.thickness
sidebar.width
pagination.size
```

**Family B (Forms & Input):**
```css
control.height.sm/md/lg
input.padding
field.gap
```

*(Other families: A, C, E, F)*

---

## Forbidden Patterns
```rust
// ❌ INCOMPLETE TOKEN USAGE

#[component]
pub fn Sidebar(children: Children) -> impl IntoView {
    view! {
        <div style="
            padding: var(--space-lg);        // ✓ Used
            background: var(--color-bg-surface);  // ✓ Used
            width: 16rem;                    // ❌ Hardcoded, should use --sidebar-width
        ">
            {children()}
        </div>
    }
}
```

### Why This is Forbidden

1. **Breaks Theme Consistency**
   - Hardcoded `16rem` doesn't respond to token changes
   - Different components use different widths
   - Theme switch doesn't affect this value

2. **Prevents Customization**
   - Enterprise customers cannot override
   - Design iterations require code changes
   - No central control point

3. **Violates Canon Contract**
   - Tokens exist to be used
   - Selective usage = partial system
   - Partial system = not a system

---

## Mandatory Token Application

### Sidebar Component (Family D)
```rust
// ✅ 100% TOKEN USAGE

#[component]
pub fn Sidebar(children: Children) -> impl IntoView {
    let ctx = expect_context::<SidebarContext>();
    
    view! {
        <SidebarPrimitive>
            <div 
                class="sidebar-root"
                data-open=move || if ctx.open.get() { "true" } else { "false" }
                style="
                    /* Core */
                    padding: var(--space-lg);
                    border-radius: var(--radius-md);
                    box-shadow: var(--shadow-md);
                    z-index: var(--z-base);
                    
                    /* Color */
                    background: var(--color-bg-surface);
                    border-right: var(--border-width-hairline) solid var(--color-border-default);
                    color: var(--color-fg-default);
                    
                    /* Typography */
                    font-family: var(--font-family-sans);
                    font-size: var(--font-size-sm);
                    font-weight: var(--font-weight-regular);
                    line-height: var(--line-height-normal);
                    
                    /* Motion */
                    transition-duration: var(--motion-duration-normal);
                    transition-timing-function: var(--motion-ease-standard);
                    
                    /* Family D */
                    width: var(--sidebar-width);
                "
            >
                {children()}
            </div>
        </SidebarPrimitive>
    }
}
```

**Tokens Used:** 16/16 applicable ✓

---

## Token Applicability Matrix

| Component Type | Canonical Tokens | Family Tokens | Total Required |
|----------------|------------------|---------------|----------------|
| Sidebar | Core, Color, Typography, Motion, State | Family D | ~18 tokens |
| Button | Core, Color, Typography, Motion, State | Family A | ~15 tokens |
| Input | Core, Color, Typography, State | Family B | ~12 tokens |
| Card | Core, Color, Typography | - | ~10 tokens |
| Modal | Core, Color, Motion, State | Family E | ~14 tokens |

---

## Token Definition Location

### `:root` Declaration (Mandatory)
```css
/* canonrs.css or tokens.css */
:root {
  /* Family D Tokens */
  --sidebar-width: 16rem;
  --nav-item-height: 2.5rem;
  --nav-indicator-thickness: 2px;
  
  /* Canonical Tokens */
  --space-lg: 1.5rem;
  --color-bg-surface: hsl(0 0% 100%);
  --font-family-sans: system-ui, -apple-system, sans-serif;
  --motion-duration-normal: 300ms;
  /* ... all other tokens */
}
```

**If token is not in `:root`, it does not exist.**

---

## Enforcement Checklist

### Component Review

- [ ] All applicable canonical tokens used
- [ ] All applicable family tokens used
- [ ] Zero hardcoded values (colors, sizes, timing)
- [ ] All tokens exist in `:root`
- [ ] Token values follow naming convention

### Token Audit
```bash
# Find hardcoded values
grep -r "16rem\|300ms\|#fff\|rgba(" src/ui/*.rs

# Should return zero results
```

---

## Real World Example: Before/After

### Before (Violates Rule)
```rust
// ❌ Partial token usage
<div style="
    width: 16rem;                          // ❌ Hardcoded
    padding: var(--space-lg);              // ✓
    background: white;                     // ❌ Hardcoded
    font-size: 14px;                       // ❌ Hardcoded
    transition: all 300ms ease;            // ❌ Hardcoded
">
```

**Problems:**
- 5 properties, 2 use tokens (40% compliant)
- Theme switch won't affect hardcoded values
- Customization requires code changes

### After (Compliant)
```rust
// ✅ 100% token usage
<div style="
    width: var(--sidebar-width);           // ✓
    padding: var(--space-lg);              // ✓
    background: var(--color-bg-surface);   // ✓
    font-size: var(--font-size-sm);        // ✓
    transition-duration: var(--motion-duration-normal);  // ✓
    transition-timing-function: var(--motion-ease-standard);  // ✓
">
```

**Result:** 6/6 properties use tokens (100% compliant)

---

## Missing Token Detection

### Lint Rule (Future)
```rust
// Pseudo-code for lint
fn check_token_usage(component: &Component) -> Result<()> {
    let hardcoded = find_hardcoded_values(component);
    let applicable_tokens = get_applicable_tokens(component.family);
    let used_tokens = find_used_tokens(component);
    
    if used_tokens.len() < applicable_tokens.len() {
        return Err("Incomplete token usage");
    }
    
    if !hardcoded.is_empty() {
        return Err(format!("Hardcoded values found: {:?}", hardcoded));
    }
    
    Ok(())
}
```

---

## Token Naming Convention
```
{category}.{subcategory}.{variant}

Examples:
color.bg.surface       // Category: color, Sub: bg, Variant: surface
space.lg               // Category: space, Variant: lg (no sub)
motion.duration.fast   // Category: motion, Sub: duration, Variant: fast
sidebar.width          // Category: sidebar, Variant: width (Family D)
```

---

## Exceptions

**Allowed hardcoded values (ONLY):**

1. **Layout primitives:** `display: flex`, `position: relative`
2. **Functional values:** `overflow: hidden`, `cursor: pointer`
3. **Resets:** `margin: 0`, `padding: 0`

**NOT allowed:**
- Any dimension (width, height, padding, margin with value)
- Any color (hex, rgb, named colors)
- Any timing (ms, s)
- Any typography (px, rem font sizes)

---

## Violation Consequences

1. **Immediate:** Visual inconsistency across components
2. **Short-term:** Theme switching partially broken
3. **Long-term:** Design system becomes patchwork
4. **Enterprise:** Cannot pass design audit

---

## Canonical Justification

A token that exists but is not used is **technical debt**.

A component that doesn't use tokens is **not part of the system**.

**Canon mandates:** 100% or 0%. Partial usage is worse than no usage.

---

## Canon References

- Canon Rule #7 — Token Governance
- Canon Rule #21 — Canonical Color Tokens
- Canon Rule #62 — Single Source of Truth Tokens
- Canon Rule #75 — Primitive CSS Prohibition

---

## Related Symptoms

If you see:
- Hardcoded `16rem`, `300ms`, `#ffffff` in component styles
- Theme switch doesn't affect all components equally
- Inconsistent spacing/colors across UI
- `var(--token-name)` returns empty

→ **This rule is violated.**

Go to: **SYMPTOMS.md → TOKEN COMPLETENESS VIOLATIONS**