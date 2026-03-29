# Canon Rule #240: Generated Artifacts Must Not Be Committed

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** build, cli, tokens
**Version:** 1.0.0
**Date:** 2026-02-13

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
