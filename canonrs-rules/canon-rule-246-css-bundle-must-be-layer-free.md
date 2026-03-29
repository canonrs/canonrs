# Canon Rule #246: CSS Bundle Must Be Layer-Free

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** css, build
**Version:** 1.0.0
**Date:** 2026-02-13

---

## Principle

The final distributed artifact (`canonrs.bundle.css`) MUST NOT contain:

- @layer directives
- @import statements
- Nested composition logic

The bundle must be fully flattened and deterministic.

---

## Problem

If @layer or @import leaks into the bundle:

- CSS order becomes tool-dependent
- Runtime cascade becomes unstable
- Products gain implicit authority over CanonRS
- Tailwind or other processors alter system hierarchy

---

## Forbidden Pattern

Inside canonrs.bundle.css:

```
@layer components { ... }   ❌
@import "./something.css";  ❌
```

---

## Canonical Pattern

Flat, ordered CSS:

```
:root { ... }
[data-theme="..."] { ... }
[data-family-x] { ... }
[data-ui] { ... }
```

No structural directives allowed in final artifact.

---

## Enforcement

- bundler.rs removes @layer
- bundler resolves all @import recursively
- CI rejects bundle containing @layer or @import

---

## Exceptions

None.

The bundle is a deployment artifact, not a development input.

---

## Version History

- 1.0.0 — Initial definition
