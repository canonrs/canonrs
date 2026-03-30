# Canon Rule #77: Monorepo Path Stability

**Status:** ENFORCED


**Severity:** HIGH
**Scope:** workspace, architecture
**Version:** 1.0.0
**Date:** 2025-01-16

---

---

## Principle

Configuration files MUST NOT hardcode internal monorepo paths.

When packages move within the monorepo (e.g., `crates/` → `packages-rust/`), ALL configuration references must update simultaneously.

**Critical insight:** Paths are relative, not absolute. Configuration must calculate paths from its own location, not assume global structure.

---

## Problem Statement

### What Happens During Package Moves
```bash
# Initial structure
monorepo/
├─ crates/
│  └─ rs-design/
└─ examples/
   └─ workbench/
      ├─ Cargo.toml  (path = "../../crates/rs-design")
      └─ postcss.config.js  (path = "../../crates/rs-design/style")

# After refactor
monorepo/
├─ packages-rust/  ← MOVED
│  └─ rs-design/
└─ examples/
   └─ workbench/
      ├─ Cargo.toml  (path = "../../crates/rs-design")  ❌ BROKEN
      └─ postcss.config.js  (../../crates/...)          ❌ BROKEN
```

**Result:**
- Build fails with "file not found"
- PostCSS cannot resolve imports
- Developer spends hours debugging

---

## Forbidden Patterns

### Hardcoded Absolute Paths
```javascript
// ❌ NEVER ALLOWED
export default {
  plugins: {
    'postcss-import': {
      resolve: (id) => {
        return '/opt/docker/monorepo/crates/rs-design/style/tokens.css';
      }
    }
  }
}
```

**Problems:**
- Machine-specific path
- Breaks on CI/CD
- Breaks on other developers' machines
- Survives find/replace during refactor

### Deep Relative Paths Without Context
```toml
# ❌ FRAGILE
[dependencies]
rs-design = { path = "../../../../../crates/rs-design" }
```

**Problems:**
- Hard to audit during refactor
- Easy to miss in search
- No indication of what `../../../../../` points to

---

## Canonical Architecture

### Path Calculation Strategy
```javascript
// ✅ CORRECT: Calculate from current file
import path from 'path';
import { fileURLToPath } from 'url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));

export default {
  plugins: {
    'postcss-import': {
      resolve: (id) => {
        if (id === '@canonrs/tailwind/tokens.css') {
          // Calculate relative to THIS config file
          return path.resolve(__dirname, '../../../../../packages-rust/rs-design/style/tokens.css');
        }
        return id;
      }
    }
  }
}
```

**Benefits:**
- Relative to config location
- Works on any machine
- Clear refactor path

---

## Configuration Update Checklist

When moving packages, update ALL of:

### 1. Cargo.toml Dependencies
```toml
# Find all references
[dependencies]
rs-design = { path = "../../crates/rs-design" }  # ← Update this

# Search command
grep -r 'path.*rs-design' examples/*/Cargo.toml
```

### 2. PostCSS Config
```javascript
// postcss.config.js
resolve: (id) => {
  if (id === '@canonrs/tailwind/tokens.css') {
    return path.resolve(__dirname, '../../crates/rs-design/...');  // ← Update
  }
}
```

### 3. CSS Imports
```css
/* tailwind.css */
@import "../../crates/rs-design/style/canonrs.css";  /* ← Update */
```

### 4. Package.json Scripts
```json
{
  "scripts": {
    "copy-tokens": "cp ../../crates/rs-design/tokens.json public/"  // ← Update
  }
}
```

### 5. Workspace Configuration
```toml
# Root Cargo.toml
[workspace]
members = [
    "crates/rs-design",  # ← Update
    "examples/workbench"
]
```

---

