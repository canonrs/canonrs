# Canon Rule #340: Passthrough Island Must Be Zero Logic

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.1.0
**Date:** 2026-04-10

**Category:** island-architecture
**Tags:** island, passthrough, api, typing, ui, contract, rust
**Language:** EN

---

**Intro:**
Passthrough islands exist only as SSR boundaries and must not contain any logic, transformation, or interpretation of props. They must forward typed data directly to UI components without modification.

**Problem:**
When passthrough islands perform transformations such as parsing, enum mapping, fallback resolution, or conditional rendering, they break CanonRS by introducing logic into a layer that must remain purely mechanical.

**Solution:**
Passthrough islands must accept fully typed props and forward them directly to UI components. All transformations must occur before the island or inside the UI layer.

**Signals:**
- `match`, `if`, or branching inside island
- `unwrap_or`, `unwrap_or_default`
- string → enum conversion
- conditional rendering
- parsing or normalization logic
- props typed as `String` when enum exists

**Search Intent:**
passthrough island logic, island should not transform props, enum vs string ui architecture

**Keywords:**
passthrough island, zero logic island, typed props rust ui, canonical island

---

## Principle

Passthrough islands are **mechanical boundaries only**.  
They do not interpret, decide, or transform.

---

## Contract

- MUST use `#[component]`, never `#[island]`
- MUST forward props 1:1 to UI
- MUST NOT contain logic, branching, or parsing
- MUST NOT mutate DOM
- MUST NOT introduce state

---

## Exceptions

None.

---

## Version History

- 1.1.0 - Clarified SSR-only boundary and zero-logic enforcement (2026-04-10)
- 1.0.0 - Initial definition (2026-04-07)
