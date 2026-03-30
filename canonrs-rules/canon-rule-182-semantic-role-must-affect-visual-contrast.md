# Canon Rule #182: Semantic Role Must Affect Visual Contrast

**Status:** ENFORCED  
**Severity:** HIGH  
**Scope:** design-system, tokens
**Version:** 1.0.0  
**Date:** 2026-01-30

---

## Principle

**Components with different semantic roles must have visually distinct backgrounds—feedback states cannot share the same background as neutral surfaces.**

- Neutral surfaces = generic containers
- Feedback surfaces = contextual communication
- Visual distinction reinforces semantic meaning

---

## Problem

When semantically different components share identical backgrounds:

- **User confusion** - can't distinguish feedback from content
- **Semantic collapse** - meaning lost in visual uniformity
- **Accessibility failure** - no visual hierarchy
- **Enterprise UX violation** - feedback must be immediately recognizable

Real case: Card (neutral) vs Callout (info feedback) → if identical backgrounds, user can't distinguish informational feedback from structural content.

---

## Forbidden Pattern

### Forbidden

```css
/* themes/dark/ui.css */
--semantic-surface-bg: hsl(var(--color-background));
--semantic-info-bg: hsl(var(--color-background)); /* ❌ Same as surface */
```

```javascript
// Browser result
getComputedStyle(card).backgroundColor ===
  getComputedStyle(callout).backgroundColor;
// true ❌ - Cannot distinguish semantic roles
```

**Why forbidden:** Feedback component visually identical to neutral container. User cannot identify informational intent.

---

## Canonical Pattern

### Canonical

```css
/* themes/dark/ui.css */
--semantic-surface-bg: hsl(var(--color-background)); /* Neutral container */
--semantic-info-bg: hsl(var(--color-muted)); /* ✅ Visually distinct */
--semantic-success-bg: hsl(var(--color-accent)); /* ✅ Visually distinct */
--semantic-error-bg: hsl(var(--color-destructive)); /* ✅ Visually distinct */
--semantic-warning-bg: hsl(var(--color-primary)); /* ✅ Visually distinct */
```

```javascript
// Browser result
getComputedStyle(card).backgroundColor; // rgb(255, 255, 255) - white
getComputedStyle(callout).backgroundColor; // rgb(249, 250, 251) - subtle gray
// Different ✅ - User can distinguish roles
```

**Why correct:** Each semantic role has unique visual treatment. Feedback is immediately recognizable.

---

## Rationale

### Semantic Hierarchy

```
Neutral (surface):    White background, no intent
Info (feedback):      Subtle gray, "here's information"
Success (feedback):   Green tint, "action succeeded"
Error (feedback):     Red tint, "requires attention"
Warning (feedback):   Amber tint, "proceed with caution"
```

Visual distinction = semantic clarity.

### Architectural Invariants

1. **Feedback requires visual weight** - must be noticeable
2. **Neutrality is the baseline** - surfaces are invisible containers
3. **Context must be visible** - semantic role = visual role

### UX Principles

- **Scanability**: User can quickly identify feedback zones
- **Hierarchy**: Important information visually elevated
- **Consistency**: Same role = same visual treatment across app

### Bugs Prevented

- Info callout looks like regular content (user ignores)
- Error state invisible (user doesn't notice failure)
- Warning blends with content (user misses critical info)
- Success confirmation not recognized (user retries action)

### Why Not Opinion

This is **information design**. Different semantic meanings require different visual encodings. Not aesthetic—functional requirement.

---

## Enforcement

### Design Review Checklist

- [ ] Feedback components have distinct backgrounds from surfaces
- [ ] Info/warning/error/success are visually distinguishable
- [ ] Neutral containers use `--semantic-surface-bg`
- [ ] Feedback uses `--semantic-{intent}-bg`

### Visual Regression Test

```typescript
test("semantic roles have distinct visual contrast", () => {
  const card = document.querySelector("[data-card]");
  const callout = document.querySelector("[data-callout]");

  const cardBg = getComputedStyle(card).backgroundColor;
  const calloutBg = getComputedStyle(callout).backgroundColor;

  expect(cardBg).not.toBe(calloutBg); // Must be visually distinct
});
```

### Contrast Validation

```bash
# Automated contrast check
node scripts/validate-semantic-contrast.js
# Fails if feedback tokens equal surface tokens
```

---

## Exceptions

**Minimal exceptions:**

If a feedback component is **explicitly neutral** (e.g., `<Callout variant="default">`), it may use surface background—but this defeats the purpose of feedback and should be rare.

General rule: **If it's feedback, it must look like feedback.**

---

## Related Rules

- **Canon Rule #171**: Themes Resolve Context, Not Palette
- **Canon Rule #172**: Semantic Tokens Are the Only Bridge Between Theme and Families

---

## Version History

- **1.0.0** — Initial version (2026-01-30)
