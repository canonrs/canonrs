# Canon Rule #55: Canonical CSS Entry Points

**Status:** ENFORCED


**Severity:** HIGH
**Scope:** build, css
**Version:** 1.0.0
**Date:** 2025-01-16

---

---

## Principle

Design System CSS **MUST** be consumed exclusively through **canonical workspace packages**.  
Internal directories such as `packages-rust/`, `crates/`, or generated Rust artifacts are **implementation details**, never valid import targets.

This rule exists to **eliminate fragile relative paths**, ensure **monorepo scalability**, and guarantee **package portability** across environments.

---

## Forbidden Patterns (ABSOLUTE)

```css
/* ❌ NEVER ALLOWED */
@import "../../../../packages-rust/rs-tailwind/tokens/theme.css";
@import "../../../crates/rs-design/src/themes/generated.css";
```

### Why this is forbidden

- PostCSS uses `enhanced-resolve`, which **breaks on deep relative paths**
- Relative paths encode repository topology into consumer code
- Any folder move breaks every consumer silently
- Makes publishing to npm impossible without refactor
- Violates separation between **artifact producer** and **artifact consumer**

---

## Canonical Architecture

### Required Workspace Layout

```
packages/
├─ canonrs-tailwind/
│  ├─ package.json
│  └─ dist/
│     ├─ tokens.css
│     └─ globals.css
│
├─ canonrs-design/
│  ├─ package.json
│  └─ dist/
│     └─ themes.css
```

**Key rule:**  
> Canonical packages **re-export artifacts**.  
> They do **not** generate them.

---

## Canonical Package Definitions

### `@canonrs/tailwind`

```json
{
  "name": "@canonrs/tailwind",
  "private": true,
  "version": "0.1.0",
  "exports": {
    "./tokens.css": "./dist/tokens.css",
    "./globals.css": "./dist/globals.css"
  }
}
```

### `@canonrs/design`

```json
{
  "name": "@canonrs/design",
  "private": true,
  "version": "0.1.0",
  "exports": {
    "./themes.css": "./dist/themes.css"
  }
}
```

---

## Mandatory Consumer Pattern

```css
/* ✅ CANONICAL */
@import "@canonrs/tailwind/tokens.css";
@import "@canonrs/tailwind/globals.css";
@import "@canonrs/design/themes.css";
```

No other import source is allowed.

---

## Enforcement Checklist

- [ ] No CSS imports reference `packages-rust/`
- [ ] No CSS imports reference `crates/`
- [ ] All CSS imports use `@canonrs/*`
- [ ] Canonical packages export only `dist/*`
- [ ] No relative paths longer than `../`

---

## Canonical Justification

This rule exists because **Design Systems are products**, not side-effects of Rust crates.

A system that cannot be:
- moved,
- published,
- versioned,
- cached,
- or reasoned about independently  

is **not enterprise-grade**.

---

## Canon References

- Canon Rule #56 — Monorepo CSS Build Pipeline
- Canon Rule #57 — PostCSS Canon Configuration

