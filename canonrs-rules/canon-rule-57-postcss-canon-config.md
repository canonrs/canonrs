# Canon Rule #57: PostCSS Canon Configuration

**Status:** ENFORCED


**Severity:** MEDIUM
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** build-tooling
**Tags:** postcss, css, tailwind, config
**Language:** EN

---

**Intro:**
Incorrect PostCSS configuration breaks module resolution and CSS ordering in monorepos. Canon setup requires ESM config, proper plugin order, and strict import sequencing.

**Problem:**
postcss misconfiguration causes unresolved imports and broken css order

**Solution:**
use esm config with postcss import first and enforce canonical ordering

**Signals:**
- tailwind import error
- css order broken
- module resolution fail

**Search Intent:**
how to configure postcss esm with

**Keywords:**
postcss esm config, tailwind postcss plugin order, css import resolution node, postcss canonical setup

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
