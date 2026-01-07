# Canon Rule #23: State Tokens (Hover, Focus, Disabled, Pressed)

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-01

## Principle
Interactive states must use **consistent, theme-independent opacity values** rather than color variants. This ensures state feedback works across all themes without requiring theme authors to define state-specific colors.

## Canonical State Tokens

### Interaction States (CSS Variables)
```css
:root {
  /* Opacity-based states (theme-independent) */
  --state-hover-opacity: 0.9;
  --state-active-opacity: 0.8;
  --state-disabled-opacity: 0.5;
  
  /* Focus ring */
  --state-focus-ring: hsl(var(--color-ring));
  --border-width-focus: 2px;
}
```

### State Application Pattern

#### ✅ CORRECT: Opacity-Based
```rust
const BASE_CLASSES: &str = "\
    hover:opacity-[var(--state-hover-opacity)] \
    active:opacity-[var(--state-active-opacity)] \
    disabled:opacity-[var(--state-disabled-opacity)] \
    focus-visible:ring-[color:var(--state-focus-ring)]";
```

#### ❌ INCORRECT: Color Variants
```rust
// Don't create separate hover colors
const HOVER_CLASSES: &str = "\
    hover:bg-primary-hover \      // ❌ Requires theme definition
    hover:border-primary-hover";  // ❌ Not portable
```

## State Categories

### 1. Hover State
**Token:** `--state-hover-opacity: 0.9`

**Usage:**
```rust
"hover:opacity-[var(--state-hover-opacity)]"
```

**Rationale:** Reduces brightness by 10%, works on any color

### 2. Active/Pressed State
**Token:** `--state-active-opacity: 0.8`

**Usage:**
```rust
"active:opacity-[var(--state-active-opacity)]"
```

**Rationale:** Deeper press feedback than hover

### 3. Disabled State
**Token:** `--state-disabled-opacity: 0.5`

**Usage:**
```rust
"disabled:opacity-[var(--state-disabled-opacity)] \
 disabled:cursor-not-allowed \
 disabled:pointer-events-none"
```

**Rationale:** 50% opacity universally signals "not available"

### 4. Focus State
**Token:** `--state-focus-ring: hsl(var(--color-ring))`

**Usage:**
```rust
"focus-visible:outline-none \
 focus-visible:ring-2 \
 focus-visible:ring-[color:var(--state-focus-ring)] \
 focus-visible:ring-offset-2"
```

**Rationale:** Uses theme's ring color for brand consistency

### 5. Loading State
**Token:** `--state-loading-opacity: 0.7`

**Usage:**
```rust
"data-[loading=true]:opacity-[var(--state-loading-opacity)] \
 data-[loading=true]:cursor-wait"
```

## State Combinations

### Layered States
```rust
// Multiple states can stack
"hover:opacity-[var(--state-hover-opacity)] \
 focus-visible:ring-2 \
 disabled:opacity-[var(--state-disabled-opacity)]"
```

### Priority Order
1. `disabled` (highest - overrides all)
2. `active` / `pressed`
3. `focus-visible`
4. `hover` (lowest)

## Prohibited Patterns

### ❌ Don't Create State Color Variants
```css
/* FORBIDDEN */
--color-primary-hover: ...;
--color-primary-active: ...;
--color-primary-disabled: ...;
```

**Why:** Makes themes brittle and non-portable

### ❌ Don't Use Hardcoded Opacity
```rust
// FORBIDDEN
"hover:opacity-90"  // Use var(--state-hover-opacity)
"disabled:opacity-50" // Use var(--state-disabled-opacity)
```

**Why:** Can't be customized per-theme

### ❌ Don't Mix Opacity + Color Changes
```rust
// FORBIDDEN
"hover:opacity-90 hover:bg-blue-600"
```

**Why:** Double-change is jarring and inconsistent

## Accessibility Requirements

### Focus Indicators
```rust
// REQUIRED: Visible focus for keyboard navigation
"focus-visible:outline-none \
 focus-visible:ring-2 \
 focus-visible:ring-[color:var(--state-focus-ring)] \
 focus-visible:ring-offset-2"
```

### Disabled State
```rust
// REQUIRED: Prevent interaction + visual feedback
"disabled:pointer-events-none \
 disabled:cursor-not-allowed \
 disabled:opacity-[var(--state-disabled-opacity)]"
```

### ARIA Attributes
```rust
view! {
    <button
        disabled=is_disabled
        aria-disabled=move || is_disabled.get()
        data-state=move || if is_active.get() { "active" } else { "idle" }
    >
}
```

## Theme Contract

### What Themes MUST Provide
- `--color-ring` (for focus state)

### What Themes SHOULD NOT Provide
- State-specific color variants
- Custom opacity values (use defaults)

### What Themes CAN Override
```css
/* Optional: Adjust state intensities */
[data-theme="high-contrast"] {
  --state-hover-opacity: 0.85;  /* Stronger hover */
  --state-disabled-opacity: 0.4; /* More obvious disabled */
}
```

## Migration Strategy

### Step 1: Audit Existing States
```bash
# Find hardcoded opacities
grep -r "opacity-[0-9]" src/ui/

# Find state color variants
grep -r "hover:bg-\|active:bg-" src/ui/
```

### Step 2: Replace with Tokens
```rust
// Before
"hover:opacity-90"

// After
"hover:opacity-[var(--state-hover-opacity)]"
```

### Step 3: Remove State Colors from Themes
```css
/* Remove these from theme.css */
--color-primary-hover: ...;
--color-primary-active: ...;
```

## Validation Checklist
- [ ] All interactive components use `--state-*-opacity` vars
- [ ] Focus states use `--state-focus-ring`
- [ ] No hardcoded opacity values
- [ ] No state-specific color variants in themes
- [ ] Disabled states prevent interaction
- [ ] Focus indicators visible for keyboard users

## References
- [WCAG 2.1 Focus Visible](https://www.w3.org/WAI/WCAG21/Understanding/focus-visible.html)
- [Material Design: States](https://m3.material.io/foundations/interaction/states)
- [Canon Rule #21: Canonical Color Tokens](./canon-rule-21-canonical-color-tokens.md)
