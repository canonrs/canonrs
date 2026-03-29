# Canon Rule #281: Cross-Crate Layer Leakage Is Forbidden

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** workspace, architecture
**Version:** 1.0.0  
**Date:** 2026-02-13

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
