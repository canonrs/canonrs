# Canon Rule #155: CSS Token Contract Is Architecture

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** design-system, css, tokens
**Version:** 1.0.0  
**Date:** 2026-01-26

---

## Principle

**CSS design tokens MUST use a single, deterministic format (numeric HSL) and NEVER mix formats within a design system.**

- Tokens define values only, not CSS functions
- Consumption applies functions at use site
- Format contract is immutable across all tokens

---

## Problem

Without uniform token format:

- **Runtime CSS failures**: `background: hsl(var(--color-primary))` breaks if token contains `hsl()` already
- **Theming impossible**: Cannot interpolate or transform wrapped values
- **Silent bugs**: Browser falls back to defaults, appearing as "CSS not loading"
- **Maintenance chaos**: Developers cannot predict token format, leading to defensive code

**Real symptom**: Button renders with `rgba(0,0,0,0)` background because `hsl(var(--primary))` receives `hsl(221 83% 53%)` instead of `221 83% 53%`.

---

## Forbidden Pattern

### ❌ Forbidden
```css
/* tokens.css - MIXED FORMATS */
--color-primary: hsl(221 83% 53%);        /* ❌ wrapped */
--color-secondary: 180 50% 60%;           /* ✅ numeric */
--color-accent: #3b82f6;                  /* ❌ hex */

/* consumption */
background: hsl(var(--color-primary));    /* BREAKS - double hsl() */
background: var(--color-accent);          /* INCONSISTENT with others */
```

**Why forbidden:**
- No contract - each token requires format inspection
- `hsl(var(--color-primary))` produces invalid `hsl(hsl(...))`
- Dark mode themes cannot transform hex values
- Developers write defensive code checking format per token

---

## Canonical Pattern

### ✅ Canonical
```css
/* tokens.css - UNIFORM FORMAT */
:root {
  --color-primary: 221 83% 53%;
  --color-secondary: 180 50% 60%;
  --color-accent: 217 91% 60%;
}

/* Semantic layer applies function */
html[data-theme] {
  --primary: var(--color-primary);
}

/* Consumption - always predictable */
.button {
  background: hsl(var(--primary));
  color: hsl(var(--primary-foreground));
}
```

**Why canonical:**
- Single format = predictable consumption
- Themes can transform numeric HSL (`calc()`, opacity)
- No format inspection needed
- Build tools can validate contract

---

## Rationale

**Architectural invariants:**

1. **Theming requires raw values**: `hsl()` wrapper prevents opacity manipulation (`hsl(var(--primary) / 0.5)` requires numeric input)
2. **Build-time validation**: Uniform format enables static analysis
3. **Developer ergonomics**: One mental model, zero format guessing
4. **Framework agnostic**: Numeric HSL works in CSS, Tailwind, CSS-in-JS

**This is not preference** - it's the only format that supports:
- Alpha channel manipulation
- Color space transformations
- Static type checking in TypeScript token generators

---

## Enforcement

**Build-time:**
```bash
# Validate all tokens are numeric HSL
grep -r "^[[:space:]]*--color-" style/tokens.css | \
  grep -v "^[[:space:]]*--color-[^:]*: [0-9.]* [0-9.]*% [0-9.]*%;" && \
  echo "❌ Token format violation" && exit 1
```

**Linting:**
```javascript
// stylelint rule
"custom-property-pattern": {
  "^color-": "^[0-9.]+ [0-9.]+% [0-9.]+%$"
}
```

**CI check:**
- All theme presets must pass format validation
- `build:design` fails on mixed formats
- No `hsl()`, `rgb()`, or hex in token definitions

---

## Exceptions

**No exceptions. This rule is absolute.**

Token format is architectural contract. Violating it breaks theming, SSR color resolution, and creates silent runtime failures.

If a value cannot be expressed in numeric HSL, it is not a color token.

---

## Version History

- **1.0.0** — Initial version (2026-01-26)
