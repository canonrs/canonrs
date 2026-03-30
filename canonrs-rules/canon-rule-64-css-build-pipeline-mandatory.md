# Canon Rule #64: CSS Build Pipeline is Mandatory

**Status:** ENFORCED


**Severity:** CRITICAL
**Scope:** build, css
**Version:** 1.0.0
**Date:** 2025-01-16

---
  

---

## The Principle

**CSS is a build-time artifact, not a runtime feature.**

Tailwind v4 does NOT run in the browser. A build step is **mandatory**.

---

## The Problem

### ❌ Wrong Assumption (No Build)
```bash
# Developer assumes:
cargo leptos serve
# → CSS auto-generates ✅

# Reality:
cargo leptos serve
# → Only compiles Rust
# → NO CSS generated 🚫
```

**What happens:**
```html
<!-- HTML references CSS -->
<link rel="stylesheet" href="/workbench.css">

<!-- But file doesn't exist -->
$ ls public/workbench.css
ls: cannot access 'public/workbench.css': No such file or directory
```

**Symptoms:**
- White unstyled page
- Tailwind classes in HTML but no styling
- `getComputedStyle()` returns empty values
- No errors in console (CSS just missing)

---

## The Solution

### ✅ Correct Pipeline (Explicit Build)
```bash
# Step 1: Generate CSS (MANDATORY)
npx @tailwindcss/cli \
  -i style/globals.css \
  -o public/workbench.css \
  --minify

# Step 2: Compile Rust
cargo leptos build

# Step 3: Serve
cargo leptos serve
```

---

## Why Cargo Leptos Does Not Generate CSS

### Leptos Responsibility
- ✅ Compiles Rust → WASM
- ✅ Bundles JavaScript
- ✅ Sets up SSR server
- ❌ Does NOT run Tailwind
- ❌ Does NOT process PostCSS
- ❌ Does NOT generate CSS

### Tailwind v4 Requirement
**Tailwind is NOT a runtime library.**

It's a **build tool** that:
1. Reads your CSS input file
2. Scans your source files for classes
3. Generates optimized CSS output
4. Exits

**No build = No CSS = No styling.**

---

## Correct Setup Step By Step

### 1. Install Tailwind CLI
```bash
npm install -D @tailwindcss/cli@latest
```

### 2. Create Input CSS
```css
/* style/globals.css */
@import "tailwindcss";
@import "/absolute/path/to/crates/rs-design/style/tokens.css";

@layer utilities {
  .bg-background { background-color: hsl(var(--color-background)); }
}
```

### 3. Add Build Script
```json
// package.json
{
  "scripts": {
    "build:css": "tailwindcss -i style/globals.css -o public/workbench.css --minify",
    "watch:css": "tailwindcss -i style/globals.css -o public/workbench.css --watch"
  }
}
```

### 4. Configure Leptos Assets
```toml
# Leptos.toml (optional but recommended)
[site]
assets-dir = "public"
```

### 5. Link CSS in HTML
```rust
// src/app.rs
view! {
    <link rel="stylesheet" href="/workbench.css"/>
}
```

### 6. Build Workflow
```bash
# Development
npm run watch:css &  # Background process
cargo leptos serve

# Production
npm run build:css
cargo leptos build --release
```

---

## Health Check Commands
```bash
# 1. Verify Tailwind CLI installed
npx @tailwindcss/cli --help
# ✅ Should show help text

# 2. Build CSS manually
npx @tailwindcss/cli -i style/globals.css -o public/workbench.css
# ✅ Should complete without errors

# 3. Verify output exists
ls -lh public/workbench.css
# ✅ Should exist and have size > 0

# 4. Verify output has content
head -20 public/workbench.css
# ✅ Should show CSS rules

# 5. Verify tokens present
grep "color-background" public/workbench.css
# ✅ Should return token definitions

# 6. Test runtime serving
curl http://localhost:3003/workbench.css | head -20
# ✅ Should return CSS content
```

---

## Common Mistakes

### ❌ Mistake 1: Assuming cargo leptos Handles CSS
```bash
# WRONG: Expects CSS to auto-generate
cargo leptos serve
# 🚫 No CSS created
```

**Fix:**
```bash
npm run build:css  # Generate CSS first
cargo leptos serve
```

### ❌ Mistake 2: Using Wrong Tailwind Package
```bash
# WRONG: Old Tailwind v3
npm install tailwindcss
```

**Fix:**
```bash
# CORRECT: Tailwind v4 CLI
npm install @tailwindcss/cli
```

