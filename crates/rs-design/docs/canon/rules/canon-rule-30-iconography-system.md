# Canon Rule #30: Iconography System

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-01

## Principle
Icons are **SVG-based, inline, and accessible**. The system uses a **single canonical icon library** (Lucide) with consistent sizing, coloring, and ARIA patterns. Icon fonts are prohibited.

## Icon Library

### Canonical Library: Lucide React
```toml
[dependencies]
# Rust binding to Lucide icons
lucide-leptos = "0.1"  # or equivalent
```

**Why Lucide?**
✅ MIT licensed (no attribution required)  
✅ 1000+ consistent icons  
✅ Designed for 24×24 grid  
✅ Tree-shakeable (only import used icons)  
✅ Active maintenance  
✅ shadcn/ui compatible

### Prohibited
❌ Font Awesome (icon font)  
❌ Material Icons (inconsistent licensing)  
❌ Custom icon fonts  
❌ Raster images (PNG, JPG) for UI icons  
❌ Multiple icon libraries in one project

## Icon Sizes

### Canonical Size Scale
```css
:root {
  --size-icon-xs: 0.875rem;  /* 14px */
  --size-icon-sm: 1rem;      /* 16px */
  --size-icon-md: 1.25rem;   /* 20px */
  --size-icon-lg: 1.5rem;    /* 24px (native Lucide size) */
  --size-icon-xl: 2rem;      /* 32px */
}
```

### Size Application
```rust
use leptos_lucide::*;

// XS (14px): Inline with small text
view! {
    <CheckIcon size=14 />
    <span class="text-xs">"Verified"</span>
}

// SM (16px): Inline with body text
view! {
    <InfoIcon size=16 />
    <span class="text-sm">"Learn more"</span>
}

// MD (20px): Buttons, inputs
view! {
    <SearchIcon size=20 />
}

// LG (24px): Standalone icons, toolbar
view! {
    <MenuIcon size=24 />
}

// XL (32px): Large touch targets, hero icons
view! {
    <AlertCircleIcon size=32 />
}
```

## Icon Colors

### Color Inheritance
```rust
// Icons inherit currentColor by default
view! {
    <button class="text-primary">
        <CheckIcon size=16 />  // Automatically primary color
        "Confirm"
    </button>
}
```

### Semantic Coloring
```rust
// Success state
view! {
    <CheckCircleIcon size=16 class="text-success" />
}

// Error state
view! {
    <XCircleIcon size=16 class="text-destructive" />
}

// Muted/inactive
view! {
    <InfoIcon size=16 class="text-muted-foreground" />
}
```

### Prohibited
❌ Hardcoded fill colors  
❌ Multi-color icons (use monochrome)  
❌ Gradient fills

## Icon Positioning

### Inline with Text
```rust
// Icon + text alignment
const ICON_TEXT: &str = "inline-flex items-center gap-2";

view! {
    <div class=ICON_TEXT>
        <CheckIcon size=16 />
        <span>"Completed"</span>
    </div>
}
```

### Button Icons
```rust
// Icon-only button
view! {
    <button class="p-2">
        <MenuIcon size=20 />
        <span class="sr-only">"Open menu"</span>
    </button>
}

// Icon + text button
view! {
    <button class="inline-flex items-center gap-2 px-4 py-2">
        <PlusIcon size=16 />
        <span>"Add Item"</span>
    </button>
}
```

### Leading/Trailing Icons
```rust
// Input with leading icon
view! {
    <div class="relative">
        <SearchIcon size=16 class="absolute left-3 top-1/2 -translate-y-1/2 text-muted-foreground" />
        <input class="pl-10" placeholder="Search..." />
    </div>
}

// Input with trailing icon
view! {
    <div class="relative">
        <input placeholder="Enter email..." />
        <CheckIcon size=16 class="absolute right-3 top-1/2 -translate-y-1/2 text-success" />
    </div>
}
```

## Accessibility

### ARIA Labels
```rust
// Icon-only buttons MUST have labels
view! {
    <button aria-label="Close dialog">
        <XIcon size=20 />
    </button>
}

// OR use sr-only text
view! {
    <button>
        <XIcon size=20 />
        <span class="sr-only">"Close dialog"</span>
    </button>
}
```

### Decorative Icons
```rust
// Icons that don't convey meaning
view! {
    <div>
        <StarIcon size=16 aria-hidden="true" />
        <span>"Featured"</span>  // Text provides meaning
    </div>
}
```

### Semantic Icons
```rust
// Icons that convey state/meaning
view! {
    <div role="alert">
        <AlertTriangleIcon size=20 aria-label="Warning" />
        <span>"This action cannot be undone"</span>
    </div>
}
```

## Icon States

