# Canon Rule #28: Responsive Grid Contract

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-01

## Principle
Responsive layout uses **mobile-first breakpoints** and a **12-column grid system** with consistent gutters. Breakpoints and grid settings are **system-wide constants** that ensure predictable layout behavior across all viewports.

## Breakpoint System

### Canonical Breakpoints
```css
:root {
  --breakpoint-sm: 640px;   /* Mobile landscape / Small tablet */
  --breakpoint-md: 768px;   /* Tablet portrait */
  --breakpoint-lg: 1024px;  /* Tablet landscape / Small desktop */
  --breakpoint-xl: 1280px;  /* Desktop */
  --breakpoint-2xl: 1536px; /* Large desktop */
}
```

### Mobile-First Media Queries
```css
/* Base: Mobile (< 640px) */
.container { width: 100%; }

/* Small devices (≥ 640px) */
@media (min-width: 640px) {
  .container { max-width: 640px; }
}

/* Medium devices (≥ 768px) */
@media (min-width: 768px) {
  .container { max-width: 768px; }
}

/* Large devices (≥ 1024px) */
@media (min-width: 1024px) {
  .container { max-width: 1024px; }
}

/* Extra large (≥ 1280px) */
@media (min-width: 1280px) {
  .container { max-width: 1280px; }
}

/* 2XL (≥ 1536px) */
@media (min-width: 1536px) {
  .container { max-width: 1536px; }
}
```

## Grid System

### 12-Column Grid
```css
:root {
  --grid-columns: 12;
  --grid-gutter-mobile: 1rem;    /* 16px */
  --grid-gutter-tablet: 1.5rem;  /* 24px */
  --grid-gutter-desktop: 2rem;   /* 32px */
}

.grid {
  display: grid;
  grid-template-columns: repeat(var(--grid-columns), 1fr);
  gap: var(--grid-gutter-mobile);
}

@media (min-width: 768px) {
  .grid { gap: var(--grid-gutter-tablet); }
}

@media (min-width: 1024px) {
  .grid { gap: var(--grid-gutter-desktop); }
}
```

### Column Spans
```rust
// Tailwind utilities for 12-column grid
"col-span-1"   // 1/12 width
"col-span-2"   // 2/12 width
"col-span-3"   // 3/12 (quarter)
"col-span-4"   // 4/12 (third)
"col-span-6"   // 6/12 (half)
"col-span-8"   // 8/12 (two-thirds)
"col-span-12"  // Full width
```

### Responsive Column Spans
```rust
// Mobile: full width, Tablet: half, Desktop: third
"col-span-12 md:col-span-6 lg:col-span-4"

// Mobile: full, Desktop: two-thirds
"col-span-12 lg:col-span-8"
```

## Container System

### Container Sizes
```css
.container {
  width: 100%;
  margin-left: auto;
  margin-right: auto;
  padding-left: var(--grid-gutter-mobile);
  padding-right: var(--grid-gutter-mobile);
}

@media (min-width: 640px) {
  .container {
    max-width: 640px;
    padding-left: var(--grid-gutter-tablet);
    padding-right: var(--grid-gutter-tablet);
  }
}

@media (min-width: 1536px) {
  .container {
    max-width: 1280px; /* Cap at 1280px for readability */
  }
}
```

### Fluid Container
```rust
// Full-width container with responsive padding
"w-full px-4 md:px-6 lg:px-8"
```

### Fixed Container
```rust
// Centered container with max-width
"container mx-auto"
```

## Responsive Utilities

### Display
```rust
// Hide on mobile, show on desktop
"hidden lg:block"

// Show on mobile, hide on desktop
"block lg:hidden"
```

### Spacing
```rust
// Responsive padding
"p-4 md:p-6 lg:p-8"

// Responsive margins
"mt-4 md:mt-8 lg:mt-12"
```

### Typography
```rust
// Responsive font sizes
"text-sm md:text-base lg:text-lg"

// Responsive headings
"text-2xl md:text-3xl lg:text-4xl"
```

## Layout Patterns

