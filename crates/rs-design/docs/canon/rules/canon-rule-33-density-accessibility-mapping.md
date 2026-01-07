# Canon Rule #33: Density & Accessibility Mapping

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-02

## Principle
Density modes scale UI elements proportionally while maintaining **WCAG 2.1 AA accessibility**. Touch targets, text sizes, and spacing adapt to user preference and input method, ensuring usability across devices and abilities.

## Density Modes

### Three Canonical Densities
```css
:root {
  --density-multiplier: 1.0; /* Default: Comfortable */
}

[data-density="compact"] {
  --density-multiplier: 0.75;
}

[data-density="comfortable"] {
  --density-multiplier: 1.0;
}

[data-density="spacious"] {
  --density-multiplier: 1.25;
}
```

### Application
```css
/* Base size */
--size-control-base: 2.5rem; /* 40px */

/* Density-scaled */
.button {
  height: calc(var(--size-control-base) * var(--density-multiplier));
  padding: calc(var(--space-sm) * var(--density-multiplier));
}
```

**Results:**
- Compact: 30px height (0.75 × 40px)
- Comfortable: 40px height (1.0 × 40px)
- Spacious: 50px height (1.25 × 40px)

## WCAG 2.1 AA Compliance Matrix

| Density | Min Touch Target | Min Text Size | Min Contrast | Keyboard Nav |
|---------|------------------|---------------|--------------|--------------|
| Compact | 30px × 30px | 14px (0.875rem) | 4.5:1 | ✅ |
| Comfortable | 40px × 40px | 16px (1rem) | 4.5:1 | ✅ |
| Spacious | 50px × 50px | 18px (1.125rem) | 4.5:1 | ✅ |

### Touch Target Requirements (WCAG 2.5.5)

**Desktop (pointer: fine):**
- Minimum: 24px × 24px
- Recommended: 32px × 32px

**Mobile (pointer: coarse):**
- Minimum: 44px × 44px (iOS HIG)
- Recommended: 48px × 48px (Material Design)

### Density Override by Input Method
```css
/* Desktop: Allow compact */
@media (pointer: fine) {
  [data-density="compact"] {
    --density-multiplier: 0.75; /* 30px minimum */
  }
}

/* Mobile: Enforce minimum 44px */
@media (pointer: coarse) {
  [data-density="compact"] {
    --density-multiplier: 1.1; /* 44px = 40px * 1.1 */
  }
  
  [data-density="comfortable"] {
    --density-multiplier: 1.1;
  }
  
  [data-density="spacious"] {
    --density-multiplier: 1.4; /* 56px */
  }
}
```

## Scaled Properties

### ✅ What Scales with Density

**Spacing:**
```css
padding: calc(var(--space-md) * var(--density-multiplier));
margin: calc(var(--space-sm) * var(--density-multiplier));
gap: calc(var(--space-xs) * var(--density-multiplier));
```

**Control Heights:**
```css
height: calc(var(--size-control-md) * var(--density-multiplier));
min-height: calc(var(--size-control-sm) * var(--density-multiplier));
```

**Icon Sizes:**
```css
width: calc(var(--size-icon-md) * var(--density-multiplier));
height: calc(var(--size-icon-md) * var(--density-multiplier));
```

**Border Radius:**
```css
border-radius: calc(var(--radius-md) * var(--density-multiplier));
```

### ❌ What Does NOT Scale

**Typography:**
```css
/* Font sizes are independent of density */
font-size: var(--font-size-md); /* No multiplier */
line-height: var(--line-height-normal); /* No multiplier */
```

**Colors:**
```css
/* Colors never scale */
background: var(--color-primary-bg);
color: var(--color-primary-fg);
```

**Borders:**
```css
/* Border width stays constant */
border-width: var(--border-width-thin); /* No multiplier */
```

**Shadows:**
```css
/* Shadows are density-independent */
box-shadow: var(--shadow-md);
```

## Component Density Mapping

### Buttons
```css
.button {
  /* Scales with density */
  height: calc(var(--size-control-md) * var(--density-multiplier));
  padding-inline: calc(var(--space-lg) * var(--density-multiplier));
  gap: calc(var(--space-sm) * var(--density-multiplier));
  
  /* Does NOT scale */
  font-size: var(--font-size-sm);
  border-width: var(--border-width-thin);
}
```

**Results:**
| Density | Height | Padding | Font |
|---------|--------|---------|------|
| Compact | 30px | 18px | 14px |
| Comfortable | 40px | 24px | 14px |
| Spacious | 50px | 30px | 14px |

### Inputs
```css
.input {
  height: calc(var(--size-control-md) * var(--density-multiplier));
  padding: calc(var(--space-sm) * var(--density-multiplier));
  font-size: var(--font-size-md); /* No scaling */
}
```

### Tables
```css
.table-row {
  height: calc(2.5rem * var(--density-multiplier));
  padding-block: calc(var(--space-xs) * var(--density-multiplier));
}

.table-cell {
  padding-inline: calc(var(--space-sm) * var(--density-multiplier));
  font-size: var(--font-size-sm); /* No scaling */
}
```

**Results:**
| Density | Row Height | Cell Padding |
|---------|------------|--------------|
| Compact | 30px | 6px |
| Comfortable | 40px | 8px |
| Spacious | 50px | 10px |

## Accessibility Features

### Focus Indicators
```css
.focusable:focus-visible {
  outline: 2px solid var(--color-ring);
  outline-offset: calc(2px * var(--density-multiplier));
  /* Offset scales, width does not */
}
```

