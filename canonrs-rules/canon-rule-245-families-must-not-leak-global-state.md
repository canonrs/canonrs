# Canon Rule #245: Families Must Not Leak Global State

**Status:** ENFORCED
**Severity:** HIGH
**Scope:** tokens, architecture
**Version:** 1.0.0
**Date:** 2026-02-13

---

## Principle

Family tokens exist for component-domain grouping.

They must not:

- Override semantic tokens
- Override theme tokens
- Mutate root contract
- Redefine primitives

---

## Problem

If families override globals:

- State becomes unpredictable
- Visual contract breaks
- Theme switching becomes unstable
- Cascade loses determinism

---

## Forbidden Pattern

Inside family-x:

```
--color-background: ...
--theme-primary: ...
--root-bg: ...
```

---

## Canonical Pattern

Family defines domain-scoped variables only:

```
--overlay-bg
--selection-active-bg
--form-field-border
--feedback-success-border
```

Families provide structure.
They do not override system layers.

---

## Enforcement

- families are generated after semantic
- families are scoped to [data-theme]
- no family token may shadow semantic/global tokens

---

## Exceptions

None.

Families are domain isolation, not system overrides.

---

## Version History

- 1.0.0 — Initial definition
