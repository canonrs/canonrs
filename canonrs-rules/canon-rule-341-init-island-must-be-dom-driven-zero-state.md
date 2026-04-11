# Canon Rule #341: Init Island Must Be DOM-Driven and Zero State

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 3.0.0
**Date:** 2026-04-11

**Category:** runtime-architecture
**Tags:** init, island, wasm, dom, state, loader, boundary
**Language:** EN

---

**Intro:**
Init islands define the boundary for lightweight client-side behavior. They must not implement logic. All behavior is delegated to the WASM init layer, which operates directly on the DOM.

**Problem:**
Placing logic, state, or event handling inside init islands creates duplication, breaks determinism, and conflicts with the CanonRS separation between UI and runtime layers.

**Solution:**
Init islands must render SSR HTML and declare behavior via `data-rs-*`. The actual behavior is executed by the `canon-init-loader` and WASM init modules.

---

## Principle

Init islands are **zero-logic boundaries**.

They declare behavior — they do not execute it.

---

## Architecture

Component (SSR HTML)
→ data-rs-* attributes
→ Init Island (boundary only)
→ canon-init-loader
→ WASM init module
→ DOM mutation
→ CSS reaction

---

## Scope

### Init MUST handle (via WASM, not island):

- hover effects
- simple click bindings
- navigation triggers (`href`)
- copy-to-clipboard
- focus handling
- attribute toggling

### Init MUST NOT handle:

- sorting
- filtering
- pagination
- selection engines
- drag interactions
- complex keyboard navigation
- multi-step state logic

---

## Island Contract

- MUST exist as a boundary component
- MUST use `#[component]` (NOT `#[island]`)
- MUST render SSR HTML only
- MUST declare behavior via `data-rs-*`
- MUST NOT use `web_sys`
- MUST NOT register event listeners
- MUST NOT contain closures
- MUST NOT contain logic (`if`, `match`, parsing)
- MUST NOT store or manage state

---

## WASM Init Contract

- MUST use `web_sys` for event handling
- MUST query DOM via selectors
- MUST read/write `data-rs-state`
- MUST be idempotent (safe for MutationObserver)
- MUST NOT depend on SSR hydration
- MUST NOT store persistent state

---

## Signals (Violation Indicators)

- presence of `#[island]`
- usage of `web_sys` inside island
- event listeners inside island
- signals (`signal`, `RwSignal`)
- local state variables (`is_open`, `active`)
- conditional logic unrelated to DOM
- parsing or transformation logic

---

## Relationship with Interaction Layer

- Init = micro-interactions (stateless, DOM-driven)
- Interaction = full behavior engine (stateful, complex)

If behavior requires coordination, persistence, or multi-step logic → it MUST be an interaction module.

---

## Example

### ❌ Forbidden

```rust
#[island]
pub fn TableInit() -> impl IntoView {
    // ❌ logic inside island
}
```

---

### ✅ Canonical

```rust
#[component]
pub fn TableIsland() -> impl IntoView {
    view! {
        <div data-rs-init="table">
            <Table />
        </div>
    }
}
```

---

## Exceptions

None.

---

## Version History

- 3.0.0 - Init island defined as zero-logic boundary; behavior moved to WASM loader (2026-04-11)
- 2.0.0 - Init moved to WASM loader (2026-04-11)
- 1.0.0 - Initial definition (2026-04-07)
