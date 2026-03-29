# Canon Rule #278: Primitive Must Not Encode Design Tokens in Rust

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** css, primitives, tokens
**Version:** 1.0.0  
**Date:** 2026-02-13

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
