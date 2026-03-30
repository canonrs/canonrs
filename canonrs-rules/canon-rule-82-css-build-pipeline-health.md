# Canon Rule #82: CSS Build Pipeline Health

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-01-12

**Category:** build-tooling
**Tags:** css, pipeline, postcss
**Language:** EN

---

**Intro:**
CSS changes do not propagate automatically without explicit compilation, leading to stale styles in development. The build pipeline must be executed and validated for every change.

**Problem:**
css changes not reflected because build pipeline is not executed

**Solution:**
run css build pipeline and verify compiled output before testing

**Signals:**
- stale styles
- css not updating
- postcss not triggered

**Search Intent:**
why css changes not applying in leptos

**Keywords:**
postcss build required, css not updating leptos, tailwind build pipeline issue, compiled css not refreshed

---

## Principle

CSS changes MUST be **explicitly compiled** before they take effect in the application.

The build pipeline is NOT automatic. Changes to:
- `canonrs.css`
- `tokens.css`
- `theme.css`
- Any imported CSS

**REQUIRE** running `npm run build:css` before `cargo leptos watch` will reflect them.

---

## Forbidden Assumptions
```bash
# ❌ WRONG WORKFLOW

# 1. Edit canonrs.css
vim packages-rust/rs-design/style/canonrs.css

# 2. Reload browser
# Expectation: Changes appear
# Reality: Nothing happens ❌
```

### Why This Fails

1. **Leptos serves static CSS**
   - CSS is compiled to `public/workbench.css`
   - Browser loads compiled artifact
   - Source changes don't affect artifact

2. **PostCSS is not automatic**
   - PostCSS runs via npm script
   - cargo-leptos doesn't trigger npm
   - Watch mode only monitors Rust files

3. **Cached artifacts persist**
   - Old CSS stays in memory
   - Hard refresh (Ctrl+Shift+R) needed
   - Multiple layers of caching

---

## Canonical Build Order
```bash
# ✅ CORRECT WORKFLOW

# 1. Edit CSS source
vim packages-rust/rs-design/style/canonrs.css

# 2. Compile CSS (MANDATORY)
cd examples/_wip/workbench
npm run build:css

# 3. Verify compilation
grep "sidebar-root" public/workbench.css  # Should find new rules

# 4. Hard refresh browser
# Ctrl + Shift + R (or Cmd + Shift + R on Mac)

# 5. If still not working, restart cargo-leptos
pkill -f "cargo leptos"
make dev
```

---

## CSS Build Pipeline Architecture
```
Source Files                    Build Process              Output
─────────────────────────────────────────────────────────────────────
canonrs.css                 →   PostCSS                 →  workbench.css
tokens.css                  →   @import resolution      →  (compiled)
theme.css                   →   Tailwind v4             →
tailwind.css (entry)        →   Token substitution      →
                                                            ↓
                                                         public/
                                                         └─ workbench.css
                                                            (served to browser)
```

**Critical path:** Source → PostCSS → Output → Browser

**Break at any step = changes invisible**

---

## PostCSS Configuration Requirements

### package.json Scripts
```json
{
  "scripts": {
    "build:css": "postcss style/tailwind.css -o public/workbench.css",
    "watch:css": "postcss style/tailwind.css -o public/workbench.css --watch"
  }
}
```

### postcss.config.js
```javascript
export default {
  plugins: {
    'postcss-import': {
      resolve: (id) => {
        if (id === '@canonrs/tailwind/tokens.css') {
          return path.resolve(__dirname, '../../../../../packages-rust/rs-design/style/tokens.css');
        }
        // ... other resolvers
        return id;
      }
    },
    '@tailwindcss/postcss': {}
  }
}
```

**Critical:** Resolvers MUST use correct relative paths.

---

## Verification Steps

### Step 1: Verify Source Change
```bash
# Check that your edit is saved
cat packages-rust/rs-design/style/canonrs.css | grep "your-new-rule"
# Should return your change
```

### Step 2: Verify Compilation
```bash
# Run build
npm run build:css

# Check output
grep "your-new-rule" public/workbench.css
# Should return your change in compiled CSS
```

### Step 3: Verify Browser Loaded New CSS
```javascript
// DevTools Console
const link = document.querySelector('link[href*="workbench.css"]');
console.log('CSS URL:', link.href);

// Force reload CSS
link.href = link.href.split('?')[0] + '?v=' + Date.now();
```

### Step 4: Verify Rule Applied
```javascript
// Check computed styles
const element = document.querySelector('.sidebar-root');
const styles = getComputedStyle(element);
console.log('Your property:', styles.getPropertyValue('your-property'));
```

