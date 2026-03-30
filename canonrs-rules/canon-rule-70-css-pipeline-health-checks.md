# Canon Rule #70: CSS Pipeline Requires Health Checks

**Status:** ENFORCED


**Severity:** MEDIUM
**Scope:** build, css
**Version:** 1.0.0
**Date:** 2025-01-16

---

---

## Principle

Every example, application, and component library **MUST** have automated validation that CSS is correctly built and contains expected content.

> **Untested pipelines fail silently.**  
> Health checks are not optional.

---

## The Problem

CSS build pipelines fail in subtle ways:
- Build command runs but outputs nothing
- File is generated but is empty
- Classes are missing due to purge errors
- Theme tokens not compiled
- Path resolution fails silently

**Without validation, these failures only surface at runtime in production.**

---

## Forbidden Pattern
```json
// ❌ NO VALIDATION
{
  "scripts": {
    "build:css": "tailwindcss -i style/globals.css -o dist/styles.css"
  }
}
```

**Problem:** If this fails, subsequent steps still run.

---

## Mandatory Health Check Script
```json
// ✅ REQUIRED
{
  "scripts": {
    "build:css": "tailwindcss -i style/globals.css -o dist/styles.css",
    "verify:css": "node scripts/verify-css.js"
  }
}
```

### Minimum Verification Script
```javascript
// scripts/verify-css.js
import { readFileSync, statSync } from 'fs';
import { exit } from 'process';

const CSS_PATH = 'dist/styles.css';
const REQUIRED_CLASSES = [
  '.bg-background',
  '.text-foreground',
  '.dark'
];

// Check 1: File exists
try {
  statSync(CSS_PATH);
} catch {
  console.error(`❌ ${CSS_PATH} does not exist`);
  exit(1);
}

// Check 2: File is not empty
const stats = statSync(CSS_PATH);
if (stats.size === 0) {
  console.error(`❌ ${CSS_PATH} is empty`);
  exit(1);
}

// Check 3: Required classes present
const css = readFileSync(CSS_PATH, 'utf-8');
for (const className of REQUIRED_CLASSES) {
  if (!css.includes(className)) {
    console.error(`❌ ${CSS_PATH} missing required class: ${className}`);
    exit(1);
  }
}

console.log('✅ CSS health check passed');
```

---

## Bash Alternative
```bash
#!/bin/bash
# scripts/verify-css.sh

CSS_FILE="dist/styles.css"

# Check 1: File exists
if [ ! -f "$CSS_FILE" ]; then
  echo "❌ $CSS_FILE does not exist"
  exit 1
fi

# Check 2: File is not empty
if [ ! -s "$CSS_FILE" ]; then
  echo "❌ $CSS_FILE is empty"
  exit 1
fi

# Check 3: Required classes
REQUIRED=(
  ".bg-background"
  ".text-foreground"
  ".dark"
)

for class in "${REQUIRED[@]}"; do
  if ! grep -q "$class" "$CSS_FILE"; then
    echo "❌ Missing class: $class"
    exit 1
  fi
done

echo "✅ CSS health check passed"
```

---

## Integration Points

### Pre-Serve Hook
```toml
# Trunk.toml
[[hooks]]
stage = "pre_build"
command = "npm"
command_arguments = ["run", "build:css"]

[[hooks]]
stage = "pre_build"
command = "bash"
command_arguments = ["scripts/verify-css.sh"]
```

**Effect:** Trunk refuses to serve if CSS validation fails.

### CI/CD Integration
```yaml
# .github/workflows/ci.yml
- name: Build CSS
  run: npm run build:css

- name: Verify CSS
  run: npm run verify:css

- name: Build App
  run: trunk build
```

**Effect:** CI fails fast if CSS is broken.

### Make Target
```makefile
# Makefile
.PHONY: verify
verify:
	@bash scripts/verify-css.sh

.PHONY: dev
dev: verify
	npm run watch:css & trunk serve --port 3001
```

