# Canon Rule #337: Hover State Must Not Override Active State

**Status:** ENFORCED
**Severity:** MEDIUM
**Version:** 1.0.0
**Date:** 2026-04-05

**Category:** css-contracts
**Tags:** css, hover, active, state, specificity
**Language:** EN

---

**Intro:**
CSS hover styles applied without guarding against active state will override the active appearance on mouse-over, creating a visual regression where the selected item loses its active styling during hover.

**Problem:**
`[data-rs-tabs-trigger]:hover` overrides `[data-rs-tabs-trigger][data-rs-state~="active"]` when the user hovers over the active tab, removing the primary color and replacing it with the muted hover style.

**Solution:**
All hover selectors on stateful components must exclude the active state via `:not([data-rs-state~="active"])`.

**Signals:**
- active tab loses its color when hovered
- selected item visually deselects on mouse-over
- hover and active styles conflict

**Search Intent:**
css hover overrides active, tab active color disappears on hover, hover state priority

**Keywords:**
css specificity hover active, not selector active state, data-rs-state hover override

---

## Principle

Active state has semantic permanence. Hover is transient. Transient states must never override permanent ones.

---

## Problem
```css
/* ❌ broken — hover overrides active */
[data-rs-tabs-trigger]:hover {
  background: var(--tabs-trigger-bg-hover);
  color: var(--tabs-trigger-fg-hover);
}
```

---

## Patterns

### Forbidden Pattern
Hover selector without `:not([data-rs-state~="active"])` on any component with an active state.

### Canonical Pattern
```css
/* ✅ hover only applies when not active */
[data-rs-tabs-trigger]:hover:not([data-rs-state~="active"]):not([disabled]) {
  background: var(--tabs-trigger-bg-hover);
  color: var(--tabs-trigger-fg-hover);
}
```

---

## Contract

### Enforcement
- every `:hover` selector on a stateful component must include `:not([data-rs-state~="active"])`
- applies to: tabs, nav items, sidebar items, any component with persistent active state
- disabled state must also be excluded: `:not([disabled])`

### Exceptions
- components with no active state (e.g. plain buttons without selection) are exempt

---

## Version History
- 1.0.0 - Initial definition
