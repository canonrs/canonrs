# Canon Rule #278: Primitive Must Not Encode Design Tokens in Rust

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13

**Category:** design-system
**Tags:** primitives, tokens, css, architecture
**Language:** EN

---

**Intro:**
Embedding design tokens directly in Rust primitives breaks token cascade and theming flexibility. Visual decisions must remain in the CSS layer.

**Problem:**
design tokens are hardcoded in rust primitives breaking theming and token cascade

**Solution:**
move all visual decisions to css using data attributes and token system

**Signals:**
- inline styles
- var(-- usage in rust
- hardcoded classes

**Search Intent:**
how to prevent hardcoded design tokens in components

**Keywords:**
design tokens rust primitives, css token separation architecture, avoid inline styles components, data attribute styling system

---

## Principle

**Primitives must never hardcode design tokens, CSS variables, colors, spacing, or layout values in Rust.**

---

## Problem

When tokens leak into Rust:

- Token cascade is broken
- Theming becomes impossible
- Structural layer becomes presentation layer
- Build-time guarantees are lost

---

## Forbidden Pattern

```rust
style="padding: 16px;"
style="color: var(--color-primary);"
class="bg-primary text-white"
```

Rust must not encode visual decisions.

---

## Canonical Pattern

```rust
<div data-button="" class=class>
```

CSS layer resolves tokens based on data-attributes.

---

## Rationale

Protects:
- Token cascade integrity
- Theme override safety
- Separation of structure and presentation

---

## Enforcement

- Code review
- Static grep for `var(--` inside primitives
- CI audit for inline styles

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version