# Canon Rule #161: Canonical CSS Load Order Is Mandatory

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** css, theming
**Version:** 1.0.0  
**Date:** 2026-01-27

---

## Principle

**CanonRS CSS must always load in a strict, predefined order.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Incorrect CSS order causes silent visual corruption.

- Tokens overridden by components
- Themes partially applied
- Layout misalignment
- Non-reproducible UI bugs

---

## Forbidden Pattern

### ❌ Forbidden

```html
<link rel="stylesheet" href="components.css" />
<link rel="stylesheet" href="tokens.css" />
```

Why this violates the rule.

---

## Canonical Pattern

### ✅ Canonical

```html
<link rel="stylesheet" href="tokens.css" />
<link rel="stylesheet" href="themes.css" />
<link rel="stylesheet" href="variants.css" />
<link rel="stylesheet" href="ui.css" />
<link rel="stylesheet" href="layouts.css" />
```

Why this complies with the rule.

---

## Rationale

CSS is a dependency graph, not a flat list.

- Enforces deterministic styling
- Preserves token invariants
- Prevents cascade inversion
- Architectural, not stylistic

---

## Enforcement

- Static HTML inspection
- CI snapshot comparison
- Review checklist

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
