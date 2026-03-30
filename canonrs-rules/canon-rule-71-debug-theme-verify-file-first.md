# Canon Rule #71: Debug Theme by Verifying File First

**Status:** ENFORCED


**Severity:** MEDIUM
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** governance
**Tags:** debugging, css, theme
**Language:** EN

---

**Intro:**
Most theming issues originate from missing or broken CSS files, not application logic. Debugging must start by verifying file existence and loading.

**Problem:**
developers debug application logic before verifying css file existence

**Solution:**
always validate css file existence serving and content before debugging code

**Signals:**
- missing css
- 404 stylesheet
- theme not applying

**Search Intent:**
how to debug theme issues in leptos

**Keywords:**
debug css file first, leptos theme debugging, missing css file fix, verify stylesheet loading

---

---

## Principle

Before debugging tokens, signals, Effects, or DOM manipulation in theme-related issues, **ALWAYS** verify that the CSS file exists and was loaded correctly.

> **The filesystem is the source of truth.**  
> Code can lie. Files cannot.

---

## The Problem

When dark mode, theming, or styling doesn't work, developers instinctively debug:
- React/Leptos state management
- CSS-in-JS token generation
- DOM class application
- JavaScript event handlers
- Framework reactivity systems

**95% of the time, the CSS file is missing or broken.**

---

## Mandatory Debug Order

### Step 1: Verify File Exists (Filesystem)
```bash
ls -lh dist/styles*.css
```

**Expected:** File exists with reasonable size (> 1KB)  
**If missing:** Build pipeline is broken, stop here

### Step 2: Verify File is Served (HTTP)
```bash
curl -I http://localhost:3001/styles-HASH.css
```

**Expected:** `200 OK`  
**If 404:** Asset not in dist/ or Trunk not serving correctly

### Step 3: Verify File Contents (Grep)
```bash
grep ".dark" dist/styles*.css
grep "bg-background" dist/styles*.css
grep "--color-background" dist/styles*.css
```

**Expected:** All required classes/tokens present  
**If missing:** Tailwind purge or compilation issue

### Step 4: Verify Browser Loaded CSS (DevTools)
```javascript
// Browser console
document.styleSheets.length
// Should be > 0

Array.from(document.styleSheets).map(s => s.href)
// Should include styles-HASH.css

Array.from(document.styleSheets)
  .flatMap(s => Array.from(s.cssRules))
  .find(r => r.selectorText === '.dark')
// Should return a CSSStyleRule
```

**Expected:** Stylesheet loaded and contains `.dark` rule  
**If missing:** CSS file exists but wasn't linked in HTML

### Step 5: ONLY THEN Debug Application Code
```javascript
// Now it's safe to check application logic
document.documentElement.classList.contains('dark')
// Is the class being applied?

getComputedStyle(document.documentElement).backgroundColor
// Does it change when .dark is toggled?
```

---

## Decision Tree
```
Theme not working
  │
  ├─→ ls dist/styles.css → Missing?
  │     └─→ FIX: Build pipeline (Rule #68, #69, #70)
  │
  ├─→ curl /styles-HASH.css → 404?
  │     └─→ FIX: Asset not in dist/ or wrong Trunk config
  │
  ├─→ grep ".dark" dist/styles.css → Not found?
  │     └─→ FIX: Tailwind config or purge issue
  │
  ├─→ document.styleSheets.length → 0?
  │     └─→ FIX: HTML missing <link> tag
  │
  └─→ All above pass?
        └─→ NOW debug application code
```

---

## Real World Case Study

### Reported Issue
"Dark mode toggle doesn't work in Leptos app"

### Developer's Initial Investigation (2 hours wasted)
1. Checked `Effect::new` logic ✓
2. Verified `classList.add("dark")` is called ✓
3. Tested signal reactivity ✓
4. Inspected DOM mutations ✓
5. Checked CSS variable definitions ✓
6. Tried different browsers ✓

### Actual Root Cause (found in 30 seconds)
```bash
$ curl -I http://localhost:3001/styles.css
HTTP/1.1 404 Not Found
```

**File never existed. All debugging was on the wrong layer.**

---

## Automated Verification Script
```bash
#!/bin/bash
# scripts/debug-theme.sh

echo "=== Theme Debug Protocol ==="
echo ""

# Step 1: File exists
echo "1️⃣ Checking if CSS file exists..."
if ls dist/styles*.css 1>/dev/null 2>&1; then
  SIZE=$(ls -lh dist/styles*.css | awk '{print $5}')
  echo "   ✅ File exists ($SIZE)"
else
  echo "   ❌ CSS file missing in dist/"
  echo "   → Run: npm run build:css"
  exit 1
fi

# Step 2: File is served
echo ""
echo "2️⃣ Checking if CSS is served..."
HASH_FILE=$(ls dist/styles*.css | head -1 | xargs basename)
HTTP_CODE=$(curl -s -o /dev/null -w "%{http_code}" http://localhost:3001/$HASH_FILE)
if [ "$HTTP_CODE" = "200" ]; then
  echo "   ✅ CSS served correctly (200 OK)"
else
  echo "   ❌ CSS returns $HTTP_CODE"
  echo "   → Check if trunk serve is running"
  exit 1
fi

# Step 3: Required classes present
echo ""
echo "3️⃣ Checking for required theme classes..."
REQUIRED=(".dark" ".bg-background" "--color-background")
for class in "${REQUIRED[@]}"; do
  if grep -q "$class" dist/styles*.css; then
    echo "   ✅ Found: $class"
  else
    echo "   ❌ Missing: $class"
    echo "   → Check Tailwind config or globals.css"
    exit 1
  fi
done

echo ""
echo "✅ All file-level checks passed"
echo ""
echo "4️⃣ Now check browser (open DevTools console):"
echo "   document.styleSheets.length"
echo "   Array.from(document.styleSheets).map(s => s.href)"
echo "   document.documentElement.classList.toggle('dark')"
```

