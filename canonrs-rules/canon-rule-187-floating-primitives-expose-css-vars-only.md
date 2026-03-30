# Canon Rule #187: Floating Primitives Enforce CSS-Variable-Only Positioning

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.1.0  
**Date:** 2026-02-03  

**Category:** governance
**Tags:** overlays, primitives, css, enforcement
**Language:** EN

---

**Intro:**
Allowing primitives to manipulate layout directly creates inconsistencies and breaks separation of concerns. Positioning must be enforced through a single contract.

**Problem:**
primitives mutate layout properties directly causing inconsistency

**Solution:**
restrict primitives to writing only css variables for positioning

**Signals:**
- inline top left
- layout mutation
- contract violation

**Search Intent:**
how to enforce css variable positioning primitives

**Keywords:**
floating ui css enforcement, overlay positioning contract, css variable layout control, frontend primitives architecture

---

## Principle

**Floating primitives MUST enforce that positioning is communicated only via CSS custom properties.**

This rule defines the **internal enforcement mechanism** of the overlay positioning contract.

---

## Enforcement Rules

- Primitives may:
  - Compute geometry
  - Write to `--floating-*` CSS variables
- Primitives MUST NOT:
  - Mutate layout properties
  - Set inline `top`, `left`, `bottom`, `right`
  - Bypass CSS rendering

---

## Forbidden Pattern

```rust
element.style().set_property("top", "120px").unwrap(); // ❌
```

---

## Canonical Pattern

```rust
style.set_property("--floating-x", "300px").ok();
style.set_property("--floating-y", "120px").ok();
```

---

## Relationship to Other Rules

- **Canon Rule #183** — where positioning logic lives
- **Canon Rule #184** — public positioning contract
- **Canon Rule #187** — enforcement of that contract

---

## Exceptions

**None. Enforcement is mandatory.**

---