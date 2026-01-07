# Canon Rule #25: Theme Presets Contract

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-01

## Principle
Theme presets define **color palettes only**. All other design decisions (spacing, typography, radius, shadows) are **system-wide constants** that themes cannot override. This ensures visual consistency and prevents theme fragmentation.

## What Themes CAN Define

### Color Palette (REQUIRED)
```typescript
export const myTheme: ThemeDefinition = {
  modes: {
    light: {
      colors: {
        // Brand
        primary: { h: 221, s: 83, l: 53 },
        primaryForeground: { h: 0, s: 0, l: 100 },
        
        // Neutrals
        secondary: { h: 220, s: 13, l: 91 },
        secondaryForeground: { h: 217, s: 19, l: 27 },
        
        // Accent
        accent: { h: 48, s: 100, l: 96 },
        accentForeground: { h: 23, s: 83, l: 31 },
        
        // Semantic
        destructive: { h: 0, s: 84, l: 60 },
        destructiveForeground: { h: 0, s: 0, l: 100 },
        
        // Base
        background: { h: 0, s: 0, l: 100 },
        foreground: { h: 0, s: 0, l: 15 },
        muted: { h: 210, s: 20, l: 98 },
        mutedForeground: { h: 220, s: 9, l: 46 },
        
        // Containers
        card: { h: 0, s: 0, l: 100 },
        cardForeground: { h: 0, s: 0, l: 15 },
        popover: { h: 0, s: 0, l: 100 },
        popoverForeground: { h: 0, s: 0, l: 15 },
        
        // Structure
        border: { h: 220, s: 13, l: 91 },
        input: { h: 220, s: 13, l: 91 },
        ring: { h: 221, s: 83, l: 53 },
        
        // Charts (optional)
        chart1: { h: 221, s: 83, l: 53 },
        chart2: { h: 212, s: 95, l: 68 },
        chart3: { h: 216, s: 92, l: 60 },
        chart4: { h: 210, s: 98, l: 78 },
        chart5: { h: 212, s: 95, l: 68 },
      },
    },
    dark: {
      colors: { /* ... */ },
    },
  },
};
```

### Color Format Rules
- **Format:** `{ h: number, s: number, l: number }`
- **Ranges:** H: 0-360, S: 0-100, L: 0-100
- **Output:** `hsl(H S% L%)` without `deg` or commas

## What Themes CANNOT Define

### ❌ Typography
```typescript
// FORBIDDEN in themes
fonts: {
  sans: 'Comic Sans',      // System-wide constant
  mono: 'Courier New',
}
```

**Why:** Font stacks are infrastructure, not aesthetics

### ❌ Spacing
```typescript
// FORBIDDEN in themes
spacing: {
  md: '2rem',  // System-wide modular scale
  lg: '3rem',
}
```

**Why:** Spacing ratios ensure mathematical harmony

### ❌ Border Radius
```typescript
// FORBIDDEN in themes
radius: {
  sm: '0.125rem',  // System-wide constant
  md: '0.375rem',
  lg: '0.5rem',
}
```

**Why:** Radius defines brand personality system-wide

### ❌ Shadows
```typescript
// FORBIDDEN in themes
shadows: {
  sm: '0 1px 2px ...',  // System-wide elevation
  md: '0 4px 6px ...',
}
```

**Why:** Shadow hierarchy is structural, not thematic

### ❌ Motion
```typescript
// FORBIDDEN in themes
motion: {
  fast: '150ms',   // System-wide timing
  normal: '300ms',
}
```

**Why:** Animation timing affects UX consistency

## Theme Metadata

### Required Fields
```typescript
{
  version: 1,              // Schema version
  id: 'my-theme',          // Unique kebab-case ID
  name: 'My Theme',        // Display name
  author: 'Your Name',     // Creator attribution
  description: '...',      // Brief description
  modes: { light, dark },  // Both modes required
}
```

### Optional Fields
```typescript
{
  homepage: 'https://...',    // Theme homepage
  repository: 'https://...',  // Source code
  license: 'MIT',             // License identifier
  tags: ['minimal', 'blue'],  // Discovery tags
}
```

## Theme Validation

