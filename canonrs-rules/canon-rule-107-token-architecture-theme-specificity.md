# Canon Rule #107: Token Architecture & Theme Specificity

**Status:** ENFORCED
**Severity:** CRITICAL
**Scope:** tokens, css, architecture
**Version:** 1.0.0
**Date:** 2025-01-16

---

## The Principle

**:root defines structure. Themes define colors.**

CSS token architecture MUST separate structural tokens (spacing, typography, shadows) from thematic tokens (colors). This prevents specificity conflicts and ensures themes can override only what they should.

---

## The Problem

### ❌ Wrong Pattern (Conflicting Specificity)
```css
/* tokens.css */
:root {
  --color-primary-bg: hsl(38 92% 50%);  /* Hardcoded default */
  --color-bg-surface: hsl(0 0% 100%);
  --space-lg: 1rem;  /* OK - structural */
}

[data-theme="clean-slate"] {
  --color-primary-bg: hsl(238 83% 66%);  /* 🚫 CANNOT override :root */
}
```

**Why this breaks:**
- `:root` has equal specificity to `[data-theme]`
- Source order determines winner (unpredictable)
- Themes cannot reliably override colors
- **Result:** Clean Slate shows Amber colors

---

## The Solution

### ✅ Correct Pattern (Separation of Concerns)
```css
/* tokens.css - STRUCTURAL ONLY */
:root {
  /* ━━━ STRUCTURAL (never change with theme) ━━━ */
  --space-xs: 0.25rem;
  --space-sm: 0.5rem;
  --space-lg: 1rem;
  --space-xl: 1.5rem;
  
  --font-size-sm: 0.875rem;
  --font-size-md: 1rem;
  --font-size-lg: 1.125rem;
  
  --radius-sm: 0.25rem;
  --radius-md: 0.375rem;
  --radius-lg: 0.5rem;
  
  --shadow-sm: 0 1px 2px 0 rgb(0 0 0 / 0.05);
  --shadow-md: 0 4px 6px -1px rgb(0 0 0 / 0.1);
  
  --z-base: 0;
  --z-popover: 1000;
  --z-modal: 3000;
  
  /* ━━━ NO COLOR TOKENS HERE ━━━ */
}

/* theme-presets.css - COLORS ONLY */
html[data-theme="clean-slate"] {
  --background: 210 40% 98%;
  --foreground: 217 33% 17%;
  --primary: 238 83% 66%;
  --primary-foreground: 0 0% 100%;
  
  /* Canon semantic tokens */
  --color-bg-surface: hsl(210 40% 98%);
  --color-bg-elevated: hsl(0 0% 100%);
  --color-primary-bg: hsl(238 83% 66%);
  --color-primary-fg: hsl(0 0% 100%);
}

html[data-theme="clean-slate"].dark {
  --background: 222 47% 11%;
  --foreground: 214 32% 91%;
  --primary: 234 89% 73%;
  
  --color-bg-surface: hsl(222 47% 11%);
  --color-bg-elevated: hsl(217 33% 17%);
  --color-primary-bg: hsl(234 89% 73%);
}
```

**Key differences:**
1. `:root` contains ZERO color tokens
2. `html[data-theme]` uses element selector for higher specificity
3. Themes control ALL color-related tokens
4. Structural tokens never conflict with themes

---

## Token Classification

### Structural Tokens (in :root)
**Never theme-specific, always system-wide**

| Category | Tokens | Rationale |
|----------|--------|-----------|
| **Spacing** | `--space-*`, `--space-control-*` | Mathematical scale |
| **Typography** | `--font-*`, `--line-height-*` | Brand identity |
| **Radius** | `--radius-*` | Personality system-wide |
| **Shadow** | `--shadow-*` | Elevation hierarchy |
| **Z-Index** | `--z-*` | Stacking context |
| **Motion** | `--motion-*` | UX consistency |
| **Border Width** | `--border-width-*` | Visual weight |

### Thematic Tokens (in [data-theme])
**Theme-specific, override per preset**

| Category | Tokens | Rationale |
|----------|--------|-----------|
| **Background** | `--color-bg-*` | Surface colors |
| **Foreground** | `--color-fg-*` | Text colors |
| **Primary** | `--color-primary-*` | Brand accent |
| **Semantic** | `--color-danger-*`, `--color-success-*` | Status colors |
| **Border** | `--color-border-*` | Structural outlines |

---

## Specificity Rules

### CSS Specificity Hierarchy
```css
/* SPECIFICITY: 0-0-0 (lowest) */
:root {
  --space-lg: 1rem;  /* ✅ OK - structural */
}

/* SPECIFICITY: 0-1-0 */
[data-theme="amber"] {
  --color-primary: ...;  /* ⚠️ Equal to :root (source order wins) */
}

/* SPECIFICITY: 0-1-1 (CORRECT) */
html[data-theme="amber"] {
  --color-primary: ...;  /* ✅ ALWAYS wins over :root */
}

/* SPECIFICITY: 0-2-1 (HIGHER) */
html[data-theme="amber"].dark {
  --color-primary: ...;  /* ✅ Wins over light mode */
}
```

### Golden Rule
**Always use `html[data-theme="..."]` for themes**

This ensures theme tokens ALWAYS override `:root` defaults, regardless of source order.

