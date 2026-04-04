# Canon Rule #333: Island CSS Must Use Descendant Selector Not Child Combinator

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-04

**Category:** css-architecture
**Tags:** css, islands, selector, leptos, wasm
**Language:** EN

---

**Intro:**
CSS selectors targeting elements inside Leptos islands must use the descendant combinator (space) instead of the direct child combinator (`>`). Leptos injects `<leptos-island>` and `<leptos-children>` wrapper elements between a parent component and its slotted children, breaking child selectors.

**Problem:**
A CSS rule using `>` between a parent component and an island child will never match because the DOM contains intermediate Leptos wrapper elements.

**Solution:**
Always use the descendant combinator (space) when writing selectors that span across island boundaries.

**Signals:**
- CSS rule is present in bundle but never applies to island children
- rule works in isolation but breaks when component is wrapped in an island
- DevTools shows the selector does not match despite correct DOM attributes

**Search Intent:**
leptos island child selector not working, css child combinator island, leptos-children breaks selector

**Keywords:**
leptos island css, child combinator island, leptos-children wrapper, descendant selector leptos

---

## Principle

Leptos islands render wrapper elements in the DOM: `<leptos-island>` wraps the island component and `<leptos-children>` wraps slotted children. These elements sit between the parent and child in the DOM tree, making `>` (direct child) selectors unable to match across island boundaries.

---

## Problem

DOM structure:
```html
<div data-rs-button-group="">         <!-- parent -->
  <leptos-children>                   <!-- injected by Leptos -->
    <leptos-island>                   <!-- injected by Leptos -->
      <button data-rs-button="">      <!-- target -->
```
```css
/* ❌ never matches — > skips leptos-children and leptos-island */
[data-rs-button-group] > [data-rs-button] {
  border-radius: 0;
}
```

---

## Patterns

### Forbidden Pattern
```css
[data-rs-button-group][data-rs-state~="attached"] > [data-rs-button] {
  border-radius: var(--button-group-radius-merge);
}
```

### Canonical Pattern
```css
[data-rs-button-group][data-rs-state~="attached"] [data-rs-button] {
  border-radius: var(--button-group-radius-merge);
}
```

---

## Contract

### Enforcement

- `>` combinator is forbidden in any selector that crosses an island boundary
- all group-to-child selectors must use the descendant combinator (space)
- this applies to all CSS files in the design system that target island children

### Exceptions

- `>` is acceptable within a single non-island component where no Leptos wrappers exist

---

## Version History

- 1.0.0 - Initial definition
