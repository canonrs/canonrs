# Canon Rule #246: CSS Bundle Must Be Layer-Free

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-02-13

**Category:** build-tooling
**Tags:** css, bundle, build, architecture
**Language:** EN

---

**Intro:**
Layer directives in final css introduce runtime instability and tool interference. Bundle must be flattened.

**Problem:**
css bundle contains layer or import directives causing instability

**Solution:**
flatten bundle fully and remove all structural directives

**Signals:**
- layer directive
- import leak
- runtime instability

**Search Intent:**
how to remove css layer directives bundle

**Keywords:**
css bundle flattening, remove @layer css build, frontend css artifact generation, design system css build output

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