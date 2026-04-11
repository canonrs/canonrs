# Canon Rule #340: Passthrough Island Must Be Zero Logic

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.2.0
**Date:** 2026-04-11

**Category:** island-architecture
**Tags:** island, passthrough, api, typing, ui, contract, rust
**Language:** EN

---

**Intro:**
Passthrough islands define a strict boundary between SSR and UI. They must not contain any logic, transformation, or interpretation of props. They forward typed data directly to UI components.

**Problem:**
When passthrough islands perform parsing, enum mapping, fallback resolution, or conditional rendering, they introduce logic into a layer that must remain purely mechanical.

**Solution:**
Passthrough islands must accept fully typed props and forward them directly to UI components. All transformations must occur before the island or inside the UI layer.

---

## Principle

Passthrough islands are **mechanical boundaries only**.  
They do not interpret, decide, or transform.

---

## Contract

- MUST exist as a boundary component
- MUST be implemented using `#[component]`
- MUST NOT use `#[island]` macro
- MUST forward props 1:1 to UI
- MUST NOT contain logic (`if`, `match`, parsing)
- MUST NOT mutate DOM
- MUST NOT introduce state

---

## Signals

- `match`, `if`, or branching inside island
- `unwrap_or`, `unwrap_or_default`
- string → enum conversion
- conditional rendering
- parsing or normalization logic
- props typed as `String` when enum exists

---

## Exceptions

None.

---

## Version History

- 1.2.0 - Clarified boundary vs #[island] macro distinction (2026-04-11)
- 1.1.0 - Clarified SSR-only boundary (2026-04-10)
- 1.0.0 - Initial definition (2026-04-07)
