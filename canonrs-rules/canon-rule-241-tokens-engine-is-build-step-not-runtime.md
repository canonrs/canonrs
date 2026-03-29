# Canon Rule #241: Tokens Engine Is Build Infrastructure, Not Runtime Code

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** tokens, architecture
**Version:** 1.0.0
**Date:** 2026-02-13

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