## Systematic Update Procedure
```bash
# Step 1: Identify all hardcoded paths
grep -r "crates/rs-design" . --include="*.toml" --include="*.js" --include="*.css"

# Step 2: Calculate new relative paths
# From each config file to new location

# Step 3: Update in batches by file type
find . -name "Cargo.toml" -exec sed -i 's|crates/rs-design|packages-rust/rs-design|g' {} +
find . -name "postcss.config.js" -exec sed -i 's|crates/rs-design|packages-rust/rs-design|g' {} +
find . -name "*.css" -exec sed -i 's|crates/rs-design|packages-rust/rs-design|g' {} +

# Step 4: Verify all references updated
grep -r "crates/rs-design" . --include="*.toml" --include="*.js" --include="*.css"
# Should return 0 results

# Step 5: Test each example builds
cd examples/minimal && cargo build
cd examples/workbench && cargo build
cd examples/leptos-tailwindv4 && cargo build
```

---

## Path Calculation Examples

### From Workbench to rs-design
```
monorepo/
├─ packages-rust/
│  └─ rs-design/
│     └─ src/
└─ examples/
   └─ _wip/
      └─ workbench/
         └─ Cargo.toml  ← Starting point

Relative path: ../../../../../packages-rust/rs-design
```

### Verification Command
```bash
cd examples/_wip/workbench
ls -la ../../../../../packages-rust/rs-design
# Should list directory contents, not error
```

---

## Real World Refactor: crates → packages-rust

### Files That Needed Updates

1. **3× Cargo.toml** (workbench, minimal, leptos-tailwindv4)  
2. **1× postcss.config.js** (workbench)  
3. **1× tailwind.css** (workbench)  
4. **1× Root Cargo.toml** (workspace exclude)

### Commands Used
```bash
# Update Cargo.toml files
sed -i 's|path = "../../crates/rs-design"|path = "../../../../packages-rust/rs-design"|' examples/minimal/Cargo.toml
sed -i 's|path = "../../crates/rs-design"|path = "../../../../packages-rust/rs-design"|' examples/leptos-tailwindv4/Cargo.toml
sed -i 's|path = "../../../crates/rs-design"|path = "../../../../../packages-rust/rs-design"|' examples/_wip/workbench/Cargo.toml

# Update PostCSS
sed -i 's|../../../crates|../../../../packages-rust|g' examples/_wip/workbench/postcss.config.js

# Update CSS imports
sed -i 's|/opt/docker/monorepo/opensource/canonrs/crates/rs-design|/opt/docker/monorepo/packages-rust/rs-design|g' examples/_wip/workbench/style/tailwind.css

# Remove old exclude
sed -i 's|exclude = \["crates/rs-design", "examples/_wip"\]|exclude = \["examples/_wip"\]|' Cargo.toml
```

---

## Prevention: Canonical Package References

### Use Workspace Dependencies (Future)
```toml
# Root Cargo.toml
[workspace.dependencies]
rs-design = { path = "packages-rust/rs-design" }

# Example Cargo.toml
[dependencies]
rs-design.workspace = true  # ← Single source of truth
```

### Use Package Aliases
```javascript
const DESIGN_SYSTEM_PATH = path.resolve(__dirname, '../../../../../packages-rust/rs-design');

resolve: (id) => {
  if (id === '@canonrs/tailwind/tokens.css') {
    return path.join(DESIGN_SYSTEM_PATH, 'style/tokens.css');
  }
}
```

---

## Canonical Justification

**Monorepos must survive refactoring.**

A system that breaks when folders move:
- Cannot scale beyond toy projects
- Cannot integrate with enterprise tooling
- Cannot be maintained by multiple teams

**Canon mandates:** Path resilience through relative, calculated references.

---

## Related Symptoms

If you see:
- "File not found" after moving packages
- Build works for some examples, fails for others
- PostCSS import resolution errors
- `cargo metadata` failures

→ **This rule is violated.**

Go to: **SYMPTOMS.md → MONOREPO PATH VIOLATIONS**