### ❌ Mistake 3: No Watch Mode
```bash
# WRONG: Must rebuild manually on CSS changes
npm run build:css
# Edit style/globals.css
# ... no update
```

**Fix:**
```bash
# CORRECT: Auto-rebuild on changes
npm run watch:css &  # Runs in background
```

### ❌ Mistake 4: Wrong Output Path
```bash
# WRONG: CSS generated in wrong location
tailwindcss -o dist/output.css

# HTML links to different path
<link href="/public/workbench.css">  # 404
```

**Fix:**
```bash
# Align output path with HTML link
tailwindcss -o public/workbench.css
```

---

## Debugging Guide

### Problem: "Page has no styling"

**Check 1: Does CSS file exist?**
```bash
ls public/workbench.css
# If missing → Run build:css
```

**Check 2: Is CSS being served?**
```bash
curl http://localhost:3003/workbench.css
# If 404 → Check assets-dir in Leptos.toml
```

**Check 3: Does CSS have content?**
```bash
wc -l public/workbench.css
# If 0 lines → Build failed, check errors
```

**Check 4: Are tokens present?**
```bash
grep "color-background" public/workbench.css
# If missing → Check @import paths
```

### Problem: "CSS outdated after changes"

**Solution: Use watch mode**
```bash
# Kill old process
pkill -f "tailwindcss.*watch"

# Start new watch
npm run watch:css &
```

### Problem: "Build succeeds but CSS empty"

**Cause: Input file has errors**
```bash
# Check for syntax errors
npx @tailwindcss/cli -i style/globals.css -o /tmp/test.css
# Read error output carefully
```

---

## Integration With Hot Reload

### Leptos Hot Reload (Rust Only)
```bash
cargo leptos watch
# ✅ Reloads on .rs changes
# ❌ Does NOT reload on .css changes
```

### CSS Hot Reload (Separate Process)
```bash
npm run watch:css
# ✅ Rebuilds on .css changes
# ⚠️ Browser needs manual refresh (Ctrl+Shift+R)
```

### Full Stack Development
```bash
# Terminal 1: CSS watch
npm run watch:css

# Terminal 2: Rust watch
cargo leptos watch

# Edit .rs → Auto reload
# Edit .css → Manual refresh
```

---

## Production Build
```bash
#!/bin/bash
# build.sh

set -e

echo "Building CSS..."
npm run build:css

echo "Building Rust..."
cargo leptos build --release

echo "Build complete!"
echo "CSS: public/workbench.css"
echo "Binary: target/release/my-app"
```

---

## Anti Patterns

### 🚫 No Build Script
```json
// WRONG: No CSS build defined
{
  "scripts": {}
}
```

### 🚫 Committing Generated CSS
```bash
# WRONG: Generated files in git
git add public/workbench.css
```

**Fix:**
```bash
# Add to .gitignore
echo "public/*.css" >> .gitignore
```

### 🚫 Manual CSS Editing
```bash
# WRONG: Editing generated file
vim public/workbench.css  # 🚫 Gets overwritten
```

---

## Comparison

| Framework | CSS Auto-Generated | Build Required | Watch Mode |
|-----------|-------------------|----------------|------------|
| **CanonRS (Leptos)** | ❌ No | ✅ Yes | ✅ Separate |
| Next.js | ✅ Yes | ❌ No (built-in) | ✅ Built-in |
| SvelteKit | ✅ Yes | ❌ No (built-in) | ✅ Built-in |
| Remix | ✅ Yes | ❌ No (built-in) | ✅ Built-in |

**Veredito:** Leptos requires **explicit CSS pipeline** (not worse, just different).

---

## Related Rules

- **Rule #62:** Single Source of Truth for Design Tokens
- **Rule #65:** data-theme Sync Responsibility

---

## Normative Status

- New Leptos apps **MUST** have `build:css` script in `package.json`
- CSS output **MUST** be in `.gitignore`
- Production builds **MUST** run CSS build before Rust build
- Documentation **MUST** include CSS build step
- PRs adding Tailwind **MUST** include build pipeline

---

**Author:** Canon Working Group  
**Replaces:** None

---

## Economic Impact

**Time saved per incident:** ~30 minutes  
**Frequency without rule:** Every new app setup  
**Annual savings (10 apps):** ~5 hours

**Root causes eliminated:**
- ❌ "Why is my page white?"
- ❌ "Tailwind classes not working"
- ❌ Assuming cargo leptos handles CSS
- ❌ Missing build:css script
