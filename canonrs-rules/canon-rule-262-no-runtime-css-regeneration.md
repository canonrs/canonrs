# Canon Rule #262: No Runtime CSS Regeneration

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-02-13

**Category:** build-tooling
**Tags:** css, runtime, build, determinism
**Language:** EN

---

**Intro:**
Runtime css generation introduces nondeterminism and hydration issues. Styles must be fully static.

**Problem:**
css is generated or mutated at runtime causing inconsistency and mismatch

**Solution:**
generate css only at build time and serve as static artifact

**Signals:**
- hydration mismatch
- style drift
- cache inconsistency

**Search Intent:**
why css should not be generated at runtime

**Keywords:**
runtime css generation problem, static css build architecture, frontend deterministic styling, avoid dynamic css injection

---

## Principle

CanonRS CSS MUST be generated at build time only.

No runtime CSS generation, mutation, or injection is allowed.

---

## Problem

Runtime generation introduces:

- Non-deterministic styling
- Hydration mismatches
- Debug vs release divergence
- Impossible caching guarantees

---

## Forbidden Pattern

Generating tokens in:

- WASM at runtime
- SSR boot phase
- Client-side JS
- Inline style injection

---

## Canonical Pattern

Build phase:

tokens-engine →
generated files →
bundle →
served static

Runtime:
Static CSS only.

---

## Rationale

Design system is infrastructure.

Infrastructure must be deterministic.

---

## Enforcement

- No dynamic CSS string building in codebase
- No token computation in WASM
- CI scans for `document.styleSheets.insertRule`

---

## Exceptions

None.

All style authority belongs to build system.

---

## Version History

- 1.0.0 — Initial definition