# Canon Rule #108: Visual Surfaces Contract

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2025-01-16

**Category:** design-system
**Tags:** surfaces, tokens, css, components
**Language:** EN

---

**Intro:**
Inconsistent visual hierarchy arises when components define their own styling instead of using semantic surfaces. This leads to design drift and complex dark mode handling.

**Problem:**
components define appearance instead of using semantic surface tokens

**Solution:**
enforce surface types and token-based styling for all components

**Signals:**
- design drift
- inconsistent ui
- dark mode break

**Search Intent:**
how to enforce design surfaces

**Keywords:**
design system surfaces, css surface tokens, semantic ui surfaces, token based styling components

---

## The Principle

**Components don't choose appearance. They obey surfaces.**

Visual hierarchy in Canon is achieved through **semantic surface types**, not ad-hoc styling. This ensures consistency, prevents design drift, and makes dark mode trivial.

---

## Surface Types (MANDATORY)

### 1. surface-base
**Usage:** Application background, primary content area  
**Token:** `--color-bg-surface`

**Where to use:**
- ✅ Page backgrounds
- ✅ Layout root
- ✅ Areas without emphasis

**Where NOT to use:**
- ❌ Cards or highlighted containers
- ❌ Navigation components
- ❌ Component previews
```css
.page-root {
  background: var(--color-bg-surface);
}
```

### 2. surface-muted
**Usage:** Auxiliary areas, navigation, grouping  
**Token:** `--color-bg-muted`

**Where to use:**
- ✅ Sidebar
- ✅ Secondary navigation
- ✅ Rail containers
- ✅ Subtle grouping areas

**Where NOT to use:**
- ❌ Primary content
- ❌ Component showcases
- ❌ Documentation blocks
```css
.sidebar {
  background: var(--color-bg-muted);
}
```

### 3. surface-elevated
**Usage:** Content in focus, highlighted information  
**Token:** `--color-bg-elevated`

**Where to use:**
- ✅ Cards
- ✅ Preview blocks
- ✅ Tables
- ✅ API documentation blocks
- ✅ Comparison tables

**ALWAYS accompanied by:**
- Border (`--color-border-muted`)
- Border radius (`--radius-lg` or `--radius-xl`)
- Shadow (`--shadow-sm` or `--shadow-md`)
```css
.card {
  background: var(--color-bg-elevated);
  border: var(--border-width-hairline) solid var(--color-border-muted);
  border-radius: var(--radius-lg);
  box-shadow: var(--shadow-sm);
}
```

### 4. surface-hero
**Usage:** Component headers, documentation intros  
**Tokens:** Uses base surface + spacing + typography scale

**Key rule:**
Hero creates hierarchy through **scale and space**, NOT color contrast.

**Where to use:**
- ✅ Component headers
- ✅ Documentation hero sections
- ✅ Section introductions

**Characteristics:**
- Larger typography (`--font-size-2xl`, `--font-size-3xl`)
- Generous spacing (`--space-3xl` top padding)
- NO custom background color
```css
.hero {
  padding: var(--space-3xl) 0 var(--space-2xl);
  background: var(--color-bg-surface); /* Same as base */
}

.hero-title {
  font-size: var(--font-size-3xl);
  font-weight: var(--font-weight-bold);
  line-height: var(--line-height-tight);
}
```

---

## Shadow Contract

| Use Case | Token |
|----------|-------|
| Light cards | `--shadow-sm` |
| Important containers | `--shadow-md` |
| Overlays | `--shadow-lg` |
| Modals | `--shadow-xl` |

### Rules
- ❌ NEVER create custom shadow values
- ❌ NEVER use Tailwind arbitrary shadows (`shadow-[0_4px_8px_...]`)
- ✅ ALWAYS use semantic shadow tokens
```css
/* ✅ CORRECT */
.preview-block {
  box-shadow: var(--shadow-md);
}

/* ❌ WRONG */
.preview-block {
  box-shadow: 0 4px 8px rgba(0,0,0,0.1);
}
```

---

## Border Contract

