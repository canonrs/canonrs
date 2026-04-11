# Canon Rule #341: Init Island Must Be DOM-Driven and Zero State

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.1.0
**Date:** 2026-04-10

**Category:** island-architecture
**Tags:** island, init, dom, state, events, web_sys, hydration
**Language:** EN

---

**Intro:**
Init islands are lightweight runtime bridges that bind events and mutate DOM state. They must be fully DOM-driven and must not own state.

**Problem:**
Using signals, internal variables, or business logic inside init islands creates duplicated state, hydration mismatch, and non-deterministic behavior.

**Solution:**
Init islands must read state from `data-rs-*`, react to browser events via `web_sys`, and write state back to the DOM.

**Signals:**
- usage of signals (`signal`, `RwSignal`)
- local state variables (`is_open`, `active`)
- business logic or branching unrelated to DOM
- prop transformation or parsing
- `on:*` event bindings instead of `web_sys`

**Search Intent:**
init island dom driven state, remove signals from island, wasm dom mutation pattern

**Keywords:**
init island, dom driven state, data-rs-state, web_sys events, zero state

---

## Principle

Init islands are **stateless DOM mutation layers**.  
State lives only in the DOM.

---

## Contract

- MUST use `#[island]`
- MUST use `web_sys` events
- MUST read/write `data-rs-state`
- MUST NOT use signals
- MUST NOT store state in Rust
- MUST NOT contain business logic
- MUST NOT control layout/rendering

---

## Exceptions

None.

---

## Version History

- 1.1.0 - Clarified role as DOM-only mutation layer (2026-04-10)
- 1.0.0 - Initial definition (2026-04-07)
