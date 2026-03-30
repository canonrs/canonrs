# Canon Rule #208: A Working Theme Toggle Is Proof of Correct Token Architecture

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** governance, architecture
**Version:** 1.0.0  
**Date:** 2026-01-30

---

## Principle

**If theme toggle breaks visual rendering, the error is in token architecture—not JavaScript or component logic.**

- Theme toggle is an architectural litmus test
- Visual bugs during toggle indicate token layer failures
- Working toggle = correct separation of concerns

---

## Problem

When theme toggle causes visual bugs:

- **Token cascade is broken** - values not resolving correctly
- **Semantic layer incomplete** - tokens missing for some contexts
- **Component coupling** - components hardcoded to one mode
- **Theme overrides wrong** - cascade order incorrect

Real discovery: JavaScript toggle code was **correct from day one**. Visual bugs were 100% caused by token layer mistakes (hardcoded colors, wrong cascade order, missing semantic mappings).

---

## Forbidden Pattern

### Forbidden
```javascript
// ❌ Attempting to fix visual bugs in JavaScript
const toggle = () => {
  html.classList.toggle("dark");
  
  // ❌ Manually forcing colors (workaround for broken tokens)
  if (html.classList.contains("dark")) {
    document.querySelectorAll('[data-button]').forEach(btn => {
      btn.style.backgroundColor = "#F59E0B";  // ❌ Symptom treatment
    });
  }
};
```

**Why forbidden:** This treats symptoms, not root cause. Token architecture is broken. Fix tokens, not JavaScript.

---

## Canonical Pattern

### Canonical
```bash
# Step 1: Verify JavaScript is correct
# Toggle adds/removes .dark class?
document.documentElement.classList.contains('dark')  # Should toggle

# Step 2: Verify preset is loaded
getComputedStyle(document.documentElement).getPropertyValue('--color-primary')
# Should return preset value (e.g., "38 92% 50%")

# Step 3: Verify semantic resolution
getComputedStyle(document.documentElement).getPropertyValue('--semantic-action-primary-bg')
# Should change when toggling .dark

# Step 4: Verify family binding
getComputedStyle(document.documentElement).getPropertyValue('--button-primary-bg')
# Should match semantic token

# Step 5: Verify component consumption
getComputedStyle(document.querySelector('[data-button]')).backgroundColor
# Should match family token
```

**Fix tokens at the failing layer. Never fix in JavaScript.**

---

## Rationale

### Theme Toggle as Architectural Test
```
Toggle adds .dark class
  ↓
CSS cascade resolves [data-theme].dark selector
  ↓
Semantic tokens update via theme overrides
  ↓
Family tokens reference semantic (no change needed)
  ↓
Components reference family (no change needed)
  ↓
Visual updates automatically
```

**If this chain breaks, the break is in CSS, not JS.**

### Architectural Invariants

1. **Toggle changes one thing** - adds/removes `.dark` class
2. **CSS does the rest** - cascade resolves everything
3. **Components are passive** - they consume tokens, don't react to toggle

### Bugs Prevented

- Fixing token bugs in JavaScript (wrong layer)
- Adding toggle logic to components (wrong abstraction)
- Forcing colors via inline styles (bypasses architecture)
- Believing toggle code is the problem (it almost never is)

### Why Not Opinion

This is **separation of concerns testing**. If changing context (dark/light) breaks rendering, context isn't properly abstracted.

---

## Enforcement

### Architecture validation test
```typescript
test("theme toggle updates all components via tokens", () => {
  const button = document.querySelector('[data-button]');
  const card = document.querySelector('[data-card]');
  
  // Dark mode
  document.documentElement.classList.add('dark');
  const darkBg = getComputedStyle(button).backgroundColor;
  const darkCardBg = getComputedStyle(card).backgroundColor;
  
  // Light mode
  document.documentElement.classList.remove('dark');
  const lightBg = getComputedStyle(button).backgroundColor;
  const lightCardBg = getComputedStyle(card).backgroundColor;
  
  // Colors changed
  expect(darkBg).not.toBe(lightBg);
  expect(darkCardBg).not.toBe(lightCardBg);
  
  // JavaScript did not touch component styles
  expect(button.style.cssText).toBe("");
  expect(card.style.cssText).toBe("");
});
```

### Debug checklist (when toggle breaks)

- [ ] Verify JavaScript: does `.dark` class toggle?
- [ ] Verify preset: does `--color-primary` exist?
- [ ] Verify theme: does `--semantic-*` change with `.dark`?
- [ ] Verify family: does `--button-*` reference `--semantic-*`?
- [ ] Verify component: does component use `--button-*` (not hardcoded)?
- [ ] Verify cascade: are themes imported last?

### Review principle

> **If toggle breaks visuals, assume tokens are wrong—not JavaScript.**

Investigate in this order:
1. Token definitions (preset, semantic, family)
2. CSS cascade order
3. Component token references
4. JavaScript (last resort, usually correct)

---

## Exceptions

**No exceptions. This rule is absolute.**

If toggle requires JavaScript beyond class manipulation, the token architecture is fundamentally broken.

---

## Version History

- **1.0.0** — Initial version (2026-01-30)
