# Canon Rule #281: Cross-Crate Layer Leakage Is Forbidden

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-02-13

**Category:** governance
**Tags:** workspace, layers, modularity, rust
**Language:** EN

---

**Intro:**
Accessing internal layers across crates creates hidden coupling and breaks modular guarantees. Public boundaries must be respected.

**Problem:**
cross crate access to internal layers breaks modular architecture

**Solution:**
restrict access to public apis and enforce visibility boundaries

**Signals:**
- internal module import
- tight coupling
- semver break risk

**Search Intent:**
how to prevent cross crate coupling rust

**Keywords:**
rust crate boundaries architecture, layer isolation workspace, pub(crate) enforcement rust, modular system design rust

---

## Principle

**A crate must never access another crate’s internal layer (primitive/ui/interactive/behavior) bypassing public boundaries.**

---

## Problem

Cross-layer leaks create:

- Hidden tight coupling
- Refactor paralysis
- Broken semver guarantees
- Implicit contracts

---

## Forbidden Pattern

```rust
use canonrs_ui_interactive::internal::resize_logic;
```

Accessing internal logic directly breaks boundaries.

---

## Canonical Pattern

```rust
use canonrs_ui::Button;
use canonrs_ui_interactive::DataTableInteractive;
```

Only public API surfaces are consumed.

---

## Rationale

Ensures:
- Package immutability
- Version isolation
- Layer purity
- Enterprise maintainability

---

## Enforcement

- Visibility restrictions (pub(crate))
- CI dependency graph validation
- Review checklist

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version