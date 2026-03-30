# Canon Rule #243: data-theme Is the Only Theme Activation Boundary

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-02-13

**Category:** design-system
**Tags:** css, theming, tokens, architecture
**Language:** EN

---

**Intro:**
Multiple theme activation mechanisms create conflicts and inconsistent behavior. A single boundary ensures deterministic theming.

**Problem:**
themes use multiple activation mechanisms causing conflicts and leaks

**Solution:**
restrict theme activation exclusively to data theme attribute

**Signals:**
- theme leak
- partial override
- inconsistent behavior

**Search Intent:**
how to enforce single theme activation css

**Keywords:**
data theme css pattern, frontend theme activation boundary, design system theming control, css theme isolation

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