### Schema Compliance
```bash
# Validate theme structure
npm run validate:themes

# Check for missing required colors
npm run check:theme-contract
```

### Required Colors Checklist
- [ ] primary + primaryForeground
- [ ] secondary + secondaryForeground
- [ ] accent + accentForeground
- [ ] destructive + destructiveForeground
- [ ] background + foreground
- [ ] muted + mutedForeground
- [ ] card + cardForeground
- [ ] popover + popoverForeground
- [ ] border + input + ring

### Optional Colors
- [ ] chart1 through chart5
- [ ] sidebar* (8 tokens)

## Theme Portability

### What Makes a Theme Portable
✅ Uses only canonical color tokens  
✅ Provides both light and dark modes  
✅ Follows HSL format specification  
✅ Passes accessibility contrast checks  
✅ No custom tokens outside contract

### What Breaks Portability
❌ Requires custom spacing/typography  
❌ Only provides light mode  
❌ Uses RGB/HEX instead of HSL  
❌ Defines non-canonical tokens  
❌ Overrides system constants

## Accessibility Requirements

### Contrast Ratios (WCAG AA)
- **Normal text:** ≥4.5:1
- **Large text:** ≥3:1
- **UI components:** ≥3:1

### Validation
```bash
# Check contrast ratios
npm run check:contrast
```

### Common Failures
```typescript
// ❌ Insufficient contrast
foreground: { h: 0, s: 0, l: 65 },  // On white bg
background: { h: 0, s: 0, l: 100 },

// ✅ Sufficient contrast
foreground: { h: 0, s: 0, l: 15 },  // Dark enough
background: { h: 0, s: 0, l: 100 },
```

## Theme Registration

### File Structure
```
src/tokens/themes/presets/
├── my-theme.ts           # Theme definition
└── README.md             # Documentation
```

### Export Pattern
```typescript
// my-theme.ts
import { ThemeDefinition } from '../../engine/schema';

export const myTheme: ThemeDefinition = { /* ... */ };
```

### Registry Addition
```typescript
// src/tokens/themes/registry.ts
import { myTheme } from './presets/my-theme';

export const THEME_REGISTRY = [
  amberMinimal,
  cleanSlate,
  myTheme,  // Add here
];
```

## Build Process

### TypeScript → CSS Generation
```bash
# Generate CSS from TypeScript themes
npm run build:themes

# Output: src/themes/generated.css
```

### Generated Output
```css
[data-theme="my-theme"] {
  --color-primary: 221 83% 53% !important;
  --color-primary-foreground: 0 0% 100% !important;
  /* ... */
}

[data-theme="my-theme"] .dark {
  --color-primary: 228 94% 67% !important;
  /* ... */
}
```

## Versioning Strategy

### Schema Versions
- **v1:** Initial canonical tokens
- **v2:** (future) Additional semantic tokens

### Breaking Changes
Theme schema updates require:
1. Major version bump
2. Migration guide
3. Backward compatibility layer

## Prohibited Customizations

### ❌ Component-Specific Overrides
```css
/* FORBIDDEN in theme.css */
[data-theme="my-theme"] .button {
  border-radius: 0; /* Use system radius */
}
```

### ❌ Layout Modifications
```css
/* FORBIDDEN in theme.css */
[data-theme="my-theme"] .container {
  max-width: 1400px; /* Use system grid */
}
```

### ❌ Breakpoint Changes
```css
/* FORBIDDEN in theme.css */
[data-theme="my-theme"] {
  --breakpoint-md: 900px; /* System constant */
}
```

## Validation Checklist
- [ ] All required color tokens defined
- [ ] Both light and dark modes provided
- [ ] HSL format used consistently
- [ ] Contrast ratios pass WCAG AA
- [ ] No typography/spacing overrides
- [ ] Schema version specified
- [ ] Theme builds without errors

## References
- [shadcn/ui Themes](https://ui.shadcn.com/themes)
- [WCAG 2.1 Contrast](https://www.w3.org/WAI/WCAG21/Understanding/contrast-minimum.html)
- [Canon Rule #21: Canonical Color Tokens](./canon-rule-21-canonical-color-tokens.md)
- [HSL Color Space](https://en.wikipedia.org/wiki/HSL_and_HSV)