---

## File Structure

### Canonical Layout
```
packages-rust/rs-design/style/
├── tokens.css              # Structural tokens only (:root)
├── theme-presets.css       # Color tokens only (html[data-theme])
└── components/
    └── *.css              # Use var(--color-*), var(--space-*)
```

### Import Order (CRITICAL)
```css
/* application.css */
@import "./tokens.css";         /* 1. Structure first */
@import "./theme-presets.css";  /* 2. Themes override */
@import "./components/*.css";   /* 3. Components use tokens */
```

**Why order matters:**
- Themes must load AFTER structural tokens
- Components must load AFTER themes
- Wrong order = themes don't apply

---

## Real-World Example

### Before (Broken)
```css
/* tokens.css */
:root {
  --space-lg: 1rem;                    /* ✅ OK */
  --color-primary-bg: hsl(38 92% 50%); /* ❌ WRONG - color in :root */
}

[data-theme="clean-slate"] {
  --color-primary-bg: hsl(238 83% 66%); /* 🚫 Cannot override */
}
```

**Symptom:** Selecting "Clean Slate" shows Amber colors.

**Console test:**
```javascript
getComputedStyle(document.documentElement)
  .getPropertyValue('--color-primary-bg')
// Returns: "hsl(38 92% 50%)" (WRONG - should be Clean Slate blue)
```

### After (Fixed)
```css
/* tokens.css */
:root {
  --space-lg: 1rem;  /* ✅ Structural only */
  /* NO COLOR TOKENS */
}

/* theme-presets.css */
html[data-theme="clean-slate"] {
  --color-primary-bg: hsl(238 83% 66%);  /* ✅ Always wins */
}
```

**Console test:**
```javascript
getComputedStyle(document.documentElement)
  .getPropertyValue('--color-primary-bg')
// Returns: "hsl(238 83% 66%)" (CORRECT - Clean Slate blue)
```

---

## Validation Checklist

### Token Audit
```bash
# Check for color tokens in :root (FORBIDDEN)
grep -A 100 "^:root {" tokens.css | grep "color-"
# Should return EMPTY

# Check for structural tokens in themes (DISCOURAGED)
grep -A 50 "\[data-theme=" theme-presets.css | grep "space-\|font-\|radius-"
# Should return MINIMAL or NONE
```

### Browser DevTools Test
```javascript
// 1. Check theme applied
document.documentElement.getAttribute('data-theme')
// Should return: "clean-slate" or "amber-minimal"

// 2. Check class applied
document.documentElement.className
// Should return: "light" or "dark"

// 3. Verify color override
getComputedStyle(document.documentElement).getPropertyValue('--color-primary-bg')
// Should return theme-specific HSL value

// 4. Verify structural token
getComputedStyle(document.documentElement).getPropertyValue('--space-lg')
// Should return: "1rem" (same for all themes)
```

---

## Anti-Patterns

### ❌ Colors in :root
```css
:root {
  --color-bg-surface: hsl(0 0% 100%);  /* 🚫 FORBIDDEN */
}
```

**Fix:** Move to `html[data-theme="default"]`

### ❌ [data-theme] without html
```css
[data-theme="amber"] {  /* ⚠️ Low specificity */
  --color-primary: ...;
}
```

**Fix:** Use `html[data-theme="amber"]`

### ❌ Structural tokens in themes
```css
html[data-theme="amber"] {
  --space-lg: 2rem;  /* 🚫 DISCOURAGED - breaks consistency */
}
```

**Fix:** Keep structural tokens in `:root` only

### ❌ !important in themes
```css
html[data-theme="amber"] {
  --color-primary: ... !important;  /* 🚫 NEVER - specificity is enough */
}
```

**Fix:** Remove `!important`, rely on specificity

---

## Benefits

### ✅ Predictable Theme Switching
- Theme tokens ALWAYS override defaults
- No source-order dependency
- Dark mode works consistently

### ✅ Maintainable Architecture
```
:root             → Infrastructure (never changes)
html[data-theme]  → Aesthetics (changes per theme)
```

### ✅ Portable Themes
- Themes are self-contained
- No hidden dependencies on :root colors
- Easy to add new themes

### ✅ Debuggable
```javascript
// Always shows active theme's value
getComputedStyle(html).getPropertyValue('--color-primary-bg')
```

---

## Related Rules

- **Rule #25:** Theme Presets Contract (what themes can define)
- **Rule #56:** Monorepo CSS Build Pipeline (how CSS artifacts are built)
- **Rule #50:** Provider Singleton Pattern (how themes are applied via React context)

---

## Normative Requirements

**MUST:**
- Structural tokens MUST live in `:root` only
- Color tokens MUST live in `html[data-theme]` selectors
- Theme selectors MUST include `html` element for specificity
- Import order MUST be: tokens → themes → components

**MUST NOT:**
- Color tokens MUST NOT appear in `:root`
- Structural tokens MUST NOT be overridden by themes
- Use `!important` in theme definitions
- Use bare `[data-theme]` without `html` element

**SHOULD:**
- Validate token separation in CI
- Test theme switching in browser DevTools
- Document theme-specific color meanings

---

**Author:** Canon Working Group  
**Supersedes:** Implicit token patterns  
**Related:** Canon Rule #25 (Theme Presets Contract)