| Situation | Token |
|-----------|-------|
| Default borders | `--color-border-default` |
| Subtle borders | `--color-border-muted` |

### Rules
- ✅ Border ALWAYS before shadow (layering order)
- ❌ Border is NEVER semantic color (danger/success)
- ✅ Border width uses `--border-width-hairline` or `--border-width-thin`
```css
/* ✅ CORRECT - Neutral border */
.elevated-card {
  border: var(--border-width-hairline) solid var(--color-border-muted);
  box-shadow: var(--shadow-sm);
}

/* ❌ WRONG - Semantic color as border */
.elevated-card {
  border: 1px solid var(--color-primary-bg); /* FORBIDDEN */
}
```

---

## Spacing Contract

| Context | Minimum Token |
|---------|---------------|
| Inside cards | `--space-lg` |
| Between sections | `--space-2xl` |
| Hero top padding | `--space-3xl` |

### Rules
- ❌ NO arbitrary padding values (`px` units)
- ❌ NO Tailwind utility padding (`p-4`, `py-8`)
- ✅ ALWAYS use semantic spacing tokens
```css
/* ✅ CORRECT */
.preview-surface {
  padding: var(--space-3xl);
  gap: var(--space-2xl);
}

/* ❌ WRONG */
.preview-surface {
  padding: 48px; /* Hardcoded */
}
```

---

## Dark Mode Law

**Dark mode does NOT redefine components. It redefines tokens.**

### ✅ Allowed
```css
html[data-theme="clean-slate"].dark {
  --color-bg-surface: hsl(222 47% 11%);
  --color-bg-elevated: hsl(217 33% 17%);
  --color-border-muted: hsl(215 14% 30%);
}
```

### ❌ Forbidden
```css
.dark .card {
  background: #1a1a1a; /* FORBIDDEN - component-level override */
}
```

**Why:**
- Component styles reference tokens
- Tokens change with theme
- Components automatically adapt
- No duplicated dark mode styles

---

## Component Mapping (CLOSED)

| Component | Surface Type |
|-----------|-------------|
| ComponentHeader | `hero` |
| PreviewBlock | `elevated` |
| ApiBlock | `elevated` |
| ComparisonBlock | `elevated` |
| RulesBlock | `elevated` |
| UsageBlock | `elevated` |
| Sidebar | `muted` |
| Page | `base` |

**This mapping is ENFORCED and CLOSED.**  
New components MUST fit into existing surface types.

---

## Real-World Examples

### ComponentHeader (Hero)
```css
.canon-component-hero {
  position: relative;
  padding: var(--space-3xl) 0 var(--space-2xl);
  margin-bottom: var(--space-3xl);
  background: var(--color-bg-surface); /* Base surface */
}

.canon-component-title {
  font-size: var(--font-size-3xl);
  font-weight: var(--font-weight-bold);
  line-height: var(--line-height-tight);
  color: var(--color-fg-default);
}
```

### PreviewBlock (Elevated)
```css
.canon-preview-surface {
  display: flex;
  flex-direction: column;
  gap: var(--space-3xl);
  
  padding: var(--space-3xl);
  
  background: linear-gradient(
    180deg,
    var(--color-bg-elevated),
    var(--color-bg-surface)
  );
  
  border: var(--border-width-hairline) solid var(--color-border-muted);
  border-radius: var(--radius-2xl);
  box-shadow: var(--shadow-xl);
}
```

### ButtonGrid Groups (Elevated Cards)
```css
.canon-button-preview-group {
  display: flex;
  flex-direction: column;
  gap: var(--space-lg);
  
  padding: var(--space-xl);
  
  background: var(--color-bg-surface);
  border-radius: var(--radius-xl);
  border: 1px solid var(--color-border-muted);
}
```

### Sidebar (Muted)
```css
.sidebar-root {
  background: var(--color-bg-muted);
  border-right: var(--border-width-hairline) solid var(--color-border-default);
}
```

---

## Anti-Patterns (FORBIDDEN)

### ❌ Creating New Colors for "Emphasis"
```css
/* WRONG */
.special-card {
  background: #f0f9ff; /* Custom blue tint - FORBIDDEN */
}
```

