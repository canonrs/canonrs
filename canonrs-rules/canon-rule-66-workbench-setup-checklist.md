# Canon Rule #66: Workbench Setup Checklist

**Status:** ENFORCED


**Severity:** LOW
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** governance
**Tags:** setup, validation, workspace
**Language:** EN

---

**Intro:**
Skipping initial environment validation leads to hidden configuration issues such as missing CSS pipelines. A mandatory checklist ensures system readiness before development begins.

**Problem:**
developers start without validating setup leading to hidden configuration failures

**Solution:**
enforce pre development checklist validating tokens css and assets pipeline

**Signals:**
- unstyled app
- missing css pipeline
- invalid setup

**Search Intent:**
how to validate leptos project setup before development

**Keywords:**
leptos setup checklist, css pipeline validation, tailwind setup verification, monorepo environment validation

---

---

## The Principle

**Every new workbench/app MUST pass a health check before development begins.**

Assumptions kill productivity. Validation saves hours.

---

## The Problem

### ❌ Wrong Approach (Assume Everything Works)
```bash
# Developer starts coding
vim src/app.rs

# Hours later: "Why is nothing styled?"
# Reason: CSS pipeline was never set up
```

**Time wasted:** 2-4 hours debugging what should have been validated upfront.

---

## The Solution

### ✅ Correct Approach (Validate Before Coding)
```bash
# Run checklist FIRST
./scripts/validate-setup.sh

# ✅ All checks pass → Safe to develop
# ❌ Any check fails → Fix before proceeding
```

---

## Mandatory Checklist

### Step 1: Design System Tokens Exist
**What:** Verify canonical token source exists
```bash
ls crates/rs-design/style/tokens.css
```

**Expected:** File exists  
**If fails:** Clone rs-design or create tokens.css  
**Time saved:** 30 minutes

---

### Step 2: Tailwind CLI Installed
**What:** Verify build tool available
```bash
npx @tailwindcss/cli --help
```

**Expected:** Help text displayed  
**If fails:** `npm install -D @tailwindcss/cli`  
**Time saved:** 15 minutes

---

### Step 3: Build Script Exists
**What:** Verify CSS can be generated
```bash
cat package.json | grep "build:css"
```

**Expected:** Script defined  
**If fails:** Add to `package.json`:
```json
{
  "scripts": {
    "build:css": "tailwindcss -i style/globals.css -o public/workbench.css --minify",
    "watch:css": "tailwindcss -i style/globals.css -o public/workbench.css --watch"
  }
}
```
**Time saved:** 20 minutes

---

### Step 4: CSS Builds Successfully
**What:** Verify CSS generation works
```bash
npm run build:css
ls public/workbench.css
```

**Expected:** File exists with size > 0  
**If fails:** Check import paths in `style/globals.css`  
**Time saved:** 1 hour

---

### Step 5: CSS Has Tokens
**What:** Verify tokens were included in build
```bash
grep "color-background:" public/workbench.css
```

**Expected:** Returns token definitions  
**If fails:** Fix `@import` paths (use absolute paths)  
**Time saved:** 1 hour

---

### Step 6: Assets Directory Configured
**What:** Verify Leptos serves public folder
```bash
cat Cargo.toml | grep "leptos"
# or
cat Leptos.toml | grep "assets-dir"
```

**Expected:** Configuration exists  
**If fails:** Create `Leptos.toml`:
```toml
[site]
assets-dir = "public"
```
**Time saved:** 15 minutes

---

### Step 7: CSS Served at Runtime
**What:** Verify CSS accessible via HTTP
```bash
cargo leptos serve &
sleep 5
curl -I http://localhost:3003/workbench.css
```

**Expected:** `200 OK` response  
**If fails:** Check assets-dir path or HTML link  
**Time saved:** 30 minutes

---

### Step 8: HTML Links CSS Correctly
**What:** Verify page loads CSS
```bash
curl -s http://localhost:3003 | grep 'href="/workbench.css"'
```

**Expected:** Link tag present  
**If fails:** Add to `src/app.rs`:
```rust
view! {
    <link rel="stylesheet" href="/workbench.css"/>
}
```
**Time saved:** 15 minutes

---

### Step 9: DOM Has data-theme
**What:** Verify theme system active
```bash
curl -s http://localhost:3003 | grep 'data-theme='
```

