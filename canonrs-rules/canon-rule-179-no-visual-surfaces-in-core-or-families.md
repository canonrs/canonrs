# Canon Rule #179: No Visual Surface Tokens in Core or Families

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** design-system, tokens, theming
**Version:** 1.0.0  
**Date:** 2026-01-29

---

## Principle

**Core and Family tokens MUST NOT define visual surface values; all visual surfaces MUST be resolved at the theme layer.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

When visual surface values are defined in Core or Family tokens:

- Dark mode breaks (e.g. white or high-luminance surfaces in dark themes)
- Themes cannot adapt surfaces independently
- Visual intent leaks into foundational layers
- Recurrent regressions occur when adding new components or variants
- Design contracts become ambiguous and non-scalable

Architectural impact:
- Theme coupling
- Visual drift across modes
- Non-deterministic UI behavior in enterprise contexts

---

## Forbidden Pattern

### Forbidden

```rust
// Core or Family token defining a visual surface directly
FamilyToken::new("color-info-light", "hsl(199 89% 96%)")
FamilyToken::new("alert-bg-info", "var(--color-info-light)")
```

Why this violates the rule:

- Encodes appearance in a foundational layer
- Is not theme-aware
- Cannot adapt across light/dark or future themes
- Breaks the Core → Theme → UI contract

---

## Canonical Pattern

### Canonical

```rust
// Core defines semantic intent only
FamilyToken::new("semantic-info-surface", "var(--theme-info-surface)")

// Family consumes semantic surface
FamilyToken::new("alert-bg-info", "var(--semantic-info-surface)")
```

```css
/* Theme resolves visual appearance */
[data-theme="canonrs"] {
  --theme-info-surface: hsl(199 89% 96%);
}

[data-theme="canonrs"].dark {
  --theme-info-surface: hsl(199 60% 18%);
}
```

Why this complies with the rule:

- Core defines intent, not appearance
- Theme owns visual decisions
- Families remain semantic and reusable
- Dark mode and future themes are deterministic

---

## Rationale

This rule exists to enforce a strict architectural separation:

- **Core** defines immutable semantic contracts
- **Themes** define visual appearance
- **Families** bind semantics to component meaning
- **UI** implements behavior

Invariants protected:
- No visual values in foundational layers
- Full theme control over surfaces
- Predictable dark mode behavior
- Enterprise-grade scalability

This is not an opinion:
- Violations produce measurable regressions
- The rule is testable and enforceable
- The separation is required for multi-theme systems

---

## Enforcement

This rule is enforced by:

- CI checks rejecting `*-light`, `*-surface` visual values in Core or Families
- Static grep-based validation in build scripts
- Mandatory code review checklist
- Build-time failure on forbidden token patterns

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