**Fix:** Use `surface-elevated` with proper shadow.

### ❌ Gradient as Surface
```css
/* WRONG */
.card {
  background: linear-gradient(to bottom, #fff, #f5f5f5); /* FORBIDDEN */
}
```

**Fix:** Use `var(--color-bg-elevated)` with shadow for depth.

### ❌ Tailwind Utilities in Blocks
```css
/* WRONG - component HTML */
<div class="bg-white dark:bg-gray-900 p-8 rounded-lg shadow-md">
```

**Fix:** Create semantic component with token-based CSS.

### ❌ Shadow Without Border
```css
/* WRONG */
.card {
  background: var(--color-bg-elevated);
  box-shadow: var(--shadow-md); /* Missing border */
}
```

**Fix:** Add border BEFORE shadow.

### ❌ Changing Visual Without Token
```css
/* WRONG */
.button-primary {
  background: #3b82f6; /* Hardcoded */
}
```

**Fix:** Use `var(--color-primary-bg)`.

---

## Golden Rule (UNBREAKABLE)

> **Components don't choose appearance. They obey surfaces.**

This means:
1. Components reference tokens, not values
2. Tokens change with theme/mode
3. Components automatically adapt
4. No component-specific dark mode CSS

---

## Validation Checklist

### Token Usage Audit
```bash
# Check for hardcoded colors
grep -r "background: #\|background: rgb\|background: hsl(" src/components/

# Check for hardcoded spacing
grep -r "padding: [0-9].*px\|margin: [0-9].*px" src/components/

# Check for Tailwind utilities in component CSS
grep -r "class=.*bg-\|class=.*p-\|class=.*shadow-" src/components/
```

### Dark Mode Test
```javascript
// 1. Switch to dark mode
document.documentElement.classList.add('dark')

// 2. Verify surfaces update
getComputedStyle(document.querySelector('.canon-preview-surface'))
  .backgroundColor
// Should change automatically

// 3. NO component-specific dark styles should exist
document.querySelectorAll('[class*="dark:"]').length
// Should return 0 in component CSS
```

### Surface Type Audit
```bash
# Every elevated surface must have border + shadow
grep -A 5 "color-bg-elevated" src/components/ | grep -E "border|box-shadow"
```

---

## Benefits

### ✅ Consistent Visual Hierarchy
- All cards look consistent
- Depth is predictable
- Users develop spatial memory

### ✅ Zero Dark Mode Duplication
```css
/* Before: 100 lines duplicated */
.card { background: white; }
.dark .card { background: #1a1a1a; }

/* After: 1 line, automatic */
.card { background: var(--color-bg-elevated); }
```

### ✅ Theme Portability
- New theme = change tokens
- Components adapt automatically
- No component refactoring

### ✅ Prevents Design Drift
```css
/* IMPOSSIBLE to do this */
.special-card { background: #custom; } /* Linter error */
```

---

## System Status After Rule #82

| Aspect | Status |
|--------|--------|
| Tokens | 🟢 CLOSED |
| Surfaces | 🟢 CONTRACT DEFINED |
| Dark Mode | 🟢 CONSISTENT |
| Design Drift | 🟢 IMPOSSIBLE |
| Scalability | 🟢 SAFE |

---

## Related Rules

- **Rule #81:** Token Architecture & Theme Specificity
- **Rule #25:** Theme Presets Contract
- **Rule #56:** Monorepo CSS Build Pipeline

---

## Normative Requirements

**MUST:**
- Components MUST use semantic surface tokens
- Elevated surfaces MUST include border + shadow
- Spacing MUST use semantic spacing tokens
- Dark mode MUST redefine tokens, not components

**MUST NOT:**
- Hardcode color values in components
- Create custom shadows
- Use Tailwind utilities for semantic components
- Add component-specific dark mode styles

**SHOULD:**
- Document surface type for each component
- Validate token usage in CI
- Test dark mode automatically

---

**Author:** Canon Working Group  
**Supersedes:** Implicit surface patterns  
**Related:** Canon Rule #81 (Token Architecture)