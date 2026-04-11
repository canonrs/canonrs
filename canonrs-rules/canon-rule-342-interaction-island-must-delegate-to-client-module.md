# Canon Rule #342: Interaction Must Be Implemented in Client Module (Island Optional)

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 2.0.0
**Date:** 2026-04-10

**Category:** interaction-architecture
**Tags:** interaction, wasm, client, behavior, dom
**Language:** EN

---

**Intro:**
Components with complex interaction must delegate all behavior to a client-side interaction module. The island is optional and must never implement interaction logic.

**Problem:**
Placing interaction logic inside islands creates:
- duplicated state
- SSR/hydration inconsistencies
- tight coupling between UI and behavior
- non-reusable logic

**Solution:**
All interaction logic must live in the client interaction layer.  
The island (if present) acts only as a bootstrap trigger.

**Signals:**
- pointer/drag logic inside island
- keyboard navigation logic inside island
- layout mutation (`getBoundingClientRect`, inline styles)
- state managed via signals in island
- loops controlling interaction state

**Search Intent:**
interaction wasm architecture, move logic to client module, island should not handle interaction

**Keywords:**
interaction engine, wasm interaction, client module, canonical interaction pattern

---

## Principle

Interaction belongs to the **client layer**, not the island.

```
Primitive = contract
UI = proxy
Island = optional bootstrap
Interaction = behavior engine
DOM = source of truth
```

---

## Contract

- Interaction logic MUST:
  - live in `canonrs-interactions-*`
  - be initialized via `init_all`
  - operate via DOM (`data-rs-*`)

- Island:
  - is OPTIONAL
  - MUST NOT implement interaction logic
  - MAY call client init

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

- 2.0.0 - Removed requirement for island, defined interaction as WASM-driven system (2026-04-10)
- 1.0.0 - Initial definition (2026-04-07)
