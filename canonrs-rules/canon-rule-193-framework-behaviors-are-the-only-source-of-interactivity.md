# Canon Rule #193: Framework Behaviors Are the Only Source of Interactivity

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-01-30

**Category:** behavior
**Tags:** behavior, ui, events, architecture
**Language:** EN

---

**Intro:**
Distributing interactivity across pages and UI components creates inconsistent behavior and increases bug surface. Interaction must be centralized.

**Problem:**
interactivity is implemented outside framework behaviors causing inconsistency

**Solution:**
implement all interactivity exclusively within framework behaviors

**Signals:**
- event in ui
- inconsistent behavior
- duplication

**Search Intent:**
how to centralize interactivity in framework behaviors

**Keywords:**
frontend behavior centralization, ui interaction architecture, event handling framework pattern, behavior layer design

---

## Principle

**All interactive behavior MUST be implemented exclusively inside framework behaviors.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Without this rule:

- Interactivity leaks into pages
- Multiple interaction models coexist
- UX becomes inconsistent across products

Observable failures:

- Event listeners in pages
- UI components handling their own logic
- Different behaviors for the same component

Architectural impact:

- Loss of canonical behavior
- Increased bug surface
- Impossible to guarantee consistency

---

## Forbidden Pattern

### Forbidden

```rust
// UI component
<button on:click=move |_| toggle()>
```

```rust
// Page file
element.add_event_listener_with_callback(...);
```

UI renders. Pages compose. Behaviors act.

---

## Canonical Pattern

### Canonical

```rust
// rs-design/behaviors/copyable.rs
pub struct CopyableBehavior { ... }
```

```rust
// page behavior
CopyableBehavior::new("copy-btn", "content").attach();
```

There is exactly **one canonical behavior** per interaction.

---

## Rationale

This rule guarantees:

- Uniform UX across products
- Single point of behavioral truth
- Testable, versioned interactivity
- Predictable system evolution

Behavior is part of the framework, not the product.

---

## Enforcement

- CI: forbid DOM APIs outside framework behaviors
- Review: UI components must be stateless
- Pages must never attach events directly

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version