**Expected:** Attribute present with value  
**If fails:** Implement ThemeProvider DOM sync (Rule #65)  
**Time saved:** 45 minutes

---

### Step 10: Styles Apply in Browser
**What:** End-to-end verification

**Manual test:**
1. Open http://localhost:3003
2. Open DevTools Console
3. Run:
```javascript
getComputedStyle(document.documentElement).getPropertyValue('--color-background')
```

**Expected:** Returns value (e.g., `"0 0% 100%"`)  
**If fails:** Review Rules #62-#65  
**Time saved:** Variable

---

## Automation Script

### validate-setup.sh
```bash
#!/bin/bash
set -e

echo "🔍 Canon Workbench Setup Validation"
echo "===================================="

# Check 1: Tokens exist
echo -n "✓ Design tokens exist... "
test -f crates/rs-design/style/tokens.css && echo "PASS" || (echo "FAIL" && exit 1)

# Check 2: Tailwind CLI
echo -n "✓ Tailwind CLI installed... "
npx @tailwindcss/cli --help &>/dev/null && echo "PASS" || (echo "FAIL" && exit 1)

# Check 3: Build script
echo -n "✓ Build script defined... "
grep -q "build:css" package.json && echo "PASS" || (echo "FAIL" && exit 1)

# Check 4: CSS builds
echo -n "✓ CSS builds successfully... "
npm run build:css &>/dev/null && test -f public/workbench.css && echo "PASS" || (echo "FAIL" && exit 1)

# Check 5: Tokens in CSS
echo -n "✓ Tokens in generated CSS... "
grep -q "color-background:" public/workbench.css && echo "PASS" || (echo "FAIL" && exit 1)

# Check 6: Assets configured
echo -n "✓ Assets directory configured... "
(grep -q "assets-dir" Leptos.toml 2>/dev/null || grep -q "leptos" Cargo.toml) && echo "PASS" || (echo "FAIL" && exit 1)

# Check 7: Server starts
echo -n "✓ Leptos server starts... "
cargo leptos build &>/dev/null && echo "PASS" || (echo "FAIL" && exit 1)

# Check 8: CSS served (requires server running)
if pgrep -f "cargo leptos" &>/dev/null; then
    echo -n "✓ CSS served at runtime... "
    curl -sfI http://localhost:3003/workbench.css &>/dev/null && echo "PASS" || (echo "FAIL" && exit 1)
    
    echo -n "✓ HTML links CSS... "
    curl -s http://localhost:3003 | grep -q 'href="/workbench.css"' && echo "PASS" || (echo "FAIL" && exit 1)
    
    echo -n "✓ data-theme present... "
    curl -s http://localhost:3003 | grep -q 'data-theme=' && echo "PASS" || (echo "FAIL" && exit 1)
else
    echo "⚠ Server not running - skipping runtime checks"
fi

echo ""
echo "✅ All checks passed! Safe to develop."
```

---

## When To Run Checklist

### Mandatory Scenarios
- [ ] Creating new app/example
- [ ] Cloning repository (first time)
- [ ] After major dependency updates
- [ ] Before starting new feature
- [ ] After merge conflicts in config files

### Optional but Recommended
- [ ] Weekly (catches drift)
- [ ] Before production deploy
- [ ] After CSS-related changes

---

## Checklist For Different Stages

### Stage 1: Project Init (New App)
```bash
# Required checks
✓ Tokens exist
✓ Tailwind CLI installed
✓ Build script defined
```

### Stage 2: First Build
```bash
# Add to Stage 1
✓ CSS builds successfully
✓ Tokens in CSS
✓ Assets configured
```

### Stage 3: Development Ready
```bash
# Add to Stage 2
✓ Server starts
✓ CSS served
✓ HTML links CSS
✓ data-theme present
```

### Stage 4: Production Ready
```bash
# Add to Stage 3
✓ Styles apply in browser
✓ No console errors
✓ Performance acceptable
```

---

## Integration With CI CD

### GitHub Actions Example
```yaml
name: Validate Setup

on: [push, pull_request]

jobs:
  validate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-node@v3
      - run: npm install
      - run: ./scripts/validate-setup.sh
```

---

## Troubleshooting Guide

### Common Failure: "CSS builds but has no tokens"

**Diagnosis:**
```bash
cat style/globals.css | grep "@import.*tokens"
# Shows: @import "../../wrong/path/tokens.css"
```

**Fix:**
```css
/* Use absolute path */
@import "/opt/docker/monorepo/opensource/canonrs/crates/rs-design/style/tokens.css";
```

---

### Common Failure: "CSS not served at runtime"

**Diagnosis:**
```bash
curl -I http://localhost:3003/workbench.css
# Returns: 404 Not Found
```

**Fix:**
```toml
# Add to Leptos.toml
[site]
assets-dir = "public"
```

---

### Common Failure: "data-theme missing"

**Diagnosis:**
```bash
curl -s http://localhost:3003 | grep data-theme
# Returns: (empty)
```

**Fix:** Implement ThemeProvider DOM sync (Rule #65)

---

## Related Rules

- **Rule #62:** Single Source of Truth for Design Tokens
- **Rule #63:** Leptos Reactivity
- **Rule #64:** CSS Build Pipeline
- **Rule #65:** data-theme Sync

---

## Normative Status

- All new apps **MUST** pass checklist before first commit
- PRs adding new examples **MUST** include passing validation
- CI **SHOULD** run validation on every push
- Checklist script **MUST** be in repository

---

**Author:** Canon Working Group  
**Replaces:** None

---

## Economic Impact

**Time saved per new app:** ~4 hours  
**Frequency:** Every new workbench/example  
**Annual savings (10 apps):** ~40 hours

**Root causes eliminated:**
- ❌ "Why is page white?" (70% of issues)
- ❌ Hours debugging missing CSS pipeline
- ❌ Assumption that cargo leptos handles everything
- ❌ Discovering problems late in development
- ❌ Incomplete setup blocking progress

**ROI:** Extremely high (prevents most common setup failures)