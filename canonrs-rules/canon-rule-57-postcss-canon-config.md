# Canon Rule #57: PostCSS Canon Configuration

**Status:** ENFORCED


**Severity:** MEDIUM
**Scope:** build, css
**Version:** 1.0.0
**Date:** 2025-01-16

---

---

## Principle

PostCSS in Canon projects **MUST**:

1. Resolve workspace packages
2. Be ESM-native
3. Respect strict CSS import ordering
4. Treat Tailwind as a PostCSS plugin (not CSS)

---

## Canonical Configuration

### `postcss.config.js` (ESM REQUIRED)

```js
import { createRequire } from "node:module";
import postcssImport from "postcss-import";
import tailwindcss from "@tailwindcss/postcss";
import autoprefixer from "autoprefixer";

const require = createRequire(import.meta.url);

export default {
  plugins: [
    postcssImport({
      resolve(id) {
        return require.resolve(id);
      }
    }),
    tailwindcss,
    autoprefixer
  ]
};
```

---

## Mandatory CSS Ordering

```css
/* 1. IMPORTS FIRST */
@import "@canonrs/tailwind/tokens.css";
@import "@canonrs/design/themes.css";

/* 2. CONFIG */
@config "../tailwind.config.js";

/* 3. TAILWIND */
@tailwind base;
@tailwind components;
@tailwind utilities;

/* 4. LAYERS */
@layer utilities { }
```

---

## Absolute Prohibitions

```css
/* ❌ NEVER */
@import "tailwindcss";
@tailwind base;
@import "@canonrs/tailwind/tokens.css";
```

This will **always** break.

---

## Enforcement Checklist

- [ ] `type: module` in package.json
- [ ] `createRequire` used
- [ ] postcss-import FIRST
- [ ] No JS imported as CSS

