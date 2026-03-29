# Canon Rule #234: Tokens Engine Is the Single Source of CSS Truth

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** tokens, css, architecture
**Version:** 1.0.0  
**Date:** 2026-02-13

---

## Principle

All design tokens MUST originate exclusively from the CanonRS Tokens Engine.

No other file, layer, or tool is allowed to define, override, or regenerate token values.

---

## Absolute Constraints

- `.generated/` directory is **read-only**
- `canonrs.bundle.css` is a **compiled artifact**
- No product may define token variables manually
- UI, Blocks, and Layouts cannot declare new tokens
- Utility frameworks have zero authority over token definition

---

## Forbidden Pattern

```css
:root {
  --color-background: hsl(0 0% 100%); /* ❌ Manual token */
}
```

```css
@layer base {
  --color-primary: red; /* ❌ Tailwind controlling tokens */
}
```

---

## Canonical Model

Tokens flow only in this direction:

```
Tokens Engine (Rust)
        ↓
.generated/*.css
        ↓
canonrs.bundle.css
        ↓
Products consume as artifact
```

No lateral generation allowed.

---

## Rationale

This protects:

1. Determinism
2. Governance
3. Version control
4. Artifact isolation
5. Enterprise rollback capability

Tokens are architecture, not styling preference.

---

## Enforcement

- CI fails if token vars are defined outside tokens engine
- `.generated/` is git-ignored and must never be edited
- Any manual token declaration is a hard violation

---

## Exceptions

**No exceptions.**
