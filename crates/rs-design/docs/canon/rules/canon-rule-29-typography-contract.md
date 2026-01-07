# Canon Rule #29: Typography Contract

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-01

## Principle
Typography creates **information hierarchy** and **readability**. Font stacks, sizes, weights, and line heights are **system-wide constants** that ensure visual consistency and performance across all themes.

## Font Stacks (System Fonts Only)

### Canonical Font Families
```css
:root {
  /* Sans-serif (default UI) */
  --font-sans: ui-sans-serif, system-ui, sans-serif, 
               'Apple Color Emoji', 'Segoe UI Emoji', 
               'Segoe UI Symbol', 'Noto Color Emoji';
  
  /* Monospace (code, data) */
  --font-mono: ui-monospace, 'SFMono-Regular', 'Menlo', 
               'Monaco', 'Consolas', 'Liberation Mono', 
               'Courier New', monospace;
  
  /* Serif (editorial content) */
  --font-serif: ui-serif, 'Georgia', 'Cambria', 
                'Times New Roman', 'Times', serif;
}
```

### Why System Fonts?
✅ Zero network requests (instant load)  
✅ Native OS rendering (crisp on all screens)  
✅ Reduced bundle size  
✅ Better privacy (no font CDN tracking)  
✅ Cross-platform consistency

### Prohibited
❌ Web fonts (Google Fonts, Adobe Fonts)  
❌ Custom font files (@font-face)  
❌ Icon fonts (use SVG icons instead)

**Exception:** Brand-specific applications may override fonts at the **application layer**, not theme layer.

## Type Scale (Modular 1.2x Ratio)

### Canonical Font Sizes
```css
:root {
  /* Base: 16px = 1rem */
  --font-size-xs: 0.75rem;    /* 12px */
  --font-size-sm: 0.875rem;   /* 14px */
  --font-size-md: 1rem;       /* 16px (base) */
  --font-size-lg: 1.125rem;   /* 18px */
  --font-size-xl: 1.25rem;    /* 20px */
  --font-size-2xl: 1.5rem;    /* 24px */
  --font-size-3xl: 1.875rem;  /* 30px */
  --font-size-4xl: 2.25rem;   /* 36px */
  --font-size-5xl: 3rem;      /* 48px */
}
```

### Usage Guidelines

**XS (12px):** Captions, labels, metadata
```rust
"text-xs"  // Small helper text
```

**SM (14px):** Body text (compact UI)
```rust
"text-sm"  // Table cells, list items
```

**MD (16px):** Default body text
```rust
"text-base"  // Paragraphs, descriptions
```

**LG (18px):** Emphasized body
```rust
"text-lg"  // Lead paragraphs
```

**XL-5XL:** Headings
```rust
"text-xl"   // H5
"text-2xl"  // H4
"text-3xl"  // H3
"text-4xl"  // H2
"text-5xl"  // H1
```

## Font Weights

### Canonical Weights
```css
:root {
  --font-weight-regular: 400;   /* Body text */
  --font-weight-medium: 500;    /* Emphasis */
  --font-weight-semibold: 600;  /* Subheadings */
  --font-weight-bold: 700;      /* Headings */
}
```

### Weight Pairing Rules
```rust
// Body text
"font-normal"  // 400 - default

// UI labels, buttons
"font-medium"  // 500 - subtle emphasis

// Section headers
"font-semibold"  // 600 - clear hierarchy

// Page titles
"font-bold"  // 700 - maximum impact
```

### Prohibited
❌ Thin (100-300) — insufficient contrast  
❌ Extra Bold (800-900) — visually harsh

## Line Heights

### Canonical Line Heights
```css
:root {
  --line-height-tight: 1.25;    /* Headings */
  --line-height-normal: 1.5;    /* Body text */
  --line-height-relaxed: 1.75;  /* Long-form reading */
}
```

### Application Rules

**Tight (1.25):** Headings, titles
```rust
"leading-tight"  // H1-H6
```

**Normal (1.5):** UI text, labels
```rust
"leading-normal"  // Default for all UI
```

**Relaxed (1.75):** Article body, documentation
```rust
"leading-relaxed"  // Long-form content
```

## Letter Spacing (Tracking)

### Canonical Tracking
```css
:root {
  --tracking-tighter: -0.05em;  /* Large headings */
  --tracking-tight: -0.025em;   /* Headings */
  --tracking-normal: 0em;       /* Body text */
  --tracking-wide: 0.025em;     /* Uppercase labels */
  --tracking-wider: 0.05em;     /* All-caps headings */
}
```

### Usage
```rust
// Large display text (48px+)
"tracking-tighter"  // Tighten for optical balance

// Uppercase labels
"tracking-wide uppercase"  // Improve readability

// Default
"tracking-normal"  // Most text
```

## Responsive Typography

### Mobile-First Scaling
```css
/* Mobile base (16px) */
:root {
  --font-size-base: 1rem;
}

/* Tablet (18px base for comfort) */
@media (min-width: 768px) {
  :root {
    --font-size-base: 1.125rem;
  }
}

/* Desktop (back to 16px for density) */
@media (min-width: 1024px) {
  :root {
    --font-size-base: 1rem;
  }
}
```

