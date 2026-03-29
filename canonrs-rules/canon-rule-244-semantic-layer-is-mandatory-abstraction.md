# Canon Rule #244: Semantic Layer Is a Mandatory Abstraction

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** tokens, css, architecture
**Version:** 1.0.0
**Date:** 2026-02-13

---

## Principle

UI, Blocks and Root must consume semantic tokens only.

Never theme tokens directly.

---

## Problem

Direct theme consumption causes:

- Tight coupling to preset vocabulary
- Impossible re-theming
- Architectural collapse
- Token naming chaos

---

## Forbidden Pattern

```
background: var(--theme-surface-bg); ❌ inside UI
border: var(--theme-border);         ❌ inside component
```

---

## Canonical Pattern

```
background: var(--color-background);
border: var(--color-border);
```

Where:

--color-* → semantic bridge → --theme-* → normalized preset value

---

## Cascade Responsibility

Preset → Theme → Semantic → Families → Root → UI

Skipping semantic layer is forbidden.

---

## Enforcement

- semantic_generator defines all --color-* bridges
- root consumes semantic only
- UI consumes semantic only

---

## Exceptions

None.

Semantic is mandatory.

---

## Version History

- 1.0.0 — Initial definition
