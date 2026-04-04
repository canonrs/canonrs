# Canon Rule #332: Group Override Selector Must Match Base Specificity

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-04

**Category:** css-architecture
**Tags:** css, specificity, cascade, group, override
**Language:** EN

---

**Intro:**
CSS selectors in group or container components that override styles of child components must have specificity greater than or equal to the base selector of the child component.

**Problem:**
A group selector with fewer conditions than the child's base selector loses the cascade silently. The override is present in the CSS but never applies.

**Solution:**
Replicate all `:not()` conditions from the base child selector in the group override selector.

**Signals:**
- group override CSS is present in the bundle but has no visual effect
- computed styles show the child's base rule winning
- adding `!important` fixes it (forbidden — signals a specificity problem)

**Search Intent:**
css group override not working, specificity lost in cascade, child selector winning over parent context

**Keywords:**
css specificity group, override cascade, not selector specificity, button group hover not applying

---

## Principle

CSS specificity is determined by the number and type of selectors. A group context selector (`[data-rs-button-group] [data-rs-button]`) does not automatically outrank a more complex base selector (`[data-rs-button]:not(...):not(...):not(...)`). The override must match or exceed the base selector's complexity.

---

## Problem
```css
/* button_ui.css — wins */
[data-rs-button][data-rs-state~="hover"]:not([data-rs-state~="disabled"]):not([data-rs-variant="ghost"]):not([data-rs-variant="link"]) {
  opacity: 0.92;
}

/* button_group_ui.css — loses (lower specificity) */
[data-rs-button-group] [data-rs-button][data-rs-state~="hover"] {
  opacity: 0.92;
}
```

---

## Patterns

### Forbidden Pattern
```css
[data-rs-button-group] [data-rs-button][data-rs-state~="hover"] {
  opacity: var(--state-opacity-hover);
}
```

### Canonical Pattern
```css
[data-rs-button-group][data-rs-state~="attached"] [data-rs-button][data-rs-state~="hover"]:not([data-rs-state~="disabled"]):not([data-rs-variant="ghost"]):not([data-rs-variant="link"]) {
  opacity: var(--state-opacity-hover);
}
```

---

## Contract

### Enforcement

- group override selectors must replicate all `:not()` conditions from the base child selector
- `!important` is forbidden as a specificity workaround
- group selectors must include their own state condition (e.g. `[data-rs-state~="attached"]`) to add specificity

### Exceptions

None.

---

## Version History

- 1.0.0 - Initial definition
