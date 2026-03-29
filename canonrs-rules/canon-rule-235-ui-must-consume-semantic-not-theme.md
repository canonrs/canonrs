# Canon Rule #235: UI Layer Must Consume Semantic Tokens, Never Theme Directly

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** tokens, ui, theming
**Version:** 1.0.0  
**Date:** 2026-02-13

---

## Principle

UI components MUST consume only semantic tokens (`--color-*`).

UI MUST NOT reference:

- `--theme-*`
- primitive tokens
- raw HSL values
- hardcoded colors

---

## Forbidden Pattern

```css
.button {
  background: var(--theme-surface-bg); /* ❌ Theme direct usage */
}
```

```css
.card {
  background: hsl(0 0% 100%); /* ❌ Raw color */
}
```

---

## Canonical Pattern

Semantic defines bridge:

```
--color-background → --theme-surface-bg
```

UI consumes only:

```
background: hsl(var(--color-background));
```

---

## Rationale

This guarantees:

- Theme swappability
- Multi-theme coexistence
- Semantic abstraction
- No UI ↔ Theme coupling

Theme changes must never require UI modification.

---

## Enforcement

CI must search for:

- `--theme-` usage in UI layer
- raw HSL values inside UI
- hardcoded colors

All are violations.

---

## Exceptions

**No exceptions.**
