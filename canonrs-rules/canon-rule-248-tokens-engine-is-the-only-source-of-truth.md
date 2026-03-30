# Canon Rule #248: Tokens Engine Is the Only Source of Truth

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-02-13

**Category:** governance
**Tags:** tokens, css, governance, build
**Language:** EN

---

**Intro:**
Manual edits to generated token files create drift and break reproducibility. Single source is required.

**Problem:**
generated token files are edited manually causing drift and inconsistency

**Solution:**
restrict all token generation to engine and forbid manual edits

**Signals:**
- diff drift
- inconsistent css
- build mismatch

**Search Intent:**
why generated tokens must not be edited

**Keywords:**
tokens engine single source, generated css immutability, design system governance tokens, frontend token build pipeline

---

## Principle

No CSS token file inside .generated/ may be edited manually.

All generated CSS must originate from tokens-engine.

---

## Problem

Manual edits create:

- Drift between Rust token definitions and CSS output
- Irreproducible builds
- Impossible diff tracking
- Silent regression during rebuild

---

## Forbidden Pattern

Editing:

```
styles/.generated/core.css
styles/.generated/semantic.css
styles/.generated/family-x.css
```

---

## Canonical Pattern

Single authority:

Rust token definitions →
tokens-engine →
.generated/ →
canonrs.css →
canonrs.bundle.css

---

## Enforcement

- .generated directory must be git-ignored
- CI rejects diffs in .generated
- Any edit requires modifying Rust token definitions

---

## Exceptions

None.

Generated files are artifacts, not editable source.

---

## Version History

- 1.0.0 — Initial definition