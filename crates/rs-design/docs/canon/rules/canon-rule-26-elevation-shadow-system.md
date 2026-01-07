# Canon Rule #26: Elevation & Shadow System

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-01

## Principle
Elevation creates visual hierarchy through **consistent shadow layers**. Shadows are **system-wide constants** that themes cannot override, ensuring predictable z-axis relationships across all themes.

## Elevation Levels

### Canonical Shadow Scale
```css
:root {
  /* Shadow parameters */
  --shadow-color: hsl(0 0% 0%);
  --shadow-x: 0px;
  --shadow-y: 4px;
  --shadow-blur: 8px;
  --shadow-spread: -1px;
  --shadow-opacity: 0.1;
  
  /* Level 0: Flat (no shadow) */
  --shadow-none: none;
  
  /* Level 1: Subtle lift */
  --shadow-2xs: 0px 1px 2px 0px hsl(0 0% 0% / 0.05);
  --shadow-xs: 0px 1px 3px 0px hsl(0 0% 0% / 0.1),
               0px 1px 2px -1px hsl(0 0% 0% / 0.1);
  
  /* Level 2: Standard elevation */
  --shadow-sm: 0px 4px 8px -1px hsl(0 0% 0% / 0.10),
               0px 1px 2px -2px hsl(0 0% 0% / 0.10);
  --shadow: 0px 4px 8px -1px hsl(0 0% 0% / 0.10),
            0px 2px 4px -2px hsl(0 0% 0% / 0.10);
  
  /* Level 3: Raised elements */
  --shadow-md: 0px 4px 8px -1px hsl(0 0% 0% / 0.10),
               0px 2px 4px -2px hsl(0 0% 0% / 0.10);
  --shadow-lg: 0px 4px 8px -1px hsl(0 0% 0% / 0.10),
               0px 4px 6px -2px hsl(0 0% 0% / 0.10);
  
  /* Level 4: Floating UI */
  --shadow-xl: 0px 4px 8px -1px hsl(0 0% 0% / 0.10),
               0px 8px 10px -2px hsl(0 0% 0% / 0.10);
  
  /* Level 5: Modal/Dialog */
  --shadow-2xl: 0px 4px 8px -1px hsl(0 0% 0% / 0.25);
}
```

## Elevation Hierarchy

### Layer Mapping
```
Level 0 (z-0):  Base surface (background)
  └─ shadow-none
  
Level 1 (z-10): Cards, panels
  └─ shadow-sm
  
Level 2 (z-20): Raised buttons, inputs
  └─ shadow-md
  
Level 3 (z-30): Dropdowns, tooltips
  └─ shadow-lg
  
Level 4 (z-40): Popovers, menus
  └─ shadow-xl
  
Level 5 (z-50): Modals, dialogs
  └─ shadow-2xl
```

## Component Shadow Assignment

### Cards & Containers
```rust
// Card component
const CARD_CLASSES: &str = "shadow-sm";  // Level 1

// Elevated card
const CARD_ELEVATED: &str = "shadow-md"; // Level 2
```

### Interactive Elements
```rust
// Button (raised)
const BUTTON_SHADOW: &str = "shadow-sm hover:shadow-md";

// Input fields
const INPUT_SHADOW: &str = "shadow-sm focus-visible:shadow-md";
```

### Floating UI
```rust
// Dropdown menu
const DROPDOWN_SHADOW: &str = "shadow-lg"; // Level 3

// Popover
const POPOVER_SHADOW: &str = "shadow-xl"; // Level 4

// Dialog
const DIALOG_SHADOW: &str = "shadow-2xl"; // Level 5
```

## Dark Mode Adjustments

### Shadow Intensity
```css
.dark {
  /* Increase shadow opacity for visibility on dark backgrounds */
  --shadow-opacity: 0.5;
  
  /* Redefine shadows with higher opacity */
  --shadow-sm: 0px 4px 8px -1px hsl(0 0% 0% / 0.50),
               0px 1px 2px -2px hsl(0 0% 0% / 0.50);
  
  --shadow-2xl: 0px 4px 8px -1px hsl(0 0% 0% / 0.75);
}
```

## Animation & Transitions

### Shadow Transitions
```rust
// Smooth shadow changes on interaction
const SHADOW_TRANSITION: &str = "\
    transition-shadow \
    duration-[var(--motion-duration-normal)] \
    ease-[var(--motion-ease-standard)]";

// Usage
view! {
    <div class="shadow-sm hover:shadow-lg transition-shadow">
}
```

## Z-Index Scale

### Canonical Z-Layers
```css
:root {
  --z-base: 0;
  --z-dropdown: 1000;
  --z-sticky: 1100;
  --z-fixed: 1200;
  --z-modal-backdrop: 1300;
  --z-modal: 1400;
  --z-popover: 1500;
  --z-tooltip: 1600;
  --z-toast: 1700;
}
```

### Component Z-Index Assignment
```rust
// Dropdown
"z-[var(--z-dropdown)]"

// Modal backdrop
"z-[var(--z-modal-backdrop)]"

// Modal content
"z-[var(--z-modal)]"

// Toast notifications (highest)
"z-[var(--z-toast)]"
```

## Prohibited Patterns

### ❌ Theme-Specific Shadows
```css
/* FORBIDDEN: Themes cannot override shadows */
[data-theme="my-theme"] {
  --shadow-sm: 0px 10px 20px ...;  /* System constant */
}
```

### ❌ Hardcoded Shadow Values
```rust
// FORBIDDEN
"shadow-[0_4px_6px_rgba(0,0,0,0.1)]"

// CORRECT
"shadow-md"
```

### ❌ Arbitrary Z-Index
```rust
// FORBIDDEN
"z-[999]"

// CORRECT
"z-[var(--z-modal)]"
```

## Accessibility Considerations

### High Contrast Mode
```css
@media (prefers-contrast: high) {
  :root {
    /* Increase shadow opacity for better definition */
    --shadow-opacity: 0.3;
  }
}
```

### Reduced Motion
```css
@media (prefers-reduced-motion: reduce) {
  * {
    /* Disable shadow transitions */
    transition-property: none !important;
  }
}
```

## Performance Optimization

### Layer Promotion
```css
/* Promote frequently animated elements to GPU layer */
.shadow-animated {
  will-change: box-shadow;
  transform: translateZ(0); /* Force GPU acceleration */
}
```

### Shadow Compositing
```css
/* Use transform for elevation instead of multiple shadows when possible */
.elevated {
  transform: translateY(-2px);
  box-shadow: var(--shadow-md);
}
```

## Validation Checklist
- [ ] All shadows use CSS variable tokens
- [ ] Z-index values from canonical scale
- [ ] No theme-specific shadow overrides
- [ ] Dark mode shadows have increased opacity
- [ ] Shadow transitions respect `prefers-reduced-motion`
- [ ] Floating UI elements use correct elevation level

## References
- [Material Design: Elevation](https://m3.material.io/styles/elevation/overview)
- [Box Shadow Generator](https://shadows.brumm.af/)
- [Canon Rule #23: State Tokens](./canon-rule-23-state-tokens.md)
