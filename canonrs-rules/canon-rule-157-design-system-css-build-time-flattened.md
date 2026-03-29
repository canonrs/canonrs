# Canon Rule #157: Design System CSS Must Be Build-Time Flattened

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** build, css, design-system
**Version:** 1.0.0  
**Date:** 2026-01-26

---

## Principle

**Applications MUST consume design system CSS as a single, flattened file with zero runtime `@import` statements.**

- Build tools (Tailwind, PostCSS) resolve imports at compile time
- Browsers receive flat CSS, not import chains
- Apps never serve "source" CSS with relative imports

---

## Problem

Without build-time flattening:

- **Silent import failures**: Browser ignores `@import "./ui/button.css"` if path breaks
- **Tokens never load**: Relative paths fail in served dist vs source
- **CSS appears loaded**: File downloads but imports fail silently
- **Components "have no style"**: HTML correct, classes exist, but rules missing

**Real symptom**: Button renders with correct `data-button` attribute but `rgba(0,0,0,0)` background. DevTools shows CSS file loaded but button rules absent.

---

## Forbidden Pattern

### ❌ Forbidden
```css
/* canonrs.css - served directly to browser */
@import "./tokens.css";
@import "./ui/button.css";
@import "./layouts/dashboard.css";

[data-button] {
  background: hsl(var(--primary));
}
```
```html
<!-- App serves source CSS directly -->
<link rel="stylesheet" href="/node_modules/@canonrs/design/style/canonrs.css" />
```

**Why forbidden:**
- `@import "./ui/button.css"` path is relative to CSS file location
- Browser resolves from request URL, not CSS file path
- `http://localhost:3000/ui/button.css` → 404
- Tokens defined in `tokens.css` never load
- CSS file "loads" (200 OK) but imports fail silently
- No browser error, no console warning

---

## Canonical Pattern

### ✅ Canonical
```css
/* style/main.css - app entry point */
@import "@canonrs/design/tokens.css";
@import "@canonrs/design/generated-themes.css";
@import "@canonrs/design/canonrs.css";
```
```bash
# Build flattens all imports
npx tailwindcss -i style/main.css -o public/style/output.css
```
```css
/* public/style/output.css - served to browser (flat) */
:root {
  --color-primary: 221 83% 53%;
  /* ... all tokens inline */
}

html[data-theme="amber"] {
  /* ... all themes inline */
}

[data-button] {
  /* ... all components inline */
}
/* Zero @import statements */
```
```html
<!-- App serves flattened artifact -->
<link rel="stylesheet" href="/style/output.css" />
```

**Why canonical:**
- Single HTTP request, zero subsequent imports
- All CSS in one file, correct cascade order
- No path resolution issues
- Works in any serving context (dev, prod, CDN)

---

## Rationale

**Browser `@import` limitations:**

1. **No module resolution**: `@import "@canonrs/design/tokens.css"` doesn't work (no node_modules)
2. **Relative path hell**: `./tokens.css` breaks when CSS served from different base URL
3. **Silent failures**: Invalid imports are ignored, not errored
4. **Performance**: Each import is blocking HTTP request

**Build-time flattening solves:**
- Resolves `@canonrs/*` aliases via PostCSS/Tailwind
- Concatenates all CSS in correct order
- Eliminates network waterfalls
- Enables tree-shaking and minification

**This is industry standard:**
- Tailwind, PostCSS, Vite, Webpack all flatten by default
- No production app serves CSS with `@import` chains
- SSR requires deterministic CSS (can't await imports)

---

## Enforcement

**Build pipeline:**
```json
{
  "scripts": {
    "build:css": "tailwindcss -i style/main.css -o public/style/output.css"
  }
}
```

**Validation:**
```bash
# Verify output has no @import
grep -c "@import" public/style/output.css && \
  echo "❌ Unresolved @import in output" && exit 1

# Verify output has actual rules
grep -c "\[data-button\]" public/style/output.css || \
  echo "❌ Components missing from output" && exit 1
```

**CI check:**
- `npm run build:css` must produce single file
- Output file must contain all component rules
- No `@import` statements in served CSS

---

## Exceptions

**No exceptions. This rule is absolute.**

Runtime `@import` is fundamentally incompatible with:
- Design system distribution
- SSR CSS injection
- Production performance requirements
- Reliable path resolution

If CSS needs dynamic loading, use code-splitting via JS modules, not CSS imports.

---

## Version History

- **1.0.0** — Initial version (2026-01-26)
