# Canon Rule #58: Leptos Assets Dev Constraint

**Status:** ENFORCED


**Severity:** MEDIUM
**Scope:** leptos, build
**Version:** 1.0.0
**Date:** 2025-01-16

---

---

## Principle

Leptos dev server **ONLY serves first-level files** from `assets-dir`.

Any subdirectory is silently replaced with a **1-byte placeholder response**.

---

## Canonical Behavior

```toml
[package.metadata.leptos]
assets-dir = "public"
```

Leptos dev server serves:
```
public/*.css      ✅
public/pkg/*.css  ❌ (1 byte placeholder)
```

---

## Required Solution

```bash
postcss input.css -o target/site/pkg/app.css
cp target/site/pkg/app.css public/app.css
```

And in HTML / Rust:

```rust
<Stylesheet href="/app.css"/>
```

---

## Forbidden Patterns

- ❌ Symlinks
- ❌ Runtime copying
- ❌ Watching target/site/pkg
- ❌ Debugging Tailwind / PostCSS

This is **Leptos behavior**, not a tooling bug.

---

## Validation Checklist

- [ ] CSS file exists in `public/*.css`
- [ ] No CSS referenced from subfolders
- [ ] Dev server restarted after adding asset

