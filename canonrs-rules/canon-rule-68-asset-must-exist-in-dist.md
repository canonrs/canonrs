# Canon Rule #68: Asset Must Exist in Final dist/

**Status:** ENFORCED


**Severity:** HIGH
**Scope:** build, workspace
**Version:** 1.0.0
**Date:** 2025-01-16

---

---

## Principle

Every asset referenced by the application **MUST** physically exist in the final `dist/` directory that is served to users.

> **Build determinism requires physical verification.**  
> References without artifacts are contract violations.

---

## The Problem

Applications often reference assets that:

- Were never generated
- Were generated in the wrong location
- Failed silently during build
- Exist in source but not in output

**Result:** 404 errors, missing styles, broken functionality — all silent until runtime.

---

## Forbidden Patterns (ABSOLUTE)

```html
<!-- ❌ NEVER ALLOWED -->
<link href="/styles.css" />
<!-- But dist/styles.css does not exist -->

<script src="/app.js"></script>
<!-- But build failed silently -->
```

### Why this is forbidden

- Silent 404s break functionality without errors
- Build tools can fail without surfacing issues
- Developers assume "the tool handles it"
- CI/CD passes but production is broken

---

## Mandatory Verification Checklist

**Before considering any build successful:**

```bash
# 1. Asset physically exists
ls dist/styles*.css || exit 1

# 2. Asset is served correctly
curl -I http://localhost:3001/styles-HASH.css | grep "200 OK"

# 3. Asset contains expected content
grep ".dark" dist/styles*.css || exit 1
grep "bg-background" dist/styles*.css || exit 1
```

**If ANY check fails → build is INVALID.**

---

## Enforcement Rules

### Rule 1: No Phantom References

Every `<link>`, `<script>`, or asset reference must point to a file that exists in `dist/`.

### Rule 2: Explicit Build Steps

Asset generation must be:

- Explicitly defined in build pipeline
- Verifiable before runtime
- Failed loudly if unsuccessful

### Rule 3: Pre-Runtime Validation

CI/CD must verify asset existence:

```yaml
# Example CI check
- name: Verify assets
  run: |
    test -f dist/styles.css || exit 1
    test -f dist/app.js || exit 1
```

---

## Canonical Pattern

### Build Pipeline (Correct)

```json
{
  "scripts": {
    "build:css": "tailwindcss -i style/globals.css -o dist/styles.css",
    "build:app": "trunk build",
    "verify": "test -f dist/styles.css && grep '.dark' dist/styles.css"
  }
}
```

### Trunk Configuration (Correct)

```toml
[[hooks]]
stage = "pre_build"
command = "npm"
command_arguments = ["run", "build:css"]

[build]
dist = "dist"
```

### HTML Reference (Correct)

```html
<!-- ✅ CANONICAL -->
<link data-trunk rel="css" href="dist/styles.css" />
```

**Trunk will:**

1. Copy `dist/styles.css` to `dist/styles-HASH.css`
2. Update HTML reference automatically
3. Asset is guaranteed to exist

---

## Debugging Protocol

When assets are missing, follow this exact order:

```bash
# 1. Verify physical file
ls -lh dist/styles.css
# If missing → build failed

# 2. Verify build command ran
npm run build:css
# Check for errors

# 3. Verify output location
# Is CSS being generated to dist/ or somewhere else?

# 4. Verify trunk configuration
cat Trunk.toml
# Does it reference the correct path?
```

---

## Real-World Case Study

**Symptom:** Dark mode toggle not working  
**Investigation:** Checked tokens, signals, Effect, DOM manipulation  
**Actual cause:** `styles.css` returned 404 — file never existed

**Time wasted:** 2+ hours on wrong layer  
**Fix time:** 2 minutes once asset verified

**Lesson:** Always verify the asset FIRST.

---

## Canonical Justification

> **Assets that don't exist can't fail loudly.**  
> They fail silently at runtime, in production, in front of users.

Enterprise systems require:

- Deterministic builds
- Verifiable outputs
- Fail-fast validation
- No phantom dependencies

This rule enforces physical reality over assumptions.

---

## Canon References

- Canon Rule #69 — CSS Build is Explicit, Never Implicit
- Canon Rule #70 — Trunk Only Serves What's in dist/
- Canon Rule #71 — CSS Pipeline Requires Health Checks
