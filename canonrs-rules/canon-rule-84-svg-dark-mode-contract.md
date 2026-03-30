# Canon Rule #84: SVG Dark Mode Contract

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-01-13

**Category:** styling-css
**Tags:** svg, dark-mode, css
**Language:** EN

---

**Intro:**
SVG files loaded via img tags do not inherit CSS properties like currentColor, causing incorrect colors in dark mode. Proper strategies are required to ensure theming compatibility.

**Problem:**
external svg images do not inherit css color leading to dark mode issues

**Solution:**
use css filters inline svg or masks to control svg coloring

**Signals:**
- icons stay black
- dark mode contrast issues
- svg color mismatch

**Search Intent:**
why svg not adapting to dark mode

**Keywords:**
svg currentcolor not working, img svg css inheritance, dark mode svg fix, svg color css filter

---

## Principle

SVG assets referenced via `<img>` tags **CANNOT** inherit `currentColor` or CSS properties.

> **`<img src="*.svg">` creates an isolated document context.**
> Color inheritance requires inline SVG or CSS filters.

---

## The Problem

Developers assume SVG strokes will adapt to dark mode by using `stroke="currentColor"`:
```svg
<!-- ❌ DOES NOT WORK with <img> -->
<svg>
  <path stroke="currentColor" />
</svg>
```
```html
<!-- ❌ currentColor is NOT inherited -->
<img src="icon.svg" />
```

**Result:** Icons remain black in dark mode, creating poor contrast and broken UX.

---

## Root Cause

`<img>` treats SVG as an **external document**:
- No access to parent CSS context
- No inheritance of `color` property
- `currentColor` resolves to SVG's internal default (black)

---

## Canonical Solutions

### Solution 1: CSS Filter (Recommended for `<img>`)

Use `filter: invert()` to adapt existing black strokes to dark mode.
```css
.icon {
  width: 24px;
  height: 24px;
}

html.dark .icon {
  filter: invert(1) hue-rotate(180deg);
}
```

**Pros:**
- Works with `<img>` tags
- No SVG file modification
- Simple implementation

**Cons:**
- Inverts ALL colors (including fills)
- May need `hue-rotate()` adjustments
- Not precise color control

---

### Solution 2: Inline SVG (Most Control)

Embed SVG directly in HTML/components to inherit CSS context.
```rust
// Leptos component
view! {
  <svg class="icon" viewBox="0 0 24 24">
    <path stroke="currentColor" d="..." />
  </svg>
}
```
```css
.icon {
  color: var(--color-fg-default);
  width: 24px;
  height: 24px;
}
```

**Pros:**
- Full CSS inheritance
- Precise color control
- Token-based theming

**Cons:**
- Larger bundle size
- Cannot cache SVG separately
- More verbose code

---

### Solution 3: CSS Mask (Advanced)

Use SVG as a mask with CSS background color.
```css
.icon {
  width: 24px;
  height: 24px;
  background: var(--color-fg-default);
  mask: url('/icons/check.svg') no-repeat center;
  mask-size: contain;
}
```

**Pros:**
- `<img>`-like simplicity
- Full color control via CSS
- Caches like normal images

**Cons:**
- Only works for single-color icons
- Requires mask-compatible SVG
- Less browser support (older IE)

---

## Decision Matrix

| Use Case | Solution | Reason |
|----------|----------|--------|
| Simple icons, black strokes | CSS Filter | Easiest, works with `<img>` |
| Token-based theming | Inline SVG | Full design system integration |
| Performance-critical | CSS Mask | Best of both worlds |
| Multi-color illustrations | Keep as `<img>`, manual dark variants | Filters distort complex colors |

---

## Anti Patterns

### ❌ Anti-Pattern 1: `currentColor` in External SVG
```svg
<!-- icons/check.svg -->
<svg>
  <path stroke="currentColor" />
</svg>
```
```html
<img src="/icons/check.svg" class="icon" />
```
**Problem:** `currentColor` resolves to black, ignoring parent CSS.

---

### ❌ Anti-Pattern 2: Hardcoded Colors
```svg
<svg>
  <path stroke="#000000" />
</svg>
```
**Problem:** Cannot adapt to any theme or dark mode.

---

### ❌ Anti-Pattern 3: JavaScript Color Injection
```js
fetch('icon.svg')
  .then(r => r.text())
  .then(svg => {
    const colored = svg.replace('black', theme.color);
    element.innerHTML = colored;
  });
```
**Problem:** Bypasses SSR, breaks hydration, performance overhead.

---

## Canonical Implementation

### Step 1: Choose Strategy
For icon systems with `<img>` tags, use **CSS Filter**.

### Step 2: Create CSS
```css
/* /style/components/icon-strip.css */
.icon-item {
  display: flex;
  flex-direction: column;
  align-items: center;
}

.icon-image {
  width: 24px;
  height: 24px;
}

html.dark .icon-image {
  filter: invert(1) hue-rotate(180deg);
}
```

### Step 3: Ensure Import Order
```css
/* /style/canonrs.css */
@import "./components/icon-strip.css";
```

### Step 4: Verify Compilation
```bash
npm run build:css
grep "html.dark .icon-image" public/workbench.css
```

---

## Testing Protocol

1. **Light Mode Test**
   - Icons appear black with good contrast
   - No color distortion

2. **Dark Mode Test**
   - Icons appear white/light with good contrast
   - Filter does not affect surrounding elements

3. **Edge Cases**
   - Multi-color SVGs may need manual variants
   - Gradients may require inline approach

---

## Performance Considerations

- **CSS Filter:** Minimal performance impact
- **Inline SVG:** Increases HTML size, but enables caching at component level
- **CSS Mask:** Similar to `<img>`, good caching

---

## Canonical Justification

> **External SVGs are isolated documents.**
> They cannot inherit CSS context by design for security and encapsulation.

This rule enforces:
- **Predictable theming** — Dark mode always works
- **Performance** — Choose right strategy for use case
- **Maintainability** — No runtime color injection hacks

---

## Canon References

- Canon Rule #7 — Token Governance
- Canon Rule #21 — Canonical Color Tokens
- Canon Rule #58 — Leptos Assets Dev Constraint
- Canon Rule #69 — Trunk Only Serves What's in dist/