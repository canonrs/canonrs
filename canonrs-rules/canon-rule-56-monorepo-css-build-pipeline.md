# Canon Rule #56: Monorepo CSS Build Pipeline

**Status:** ENFORCED


**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** build-tooling
**Tags:** css, pipeline, monorepo, artifacts
**Language:** EN

---

**Intro:**
Directly importing CSS from generators or crates breaks build consistency and portability. CSS must be generated and copied into canonical packages as immutable artifacts.

**Problem:**
css is consumed directly from generators instead of build artifacts

**Solution:**
enforce build pipeline that copies generated css into canonical dist packages

**Signals:**
- css from crates
- missing dist css
- inconsistent builds

**Search Intent:**
how to build css pipeline in

**Keywords:**
css build pipeline monorepo, design tokens build artifacts, copy vs link css, canonical dist css

---

---

## Principle

CSS in a monorepo **MUST** be produced as **build artifacts**, then **copied** into canonical workspace packages.

CSS **MUST NEVER** be imported directly from generators, Rust crates, or internal folders.

---

## Canonical Pipeline

```
Rust / JSON Tokens
        ↓
CSS Generation (Rust / Scripts)
        ↓
build-canon-css.sh
        ↓
packages/*/dist/*.css
        ↓
pnpm workspace resolution
        ↓
Application CSS bundling
```

---

## Required Script

### `scripts/build-canon-css.sh`

```bash
#!/usr/bin/env bash
set -euo pipefail

echo "🔨 Building Canon CSS artifacts..."

mkdir -p packages/canonrs-tailwind/dist
mkdir -p packages/canonrs-design/dist

cp packages-rust/rs-tailwind/tokens/theme.css \
   packages/canonrs-tailwind/dist/tokens.css

cp packages-rust/rs-tailwind/globals.css \
   packages/canonrs-tailwind/dist/globals.css

cp packages-rust/rs-design/themes/generated.css \
   packages/canonrs-design/dist/themes.css

echo "✅ Canon CSS artifacts ready"
```

---

## Critical Constraints

- This script **copies**, never links
- This script **fails fast**
- This script is **idempotent**
- This script runs **before pnpm install**

---

## CI Requirement

CI **MUST** fail if:
- any dist file is missing
- any dist file is empty
- script does not run before CSS bundling

---

## Validation Checklist

- [ ] `packages/*/dist/*.css` exist
- [ ] No CSS imports internal paths
- [ ] build-canon-css.sh runs in CI
- [ ] pnpm install resolves @canonrs/*
