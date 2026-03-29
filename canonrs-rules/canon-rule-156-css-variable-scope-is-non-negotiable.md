# Canon Rule #156: CSS Variable Scope Is Non-Negotiable

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** design-system, css, theming
**Version:** 1.0.0  
**Date:** 2026-01-26

---

## Principle

**CSS variable mappings MUST be defined in the same selector scope as the source variables they reference.**

- `:root` does NOT inherit from `html[data-theme]`
- Mappings in wrong scope resolve to undefined
- Scope must match theme definition exactly

---

## Problem

Without scope alignment:

- **Variables resolve to empty**: `var(--primary)` returns `""` in browser
- **Themes don't apply**: Button stays default browser color despite theme being set
- **Silent failures**: CSS loads, HTML correct, but values never populate
- **Debugging nightmare**: Everything "looks right" but doesn't work

**Real symptom**: `getComputedStyle(document.documentElement).getPropertyValue('--primary')` returns empty string despite `--color-primary` being defined in theme.

---

## Forbidden Pattern

### ❌ Forbidden
```css
/* generated-themes.css */
html[data-theme="amber"] {
  --color-primary: 37.69 92% 50%;
  --color-secondary: 221 83% 53%;
}

/* tokens.css - WRONG SCOPE */
:root {
  --primary: var(--color-primary);        /* ❌ UNDEFINED */
  --secondary: var(--color-secondary);    /* ❌ UNDEFINED */
}

/* Result: var(--primary) is empty at runtime */
```

**Why forbidden:**
- CSS variables do NOT traverse up selector specificity
- `:root` executes before `html[data-theme]` in cascade
- `var(--color-primary)` in `:root` has no value to reference
- Creates illusion of working code that fails silently

---

## Canonical Pattern

### ✅ Canonical
```css
/* generated-themes.css */
html[data-theme="amber"] {
  --color-primary: 37.69 92% 50%;
  --color-secondary: 221 83% 53%;
}

/* tokens.css - CORRECT SCOPE */
html[data-theme] {
  --primary: var(--color-primary);        /* ✅ DEFINED */
  --secondary: var(--color-secondary);    /* ✅ DEFINED */
}

/* Consumption works */
.button {
  background: hsl(var(--primary));        /* ✅ Resolves correctly */
}
```

**Why canonical:**
- Mapping exists in same scope as source
- `html[data-theme]` matches theme selector specificity
- Variables resolve deterministically
- Works for all themes without scope conflicts

---

## Rationale

**CSS cascade rules:**

1. **Specificity matters**: `html[data-theme]` (0,1,1) beats `:root` (0,1,0)
2. **Declaration order**: `:root` loads before themed selectors
3. **No upward reference**: `var()` only resolves within current scope or inherited

**This is CSS spec, not opinion:**
```
html[data-theme] defines --color-primary
  ↓ (can reference here)
html[data-theme] maps --primary: var(--color-primary) ✅
  
:root tries to map --primary: var(--color-primary) ❌
  ↑ (cannot reference - wrong scope)
html[data-theme] defines --color-primary
```

**Why it matters:**
- Themes must be hot-swappable (change `data-theme` value)
- SSR must match CSR scope resolution
- Dark mode requires scope precision

---

## Enforcement

**Runtime check:**
```javascript
// In browser console or E2E test
const primary = getComputedStyle(document.documentElement)
  .getPropertyValue('--primary');

if (primary === '') {
  throw new Error('Variable scope violation: --primary undefined');
}
```

**Build-time validation:**
```bash
# Ensure mappings are in html[data-theme], not :root
grep -A 5 "^:root {$" style/tokens.css | \
  grep "var(--color-" && \
  echo "❌ Scope violation: theme mapping in :root" && exit 1
```

**Linting:**
- Theme variable references must be in `html[data-theme]` block
- `:root` can only define base tokens, never map themed variables

---

## Exceptions

**No exceptions. This rule is absolute.**

CSS variable scope is W3C spec behavior. Violating it creates runtime failures that cannot be "worked around" - the variables simply don't resolve.

If a mapping is needed globally, the source variable must also be global (`:root`).

---

## Version History

- **1.0.0** — Initial version (2026-01-26)