### Interactive States
```rust
// Hover effect
view! {
    <button class="group">
        <HeartIcon 
            size=20 
            class="group-hover:text-destructive transition-colors"
        />
    </button>
}

// Active/Selected state
view! {
    <button data-state=is_active class="data-[state=true]:text-primary">
        <CheckIcon size=16 />
    </button>
}
```

### Loading State
```rust
// Spinning loader
view! {
    <Loader2Icon size=20 class="animate-spin" />
}
```

### Disabled State
```rust
view! {
    <button disabled class="disabled:opacity-50">
        <SaveIcon size=16 />
        "Save"
    </button>
}
```

## Icon Variants

### Outlined vs Filled
```rust
// Lucide icons are outlined by default (preferred)
view! {
    <HeartIcon size=20 />  // Outlined
}

// Use filled sparingly (active states)
view! {
    <HeartIcon size=20 class="fill-current" />  // Filled
}
```

### Stroke Width
```rust
// Default stroke width: 2px
view! {
    <Icon size=24 stroke-width="2" />
}

// Thin icons (delicate UI)
view! {
    <Icon size=24 stroke-width="1" />
}

// Bold icons (emphasis)
view! {
    <Icon size=24 stroke-width="3" />
}
```

## Performance

### Tree Shaking
```rust
// ✅ CORRECT: Import only used icons
use leptos_lucide::{CheckIcon, XIcon, MenuIcon};

// ❌ FORBIDDEN: Import all icons
use leptos_lucide::*;  // Only in examples
```

### SVG Optimization
```rust
// Icons should be optimized with SVGO
// Lucide icons are pre-optimized
```

### Lazy Loading
```rust
// Icons used below fold can be lazy loaded
#[component]
fn LazyIcon() -> impl IntoView {
    let IconComponent = || async {
        // Dynamic import for code splitting
        leptos::spawn_local(async {
            // Load icon component
        });
    };
}
```

## Custom Icons

### When Allowed
✅ Brand logos (as SVG)  
✅ Product-specific icons not in Lucide  
✅ Data visualizations

### Requirements
```rust
// Custom icons MUST:
// 1. Be SVG format
// 2. Use currentColor for fills/strokes
// 3. Have viewBox="0 0 24 24" (24×24 grid)
// 4. Be optimized with SVGO
// 5. Include ARIA labels

view! {
    <svg 
        width="24" 
        height="24" 
        viewBox="0 0 24 24" 
        fill="none" 
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-label="Custom brand icon"
    >
        <path d="..." />
    </svg>
}
```

## Icon Grid System

### 24×24 Grid (Lucide Standard)
```
All icons fit within a 24×24 pixel grid
Optical alignment takes priority over mathematical centering
2px default stroke width
Rounded line caps and joins
```

### Alignment Rules
```rust
// Vertically center with text
"inline-flex items-center"

// Baseline align with text
"inline-flex items-baseline"

// Pixel-perfect alignment
"inline-block align-middle"
```

## Dark Mode

### Icon Visibility
```css
/* Icons inherit theme colors automatically */
.dark .icon {
  /* No special handling needed */
  color: inherit;
}

/* Reduce stroke width in dark mode for optical balance */
.dark {
  --icon-stroke-width: 1.5px;  /* Instead of 2px */
}
```

## Prohibited Patterns

### ❌ Icon Fonts
```html
<!-- FORBIDDEN -->
<i class="fa fa-check"></i>
<span class="material-icons">check</span>
```

### ❌ Raster Icons
```html
<!-- FORBIDDEN -->
<img src="icon.png" alt="Check" />
```

### ❌ Inline SVG Strings
```rust
// FORBIDDEN
const ICON_SVG: &str = "<svg>...</svg>";

// CORRECT
use leptos_lucide::CheckIcon;
```

### ❌ Hardcoded Sizes
```rust
// FORBIDDEN
<Icon size=18 />  // Not in scale

// CORRECT
<Icon size=16 />  // Uses canonical size
```

## Validation Checklist
- [ ] All icons from Lucide library
- [ ] Icon sizes use canonical scale (14, 16, 20, 24, 32)
- [ ] Icons use currentColor for theming
- [ ] Icon-only buttons have ARIA labels
- [ ] Decorative icons have aria-hidden="true"
- [ ] No icon fonts used
- [ ] No raster image icons
- [ ] Custom icons follow 24×24 grid

## References
- [Lucide Icons](https://lucide.dev/)
- [WCAG 2.1: Non-text Content](https://www.w3.org/WAI/WCAG21/Understanding/non-text-content.html)
- [SVG Accessibility](https://www.w3.org/WAI/WCAG21/Techniques/aria/ARIA24.html)
- [Canon Rule #23: State Tokens](./canon-rule-23-state-tokens.md)
