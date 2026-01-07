# Canon Rule #22: Tailwind v4 + Rust Integration

**Status:** ✅ Enforced  
**Version:** 1.0.0  
**Date:** 2025-01-01

## Principle
Tailwind v4's JIT compiler **cannot reliably parse Rust syntax** for arbitrary value generation. All Tailwind utilities used in Rust components must be **pre-compiled** via explicit class definitions or `@layer utilities`.

## The Problem

### ❌ What DOESN'T Work
```rust
// Tailwind v4 JIT CANNOT generate these from .rs files
view! {
    <button class="bg-[hsl(var(--color-primary))]">  // NOT COMPILED
    <div class="h-[var(--size-control-md)]">         // NOT COMPILED
    <span class="text-[hsl(var(--color-accent))]">   // NOT COMPILED
}
```

**Why:** Tailwind's content scanner treats Rust syntax (`var()`, `::`, etc.) as invalid CSS and skips generation.

### ✅ What DOES Work
```rust
// Pre-defined utilities in tailwind.css
view! {
    <button class="bg-primary">      // ✅ Compiled
    <div class="h-control-md">       // ✅ Compiled
    <span class="text-accent">       // ✅ Compiled
}
```

## Solution Architecture

### Layer 1: CSS Variable Definitions
**File:** `packages-rust/rs-tailwind/tokens/theme.css`
```css
@theme inline {
  --color-primary: 38 92% 50%;
  --color-background: 0 0% 100%;
  --size-control-md: 2.5rem;
  /* ... all design tokens */
}
```

### Layer 2: Utility Class Generation
**File:** `apps/*/style/tailwind.css`
```css
@import "tailwindcss";
@import "../../../../packages-rust/rs-tailwind/tokens/theme.css";

@layer utilities {
  /* Color utilities */
  .bg-primary { background-color: hsl(var(--color-primary)); }
  .bg-secondary { background-color: hsl(var(--color-secondary)); }
  .text-foreground { color: hsl(var(--color-foreground)); }
  
  /* Size utilities */
  .h-control-md { height: var(--size-control-md); }
  .w-control-md { width: var(--size-control-md); }
  
  /* Border utilities */
  .border-border { border-color: hsl(var(--color-border)); }
  .rounded-radius-md { border-radius: var(--radius-md); }
}
```

### Layer 3: Rust Component Usage
**File:** `packages-rust/rs-design/src/ui/button/variants.rs`
```rust
impl ButtonVariant {
    pub fn classes(&self) -> &'static str {
        match self {
            Self::Solid => "bg-primary text-primary-foreground border-primary",
            Self::Outline => "bg-background text-foreground border-border",
            Self::Ghost => "bg-transparent text-muted-foreground",
        }
    }
}
```

## Build Pipeline

### PostCSS Configuration
**File:** `postcss.config.js` (ESM format required)
```javascript
export default {
  plugins: {
    "@tailwindcss/postcss": {},
    autoprefixer: {},
  },
};
```

### Tailwind Configuration
**File:** `tailwind.config.js`
```javascript
export default {
  content: [
    "./index.html",
    "./src/**/*.rs",
    "../../packages-rust/rs-design/src/**/*.rs",
  ],
};
```

### Package.json Scripts
```json
{
  "type": "module",
  "scripts": {
    "build:css": "postcss style/tailwind.css -o target/site/pkg/app.css",
    "watch:css": "postcss style/tailwind.css -o target/site/pkg/app.css --watch"
  },
  "devDependencies": {
    "tailwindcss": "^4.1.17",
    "@tailwindcss/postcss": "^4.1.17",
    "postcss": "^8.4.49",
    "postcss-cli": "^11.0.0",
    "autoprefixer": "^10.4.20"
  }
}
```

## Critical Rules

### Rule 1: No Arbitrary Values in Rust
```rust
// ❌ FORBIDDEN
class="bg-[hsl(var(--color-primary))]"
class="h-[var(--size-control-md)]"
class="rounded-[var(--radius-md)]"

// ✅ REQUIRED
class="bg-primary"
class="h-control-md"
class="rounded-radius-md"
```

### Rule 2: All Utilities Must Be Pre-Defined
Every utility class used in `.rs` files must exist in `@layer utilities` in the CSS entry point.

### Rule 3: CSS Import Order Matters
```css
/* 1. Core Tailwind */
@import "tailwindcss";

/* 2. Config */
@config "../tailwind.config.js";

/* 3. Design tokens */
@import "path/to/theme.css";

/* 4. Global styles */
@import "path/to/globals.css";

/* 5. Utilities layer */
@layer utilities {
  /* All custom utilities */
}

/* 6. Custom CSS (no @import/@theme after this) */
aside:not(.data-open) .sidebar-container {
  width: 0 !important;
}
```

### Rule 4: Dual Build Workflow
```bash
# Terminal 1: Rust + Dev Server
cargo leptos watch

# Terminal 2: CSS Watch
npm run watch:css
```

## Why Not Tailwind CLI?

### Problem: Monorepo Scanner Bug
`@tailwindcss/cli@next` (v4.0.0) has critical bugs:
- Fails on relative paths: `../../../../packages-rust/`
- Cannot parse `@source` directive with globs
- Throws: `Missing field 'negated' on ScannerOptions.sources`

### Solution: PostCSS Plugin
The `@tailwindcss/postcss` plugin:
- ✅ Stable and officially supported
- ✅ Proper `@import` resolution
- ✅ Works with monorepos
- ✅ Compatible with cargo-leptos

## Common Pitfalls

### Pitfall 1: Inline Styles Don't Help
```rust
// ❌ Still doesn't work
view! {
    <button style="background-color: hsl(var(--color-primary))">
}
```
Use pre-defined utilities instead.

### Pitfall 2: Purging Issues
If utilities aren't appearing:
```bash
# Check if utility exists in compiled CSS
grep "\.bg-primary" target/site/pkg/app.css

# If missing, add to @layer utilities
```

### Pitfall 3: Cache Issues
After CSS changes:
```bash
# Clear browser cache
CTRL+SHIFT+R

# Rebuild CSS
npm run build:css
```

## Validation Checklist
- [ ] All color utilities pre-defined in `@layer utilities`
- [ ] No arbitrary values in `.rs` files
- [ ] PostCSS build runs on CSS changes
- [ ] Utilities appear in compiled CSS
- [ ] Components render with correct colors

## References
- [Tailwind v4 Documentation](https://tailwindcss.com/docs)
- [PostCSS Documentation](https://postcss.org/)
- [Canon Rule #21: Canonical Color Tokens](./21-canonical-color-tokens.md)

## Lessons Learned
1. JIT scanners optimize for JavaScript/HTML, not Rust
2. Pre-compilation > Runtime generation for type-safe languages
3. PostCSS plugins > CLI tools for complex monorepos
4. Explicit utilities > Magic arbitrary values
