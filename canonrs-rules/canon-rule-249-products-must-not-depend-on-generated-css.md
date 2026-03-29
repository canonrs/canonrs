# Canon Rule #249: Products Must Not Depend on .generated CSS

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** css, architecture
**Version:** 1.0.0
**Date:** 2026-02-13

---

## Principle

Products MUST consume only the distributed artifact:

    canonrs.bundle.css

Products MUST NOT reference:

    styles/.generated/*
    canonrs.css (entry file)
    internal token layer files

---

## Problem

When products reference internal CSS layers:

- They become coupled to CanonRS internals
- Directory refactors break products
- Token evolution becomes blocked
- Versioning becomes impossible

---

## Forbidden Pattern

<link rel="stylesheet" href="../../rs-canonrs/styles/.generated/root.css">  ❌  
<link rel="stylesheet" href="../../rs-canonrs/styles/canonrs.css">         ❌

---

## Canonical Pattern

<link rel="stylesheet" href="/canonrs.bundle.css">  ✅

Single artifact consumption only.

---

## Enforcement

- CI scans products for references to `.generated`
- CI forbids imports of canonrs.css
- Only bundle may be served publicly

---

## Exceptions

None.

Products consume artifacts, never internal structure.

---

## Version History

- 1.0.0 — Initial definition
