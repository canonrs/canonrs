# Canon Rule #203: Semantic Tokens Are the Only Bridge Between Theme and Families

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-01-30

**Category:** design-system
**Tags:** tokens, semantic, architecture, css
**Language:** EN

---

**Intro:**
Skipping semantic tokens couples components directly to palette and breaks context resolution. A strict token cascade is required.

**Problem:**
components or families bypass semantic tokens causing coupling and context loss

**Solution:**
enforce semantic tokens as the only bridge between theme and families

**Signals:**
- token bypass
- context loss
- refactor break

**Search Intent:**
how to enforce semantic token layer design system

**Keywords:**
semantic token architecture, design system token cascade, frontend dependency inversion tokens, css token layering pattern

---

## Principle

**Family tokens must reference semantic tokens exclusively—never preset or theme tokens directly.**

- Families bind to `--semantic-*`
- Components bind to `--family-*` (e.g., `--button-primary-bg`)
- No component or family skips the semantic layer

---

## Problem

When families or components reference preset/theme tokens directly:

- **Context is lost** - component doesn't know "why" it's using a color
- **Theme switching breaks** - components hardwired to specific palette
- **Semantic drift** - "primary" means different things in different components
- **Refactor impossible** - changing semantic meaning requires editing every component

Real bug: Button used `--color-primary` directly → worked in dark, broke in light (context wasn't resolved).

---

## Forbidden Pattern

### Forbidden
```css
/* family-c-forms.css */
[data-theme="canonrs"] {
  --button-primary-bg: hsl(var(--color-primary)); /* ❌ Skips semantic layer */
}
```
```css
/* button_ui.css */
[data-button][data-ui-variant="solid"] {
  background: hsl(var(--color-primary)); /* ❌ Component directly using preset */
}
```

**Why forbidden:** Component is now coupled to preset. Semantic meaning is lost. Refactoring semantic layer has no effect.

---

## Canonical Pattern

### Canonical
```css
/* themes/dark/ui.css - Theme resolves context */
@layer theme {
  [data-theme="canonrs"].dark {
    --semantic-action-primary-bg: hsl(var(--color-primary));
    --semantic-action-primary-fg: hsl(var(--color-primary-foreground));
  }
}
```
```css
/* family-c-forms.css - Family binds to semantic */
[data-theme="canonrs"] {
  --button-primary-bg: var(--semantic-action-primary-bg);
  --button-primary-fg: var(--semantic-action-primary-fg);
}
```
```css
/* button_ui.css - Component uses family */
[data-button][data-ui-variant="solid"] {
  background: var(--button-primary-bg);
  color: var(--button-primary-fg);
}
```

**Why correct:** Each layer has one job. Component knows "I'm a primary action," not "I'm amber."

---

## Rationale

### Token Cascade
```
Preset:    --color-primary (palette)
           ↓
Theme:     --semantic-action-primary-bg (context)
           ↓
Family:    --button-primary-bg (contract)
           ↓
Component: background: var(--button-primary-bg) (consumption)
```

**Every layer is replaceable without breaking downstream layers.**

### Architectural Invariants

1. **Semantic is the contract** - families and components depend on it
2. **Preset/theme can change** - semantic absorbs the change
3. **Components are context-agnostic** - they consume intent, not color

### Bugs Prevented

- Button breaks when semantic "primary" changes meaning
- Light/dark inconsistency (component bypasses context resolution)
- Impossible to globally change "what primary means"

### Why Not Opinion

This is **dependency inversion**. High-level components depend on abstraction (semantic), not concretion (preset).

---

## Enforcement

### Static analysis
```bash
# Families must not reference --color-*
grep -r "var(--color-" styles/.generated/family-*.css && exit 1
```

### Linter rule
```yaml
# Family tokens
family-*.css:
  - no-direct-preset-reference
  - require-semantic-tokens-only

# Component CSS
**/*_ui.css:
  - no-preset-tokens
  - no-semantic-tokens
  - require-family-tokens-only
```

### Review checklist

- [ ] Families reference only `--semantic-*`
- [ ] Components reference only `--button-*`, `--card-*`, etc (family tokens)
- [ ] No `--color-*` or `--semantic-*` in component CSS

---

## Exceptions

**No exceptions. This rule is absolute.**

If a component needs a value that doesn't fit the semantic layer, create a new semantic token. If it's truly one-off, it's not a token—hardcode it in that component.

---

## Version History

- **1.0.0** — Initial version (2026-01-30)