# Canon Rule #85: Leptos Asset Pipeline in Dev Mode

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-01-13

**Category:** build-tooling
**Tags:** leptos, assets, pipeline
**Language:** EN

---

**Intro:**
Leptos development mode does not use Trunk for asset handling, leading to missing files when relying on Trunk hooks. Assets must be served from the configured public directory.

**Problem:**
assets not served in dev because trunk hooks are ignored

**Solution:**
place assets in public directory and configure leptos asset serving

**Signals:**
- 404 assets
- missing files in dev
- trunk hooks ignored

**Search Intent:**
why assets not loading in leptos dev mode

**Keywords:**
leptos asset pipeline dev, trunk ignored leptos watch, public assets directory leptos, leptos asset serving config

---

## Principle

`cargo leptos watch` (via `make dev`) uses **Leptos asset serving**, NOT Trunk.

> **Trunk hooks and `data-trunk` directives are IGNORED in Leptos dev mode.**
> Assets MUST be in `public/` directory specified in `Cargo.toml`.

---

## The Problem

Developers assume `Trunk.toml` hooks will copy assets during `make dev`:
```toml
# Trunk.toml
[[hooks]]
stage = "pre_build"
command = "cp"
command_arguments = ["-r", "assets", "dist/"]
```

**Result:** Hook never executes. Assets return 404. Developer wastes hours debugging Trunk.

---

## Root Cause

**Leptos has TWO build modes:**

1. **Development (`cargo leptos watch`):**
   - Uses Leptos built-in dev server
   - Serves from `assets-dir` in `Cargo.toml`
   - **IGNORES** `Trunk.toml` completely

2. **Production (`cargo leptos build`):**
   - Uses Trunk for bundling
   - Processes `Trunk.toml` hooks
   - Outputs to `dist/`

**Most developers run `make dev`, which uses mode #1.**

---

## Canonical Asset Pipeline Development

### Step 1: Check Leptos Configuration
```toml
# Cargo.toml
[package.metadata.leptos]
assets-dir = "public"
site-root = "target/site"
```

**Assets MUST be in `public/` directory.**

### Step 2: Copy Assets to `public/`
```bash
# Manual copy (for development)
mkdir -p public/assets
cp -r /path/to/source/assets/symbols public/assets/
```

### Step 3: Reference in Code
```rust
// Correct path relative to public/
<img src="/assets/symbols/icon.svg" />
```

### Step 4: Verify Serving
```bash
# With make dev running
curl http://localhost:3003/assets/symbols/icon.svg
# Should return SVG content, not 404
```

---

## Canonical Asset Pipeline Production

### Step 1: Configure Trunk Hooks
```toml
# Trunk.toml
[[hooks]]
stage = "build"
command = "cp"
command_arguments = ["-r", "public/assets", "dist/"]
```

### Step 2: Build for Production
```bash
cargo leptos build --release
```

### Step 3: Verify Output
```bash
ls -la dist/assets/symbols/
# Assets should exist in dist/
```

---

## Decision Matrix

| Context | Asset Location | Tool | Hook Execution |
|---------|---------------|------|----------------|
| `make dev` | `public/` | Leptos dev server | ❌ No |
| `cargo leptos watch` | `public/` | Leptos dev server | ❌ No |
| `cargo leptos build` | `dist/` | Trunk | ✅ Yes |
| `trunk serve` | `dist/` | Trunk | ✅ Yes |

---

## Anti Patterns

### ❌ Anti-Pattern 1: Relying on Trunk Hooks in Dev
```toml
# Trunk.toml
[[hooks]]
stage = "pre_build"
command = "cp"
command_arguments = ["assets", "public/"]
```
```bash
make dev  # Hook does NOT run
```

**Problem:** Trunk is not involved in `cargo leptos watch`.

---

### ❌ Anti-Pattern 2: Assets in `dist/` During Dev
```rust
<img src="/dist/assets/icon.svg" />
```

**Problem:** Leptos serves from `public/`, not `dist/`.

---

### ❌ Anti-Pattern 3: `data-trunk` Directives in Dev
```html
<link data-trunk rel="copy-dir" href="assets" />
```

**Problem:** Leptos dev server ignores `data-trunk` attributes.

---

## Debugging Protocol

When assets return 404 during `make dev`:
```bash
# 1. Verify Leptos config
cat Cargo.toml | grep -A 5 "\[package.metadata.leptos\]"
# Check assets-dir value

# 2. Check assets physically exist
ls -la public/assets/

# 3. Verify path in code matches public/
grep -r "src=\"/assets" src/

# 4. Test direct access
curl http://localhost:3003/assets/symbols/icon.svg

# 5. If still failing, restart dev server
# Ctrl+C and run make dev again
```

---

## Canonical Workflow

### For Development (Daily Work)
```bash
# One-time setup
mkdir -p public/assets
cp -r /source/assets/symbols public/assets/

# Daily workflow
make dev
# Assets served from public/
```

### For Production Builds
```bash
# Trunk handles asset pipeline
cargo leptos build --release
# Assets copied to dist/ via hooks
```

---

## Integration With Monorepo

When assets are in shared packages:
```bash
# Development: Manual copy to public/
cp -r /opt/docker/monorepo/packages-rust/rs-design/assets/symbols \
     public/assets/

# Production: Trunk hook copies from source
[[hooks]]
stage = "build"
command = "cp"
command_arguments = [
  "-r",
  "/opt/docker/monorepo/packages-rust/rs-design/assets/symbols",
  "dist/assets/"
]
```

---

## Makefile Integration
```makefile
.PHONY: dev setup-assets

setup-assets:
	mkdir -p public/assets
	cp -r /source/assets/symbols public/assets/

dev: setup-assets
	cargo leptos watch
```

**Benefit:** Assets automatically updated on `make dev`.

---

## Canonical Justification

> **Leptos and Trunk are separate tools with different asset models.**
> Development speed requires lightweight serving without full bundling.

This rule enforces:
- **No wasted time** debugging Trunk in dev mode
- **Clear separation** between dev and prod pipelines
- **Predictable behavior** across environments

---

## Canon References

- Canon Rule #58 — Leptos Assets Dev Constraint (subfolder limitation)
- Canon Rule #69 — Trunk Only Serves What's in dist/
- Canon Rule #68 — Asset Must Exist in Final dist/
- Canon Rule #56 — Monorepo CSS Build Pipeline