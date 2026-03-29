# Canon Rule #177: canonrs.css Is a Generated Artifact, Never a Design Surface

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** build
**Version:** 1.0.0  
**Date:** 2026-01-30

---

## Principle

**The `canonrs.css` file is generated output—manual edits are forbidden and will be overwritten on next build.**

- `canonrs.css` is read-only
- All changes go through source files (tokens, presets, components)
- Build script regenerates `canonrs.css` from scratch every time

---

## Problem

When developers edit `canonrs.css` directly:

- **Silent data loss** - next build overwrites manual changes
- **Source divergence** - output doesn't match source
- **Debugging nightmare** - changes appear in browser but not in git
- **Team confusion** - some devs edit source, others edit output
- **Build non-determinism** - same source produces different outputs

Real bug: Manual fix in `canonrs.css` → worked locally → git ignored file → CI built different CSS → production broken.

---

## Forbidden Pattern

### ❌ Forbidden
```bash
# Editing generated file
vim styles/canonrs.css
```
```css
/* styles/canonrs.css */
@import "./ui/button_ui.css";
@import "./custom-fix.css";  /* ❌ Manual addition */
```
```bash
# Committing generated file
git add styles/canonrs.css
git commit -m "fix: update button colors"  # ❌ Wrong layer
```

**Why forbidden:** Next build overwrites this. Change is ephemeral. Source of truth is violated.

---

## Canonical Pattern

### ✅ Canonical
```bash
# Edit source files
vim styles/ui/button_ui.css          # Component CSS
vim src/tokens/themes/presets/canonrs.ts  # Preset colors
vim styles/themes/dark/ui.css        # Theme mappings
```
```bash
# Regenerate canonrs.css
npm run build
```
```bash
# Commit source changes only
git add styles/ui/button_ui.css
git add src/tokens/themes/presets/canonrs.ts
git commit -m "feat: update button colors in canonrs preset"
```
```bash
# .gitignore ensures canonrs.css is never committed
echo "styles/canonrs.css" >> .gitignore
```

**Why correct:** Source is versioned. Output is ephemeral. Build is deterministic.

---

## Rationale

### Build Determinism
```
Source (versioned) → Build Script → Output (ephemeral)
```

If output is modified, build is no longer deterministic. CI and local environments diverge.

### Single Source of Truth

Manual edits create **two sources of truth**:
1. Source files (git)
2. Generated file (local filesystem)

This violates DRY and causes drift.

### Architectural Invariants

1. **Generated files are artifacts** - never design surfaces
2. **Source files are truth** - all changes happen here
3. **Build is pure function** - same source → same output

### Bugs Prevented

- Developer edits `canonrs.css` → build overwrites → confusion
- CI builds different CSS than local (manual edits not in git)
- Merge conflicts in generated files
- Unable to reproduce builds

### Why Not Opinion

This is **build system hygiene**. Generated artifacts must be reproducible. Manual edits break reproducibility.

---

## Enforcement

### .gitignore
```gitignore
# Generated CSS - DO NOT COMMIT
styles/canonrs.css
styles/.generated/
dist/
```

### Pre-commit hook
```bash
#!/bin/bash
# .git/hooks/pre-commit
if git diff --cached --name-only | grep "canonrs.css"; then
  echo "❌ ERROR: canonrs.css is generated. Edit source files instead."
  exit 1
fi
```

### Build script header
```bash
# scripts/core/generate-canonrs-entry.sh
echo "/* AUTO-GENERATED - DO NOT EDIT */" > "$OUTPUT"
echo "/* Run: npm run build to regenerate */" >> "$OUTPUT"
```

### File permissions (optional)
```bash
# Make canonrs.css read-only after build
chmod 444 styles/canonrs.css
```

### Review checklist

- [ ] `canonrs.css` is in `.gitignore`
- [ ] No manual edits to `canonrs.css` in commits
- [ ] All changes go through source files → rebuild

---

## Exceptions

**No exceptions. This rule is absolute.**

If the build script doesn't produce the CSS you need, fix the build script—don't manually edit the output.

---

## Version History

- **1.0.0** — Initial version (2026-01-30)
