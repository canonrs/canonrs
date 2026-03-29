# Canon Rule #62: Single Source of Truth for Design Tokens

**Status:** ENFORCED


**Severity:** CRITICAL
**Scope:** design-system, tokens, architecture
**Version:** 1.0.0
**Date:** 2025-01-16

---
  

---

## The Principle

**Design tokens MUST exist in exactly ONE canonical location.**

All other references are **build artifacts** or **generated outputs**, never sources of truth.

---

## The Problem

### ❌ Wrong Pattern (Multiple Sources)
```
├── crates/rs-design/style/tokens.css          ← "Source 1"
├── packages-js/canonrs-design/tokens.css      ← "Source 2" 
├── examples/workbench/style/tokens.css        ← "Source 3"
└── frontend/styles/tokens.css                 ← "Source 4"
```

**Why this is wrong:**
- Tokens diverge between files
- Impossible to know which is authoritative
- Changes must be synchronized manually
- Theme changes require updates in 4+ places
- No clear ownership

**Symptoms:**
```bash
grep "color-background:" */tokens.css
# Returns 4 different values
```

---

## The Solution

### ✅ Correct Pattern (Single Source)
```
monorepo/
├── crates/rs-design/style/
│   ├── tokens.css       ← ÚNICA FONTE DA VERDADE
│   ├── main.css
│   └── theme.css
│
└── examples/workbench/
    ├── style/globals.css
    │   @import "/absolute/path/to/crates/rs-design/style/tokens.css"
    └── public/workbench.css  ← ARTIFACT (gerado via build)
```

**Single Source Rules:**
1. Tokens defined in: `crates/rs-design/style/tokens.css`
2. Apps import via **absolute paths** or **valid relative paths**
3. Generated CSS is artifact, never edited manually
4. Changes propagate via rebuild, not sync

---

## Architecture Layers

### Layer 1: Source of Truth (rs-design)
**Location:** `crates/rs-design/style/tokens.css`  
**Responsibility:** Define all design tokens
```css
/* ÚNICA FONTE DA VERDADE */
@theme inline {
  --color-background: 0 0% 100%;
  --color-foreground: 0 0% 3.9%;
  --color-primary: 38 92% 50%;
  --color-border: 0 0% 89.8%;
}
```

### Layer 2: Consumption (Applications)
**Location:** `examples/*/style/globals.css`  
**Responsibility:** Import canonical tokens
```css
/* NÃO define tokens, apenas importa */
@import "tailwindcss";
@import "/opt/docker/monorepo/opensource/canonrs/crates/rs-design/style/tokens.css";

@layer utilities {
  .bg-background { background-color: hsl(var(--color-background)); }
}
```

### Layer 3: Build Artifact (Generated)
**Location:** `examples/*/public/workbench.css`  
**Responsibility:** Compiled CSS for runtime
```bash
# Gerado via build, NUNCA editado manualmente
npx @tailwindcss/cli -i style/globals.css -o public/workbench.css
```

---

## Tailwind v4 Requirements

### ⚠️ Critical: Tailwind v4 Does NOT Resolve Node Modules

**This FAILS silently:**
```css
@import "@canonrs/design/tokens.css";  /* 🚫 Not resolved */
```

**This WORKS:**
```css
@import "/absolute/path/to/tokens.css";  /* ✅ Resolved */
```

**Why:** Tailwind v4 uses native CSS `@import`, which:
- Does NOT use Node module resolution
- Does NOT support `node_modules/` lookups
- Requires filesystem paths (absolute or valid relative)

---

## Implementation Checklist

When setting up design tokens:

