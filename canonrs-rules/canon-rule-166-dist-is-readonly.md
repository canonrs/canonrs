# Canon Rule #166: Dist Is Read-Only

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-01-28

**Category:** build-tooling
**Tags:** build, dist, artifacts, css
**Language:** EN

---

**Intro:**
Editing build artifacts directly breaks reproducibility and hides underlying architectural issues. Changes become non-traceable and inconsistent across environments.

**Problem:**
dist files are manually edited causing non reproducible builds

**Solution:**
apply all fixes in source and regenerate dist via build process

**Signals:**
- manual edit
- inconsistent build
- hidden bug

**Search Intent:**
why not edit dist folder build artifacts

**Keywords:**
dist folder readonly, build artifact immutability, frontend reproducible builds, css build output integrity

---

## Principle

**The dist/ directory is a build artifact and must never be edited manually.**

---

## Problem

Editing files in dist/ creates non-reproducible builds and hides architectural errors.

---

## Forbidden Pattern

❌ Editing any file under dist/ directly.

---

## Canonical Pattern

✅ All fixes must be applied in the source directory and propagated via build.

---

## Enforcement

- Code review
- CI validation
- Build reproducibility check

---

## Exceptions

None.
