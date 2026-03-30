# Canon Rule #201: Presets Are the Only Source of Color Values

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-01-30

**Category:** design-system
**Tags:** tokens, colors, theming, css
**Language:** EN

---

**Intro:**
Defining color values outside presets creates duplication and breaks palette switching. Color data must remain centralized.

**Problem:**
color values are defined outside presets causing duplication and inconsistency

**Solution:**
define all color values exclusively within preset definitions

**Signals:**
- color drift
- palette break
- hardcoded color

**Search Intent:**
how to centralize color values in design system

**Keywords:**
design system color source, preset color architecture, css color token centralization, theming palette management

---

## Principle

**Absolute color values (HSL, hex, RGB) must only exist in preset definitions.**

- No color literals in themes
- No color literals in semantic tokens
- No color literals in family tokens
- Presets are the single source of truth for palette

---

## Problem

When themes or semantic tokens define colors directly:

- **Preset is overridden** - palette switching breaks
- **Light mode inconsistency** - different colors than dark
- **Maintenance nightmare** - color changes require editing multiple files
- **Cascade violations** - override order becomes unpredictable

Real bug: `themes/light/ui.css` defined `hsl(210 40% 98%)` → preset amber was ignored → button turned gray in light mode.

---

## Forbidden Pattern

### Forbidden
```css
/* themes/light/ui.css */
@layer theme {
  [data-theme="canonrs"]:not(.dark) {
    --semantic-action-primary-bg: hsl(210 40% 98%); /* ❌ Hardcoded color */
  }
}
```
```css
/* tokens/base/ui.css */
:root {
  --semantic-surface-bg: hsl(0 0% 100%); /* ❌ Hardcoded white */
}
```

**Why forbidden:** Preset values cannot override these. Palette switching is impossible.

---

## Canonical Pattern

### Canonical
```css
/* themes/light/ui.css */
@layer theme {
  [data-theme="canonrs"]:not(.dark) {
    --semantic-action-primary-bg: hsl(var(--color-primary)); /* ✅ References preset */
  }
}
```
```typescript
/* src/tokens/themes/presets/canonrs.ts */
export const canonrsTheme: ThemeDefinition = {
  modes: {
    light: {
      colors: {
        primary: { h: 38, s: 92, l: 50 }  // ✅ Color defined here only
      }
    }
  }
};
```

**Why correct:** Changing preset changes all derived tokens. One source of truth.

---

## Rationale

### Architectural Invariants

1. **Palette is a design decision** - must be centralized
2. **Themes resolve context** - not palette
3. **Components are palette-agnostic** - they reference tokens, not colors

### Bugs Prevented

- Preset switching breaking visual consistency
- Light/dark mode color mismatches
- Maintenance drift (colors defined in 5 places)
- Override cascade conflicts

### Why Not Opinion

Color values are **data**. Data must have a single source. This is database normalization applied to CSS.

---

## Enforcement

### Build-time validation
```bash
# Fail if themes contain color literals
grep -r "hsl([0-9]" styles/themes/ && exit 1
```

### Linter rule
```yaml
# stylelint
rules:
  color-no-literal-in-themes:
    - error
    - except: ["presets/*.css"]
```

### Review checklist

- [ ] All color changes go through preset files
- [ ] Theme files only reference `var(--color-*)`
- [ ] Semantic tokens reference `var(--color-*)` or `var(--semantic-*)`

---

## Exceptions

**No exceptions. This rule is absolute.**

If a color value must exist, it belongs in a preset. If it's not palette-dependent, it's not a color token—it's a component-specific value and belongs in that component's CSS.

---

## Version History

- **1.0.0** — Initial version (2026-01-30)