**Effect:** Dev server won't start with broken CSS.

---

## Comprehensive Health Check

For design systems and critical apps:
```javascript
// scripts/verify-css.js (comprehensive)
import { readFileSync, statSync } from 'fs';

const CSS_PATH = 'dist/styles.css';

const checks = [
  {
    name: 'File exists',
    test: () => {
      try {
        statSync(CSS_PATH);
        return true;
      } catch {
        return false;
      }
    }
  },
  {
    name: 'File is not empty',
    test: () => statSync(CSS_PATH).size > 0
  },
  {
    name: 'Contains theme tokens',
    test: () => {
      const css = readFileSync(CSS_PATH, 'utf-8');
      return css.includes('--color-background') &&
             css.includes('--color-foreground');
    }
  },
  {
    name: 'Contains utility classes',
    test: () => {
      const css = readFileSync(CSS_PATH, 'utf-8');
      return ['.bg-background', '.text-foreground', '.flex', '.p-4']
        .every(c => css.includes(c));
    }
  },
  {
    name: 'Dark mode variant exists',
    test: () => {
      const css = readFileSync(CSS_PATH, 'utf-8');
      return css.includes('.dark');
    }
  },
  {
    name: 'No duplicate declarations',
    test: () => {
      const css = readFileSync(CSS_PATH, 'utf-8');
      const bgCount = (css.match(/\.bg-background\{/g) || []).length;
      return bgCount === 1;
    }
  },
  {
    name: 'File size reasonable',
    test: () => {
      const size = statSync(CSS_PATH).size;
      return size > 1000 && size < 1000000; // 1KB - 1MB
    }
  }
];

let failed = false;
for (const check of checks) {
  const passed = check.test();
  console.log(passed ? `✅ ${check.name}` : `❌ ${check.name}`);
  if (!passed) failed = true;
}

if (failed) {
  console.error('\n❌ CSS health check failed');
  process.exit(1);
}

console.log('\n✅ All CSS health checks passed');
```

---

## Example Specific Checks

Different apps require different validations:

### Component Library
```javascript
const REQUIRED = [
  '.btn', '.btn-primary', '.btn-secondary',
  '.card', '.card-header', '.card-body',
  '.input', '.input-error'
];
```

### Landing Page
```javascript
const REQUIRED = [
  '.hero', '.section', '.cta',
  '.bg-gradient', '.text-gradient'
];
```

### Design System
```javascript
const REQUIRED_TOKENS = [
  '--spacing-1', '--spacing-2', '--spacing-4',
  '--color-primary', '--color-secondary',
  '--radius-sm', '--radius-md'
];
```

---

## Failure Examples

### Silent Build Failure
```bash
$ npm run build:css
# No output, no error
$ ls dist/styles.css
# File doesn't exist
```

**Without health check:** Continues to `trunk serve`, app loads unstyled  
**With health check:** Fails immediately with clear error

### Empty Output
```bash
$ npm run build:css
# Runs successfully
$ cat dist/styles.css
# Empty file
```

**Without health check:** App loads, no styles applied  
**With health check:** Detects empty file, fails build

### Missing Classes
```bash
$ npm run build:css
# Tailwind purges too aggressively
$ grep ".dark" dist/styles.css
# No results
```

**Without health check:** Dark mode silently broken  
**With health check:** Fails with "Missing class: .dark"

---

## Canonical Justification

> **Build pipelines are code.**  
> Code without tests is broken by default.

CSS build failures are:
- **Silent** — no runtime errors
- **Subtle** — partial functionality works
- **Persistent** — caching hides issues
- **Production-critical** — affects all users

Health checks are the only way to guarantee correctness.

---

## Canon References

- Canon Rule #68 — Asset Must Exist in Final dist/
- Canon Rule #69 — Trunk Only Serves What's in dist/
- Canon Rule #71 — Debug Theme by Verifying File First
