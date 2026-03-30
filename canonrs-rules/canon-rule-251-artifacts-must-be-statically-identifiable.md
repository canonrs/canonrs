# Canon Rule #251: All Build Artifacts Must Be Statically Identifiable

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-02-13  

**Category:** build-tooling
**Tags:** build, artifacts, naming, determinism
**Language:** EN

---

**Intro:**
Dynamic artifact naming causes cache issues and deployment confusion. Artifacts must be deterministic and identifiable.

**Problem:**
artifacts use dynamic naming causing cache corruption and instability

**Solution:**
use deterministic naming conventions tied to source identity

**Signals:**
- cache issue
- mismatch artifact
- deployment confusion

**Search Intent:**
how to enforce deterministic artifact naming

**Keywords:**
artifact naming deterministic build, css wasm naming convention, frontend build artifact control, cache safe file naming

---

## Principle

All generated artifacts (CSS, WASM, bundles, manifests)
MUST have deterministic file names and identifiable source ownership.

---

## Problem

Dynamic artifact naming causes:

- Cache corruption
- Hard-to-debug mismatches
- Deployment confusion
- CI instability

---

## Canonical Requirements

- CSS artifacts must have canonical naming
- WASM artifacts must match product hyphenated name
- No runtime-generated filenames
- No implicit temp bundles

---

## Example

Allowed:

```
canonrs.bundle.css
canonrs-site.wasm
```

Disallowed:

```
bundle-20260213.css
wasm_build_3434.wasm
```

---

## Rationale

Artifacts are part of architecture.
Ephemeral names break reproducibility.

---

## Enforcement

CI must:

- Detect timestamp-based filenames
- Detect random hash-only filenames unless explicitly versioned

---

## Exceptions

Version-hashed filenames allowed ONLY if hash is content-derived and deterministic.