### Sidebar Layout
```rust
view! {
    <div class="grid grid-cols-1 lg:grid-cols-12 gap-6">
        // Sidebar: full width on mobile, 3 columns on desktop
        <aside class="lg:col-span-3">
        
        // Main: full width on mobile, 9 columns on desktop
        <main class="lg:col-span-9">
    </div>
}
```

### Card Grid
```rust
view! {
    <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
        // 1 column mobile, 2 tablet, 3 desktop, 4 large desktop
        <Card />
        <Card />
        <Card />
    </div>
}
```

### Dashboard Layout
```rust
view! {
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-12 gap-6">
        // Stats: 2 columns on tablet, 4 columns on desktop
        <div class="md:col-span-1 lg:col-span-3"><Stat /></div>
        <div class="md:col-span-1 lg:col-span-3"><Stat /></div>
        <div class="md:col-span-1 lg:col-span-3"><Stat /></div>
        <div class="md:col-span-1 lg:col-span-3"><Stat /></div>
        
        // Chart: full width on mobile, 8 columns on desktop
        <div class="md:col-span-2 lg:col-span-8"><Chart /></div>
        
        // Sidebar: full width on mobile, 4 columns on desktop
        <div class="md:col-span-2 lg:col-span-4"><Activity /></div>
    </div>
}
```

## Flexbox Patterns

### Responsive Flex Direction
```rust
// Stack on mobile, row on desktop
"flex flex-col lg:flex-row"

// Reverse order on desktop
"flex flex-col-reverse lg:flex-row"
```

### Responsive Alignment
```rust
// Center on mobile, left on desktop
"items-center lg:items-start"

// Space between on desktop only
"justify-start lg:justify-between"
```

## Touch vs Desktop

### Touch Targets
```css
@media (pointer: coarse) {
  /* Increase tap targets for touch devices */
  :root {
    --size-control-min: 44px; /* WCAG minimum */
  }
}
```

### Hover States
```css
@media (hover: hover) {
  /* Only apply hover effects on devices that support hover */
  .button:hover {
    opacity: var(--state-hover-opacity);
  }
}
```

## Orientation Detection

### Landscape vs Portrait
```css
@media (orientation: landscape) {
  /* Optimize for landscape */
  .sidebar {
    display: block;
  }
}

@media (orientation: portrait) {
  /* Optimize for portrait */
  .sidebar {
    display: none;
  }
}
```

## Prohibited Patterns

### ❌ Custom Breakpoints
```css
/* FORBIDDEN: Use canonical breakpoints */
@media (min-width: 900px) { /* Not a standard breakpoint */ }
```

### ❌ Desktop-First Approach
```css
/* FORBIDDEN */
@media (max-width: 1024px) { /* Use min-width */ }
```

### ❌ Fixed Pixel Widths
```rust
// FORBIDDEN
"w-[800px]"  // Not responsive

// CORRECT
"w-full max-w-4xl"
```

### ❌ Breakpoint-Specific Classes in Components
```rust
// FORBIDDEN (component shouldn't know about layout)
const BUTTON_CLASSES: &str = "w-full lg:w-auto";

// CORRECT (layout decides)
view! {
    <div class="w-full lg:w-auto">
        <Button />
    </div>
}
```

## Testing Checklist
- [ ] Layout tested at all breakpoints (sm, md, lg, xl, 2xl)
- [ ] Touch targets ≥44px on mobile
- [ ] Text remains readable at all sizes
- [ ] Horizontal scroll never occurs
- [ ] Grid maintains alignment at all breakpoints
- [ ] Orientation changes handled gracefully

## Validation Tools
```bash
# Test responsive breakpoints
npm run test:responsive

# Lighthouse mobile score
npm run lighthouse:mobile
```

## References
- [Tailwind Breakpoints](https://tailwindcss.com/docs/responsive-design)
- [WCAG 2.5.5 Target Size](https://www.w3.org/WAI/WCAG21/Understanding/target-size.html)
- [CSS Grid Layout](https://developer.mozilla.org/en-US/docs/Web/CSS/CSS_Grid_Layout)
- [Canon Rule #24: Density & Size Scaling](./canon-rule-24-density-size-scaling.md)