**Usage:**
```bash
bash scripts/debug-theme.sh
```

---

## Common Mistakes

### Mistake 1: Debugging Framework First
```rust
// ❌ WRONG PRIORITY
Effect::new(move |_| {
    // Spending hours here when CSS doesn't exist
    web_sys::console::log_1(&"Effect running".into());
    let is_dark = dark_mode.get();
    // ...
});
```

**Correct approach:**
```bash
# ✅ VERIFY FILE FIRST
ls dist/styles.css && grep ".dark" dist/styles.css
# Only then debug Effect
```

### Mistake 2: Trusting Build Logs
```bash
$ npm run build:css
✓ Done in 142ms
```

**Assumption:** "It says 'Done', so it worked"  
**Reality:** Could be empty file or wrong output location

**Correct approach:**
```bash
npm run build:css && ls -lh dist/styles.css
```

### Mistake 3: Assuming Hot Reload Works

**Developer:** "I changed globals.css, saved, but nothing changed"  
**Assumption:** Hot reload is broken

**Reality:**
```bash
$ ls -lh dist/styles.css
-rw-r--r-- 1 user user 5.2K Jan 08 20:15 dist/styles.css
# File timestamp hasn't changed → build didn't run
```

**Correct approach:**
```bash
# Verify file was regenerated
npm run build:css && ls -lh dist/styles.css
```

---

## Browser Based Verification

### Check 1: Stylesheet Count
```javascript
console.log('Loaded stylesheets:', document.styleSheets.length);
// Expected: At least 1
```

### Check 2: Stylesheet URLs
```javascript
Array.from(document.styleSheets).forEach(sheet => {
  console.log('Stylesheet:', sheet.href);
});
// Expected: Should include styles-HASH.css
```

### Check 3: Dark Mode Rule Exists
```javascript
const darkRule = Array.from(document.styleSheets)
  .flatMap(s => {
    try {
      return Array.from(s.cssRules);
    } catch {
      return [];
    }
  })
  .find(r => r.selectorText === '.dark');

console.log('Dark mode rule:', darkRule);
// Expected: CSSStyleRule object
```

### Check 4: Computed Styles
```javascript
const html = document.documentElement;

// Before
console.log('Before:', getComputedStyle(html).backgroundColor);

// Toggle
html.classList.toggle('dark');

// After
console.log('After:', getComputedStyle(html).backgroundColor);
// Expected: Different values
```

---

## Integration With Development Workflow

### Before Starting Work
```bash
make verify  # Runs CSS health checks
make dev     # Only starts if verify passes
```

### During Development
```bash
# Terminal 1: Watch CSS
npm run watch:css

# Terminal 2: Monitor output
watch -n 1 ls -lh dist/styles.css

# Terminal 3: Dev server
trunk serve --port 3001
```

### When Issues Arise
```bash
# Don't debug code first
# Run the debug protocol
bash scripts/debug-theme.sh
```

---

## Canonical Justification

> **Debugging is deterministic.**  
> Start with what can be verified objectively.

The filesystem and HTTP responses are:
- **Deterministic** — files exist or they don't
- **Observable** — can be inspected directly
- **Independent** — not affected by framework bugs
- **Fast** — verification takes seconds

Application code is:
- **Stateful** — depends on runtime conditions
- **Complex** — many moving parts
- **Framework-dependent** — different debugging per framework
- **Slow** — requires reproduction steps

**Always verify the foundation before debugging the application.**

---

## Time Savings Analysis

| Issue | Wrong Debug Path | Correct Debug Path | Time Saved |
|-------|-----------------|-------------------|------------|
| CSS 404 | 2h debugging Effect | 30s checking file | 1h 59m |
| Empty CSS | 1h checking tokens | 10s checking file size | 50m |
| Missing .dark | 1h debugging classList | 20s grepping file | 40m |
| Wrong output path | 3h debugging Tailwind | 1m checking dist/ | 2h 59m |

**Average time wasted per theme issue: 1-3 hours**  
**Time with verification-first: < 5 minutes**

---

## Canon References

- Canon Rule #68 — Asset Must Exist in Final dist/
- Canon Rule #69 — Trunk Only Serves What's in dist/
- Canon Rule #70 — CSS Pipeline Requires Health Checks
- Canon Rule #67 — Leptos CSR Does NOT Load CSS via `<Stylesheet />`