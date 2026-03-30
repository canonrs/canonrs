# Canon Rule #154: Deterministic Layout via Canonical CSS (SSR-Safe)

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-01-26

**Category:** styling-css
**Tags:** css, layout, ssr, hydration
**Language:** EN

---

**Intro:**
Runtime layout manipulation causes DOM divergence between SSR and client hydration, leading to errors. Layout must be resolved deterministically via CSS.

**Problem:**
layout is controlled by runtime logic instead of css causing hydration mismatch

**Solution:**
define all layout positioning in css and use data attributes for state

**Signals:**
- hydration mismatch
- layout drift
- runtime positioning

**Search Intent:**
how to fix layout hydration mismatch using css

**Keywords:**
css deterministic layout, ssr layout mismatch, data attribute css pattern, layout rendering consistency

---

## Principle

**All layout positioning MUST be resolved exclusively by canonical CSS, never by runtime DOM manipulation or utility-based composition.**

---

## The Problem

Without this rule, SSR applications suffer from **hydration mismatch panics** caused by DOM divergence between server render and client hydration.

Observable symptoms include:

- Hydration errors when toggling layout elements
- Runtime panics in Leptos / React SSR
- Inconsistent sidebar positioning between SSR and CSR
- Hidden coupling between JS logic and layout
- Impossible-to-debug UI drift

These failures commonly arise when layout decisions are deferred to runtime utilities or conditional wrappers.

---

## Forbidden Patterns

### ❌ Forbidden

```rust
// Conditional wrapper changes DOM structure after hydration
{is_collapsed && view! { <div class="ml-auto">...</div> }}
```

```html
<!-- Runtime utility-based positioning -->
<button class="flex justify-end ml-auto">Toggle</button>
```

```css
/* Inline or dynamic layout styles */
style="margin-left: auto;"
```

These patterns violate the rule by:

- Changing DOM structure between SSR and CSR
- Encoding layout logic in runtime composition
- Making layout non-deterministic

---

## Canonical Pattern

### ✅ Canonical

```html
<button class="sidebar-toggle" data-sidebar-toggle></button>
```

```css
/* Canonical layout positioning */
.sidebar-toggle {
  position: absolute;
  top: var(--spacing-md);
  right: var(--spacing-md);
}
```

```html
<html data-sidebar="collapsed"></html>
```

```css
/* State-driven layout via data attributes */
html[data-sidebar="collapsed"] .sidebar {
  width: var(--sidebar-width-collapsed);
}
```

This respects the rule because:

- DOM structure is static and identical in SSR and CSR
- Layout is fully resolved in CSS
- State is expressed declaratively via `data-*`
- JavaScript only triggers state changes, not layout

---

## Rationale

This rule enforces **deterministic rendering**, a core invariant of SSR systems.

It guarantees:

- Identical DOM trees between server and client
- Zero hydration mismatch
- Clear separation of concerns:
  - CSS defines layout
  - Data attributes define state
  - JS triggers actions only

This is not stylistic preference.  
It is a **hard architectural boundary** required for correctness, stability, and enterprise-scale maintainability.

---

## Enforcement

This rule MUST be enforced via:

- Code review (reject runtime layout composition)
- Static analysis (detect utility-based layout in critical zones)
- CI checks on forbidden class usage in layouts
- Architectural audits for SSR paths

Any violation is a build-blocking defect.

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial canonical version