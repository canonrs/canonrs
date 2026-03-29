# Canon Rule #262: No Runtime CSS Regeneration

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** build, architecture
**Version:** 1.0.0
**Date:** 2026-02-13

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
