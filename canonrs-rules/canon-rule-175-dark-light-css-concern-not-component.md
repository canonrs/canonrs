# Canon Rule #175: Dark/Light Is a CSS Concern, Not a Component Concern

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** components, css
**Version:** 1.0.0  
**Date:** 2026-01-30

---

## Principle

**Components must never reference `.dark` class or implement dark/light logic—theme system handles all context resolution.**

- Components are theme-agnostic
- Dark/light is CSS variable resolution, not selector logic
- No component CSS contains `.dark` selectors

---

## Problem

When components implement dark/light logic:

- **Duplication** - every component reimplements theme switching
- **Inconsistency** - components interpret "dark" differently
- **Coupling** - components coupled to theme implementation
- **Maintenance explosion** - changing theme strategy requires editing all components
- **Hydration mismatches** - SSR and CSR disagree on theme state

Real discovery: Button worked perfectly in dark AND light **without any component changes**—proof that theme system is complete.

---

## Forbidden Pattern

### Forbidden
```css
/* button_ui.css */
[data-button][data-ui-variant="solid"] {
  background: var(--button-primary-bg);
}

/* ❌ Component knows about dark mode */
.dark [data-button][data-ui-variant="solid"] {
  background: hsl(38 92% 50%);
}
```
```css
/* ❌ Component has dark-specific selector */
[data-button][data-ui-variant="solid"].dark {
  background: var(--button-primary-bg-dark);
}
```

**Why forbidden:** Component is now aware of theme context. Violates separation of concerns. Creates maintenance burden.

---

## Canonical Pattern

### Canonical
```css
/* button_ui.css - Component is theme-agnostic */
[data-button][data-ui-variant="solid"] {
  background: var(--button-primary-bg);  /* ✅ No dark/light logic */
  color: var(--button-primary-fg);
}
```
```css
/* themes/dark/ui.css - Theme resolves context */
@layer theme {
  [data-theme="canonrs"].dark {
    --semantic-action-primary-bg: hsl(var(--color-primary));
  }
}
```
```css
/* themes/light/ui.css */
@layer theme {
  [data-theme="canonrs"]:not(.dark) {
    --semantic-action-primary-bg: hsl(var(--color-primary));
  }
}
```

**Why correct:** Component references stable token. Theme system resolves value based on context. Component code unchanged for dark/light.

---

## Rationale

### Separation of Concerns
```
Component:  "I need a primary action background"
Theme:      "In dark mode, primary action is hsl(38 92% 50%)"
```

Component doesn't know or care about dark/light. Theme injects the right value.

### Architectural Invariants

1. **Components consume contracts** - they reference `--button-*` tokens
2. **Themes provide implementations** - they resolve semantic → values
3. **No component logic for context** - context is CSS cascade, not selectors

### Bugs Prevented

- Component implements dark mode incorrectly
- Inconsistent dark/light across components
- SSR/CSR hydration mismatch (component logic differs)
- Theme refactor requires editing 78 components

### Why Not Opinion

This is **interface segregation**. Components depend only on the interface they need (`--button-primary-bg`), not implementation details (`.dark` class).

---

## Enforcement

### Linter Rule
```bash
# No component CSS can reference .dark
grep -r "\.dark" styles/ui/*.css && exit 1
grep -r "\.dark" styles/blocks/*.css && exit 1
grep -r "\.dark" styles/layouts/*.css && exit 1
```

### Static Analysis
```yaml
# stylelint
selectors-no-theme-classes:
  - error
  - forbidden: [".dark", ".light"]
    scope: ["ui/**", "blocks/**", "layouts/**"]
```

### Review Checklist

- [ ] Component CSS has no `.dark` or `.light` selectors
- [ ] Component CSS has no `[data-theme]` selectors
- [ ] Components only reference family tokens (`--button-*`, `--card-*`)

---

## Exceptions

**No exceptions. This rule is absolute.**

If a component needs context-aware styling, the token layer is incomplete. Create semantic tokens to express that context.

---

## Version History

- **1.0.0** — Initial version (2026-01-30)
