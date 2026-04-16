# Canon Rule #342: Interaction Boundary Must Delegate to Client Module

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 4.0.0
**Date:** 2026-04-16

**Category:** interaction-architecture
**Tags:** interaction, boundary, wasm, client, behavior, dom
**Language:** EN

---

**Intro:**
Components with complex interaction must delegate all behavior to a client-side interaction module. The boundary defines the scope and may act as a bootstrap trigger, but must never implement interaction logic.

**Problem:**
Placing interaction logic inside boundaries creates:
- duplicated state
- SSR/hydration inconsistencies
- tight coupling between UI and behavior
- non-reusable logic

**Solution:**
All interaction logic must live in the client interaction layer.
The boundary exists as a scope delimiter and may trigger initialization, but does not contain behavior.

---

## Principle

Interaction belongs to the **client layer**, not the boundary.

```
Primitive = contract
UI = proxy
Boundary = scope delimiter (and optional bootstrap)
Interaction = behavior engine
DOM = source of truth
```

---

## Architecture

```
Component (SSR HTML)
-> data-rs-* attributes
-> Interaction Boundary (#[island])
-> canon-loader
-> WASM interaction module
-> DOM mutation
-> CSS reaction
```

---

## Decision Rule

An interaction module MUST be used when:

- behavior affects multiple elements
- behavior requires coordination between nodes
- behavior requires memory of previous state
- behavior includes keyboard navigation beyond simple focus
- behavior enforces exclusivity (e.g. tabs, accordion, radio group)
- behavior mutates layout or geometry

Otherwise -> MUST be implemented as **init**.

---

## Interaction Types

- **Selection** - tabs, menu, radio group, command
- **Coordination** - accordion, navigation-menu
- **Input Orchestration** - otp, combobox
- **Layout** - resizable, drag/drop
- **Virtualization** - list, table

---

## State Contract

- DOM (`data-rs-state`) is the single source of truth
- Interaction modules MUST read and write `data-rs-*` attributes
- Interaction modules MUST NOT maintain internal persistent state
- All state transitions MUST be reflected in DOM attributes

---

## Contract

### Interaction Layer

- MUST live in `canonrs-interactions-*`
- MUST expose an initialization entrypoint
- MUST operate via DOM (`data-rs-*`)
- MUST be idempotent (safe for MutationObserver re-init)
- MUST NOT depend on SSR hydration

---

### Boundary

- MUST exist as a boundary component
- MUST use `#[island]` for interaction bootstrap
- MUST NOT implement interaction logic
- MUST NOT contain pointer/drag/keyboard logic
- MUST NOT manage interaction state

---

## Signals (Violation Indicators)

- pointer or drag logic inside boundary
- keyboard navigation logic inside boundary
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

- 4.0.0 - Renamed island to boundary; added Decision Rule, Interaction Types, State Contract (2026-04-16)
- 3.0.0 - Island redefined as mandatory boundary; clarified bootstrap role (2026-04-11)
- 2.0.0 - Interaction defined as WASM-driven system (2026-04-10)
- 1.0.0 - Initial definition (2026-04-07)
