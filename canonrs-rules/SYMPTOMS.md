# Canon Rules — Symptom-Oriented Debug Guide

This file maps **real-world symptoms** to the **correct Canon Rules**  
and defines the **mandatory debug order**.

Do NOT jump between symptoms.
Pick the one that matches reality.

---

## 🟥 Symptom: Page Loads but No Canon Styles Applied

### Decision Trigger
UI renders with Tailwind base styles, but Canon tokens/themes missing.

### Mandatory Debug Order
1. **Rule #55** — Verify imports use `@canonrs/*` not relative paths
2. **Rule #56** — Confirm dist/ files exist in canonical packages
3. **Rule #57** — Check PostCSS config has `postcss-import`
4. **Rule #61** — Grep for forbidden `../../../../` imports
5. **Rule #60** — Verify artifacts weren't manually edited

⚠️ Do NOT debug React/Leptos components before step 1.

---

## 🟥 Symptom: PostCSS Warnings "@import must precede"

### Decision Trigger
Build succeeds but PostCSS warns about import order.

### Mandatory Debug Order
1. **Rule #57** — Verify `@import "@canonrs/*"` comes BEFORE `@import "tailwindcss"`
2. **Rule #55** — Check all Canon imports are at top of file
3. **Rule #61** — Ensure no relative imports mixed in

Canon imports → Tailwind import → Everything else

---

## 🟥 Symptom: CSS 404 or Cannot Resolve @canonrs

### Decision Trigger
Build fails with "Cannot resolve @canonrs/tailwind/tokens.css"

### Mandatory Debug Order
1. **Rule #55** — Verify canonical packages exist in `packages/`
2. **Rule #56** — Check `dist/*.css` files exist
3. **Rule #55** — Verify `package.json` exports match
4. **Rule #57** — Confirm PostCSS can resolve workspace packages

---

## 🟥 Symptom: Leptos Dev Server Returns 1-byte CSS

### Decision Trigger
CSS request returns but file is empty or 1 byte.

### Mandatory Debug Order
1. **Rule #58** — Verify CSS is in `public/*.css` NOT `public/pkg/*.css`
2. **Rule #58** — Confirm build copies to first-level public/
3. **Rule #56** — Check build script output location

Leptos ONLY serves first-level files from assets-dir.

---

## 🟥 Symptom: Works in Production but Not Dev

### Decision Trigger
Build output serves correctly, dev server doesn't.

### Mandatory Debug Order
1. **Rule #58** — Leptos dev behavior differs from prod
2. **Rule #56** — Verify dev build also populates public/
3. **Rule #60** — Confirm no manual edits to dist/

---

## 🟥 Symptom: Canon CSS Present but Tokens Don't Apply

### Decision Trigger
CSS loaded, variables defined, but components unstyled.

### Mandatory Debug Order
1. **Rule #59** — Check CSS cascade order
2. **Rule #60** — Verify tokens.css not overwritten
3. **Rule #55** — Confirm all three Canon files imported:
   - tokens.css
   - globals.css
   - themes.css

---

## 🟥 Symptom: New App Setup Confusion

### Decision Trigger
Starting fresh, unsure what order to set things up.

### Mandatory Action
➡️ **Apply Rules #55, #56, #58 in order**

1. Create canonical packages (Rule #55)
2. Set up build pipeline (Rule #56)
3. Configure Leptos assets (Rule #58)
4. Import using canonical paths (Rule #55)

---

## Absolute Rule

> **If canonical packages don't exist or imports use relative paths, the bug is NOT in your code.**

This is enforced by:
- Rule #55
- Rule #56
- Rule #61

---

## How to Use This File

1. Match symptom exactly
2. Follow debug order strictly
3. Apply rules in listed sequence
4. Do not use relative CSS imports ever

This document is designed for:
- Humans under stress
- AI systems with partial context
- Deterministic debugging

It is not optional reading.
