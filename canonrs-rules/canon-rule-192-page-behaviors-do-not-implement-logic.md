# Canon Rule #192: Page Behaviors Must Not Implement New Logic

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Scope:** architecture, behavior, pages
**Version:** 1.0.0  
**Date:** 2026-01-30

---

## Principle

**Page behaviors MUST NOT implement business logic, calculations, or state machines.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Without this rule:

- Page behaviors slowly turn into controllers
- Logic becomes duplicated across pages
- Framework behaviors are bypassed
- The architecture collapses into ad-hoc scripts

Observable failures:

- Conditional logic inside page behaviors
- Geometry, filtering, or sorting logic in page wiring
- One-off logic written “just for this page”

Architectural impact:

- Loss of reuse
- Unversioned behavior
- Hard-to-test product code

---

## Forbidden Pattern

### ❌ Forbidden

```rust
pub fn init_page_behavior() {
    if window_width < 768 {
        reposition_overlay(); // ❌ logic
    }
}
```

```rust
let filtered = rows.iter().filter(...); // ❌ logic
```

Page behaviors must not decide *what* happens — only *what is connected*.

---

## Canonical Pattern

### ✅ Canonical

```rust
pub fn init_page_behavior() {
    DataTableBehavior::new(...).attach();
    TooltipBehavior::new(...).attach();
}
```

All logic lives in **framework behaviors**.

---

## Rationale

This rule preserves:

- Clear ownership of logic
- Framework-level reuse
- Deterministic behavior evolution
- Clean product codebases

Page behaviors are **composition glue**, not execution engines.

---

## Enforcement

- Review checklist: no `if`, `match`, or calculations in page behaviors
- CI: forbid math, filtering, or geometry outside `rs-design`
- Page behaviors must only call `.new()` / `.attach()`

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version
