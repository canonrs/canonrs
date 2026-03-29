# Canon Rule #207: Preset Switching Must Never Change Component CSS

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** architecture, tokens, components
**Version:** 1.0.0  
**Date:** 2026-01-30

---

## Principle

**Changing theme preset (amber → blue → green) must only change CSS variable values—never component structure or selectors.**

- Preset switching is runtime variable substitution
- No component rebuild required
- No component CSS changes

---

## Problem

When preset changes require component edits:

- **Not scalable** - can't ship multiple presets to users
- **Build-time coupling** - preset becomes a compile-time constant
- **User theming impossible** - users can't customize colors
- **Maintenance explosion** - adding preset = editing components

Real test: Amber preset worked in dark AND light **without touching any component CSS**—proof of correct architecture.

---

## Forbidden Pattern

### ❌ Forbidden
```css
/* button_ui.css - Component hardcoded to preset */
[data-button][data-ui-variant="solid"] {
  background: hsl(38 92% 50%);  /* ❌ Amber hardcoded */
}
```
```css
/* button_ui.css - Component aware of preset */
[data-theme="amber"] [data-button] {
  background: hsl(38 92% 50%);
}

[data-theme="blue"] [data-button] {
  background: hsl(210 100% 50%);
}
```

**Why forbidden:** Component must be rebuilt for every preset. User theming is impossible.

---

## Canonical Pattern

### ✅ Canonical
```css
/* button_ui.css - Preset-agnostic */
[data-button][data-ui-variant="solid"] {
  background: var(--button-primary-bg);  /* ✅ Token reference only */
  color: var(--button-primary-fg);
}
```
```typescript
/* presets/amber.ts */
export const amber: ThemeDefinition = {
  modes: {
    dark: {
      colors: {
        primary: { h: 38, s: 92, l: 50 }  /* Amber */
      }
    }
  }
};
```
```typescript
/* presets/blue.ts */
export const blue: ThemeDefinition = {
  modes: {
    dark: {
      colors: {
        primary: { h: 210, s: 100, l: 50 }  /* Blue */
      }
    }
  }
};
```
```html
<!-- Runtime preset switching -->
<html data-theme="amber" class="dark">
<html data-theme="blue" class="dark">
```

**Why correct:** Component CSS never changes. Only CSS variable values change. Instant theme switching.

---

## Rationale

### Token Indirection
```
Component:  background: var(--button-primary-bg)
            ↓
Family:     --button-primary-bg: var(--semantic-action-primary-bg)
            ↓
Theme:      --semantic-action-primary-bg: hsl(var(--color-primary))
            ↓
Preset:     --color-primary: 38 92% 50%  (ONLY THING THAT CHANGES)
```

Preset swap = change one value. Entire UI updates.

### Architectural Invariants

1. **Components are data-agnostic** - they reference contracts
2. **Contracts are stable** - token names never change
3. **Values are dynamic** - presets provide different values for same tokens

### Bugs Prevented

- Component rebuild required for preset change
- User can't customize colors
- Theme switching requires page reload
- Multiple presets bloat bundle size

### Why Not Opinion

This is **dependency injection**. Components depend on abstractions (tokens), not concrete values (colors). Allows runtime substitution.

---

## Enforcement

### Build verification
```bash
# Component CSS must not change between builds with different presets
PRESET=amber npm run build && cp dist/canonrs.css canonrs-amber.css
PRESET=blue npm run build && cp dist/canonrs.css canonrs-blue.css

# Component sections must be identical
diff <(grep -A 50 "button_ui.css" canonrs-amber.css) \
     <(grep -A 50 "button_ui.css" canonrs-blue.css) \
  && echo "✅ Components are preset-agnostic"
```

### Architecture test
```typescript
// Test: switching preset without rebuild
test("preset switching updates colors without component changes", () => {
  const root = document.documentElement;
  
  // Amber preset
  root.setAttribute("data-theme", "amber");
  expect(getComputedStyle(button).backgroundColor).toBe("rgb(245, 158, 11)");
  
  // Blue preset (no rebuild, just change attribute)
  root.setAttribute("data-theme", "blue");
  expect(getComputedStyle(button).backgroundColor).toBe("rgb(59, 130, 246)");
  
  // Component CSS never changed
  expect(button.style.cssText).toBe("");
});
```

### Review checklist

- [ ] Components reference only family tokens
- [ ] No color values in component CSS
- [ ] Preset switching is attribute change, not rebuild

---

## Exceptions

**No exceptions. This rule is absolute.**

If changing preset requires changing component CSS, the token architecture is broken. Fix the token layer.

---

## Version History

- **1.0.0** — Initial version (2026-01-30)