---

## Common Build Failures

### Failure 1: PostCSS Path Resolution
```bash
# ❌ ERROR
[Error: ENOENT: no such file or directory, stat '.../tokens.css']
```

**Cause:** Incorrect path in postcss.config.js
**Solution:** Use `path.resolve(__dirname, 'correct/relative/path')`

### Failure 2: Import Order
```bash
# ❌ ERROR
PostCSS: @import must precede all other statements
```

**Cause:** `@import` after CSS rules
**Solution:** Move all `@import` to top of file

### Failure 3: Missing Output Directory
```bash
# ❌ ERROR
ENOENT: no such file or directory, open 'public/workbench.css'
```

**Cause:** `public/` folder doesn't exist
**Solution:** `mkdir -p public`

### Failure 4: Stale Cache
```bash
# Changes compiled but not visible in browser
```

**Cause:** Browser cached old CSS
**Solution:** Hard refresh (Ctrl+Shift+R)

---

## Automated Watch
```bash
# Terminal 1: Watch CSS
npm run watch:css

# Terminal 2: Watch Rust
make dev
```

**Benefits:**
- Automatic recompilation on CSS changes
- Faster iteration
- Fewer manual steps

**Limitations:**
- Still requires browser refresh
- Doesn't catch PostCSS config errors
- Can get out of sync with Rust watch

---

## Build Health Checks

### Pre-flight Checklist
```bash
# Run before starting development
cd examples/_wip/workbench

# 1. Check PostCSS installed
npx postcss --version  # Should return version number

# 2. Check output directory exists
ls -la public/  # Should show workbench.css

# 3. Run initial build
npm run build:css  # Should complete without errors

# 4. Verify output size
ls -lh public/workbench.css  # Should be >50KB (has content)

# 5. Check for syntax errors
grep "syntax error" public/workbench.css  # Should return nothing
```

### Runtime Health Check
```javascript
// Add to browser console for debugging
function checkCSSHealth() {
    const link = document.querySelector('link[href*="workbench.css"]');
    
    if (!link) {
        console.error('❌ workbench.css not loaded');
        return;
    }
    
    fetch(link.href)
        .then(r => r.text())
        .then(css => {
            console.log('✓ CSS size:', css.length, 'bytes');
            console.log('✓ Contains tokens:', css.includes('--sidebar-width'));
            console.log('✓ Contains canon:', css.includes('.canon-page'));
        });
}
```

---

## Error Recovery Procedure
```bash
# If CSS is completely broken:

# 1. Clean everything
rm -rf public/workbench.css
rm -rf node_modules/.cache

# 2. Reinstall dependencies
npm install

# 3. Rebuild from scratch
npm run build:css

# 4. Verify output
ls -lh public/workbench.css
head -50 public/workbench.css

# 5. Restart dev server
pkill -f "cargo leptos"
make dev

# 6. Hard refresh browser
# Ctrl + Shift + R
```

---

## Build Order Dependencies
```
CSS Source Change
    ↓
npm run build:css (MUST RUN)
    ↓
public/workbench.css updated
    ↓
Browser hard refresh (MUST DO)
    ↓
New styles visible
```

**Any step skipped = changes invisible**

---

## Enforcement Checklist

- [ ] PostCSS installed (`npm list postcss`)
- [ ] Build script exists in package.json
- [ ] Output directory exists (`public/`)
- [ ] postcss.config.js has correct paths
- [ ] Initial build runs without errors
- [ ] Output file has content (>50KB)
- [ ] Browser loads from `public/` directory

---

## Canonical Justification

**Design systems are compiled artifacts.**

Expecting CSS to "just work" without compilation is like expecting Rust to run without `cargo build`.

The pipeline exists to:
- Resolve imports
- Apply transformations
- Optimize output
- Ensure consistency

**Canon mandates:** Explicit compilation over implicit magic.

---

## Canon References

- Canon Rule #55 — Canonical CSS Entry Points
- Canon Rule #56 — Monorepo CSS Build Pipeline
- Canon Rule #57 — PostCSS Canon Config
- Canon Rule #64 — CSS Build Pipeline Mandatory

---

## Related Symptoms

If you see:
- CSS changes don't appear in browser
- "File not found" errors in PostCSS
- Stale styles persist after edits
- Some tokens work, others don't
- Build succeeds but browser shows old CSS

→ **This rule is violated.**

Go to: **SYMPTOMS.md → CSS BUILD PIPELINE FAILURES**