### Responsive Headings
```rust
// H1: Mobile 30px → Desktop 48px
"text-3xl lg:text-5xl"

// H2: Mobile 24px → Desktop 36px
"text-2xl lg:text-4xl"

// Body: Always 16px (don't scale)
"text-base"
```

## Accessibility Requirements

### Minimum Contrast Ratios (WCAG AA)
- **Normal text (< 18px):** 4.5:1
- **Large text (≥ 18px):** 3:1
- **UI components:** 3:1

### Readable Line Lengths
```css
/* Optimal: 45-75 characters per line */
.prose {
  max-width: 65ch;  /* ~65 characters */
}
```

### Scalable Text
```css
/* NEVER use fixed pixel sizes */
❌ font-size: 14px;

/* ALWAYS use rem for scalability */
✅ font-size: 0.875rem;  /* Scales with user preference */
```

### Zoom Support
Text must remain readable at 200% zoom:
```css
/* Use relative units */
✅ padding: 1rem;
✅ margin: 0.5em;

/* Avoid fixed containers */
❌ width: 300px;
✅ max-width: 20rem;
```

## Component Typography Patterns

### Button Text
```rust
const BUTTON_TEXT: &str = "\
    font-medium \
    text-sm \
    leading-normal";
```

### Input Labels
```rust
const LABEL_TEXT: &str = "\
    font-medium \
    text-sm \
    leading-normal";
```

### Helper Text
```rust
const HELPER_TEXT: &str = "\
    font-normal \
    text-xs \
    leading-normal \
    text-muted-foreground";
```

### Card Titles
```rust
const CARD_TITLE: &str = "\
    font-semibold \
    text-lg \
    leading-tight";
```

### Table Headers
```rust
const TABLE_HEADER: &str = "\
    font-medium \
    text-sm \
    leading-tight \
    uppercase \
    tracking-wide";
```

## Prose Styling

### Long-Form Content
```rust
const PROSE_CLASSES: &str = "\
    font-serif \
    text-base \
    leading-relaxed \
    max-w-prose \
    text-foreground";
```

### Article Headings
```css
.prose h1 { @apply text-4xl font-bold leading-tight mt-8 mb-4; }
.prose h2 { @apply text-3xl font-semibold leading-tight mt-6 mb-3; }
.prose h3 { @apply text-2xl font-semibold leading-tight mt-4 mb-2; }
.prose p  { @apply text-base leading-relaxed mb-4; }
```

## Code & Monospace

### Inline Code
```rust
const INLINE_CODE: &str = "\
    font-mono \
    text-sm \
    bg-muted \
    px-1 \
    py-0.5 \
    rounded";
```

### Code Blocks
```rust
const CODE_BLOCK: &str = "\
    font-mono \
    text-sm \
    leading-relaxed \
    bg-muted \
    p-4 \
    rounded-lg \
    overflow-x-auto";
```

## Dark Mode Typography

### Adjustments for Readability
```css
.dark {
  /* Slightly reduce font weight for dark backgrounds */
  --font-weight-regular: 350;  /* Instead of 400 */
  --font-weight-medium: 450;   /* Instead of 500 */
  
  /* Increase line height slightly */
  --line-height-normal: 1.6;   /* Instead of 1.5 */
}
```

## Prohibited Patterns

### ❌ Hardcoded Font Families
```rust
// FORBIDDEN
"font-['Arial']"
"font-['Helvetica']"

// CORRECT
"font-sans"
```

### ❌ Pixel-Based Font Sizes
```rust
// FORBIDDEN
"text-[14px]"

// CORRECT
"text-sm"  // Uses var(--font-size-sm)
```

### ❌ Non-Standard Weights
```rust
// FORBIDDEN
"font-[350]"
"font-[450]"

// CORRECT
"font-normal"
"font-medium"
```

### ❌ Custom Line Heights
```rust
// FORBIDDEN
"leading-[1.3]"

// CORRECT
"leading-tight"
"leading-normal"
```

## Performance Optimization

### Font Loading Strategy
```html
<!-- Preload system fonts (no action needed) -->
<!-- System fonts load instantly -->
```

### Font Display
```css
/* Not applicable - system fonts don't need font-display */
```

### Subsetting
```css
/* Not applicable - system fonts are pre-installed */
```

## Validation Checklist
- [ ] All font families use system font stacks
- [ ] Font sizes use canonical scale (xs → 5xl)
- [ ] Font weights limited to 400, 500, 600, 700
- [ ] Line heights use tight/normal/relaxed
- [ ] Text remains readable at 200% zoom
- [ ] Contrast ratios pass WCAG AA
- [ ] No hardcoded pixel sizes
- [ ] No web fonts or @font-face

## References
- [WCAG 2.1: Text Spacing](https://www.w3.org/WAI/WCAG21/Understanding/text-spacing.html)
- [WCAG 2.1: Contrast Minimum](https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html)
- [Modular Scale](https://www.modularscale.com/)
- [System Font Stack](https://systemfontstack.com/)
- [Canon Rule #24: Density & Size Scaling](./canon-rule-24-density-size-scaling.md)
