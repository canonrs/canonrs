# Canon Rule #166: Dist Is Read-Only

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** build, css
**Version:** 1.0.0  
**Date:** 2026-01-28

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

