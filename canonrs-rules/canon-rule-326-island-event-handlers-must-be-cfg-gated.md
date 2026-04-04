# Canon Rule #326: Island Event Handlers Must Be Cfg-Gated

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-04

**Category:** islands-architecture
**Tags:** islands, cfg, event-handlers, leptos
**Language:** EN

---

**Intro:**
Event handler logic inside islands must be conditionally compiled with cfg feature flags.

**Problem:**
web-sys and wasm-bindgen APIs are unavailable during SSR compilation

**Solution:**
gate all client-only logic with cfg(feature = hydrate)

**Signals:**
- web_sys not found in SSR build
- wasm_bindgen unavailable
- JsCast not in scope

**Search Intent:**
leptos island event handler SSR compile error

**Keywords:**
leptos island cfg hydrate, web_sys SSR error, island event handler cfg, wasm_bindgen SSR

---

## Principle

Islands compile on both SSR and WASM targets. Client-only APIs must be cfg-gated to prevent SSR compilation failures.

---

## Problem

web_sys, wasm_bindgen and JsCast are only available in the WASM target. Using them without cfg gates causes SSR build failures.

---

## Patterns

### Forbidden Pattern

Using web_sys or JsCast inside an event handler without a cfg(feature = hydrate) gate.

### Canonical Pattern

Two handler definitions: one with cfg(feature = hydrate) containing the real logic, one with cfg(not(feature = hydrate)) as a no-op. Both use the same variable name so the single view! block compiles on both sides.

---

## Contract

### Enforcement

- all web_sys, wasm_bindgen, JsCast usage must be inside cfg(feature = hydrate)
- SSR no-op handlers must consume captured variables to avoid unused warnings
- the view! block must use the same handler name on both sides

### Exceptions

None.

---

## Version History

- 1.0.0 - Initial definition
