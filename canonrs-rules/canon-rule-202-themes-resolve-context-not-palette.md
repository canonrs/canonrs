# Canon Rule #202: Themes Resolve Context, Not Palette

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** tokens, theming
**Version:** 1.0.0  
**Date:** 2026-01-30

---

## Principle

**Theme files (light/dark) map semantic meaning to preset variables—never define color values.**

- Theme = context resolver (light vs dark)
- Preset = palette definer (amber, blue, green)
- Theme references preset, never replaces it

---

## Problem

When themes define color values instead of referencing presets:

- **Preset switching breaks** - theme hardcoded values override preset
- **Duplicate truth** - same color defined in preset AND theme
- **Semantic drift** - `--semantic-action-primary-bg` means different things in different contexts
- **Maintenance explosion** - changing brand color requires editing themes AND presets

Real bug: Theme defined `--semantic-action-primary-bg: hsl(38 92% 50%)` → when preset changed, theme didn't follow → inconsistent UI.

---

## Forbidden Pattern

### ❌ Forbidden
```css
/* themes/light/ui.css */
@layer theme {
  [data-theme="canonrs"]:not(.dark) {
    /* ❌ Theme defining color value */
    --semantic-action-primary-bg: hsl(38 92% 50%);
    --semantic-surface-bg: hsl(0 0% 100%);
  }
}
```

**Why forbidden:** Theme is now the source of truth, not the preset. Palette switching is impossible.

---

## Canonical Pattern

### ✅ Canonical
```css
/* themes/light/ui.css */
@layer theme {
  [data-theme="canonrs"]:not(.dark) {
    /* ✅ Theme mapping semantic → preset */
    --semantic-action-primary-bg: hsl(var(--color-primary));
    --semantic-action-primary-fg: hsl(var(--color-primary-foreground));
    --semantic-surface-bg: hsl(var(--color-background));
    --semantic-surface-fg: hsl(var(--color-foreground));
  }
}
```
```typescript
/* Preset defines palette */
export const canonrsTheme: ThemeDefinition = {
  modes: {
    light: {
      colors: {
        primary: { h: 38, s: 92, l: 50 },
        background: { h: 0, s: 0, l: 100 }
      }
    }
  }
};
```

**Why correct:** Theme resolves "what does primary mean in light mode" by pointing to preset. Preset defines "what color is primary."

---

## Rationale

### Separation of Concerns
```
Preset:  "Primary is amber (38 92% 50%)"
Theme:   "In light mode, action-primary uses preset's primary"
Family:  "Buttons use semantic action-primary"
```

Each layer has **one job**.

### Architectural Invariants

1. **Palette changes happen in one place** (preset)
2. **Context changes happen in one place** (theme)
3. **Component contracts stay stable** (families/components)

### Bugs Prevented

- Preset amber → theme blue (user sees blue, preset ignored)
- Light/dark inconsistency (different hardcoded values)
- Brand color changes requiring 50+ file edits

### Why Not Opinion

This is **referential integrity**. Theme is a foreign key to preset. Denormalization breaks data consistency.

---

## Enforcement

### Static analysis
```bash
# Themes must not contain color literals
grep -E "hsl\([0-9]|#[0-9a-fA-F]{3,6}" styles/themes/**/*.css && exit 1
```

### Linter rule
```yaml
# CSS variable references only
themes/**/*.css:
  - no-color-literals
  - require-var-reference: ["--color-*"]
```

### Review checklist

- [ ] Theme files contain only `var(--color-*)` references
- [ ] Semantic tokens map to preset tokens
- [ ] No HSL/hex/RGB values in theme files

---

## Exceptions

**No exceptions. This rule is absolute.**

If a value must be hardcoded, it's not context-dependent—it's not a theme concern. Move it to component CSS or base tokens.

---

## Version History

- **1.0.0** — Initial version (2026-01-30)
