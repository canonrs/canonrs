# Canon Rule #243: data-theme Is the Only Theme Activation Boundary

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** css, theming, architecture
**Version:** 1.0.0
**Date:** 2026-02-13

---

## Principle

[data-theme] is the only valid activation boundary for themes.

No theme may rely on:

- :root overrides
- body classes
- global resets
- implicit dark-mode inheritance

---

## Problem

Multiple activation mechanisms cause:

- Theme leakage
- Partial overrides
- Conflicting scopes
- Unpredictable dark behavior

---

## Forbidden Pattern

```
:root { --theme-bg: ... }          ❌
body.dark { ... }                  ❌
html[data-theme=...] .component    ❌
```

---

## Canonical Pattern

```
[data-theme="clean-slate"] { ... }
[data-theme="clean-slate"].dark { ... }
```

Everything theme-bound must be inside [data-theme].

---

## Enforcement

- theme_generator outputs only [data-theme] scoped variables
- semantic layer consumes theme tokens only
- root layer never references preset values directly

---

## Exceptions

None.

[data-theme] is a hard boundary.

---

## Version History

- 1.0.0 — Initial definition
