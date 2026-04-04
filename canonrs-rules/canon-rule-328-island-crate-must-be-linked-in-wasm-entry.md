# Canon Rule #328: Island Crate Must Be Linked in WASM Entry Point

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-04

**Category:** islands-architecture
**Tags:** islands, wasm, workspace, linking
**Language:** EN

---

**Intro:**
In a Rust workspace, the crate containing islands must be explicitly linked in the WASM entry point crate.

**Problem:**
islands defined in a separate crate are silently excluded from the WASM bundle

**Solution:**
add the island crate as a dependency and use it explicitly in the client crate

**Signals:**
- island function missing from generated JS
- window.__leptos_islands undefined
- island never hydrates

**Search Intent:**
leptos island workspace not working wasm

**Keywords:**
leptos island workspace, island not found wasm, wasm bundle missing island, use crate as workspace

---

## Principle

wasm-bindgen only includes code reachable from the WASM entry point. Islands in separate crates must be explicitly referenced or they are silently excluded from the bundle.

---

## Problem

In a workspace where islands live in canonrs-server but the WASM entry is canonrs-client, the islands are silently excluded from the bundle. The JS output contains zero references to the island functions.

---

## Patterns

### Forbidden Pattern

The WASM entry crate does not declare the island crate as a dependency.

### Canonical Pattern

The island crate is declared as an optional dependency in the WASM entry crate, activated via the hydrate feature. The lib.rs of the WASM entry includes use island_crate as _ with allow(unused_imports).

---

## Contract

### Enforcement

- the crate containing island components must be a dependency of the WASM entry crate
- use island_crate as _ must appear in the WASM entry lib.rs
- islands must be exported from the crate public API

### Exceptions

None.

---

## Version History

- 1.0.0 - Initial definition
