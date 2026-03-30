# Canon Rule #158: Design System Packages Are Immutable Contracts (No Direct File Imports)

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-01-26

**Category:** governance
**Tags:** design-system, packages, versioning, architecture
**Language:** EN

---

**Intro:**
Direct file imports from design system internals create tight coupling and break versioning guarantees. Applications become fragile and fail across environments.

**Problem:**
apps import design system files directly causing coupling and breakage

**Solution:**
consume design system only through versioned package exports

**Signals:**
- path import
- ci break
- coupling

**Search Intent:**
how to consume design system via

**Keywords:**
design system package contract, no relative imports architecture, frontend package boundaries, versioned design system usage

---

## Principle

**Applications MUST consume design system artifacts only through versioned package exports, never through direct file system imports.**

- Design systems are distributed packages, not shared folders
- Apps depend on `@canonrs/design`, not `../../packages-rust/rs-design/style/`
- File paths create coupling; package exports create contracts
- Changes propagate via version bumps, not file edits

---

## Problem

Without package boundaries:

- **Silent breakage**: App imports `../../rs-design/style/button.css`, design system refactors → app breaks
- **No versioning**: Cannot track which app uses which design system version
- **Circular dependencies**: Apps and design system become interdependent
- **Build fragility**: File paths break across environments (dev, CI, prod)
- **Untracked changes**: CSS updates don't trigger app rebuilds

**Real symptom**: App works locally, breaks in CI because relative path resolves differently. Or: design system refactor breaks 5 apps silently because they imported internal files directly.

---

## Forbidden Pattern

### ❌ Forbidden
```css
/* App: style/main.css - DIRECT FILE IMPORT */
@import "../../packages-rust/rs-design/style/tokens.css";
@import "../../packages-rust/rs-design/style/ui/button.css";
```
```javascript
// App: package.json - FILE PATH DEPENDENCY
{
  "dependencies": {
    "design-system": "file:../../packages-rust/rs-design"
  }
}
```
```rust
// App: Cargo.toml - PATH DEPENDENCY IN PRODUCTION
[dependencies]
rs-design = { path = "../../packages-rust/rs-design" }
```

**Why forbidden:**
- **Coupling**: App knows design system internal structure
- **Fragility**: Path breaks if either package moves
- **No contract**: Design system can't refactor without checking all consumers
- **CI failures**: Paths resolve differently in different environments
- **No versioning**: Cannot rollback or pin versions

---

## Canonical Pattern

### ✅ Canonical

**Design System: package.json**
```json
{
  "name": "@canonrs/design",
  "version": "0.1.0",
  "exports": {
    "./tokens.css": {
      "style": "./dist/tokens.css",
      "default": "./dist/tokens.css"
    },
    "./themes.css": {
      "style": "./dist/themes.css",
      "default": "./dist/themes.css"
    },
    "./canonrs.css": {
      "style": "./dist/canonrs.css",
      "default": "./dist/canonrs.css"
    }
  }
}
```

**App: style/main.css**
```css
/* Package imports - STABLE API */
@import "@canonrs/design/tokens.css";
@import "@canonrs/design/themes.css";
@import "@canonrs/design/canonrs.css";
```

**App: package.json**
```json
{
  "dependencies": {
    "@canonrs/design": "^0.1.0"
  }
}
```

**Monorepo: pnpm-workspace.yaml**
```yaml
packages:
  - 'packages/*'
  - 'apps/*'
```

**Why canonical:**
- **Versioned contract**: Apps pin `^0.1.0`, not file paths
- **Refactor safety**: Design system can reorganize internals freely
- **Environment agnostic**: Works in dev, CI, prod, Docker
- **Explicit API**: Only exported files are public
- **Rollback support**: `npm install @canonrs/design@0.0.9`

---

## Rationale

**Package boundaries are architectural:**

1. **Versioning enables change**: Without versions, any change is breaking
2. **Exports define API surface**: Only exported files are stable
3. **Dependency graph is auditable**: `npm ls` shows what depends on what
4. **CI reproducibility**: Package versions lock behavior across environments

**This is not "best practice" — it's how package ecosystems work:**
- npm, cargo, pip all use package boundaries, not file paths
- Relative imports (`../`) are for same-package files only
- Cross-package imports must go through package manager

**Real-world parallel:**
```javascript
// ❌ No one does this
import { Button } from "../../node_modules/react/src/Button.js";

// ✅ Everyone does this
import { Button } from "react";
```

Design systems are packages. Treat them as such.

---

## Enforcement

**Linting:**
```bash
# Detect direct file imports in apps
grep -r "@import.*\.\./.*rs-design" apps/*/style/ && \
  echo "❌ Direct file import detected" && exit 1
```

**Cargo.toml check:**
```bash
# In production, no path dependencies
grep "path.*rs-design" apps/*/Cargo.toml | grep -v "# dev-only" && \
  echo "❌ Path dependency in production" && exit 1
```

**CI validation:**
```yaml
# Ensure design system is consumed as package
- name: Check design system imports
  run: |
    if grep -r "file:.*rs-design" apps/*/package.json; then
      echo "Apps must use published package, not file: paths"
      exit 1
    fi
```

**Build-time:**
- Apps must resolve `@canonrs/design` via workspace, not path
- `npm run build` must work with only `node_modules/@canonrs/design` present
- Design system internal files must not be accessible to apps

---

## Exceptions

**Development-only exception:**
```toml
# Cargo.toml - workspace member can use path for hot reload
[dependencies]
rs-design = { path = "../../packages-rust/rs-design" }  # dev-only

# But must also support:
# rs-design = "0.1.0"  # production
```

This is acceptable **only if**:
- App is in same monorepo as design system
- Production build uses published version
- Path is documented as dev-only

**Production deployment must always use versioned package.**

---

## Version History

- **1.0.0** — Initial version (2026-01-26)