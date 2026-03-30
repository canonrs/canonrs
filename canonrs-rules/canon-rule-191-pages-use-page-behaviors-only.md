# Canon Rule #191: Pages Must Delegate Wiring to Page Behaviors Only

**Status:** ENFORCED  
**Severity:** CRITICAL  
**Version:** 1.0.0  
**Date:** 2026-01-30

**Category:** behavior
**Tags:** pages, behavior, architecture, composition
**Language:** EN

---

**Intro:**
Embedding logic or wiring directly in pages leads to uncontrolled growth and architectural drift. Pages must remain declarative entry points.

**Problem:**
pages contain logic and wiring causing coupling and growth

**Solution:**
delegate all wiring to page behaviors and keep pages logic free

**Signals:**
- logic in page
- event listener
- coupling

**Search Intent:**
how to separate page logic from behavior layer

**Keywords:**
page behavior architecture, frontend page composition pattern, ui wiring separation, behavior layer design

---

## Principle

**Pages MUST delegate all client-side wiring to a corresponding page-level behavior and MUST NOT contain logic themselves.**

- Objective
- Verifiable
- One clear boundary

---

## Problem

Without this rule:

- Pages start accumulating logic
- Page files grow uncontrollably
- Behavior usage becomes inconsistent
- The distinction between framework behavior and product wiring blurs

Observable failures:

- Event listeners created inside pages
- Pages importing framework primitives or behaviors directly
- Logic duplicated across multiple pages

Architectural impact:

- Loss of clear composition boundaries
- Tight coupling between pages and implementation details
- Reduced portability across products

---

## Canonical Architecture

```
rs-design/
├── primitives/   → infrastructure (calculation, low-level)
├── behaviors/    → reusable framework behavior
└── ui/           → pure rendering (HTML + CSS hooks)

product-site/
├── pages/        → composition + markup (ZERO logic)
└── behaviors/    → page wiring only (ZERO logic)
```

Each page has a **1:1 page behavior**.

---

## Forbidden Pattern

### Forbidden

```rust
// page.rs
use rs_design::behaviors::datatable::DataTableBehavior;

pub fn Page() {
    DataTableBehavior::new(...).init(); // ❌ page calling framework behavior
}
```

```rust
// page.rs
use web_sys::EventTarget;

let el = document.get_element_by_id("btn").unwrap();
el.add_event_listener_with_callback(...); // ❌ logic inside page
```

Pages must never own wiring or logic.

---

## Canonical Pattern

### Canonical

```rust
// pages/page_a.rs
use crate::behaviors::page_a_behavior::init_page_a_behavior;

pub fn PageA() {
    init_page_a_behavior();
}
```

```rust
// behaviors/page_a_behavior.rs
use rs_design::behaviors::*;

pub fn init_page_a_behavior() {
    DataTableBehavior::new(...).attach();
    TooltipBehavior::new(...).attach();
}
```

- Pages only delegate
- Page behaviors only wire
- Framework behaviors implement logic

---

## Rationale

This rule enforces a strict separation of concerns:

- Pages define structure and composition
- Page behaviors define wiring
- Framework behaviors define behavior
- Primitives define infrastructure

This guarantees:

- Predictable page size
- Zero logic leakage into pages
- Clean product-level customization
- Reusable and versioned framework behavior

Pages become declarative entry points, not execution units.

---

## Enforcement

- CI: forbid DOM APIs inside `/pages`
- Review: page files must only call page behavior
- Naming rule: `page_name.rs` ↔ `page_name_behavior.rs`
- Pages must not import `rs-design::primitives` or framework behaviors directly

---

## Exceptions

> **No exceptions. This rule is absolute.**

---

## Version History

- **1.0.0** — Initial version