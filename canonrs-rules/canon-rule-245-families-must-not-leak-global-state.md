# Canon Rule #245: Families Must Not Leak Global State

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-02-13

**Category:** design-system
**Tags:** tokens, families, architecture, css
**Language:** EN

---

**Intro:**
Families overriding global tokens break cascade determinism and theme stability. Isolation is required.

**Problem:**
family tokens override global layers causing unpredictable state

**Solution:**
restrict families to domain scoped tokens without overriding system layers

**Signals:**
- override conflict
- theme instability
- cascade break

**Search Intent:**
how to prevent token override in families

**Keywords:**
family token isolation, design system layering css, token override prevention, frontend cascade architecture

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
## Patterns

### Forbidden Pattern

Inside family-x:

```
--color-background: ...
--theme-primary: ...
--root-bg: ...
```

---

### Canonical Pattern

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
## Contract

### Enforcement

- families are generated after semantic
- families are scoped to [data-theme]
- no family token may shadow semantic/global tokens

---

### Exceptions

None.

Families are domain isolation, not system overrides.

---

## Version History

- 1.0.0 — Initial definition