- [ ] Tokens exist in ONE file: `crates/rs-design/style/tokens.css`
- [ ] Apps use **absolute paths** in `@import`
- [ ] `postcss.config.js` NOT used for path resolution (doesn't work)
- [ ] Build script generates `public/*.css` artifact
- [ ] Test: `grep "color-background:" public/*.css` returns values
- [ ] NO token definitions duplicated across files

---

## Health Check Commands
```bash
# 1. Verify single source exists
ls crates/rs-design/style/tokens.css
# ✅ Should exist

# 2. Verify no duplicates
find . -name "tokens.css" -type f
# ✅ Should return ONLY crates/rs-design/style/tokens.css

# 3. Verify build artifact has tokens
grep "color-background:" examples/workbench/public/workbench.css
# ✅ Should return: --color-background: 0 0% 100%;

# 4. Verify no @import remains unresolved
grep "@import.*tokens" examples/workbench/public/workbench.css
# ✅ Should return NOTHING (all imports processed)

# 5. Verify runtime CSS is served
curl http://localhost:3003/workbench.css | head -20
# ✅ Should return compiled CSS with tokens
```

---

## Anti-Patterns to Avoid

### 🚫 Copying Tokens to Multiple Files
```bash
# WRONG
cp crates/rs-design/style/tokens.css examples/app1/tokens.css
cp crates/rs-design/style/tokens.css examples/app2/tokens.css
```

### 🚫 Node Module Path Resolution
```css
/* WRONG - Tailwind v4 ignores this */
@import "@canonrs/design/tokens.css";
```

### 🚫 Editing Generated CSS
```bash
# WRONG
vim public/workbench.css  # 🚫 This is an artifact
```

### 🚫 Relative Paths Without Validation
```css
/* WRONG - breaks if file moves */
@import "../../tokens.css";  /* 🚫 Fragile */
```

---

## Correct Build Pipeline

### Step 1: Define Tokens (Once)
```css
/* crates/rs-design/style/tokens.css */
@theme inline {
  --color-background: 0 0% 100%;
}
```

### Step 2: Import in App (Absolute Path)
```css
/* examples/workbench/style/globals.css */
@import "tailwindcss";
@import "/opt/docker/monorepo/opensource/canonrs/crates/rs-design/style/tokens.css";
```

### Step 3: Build CSS (Generates Artifact)
```bash
npx @tailwindcss/cli \
  -i style/globals.css \
  -o public/workbench.css \
  --minify
```

### Step 4: Serve Artifact
```html
<!-- examples/workbench/index.html -->
<link rel="stylesheet" href="/workbench.css">
```

---

## Comparison: CanonRS vs Others

| Approach | Source Files | Build Required | Tailwind v4 Compatible |
|----------|--------------|----------------|------------------------|
| **CanonRS** | 1 (canonical) | ✅ Yes | ✅ Yes |
| Turborepo shared | Multiple | ✅ Yes | ⚠️ Needs config |
| Nx libraries | Multiple | ✅ Yes | ⚠️ Needs config |
| Copy-paste | Many (diverge) | ❌ No | ✅ Yes (but breaks) |

**Veredito:** CanonRS é **mais rigoroso** mas **menos frágil**.

---

## Debugging Guide

### Problem: "CSS has no tokens"
```bash
# Check if tokens are in source
cat crates/rs-design/style/tokens.css | grep "color-background"
# ✅ Should exist

# Check if build ran
ls -lh public/workbench.css
# ✅ Should exist and be recent

# Check if tokens made it to artifact
grep "color-background" public/workbench.css
# ✅ Should return token definition
```

### Problem: "@import not resolved"
```bash
# Check import path
cat style/globals.css | grep "@import.*tokens"
# ❌ If shows @import, path is wrong

# Fix: Use absolute path
@import "/opt/docker/monorepo/opensource/canonrs/crates/rs-design/style/tokens.css";
```

### Problem: "Changes don't appear"
```bash
# Rebuild CSS
npx @tailwindcss/cli -i style/globals.css -o public/workbench.css

# Hard refresh browser
Ctrl+Shift+R
```

---

## Related Rules

- **Rule #1:** Design System as Source of Truth
- **Rule #64:** CSS Build Pipeline is Mandatory
- **Rule #65:** data-theme Sync Responsibility

---

## Normative Status

- Violations **MUST** block PRs
- Multiple token sources **MUST NOT** exist
- Apps **MUST** import from canonical location
- Build artifacts **MUST NOT** be committed to git (add to `.gitignore`)
- Token changes **MUST** propagate via rebuild only

---

**Author:** Canon Working Group  
**Replaces:** None

---

## Economic Impact

**Time saved per incident:** ~2 hours  
**Frequency without rule:** Every new app setup  
**Annual savings (10 apps):** ~20 hours

**Root causes eliminated:**
- ❌ Token divergence
- ❌ "Which file is source?"
- ❌ Sync failures
- ❌ Tailwind import errors
