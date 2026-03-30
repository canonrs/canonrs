# Canon Rule #204: Theme Files Must Be Last in the Cascade

**Status:** ENFORCED  
**Severity:** HIGH  
**Version:** 1.0.0  
**Date:** 2026-01-30

**Category:** styling-css
**Tags:** css, cascade, themes, order
**Language:** EN

---

**Intro:**
Incorrect import order causes theme overrides to be ignored and breaks light dark context resolution. CSS cascade must enforce deterministic override priority.

**Problem:**
theme files are imported before other css causing overrides to fail

**Solution:**
import theme files last in the css cascade after all tokens and components

**Signals:**
- theme not applied
- wrong colors
- override failure

**Search Intent:**
how to fix css cascade theme override order

**Keywords:**
css cascade theme order, theme override last import, design system css order, light dark css issue

---

## Principle

**Theme files (light/dark overrides) must be imported after all other token and component CSS.**

- Themes are contextual overrides
- Overrides only work if they come last
- Order: presets → base → families → components → themes

---

## Problem

When theme files are imported early in the cascade:

- **Overrides don't apply** - subsequent imports overwrite theme values
- **Context resolution fails** - semantic tokens defined after theme win
- **Light/dark broken** - theme class has no effect
- **Debugging nightmare** - values appear correct in DevTools but don't render

Real bug: Theme imported before families → family tokens overwrote theme values → light mode showed wrong colors.

---

## Forbidden Pattern

### Forbidden
```css
/* canonrs.css - WRONG ORDER */
@import "./.generated/themes.css";          /* ❌ Theme too early */
@import "./tokens/base/ui.css";
@import "./.generated/family-c-forms.css";  /* This overwrites theme */
@import "./ui/button_ui.css";
```

**Why forbidden:** Families and components imported after themes can redefine the same variables, overwriting theme context.

---

## Canonical Pattern

### Canonical
```css
/* canonrs.css - CORRECT ORDER */
@import "./.generated/core.css";
@import "./.generated/themes.css";          /* Presets */
@import "./tokens/base/globals.css";
@import "./tokens/base/core.css";
@import "./tokens/base/ui.css";
@import "./tokens/base/layout.css";
@import "./tokens/base/blocks.css";
@import "./.generated/family-a-overlay.css";
@import "./.generated/family-b-selection.css";
@import "./.generated/family-c-forms.css";
/* ... all families ... */
@import "./ui/accordion_ui.css";
@import "./ui/button_ui.css";
/* ... all components ... */
@import "./layouts/page_layout_layout.css";
/* ... all layouts ... */
@import "./blocks/card_block.css";
/* ... all blocks ... */
@import "./themes/light/ui.css";            /* ✅ Theme overrides LAST */
@import "./themes/dark/ui.css";             /* ✅ Theme overrides LAST */
```

**Why correct:** Themes override everything. No CSS after themes can interfere with context resolution.

---

## Rationale

### CSS Cascade Rules

CSS is **last-write-wins** for same-specificity rules. Themes must write last.

### Architectural Invariants

1. **Base tokens define defaults**
2. **Families define contracts**
3. **Components consume contracts**
4. **Themes provide context** (last layer, highest priority for same variables)

### Bugs Prevented

- Light mode showing dark colors (theme overridden)
- Dark mode class having no effect
- Semantic tokens not resolving correctly
- Component styles ignoring theme context

### Why Not Opinion

This is **CSS specificity and cascade order**. Not subjective. Themes are overrides—overrides must come last.

---

## Enforcement

### Build-time validation
```bash
# Script: scripts/core/generate-canonrs-entry.sh
# Themes MUST be the last @imports
echo "@import \"./themes/light/ui.css\";" >> "$OUTPUT"
echo "@import \"./themes/dark/ui.css\";" >> "$OUTPUT"
# Nothing after this point
```

### Linter rule
```yaml
# stylelint - custom rule
import-order:
  - presets
  - base
  - families
  - components
  - themes  # Must be last
```

### Review checklist

- [ ] `canonrs.css` imports themes as final step
- [ ] No CSS files imported after `themes/*.css`
- [ ] `generate-canonrs-entry.sh` enforces theme order

---

## Exceptions

**No exceptions. This rule is absolute.**

If you need a value to override themes, it's not a theme concern—it's a component-specific override and belongs in that component's CSS with higher specificity.

---

## Version History

- **1.0.0** — Initial version (2026-01-30)