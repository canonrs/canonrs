# Canon Rule #267: Data Attributes Are Contract, Not Styling Mechanism

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-02-13

**Category:** component-architecture
**Tags:** data-attributes, contracts, css, behavior
**Language:** EN

---

**Intro:**
Using classes for state or behavior creates coupling and ambiguity. Structural contracts must be explicit.

**Problem:**
classes are used to represent state causing coupling and ambiguity

**Solution:**
use data attributes to represent structural and behavioral contracts

**Signals:**
- class misuse
- state ambiguity
- coupling issue

**Search Intent:**
when to use data attributes vs classes

**Keywords:**
data attributes contract pattern, css class misuse state, frontend semantic attributes, ui behavior contract attributes

---

## Principle

**Data-attributes MUST represent structural or behavioral contracts, not visual styling shortcuts.**

---

## Problem

Without this rule:

- CSS and architecture couple incorrectly
- Classes abused for state contracts
- Visual variants leak into runtime logic
- Style and behavior semantics blur

Observable symptoms:

- `class="collapsed"`
- `class="variant-solid"`
- Using class to represent feature activation

Architectural impact:

- Hard-coded styling logic
- Contract ambiguity
- Fragile UI-state coupling

---

## Forbidden Pattern

### Forbidden

```rust
class="density-compact"
class="resize-enabled"
```

Why this violates:

Classes are presentational.
They cannot represent canonical state contract.

---

## Canonical Pattern

### Canonical

```rust
data-density="compact"
data-resize-enabled="true"
attr:data-ui-variant="solid"
```

Why this complies:

- Explicit contract
- Behavior-friendly
- CSS-token compatible
- Machine-readable invariant

---

## Rationale

Protects:

- Structural clarity
- Behavior registry compatibility
- Token-based CSS layering
- Declarative architecture

Data-attributes are semantic contracts.
Classes are layout utilities.

These must never be conflated.

---

## Enforcement

- Code review rejects structural contract encoded as class
- Linter rule for `variant-` prefix in class
- CSS contract mapped only via `[data-*]` selectors
- Documentation requires feature attributes defined explicitly

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version