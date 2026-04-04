# Canon Rule #329: HydrationScripts Must Declare Islands Mode

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-04

**Category:** islands-architecture
**Tags:** islands, hydration, bootstrap, leptos
**Language:** EN

---

**Intro:**
The HydrationScripts component must receive islands=true to enable island bootstrap in Leptos 0.8.

**Problem:**
without islands=true the runtime does not connect leptos-island elements to WASM functions

**Solution:**
pass islands=true to HydrationScripts in the app shell

**Signals:**
- islands render as static HTML
- no interaction after hydration
- window.__leptos_islands undefined

**Search Intent:**
leptos island not hydrating HydrationScripts

**Keywords:**
HydrationScripts islands, leptos island bootstrap, island not interactive, leptos 0.8 islands setup

---

## Principle

HydrationScripts with islands=true generates the bootstrap code that connects leptos-island DOM elements to their corresponding WASM functions. Without this prop, islands are rendered as static HTML with no interactivity.

---

## Problem

Without islands=true, the Leptos runtime initializes but never scans the DOM for leptos-island elements. The WASM functions are exported but never called. Islands appear to render correctly but have zero interactivity.

---

## Patterns

### Forbidden Pattern

HydrationScripts without the islands prop in an app that uses island components.

### Canonical Pattern

HydrationScripts with islands=true in the head of the app shell.

---

## Contract

### Enforcement

- HydrationScripts must include islands=true when any island component exists in the app
- leptos::mount::hydrate_islands() must be called in the hydrate() wasm entry point
- leptos must be declared with the islands feature in Cargo.toml

### Exceptions

None.

---

## Version History

- 1.0.0 - Initial definition