### Disabled States
```css
.disabled {
  opacity: var(--state-disabled-opacity); /* 0.5 */
  cursor: not-allowed;
  /* Opacity is density-independent */
}
```

### Hover/Active States
```css
.interactive:hover {
  /* Padding increases on hover for better feedback */
  padding: calc(
    var(--space-sm) * var(--density-multiplier) * 1.1
  );
}
```

## Responsive Breakpoints

### Mobile-First Scaling
```css
/* Mobile: Always enforce minimum touch targets */
@media (max-width: 768px) {
  :root {
    --density-multiplier: max(1.1, var(--density-multiplier));
  }
}

/* Tablet: Allow comfortable or spacious */
@media (min-width: 768px) and (max-width: 1024px) {
  :root {
    --density-multiplier: max(1.0, var(--density-multiplier));
  }
}

/* Desktop: All densities allowed */
@media (min-width: 1024px) {
  [data-density="compact"] {
    --density-multiplier: 0.75;
  }
}
```

## User Preference Detection

### Prefers Reduced Motion
```css
@media (prefers-reduced-motion: reduce) {
  * {
    /* Density still applies, but animations are disabled */
    transition-duration: 0.01ms !important;
  }
}
```

### Prefers Contrast
```css
@media (prefers-contrast: more) {
  :root {
    /* Increase touch targets for high contrast users */
    --density-multiplier: max(1.1, var(--density-multiplier));
  }
}
```

### Forced Colors Mode
```css
@media (forced-colors: active) {
  .button {
    /* Maintain density, adjust borders for visibility */
    border: calc(2px * var(--density-multiplier)) solid CanvasText;
  }
}
```

## Testing Matrix

### Manual Testing

| Test Case | Compact | Comfortable | Spacious |
|-----------|---------|-------------|----------|
| Desktop + Mouse | ✅ 30px | ✅ 40px | ✅ 50px |
| Desktop + Keyboard | ✅ Focus visible | ✅ Focus visible | ✅ Focus visible |
| Mobile + Touch | ⚠️ Override to 44px | ✅ 44px | ✅ 56px |
| Screen Reader | ✅ Same semantic | ✅ Same semantic | ✅ Same semantic |
| Zoom 200% | ✅ No overflow | ✅ No overflow | ✅ No overflow |

### Automated Testing
```rust
#[test]
fn test_density_compliance() {
    // Compact minimum
    assert!(calculate_size("compact") >= 30.0);
    
    // Mobile minimum
    assert!(calculate_touch_target("compact", "coarse") >= 44.0);
    
    // Spacious maximum (reasonable limit)
    assert!(calculate_size("spacious") <= 60.0);
}
```

## Validation Checklist

- [ ] All interactive elements meet minimum touch target (44px mobile)
- [ ] Text remains readable at all densities (min 14px)
- [ ] Focus indicators visible at all densities
- [ ] Keyboard navigation works at all densities
- [ ] No horizontal scroll at 200% zoom
- [ ] Spacing proportional across densities
- [ ] Typography does NOT scale with density
- [ ] Colors maintain contrast ratios (4.5:1 minimum)

## Prohibited Patterns

### ❌ Scaling Typography with Density
```css
/* WRONG - text becomes too small in compact */
font-size: calc(var(--font-size-md) * var(--density-multiplier));
```

### ❌ Ignoring Mobile Touch Targets
```css
/* WRONG - too small for touch */
@media (pointer: coarse) {
  [data-density="compact"] {
    --density-multiplier: 0.75; /* Results in 30px - too small! */
  }
}
```

### ❌ Fixed Pixel Sizes
```css
/* WRONG - doesn't scale with density */
.button {
  height: 40px; /* Use calc() with density multiplier */
}
```

### ❌ Inconsistent Scaling
```css
/* WRONG - some properties scale, others don't */
.card {
  padding: calc(var(--space-md) * var(--density-multiplier)); /* Scales */
  gap: var(--space-sm); /* Doesn't scale - inconsistent! */
}
```

## Implementation Example
```rust
// Rust component with density awareness
#[component]
pub fn Button(
    #[prop(optional)] density_aware: bool,
) -> impl IntoView {
    let base_class = if density_aware {
        // Scales with data-density attribute
        "h-[calc(var(--size-control-md)*var(--density-multiplier))] \
         px-[calc(var(--space-lg)*var(--density-multiplier))]"
    } else {
        // Fixed size (density-independent)
        "h-10 px-6"
    };
    
    view! {
        <button class=base_class>
            {children()}
        </button>
    }
}
```

## References

- [WCAG 2.1: Target Size](https://www.w3.org/WAI/WCAG21/Understanding/target-size.html)
- [WCAG 2.1: Contrast Minimum](https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html)
- [Apple HIG: Touch Targets](https://developer.apple.com/design/human-interface-guidelines/ios/visual-design/adaptivity-and-layout/)
- [Material Design: Touch Targets](https://material.io/design/usability/accessibility.html#layout-and-typography)
- [Canon Rule #24: Density & Size Scaling](./canon-rule-24-density-size-scaling.md)
- [Canon Rule #31: Accessibility Contract](./canon-rule-31-accessibility-contract.md)

---

**Versioning:** Semantic (MAJOR.MINOR.PATCH)  
**Compliance:** Enforced via automated testing  
**Accessibility:** WCAG 2.1 AA mandatory
