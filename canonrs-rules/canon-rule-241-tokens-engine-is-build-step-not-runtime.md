# Canon Rule #241: Tokens Engine Is Build Infrastructure, Not Runtime Code

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-02-13

**Category:** governance
**Tags:** tokens, build, runtime, architecture
**Language:** EN

---

**Intro:**
Including build tooling in runtime creates dependency leaks and bloated binaries. Tokens engine must remain isolated.

**Problem:**
tokens engine is used at runtime causing dependency leakage and architectural violation

**Solution:**
restrict tokens engine to build time and prevent runtime imports

**Signals:**
- dependency leak
- runtime bloat
- build graph issue

**Search Intent:**
why tokens engine not runtime dependency

**Keywords:**
tokens engine build only, runtime dependency leak rust, design system generator separation, frontend build vs runtime

---

## Principle

The tokens-engine is a build-time tool.

It must never be a runtime dependency.

---

## Problem

If tokens-engine becomes runtime:

- Product crates depend on build tooling
- Runtime binaries grow incorrectly
- Architectural layering collapses
- Feature flags leak
- Workspace graph becomes unstable

---

## Forbidden Pattern

```
canonrs-ui → canonrs-tokens (bin)
products → tokens-engine
```

Or:

```
use canonrs_tokens::bin::*;
```

---

## Canonical Pattern

```
tokens-engine (bin)
   ↓ generates CSS
rs-design (library)
   ↓ consumed by
canonrs-ui
   ↓ consumed by
products
```

No runtime code depends on generator code.

---

## Enforcement

- tokens-engine exists only as binary.
- Library modules expose data structures only.
- Runtime crates must not reference generator modules.

---

## Exceptions

None.

Build tooling must remain isolated.

---

## Version History

- 1.0.0 — Initial definition