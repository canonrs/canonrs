# Canon Rule #24: Density & Size Scaling

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-01

## Principle
UI density and component sizes must be **data-attribute driven** and **mathematically consistent** across the design system. Size scales follow a **modular scale** (1.25x ratio) for visual harmony.

## Density Levels

### Canonical Density Modes
```rust
pub enum Density {
    Compact,     // High information density
    Comfortable, // Default (recommended)
    Spacious,    // Maximum accessibility
}
```

### CSS Implementation
```css
:root {
  /* Comfortable (default) */
  --density-multiplier: 1.0;
}

[data-density="compact"] {
  --density-multiplier: 0.75;
}

[data-density="spacious"] {
  --density-multiplier: 1.25;
}
```

### Application
```html
<html data-density="comfortable">
```

## Size Token Architecture

### Base Size Scale (Modular 1.25x)
```css
:root {
  /* Control heights */
  --size-control-xs: 1.5rem;   /* 24px */
  --size-control-sm: 2rem;     /* 32px */
  --size-control-md: 2.5rem;   /* 40px - base */
  --size-control-lg: 3.125rem; /* 50px (40 × 1.25) */
  --size-control-xl: 3.75rem;  /* 60px (50 × 1.2) */
  
  /* Spacing scale */
  --space-xs: 0.25rem;  /* 4px */
  --space-sm: 0.5rem;   /* 8px */
  --space-md: 1rem;     /* 16px - base */
  --space-lg: 1.5rem;   /* 24px */
  --space-xl: 2rem;     /* 32px */
  --space-2xl: 3rem;    /* 48px */
  
  /* Icon sizes */
  --size-icon-xs: 0.875rem; /* 14px */
  --size-icon-sm: 1rem;     /* 16px */
  --size-icon-md: 1.25rem;  /* 20px */
  --size-icon-lg: 1.5rem;   /* 24px */
  --size-icon-xl: 2rem;     /* 32px */
}
```

### Density-Aware Sizing
```css
/* Multiply all sizes by density */
[data-density="compact"] {
  --size-control-md: calc(2.5rem * 0.75);  /* 30px */
  --space-md: calc(1rem * 0.75);           /* 12px */
}

[data-density="spacious"] {
  --size-control-md: calc(2.5rem * 1.25);  /* 50px */
  --space-md: calc(1rem * 1.25);           /* 20px */
}
```

## Component Size Implementation

### Button Sizes
```rust
pub enum ButtonSize {
    Xs,  // --size-control-xs
    Sm,  // --size-control-sm
    Md,  // --size-control-md (default)
    Lg,  // --size-control-lg
    Xl,  // --size-control-xl
    Icon, // square: --size-control-md
}

impl ButtonSize {
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Xs => "px-2 text-xs",
            Self::Sm => "px-3 text-sm",
            Self::Md => "px-4 text-base",
            Self::Lg => "px-6 text-lg",
            Self::Xl => "px-8 text-xl",
            Self::Icon => "p-0",
        }
    }
    
    pub fn style(&self) -> &'static str {
        match self {
            Self::Xs => "height: var(--size-control-xs);",
            Self::Sm => "height: var(--size-control-sm);",
            Self::Md => "height: var(--size-control-md);",
            Self::Lg => "height: var(--size-control-lg);",
            Self::Xl => "height: var(--size-control-xl);",
            Self::Icon => "height: var(--size-control-md); width: var(--size-control-md);",
        }
    }
}
```

### Input Sizes
```rust
pub enum InputSize {
    Sm, // --size-control-sm
    Md, // --size-control-md (default)
    Lg, // --size-control-lg
}
```

## Spacing System

### Field Padding
```css
:root {
  --field-padding-y: var(--space-sm);  /* 8px vertical */
  --field-padding-x: var(--space-md);  /* 16px horizontal */
  --field-height: var(--size-control-md);
}
```

### Component Gaps
```rust
// Consistent spacing between elements
"gap-2"  // --space-sm (8px)
"gap-4"  // --space-md (16px)
"gap-6"  // --space-lg (24px)
```

## Typography Scale

### Font Sizes (Modular 1.2x)
```css
:root {
  --font-size-xs: 0.75rem;   /* 12px */
  --font-size-sm: 0.875rem;  /* 14px */
  --font-size-md: 1rem;      /* 16px - base */
  --font-size-lg: 1.125rem;  /* 18px */
  --font-size-xl: 1.25rem;   /* 20px */
  --font-size-2xl: 1.5rem;   /* 24px */
  --font-size-3xl: 1.875rem; /* 30px */
}
```

### Line Heights
```css
:root {
  --line-height-tight: 1.25;
  --line-height-normal: 1.5;
  --line-height-relaxed: 1.75;
}
```

## Density Context Provider

### Rust Implementation
```rust
#[derive(Clone, Copy, PartialEq)]
pub enum Density {
    Compact,
    Comfortable,
    Spacious,
}

#[component]
pub fn DensityProvider(
    density: RwSignal<Density>,
    children: Children,
) -> impl IntoView {
    Effect::new(move |_| {
        let d = density.get();
        if let Some(html) = document().document_element() {
            html.set_attribute("data-density", d.as_str()).ok();
        }
    });
    
    view! { {children()} }
}
```

## Responsive Density

### Breakpoint-Based
```css
/* Mobile: always compact */
@media (max-width: 640px) {
  :root {
    --density-multiplier: 0.75;
  }
}

/* Desktop: user preference */
@media (min-width: 1024px) {
  [data-density="spacious"] {
    --density-multiplier: 1.25;
  }
}
```

## Touch Target Compliance

### Minimum Touch Sizes (WCAG 2.5.5)
```css
/* Ensure 44×44px minimum on touch devices */
@media (pointer: coarse) {
  --size-control-min: 2.75rem; /* 44px */
  
  [data-density="compact"] {
    --size-control-md: max(var(--size-control-md), var(--size-control-min));
  }
}
```

## Prohibited Patterns

### ❌ Hardcoded Pixel Values
```rust
// FORBIDDEN
"h-10 px-4"  // Use size tokens
```

### ❌ Inconsistent Scales
```css
/* FORBIDDEN */
--size-control-sm: 28px;  /* Breaks 1.25x ratio */
--size-control-md: 40px;
--size-control-lg: 55px;  /* Should be 50px */
```

### ❌ Density-Unaware Components
```rust
// FORBIDDEN: Fixed size ignores density
const FIXED_HEIGHT: &str = "h-10";

// CORRECT: Density-responsive
const ADAPTIVE_HEIGHT: &str = ""; // Use inline style with var()
```

## Validation Checklist
- [ ] All sizes use CSS variables
- [ ] Scale follows 1.25x modular ratio
- [ ] Density multiplier applies system-wide
- [ ] Touch targets ≥44px on mobile
- [ ] Typography scales proportionally
- [ ] No hardcoded pixel values in components

## References
- [Modular Scale](https://www.modularscale.com/)
- [WCAG 2.5.5 Target Size](https://www.w3.org/WAI/WCAG21/Understanding/target-size.html)
- [Material Design: Density](https://m3.material.io/foundations/layout/applying-layout/compact)
