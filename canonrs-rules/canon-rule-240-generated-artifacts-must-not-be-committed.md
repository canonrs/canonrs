# Canon Rule #240: Generated Artifacts Must Not Be Committed

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-02-13

**Category:** build-tooling
**Tags:** build, artifacts, ci, tokens
**Language:** EN

---

**Intro:**
Committing generated artifacts causes merge conflicts and version drift. Build outputs must remain reproducible and excluded from source control.

**Problem:**
generated files are committed causing conflicts drift and stale artifacts

**Solution:**
exclude all generated outputs via gitignore and regenerate during build or ci

**Signals:**
- merge conflict
- diff noise
- stale artifact

**Search Intent:**
why not commit generated files build

**Keywords:**
generated artifacts gitignore, build reproducibility rust, ci regenerate artifacts, css bundle commit issue

---

## Principle

Generated artifacts must never be committed to the repository.

Only source of truth is committed.
Generated output is reproducible.

---

## Applies To

- styles/.generated/*
- canonrs.bundle.css
- CLI-generated metadata blocks
- Auto-generated workspace files
- Any deterministic build output

---

## Problem

Committing generated files causes:

- Merge conflicts
- Diff pollution
- Version skew
- Stale artifacts
- False architectural drift

---

## Forbidden Pattern

```
git add styles/.generated/*
git commit -m "update generated css"
```

---

## Canonical Pattern

```
.gitignore
-------------
styles/.generated/
canonrs.bundle.css
```

Generation happens:

- On build
- On CLI sync
- In CI before deploy

---

## Enforcement

CI must regenerate artifacts and verify:

```
canonrs sync
cargo run --bin tokens-engine
git diff --quiet
```

If diff exists → build fails.

---

## Exceptions

None.

Generated files are build products, not architecture.

---

## Version History

- 1.0.0 — Initial definition