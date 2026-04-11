# Canon Rule #342: Interaction Island Must Delegate to Client Module

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 3.0.0
**Date:** 2026-04-11

**Category:** interaction-architecture
**Tags:** interaction, island, wasm, client, behavior, dom
**Language:** EN

---

**Intro:**
Components with complex interaction must delegate all behavior to a client-side interaction module. The island defines the boundary and may act as a bootstrap trigger, but must never implement interaction logic.

**Problem:**
Placing interaction logic inside islands creates:
- duplicated state
- SSR/hydration inconsistencies
- tight coupling between UI and behavior
- non-reusable logic

**Solution:**
All interaction logic must live in the client interaction layer.  
The island exists as a boundary and may trigger initialization, but does not contain behavior.

---

## Principle

Interaction belongs to the **client layer**, not the island.

```
Primitive = contract
UI = proxy
Island = boundary (and optional bootstrap)
Interaction = behavior engine
DOM = source of truth
```

---

## Architecture

Component (SSR HTML)
→ data-rs-* attributes
→ Interaction Island (boundary)
→ canon-loader
→ WASM interaction module
→ DOM mutation
→ CSS reaction

---

## Contract

### Interaction Layer

- MUST live in `canonrs-interactions-*`
- MUST expose `init_all()` or equivalent
- MUST operate via DOM (`data-rs-*`)
- MUST NOT depend on SSR hydration

---

### Island

- MUST exist as a boundary component
- MAY use `#[island]` for bootstrap (optional)
- MUST NOT implement interaction logic
- MUST NOT contain pointer/drag/keyboard logic
- MUST NOT manage interaction state

---

## Signals (Violation Indicators)

- pointer or drag logic inside island
- keyboard navigation logic inside island
- layout mutation (`getBoundingClientRect`, inline styles)
- signals controlling interaction state
- loops managing interaction behavior

---

## Applies to

- datatable
- resizable
- slider
- drag/drop
- virtual list
- any complex interaction system

---

## Exceptions

None.

---

## Version History

- 3.0.0 - Island redefined as mandatory boundary; clarified bootstrap role (2026-04-11)
- 2.0.0 - Interaction defined as WASM-driven system (2026-04-10)
- 1.0.0 - Initial definition (2026-04-07)
