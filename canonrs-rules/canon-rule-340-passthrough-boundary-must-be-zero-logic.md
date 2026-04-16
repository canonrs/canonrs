# Canon Rule #340: Passthrough Boundary Must Be Zero Logic

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 2.0.0
**Date:** 2026-04-16

**Category:** boundary-architecture
**Tags:** boundary, passthrough, api, typing, ui, contract, rust
**Language:** EN

---

**Intro:**
Passthrough boundaries define a strict boundary between SSR and UI. They must not contain any logic, transformation, or interpretation of props. They forward typed data directly to UI components.

**Problem:**
When passthrough boundaries perform parsing, enum mapping, fallback resolution, or conditional rendering, they introduce logic into a layer that must remain purely mechanical.

**Solution:**
Passthrough boundaries must accept fully typed props and forward them directly to UI components. All transformations must occur before the boundary or inside the UI layer.

---

## Principle

Passthrough boundaries are **mechanical boundaries only**.
They do not interpret, decide, or transform.

---

## Architecture

```
Component (SSR HTML)
-> typed props
-> Passthrough Boundary (#[component])
-> UI component (rendering only)
-> Primitive (HTML contract)
```

---

## Boundary Types

CanonRS defines three boundary types:

- **Passthrough** (#[component]) — forwards props 1:1 to UI, zero logic
- **Init** (#[component]) — declares behavior via data-rs-*, delegates to WASM init
- **Interaction** (#[island]) — bootstraps interaction engine, delegates to WASM interaction

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

## Signals (Violation Indicators)

- `match`, `if`, or branching inside boundary
- `unwrap_or`, `unwrap_or_default`
- string to enum conversion
- conditional rendering
- parsing or normalization logic
- props typed as `String` when enum exists

---

## Example

### Forbidden

```rust
#[component]
pub fn RadioBoundary(selected: String) -> impl IntoView {
    let state = if selected == "true" { SelectionState::Selected } else { SelectionState::Unselected };
    view! { <RadioUi selected=state /> }
}
```

### Canonical

```rust
#[component]
pub fn RadioBoundary(
    #[prop(default = SelectionState::Unselected)] selected: SelectionState,
) -> impl IntoView {
    view! { <RadioUi selected=selected /> }
}
```

---

## Exceptions

None.

---

## Version History

- 2.0.0 - Renamed island to boundary; added Architecture, Boundary Types, examples (2026-04-16)
- 1.2.0 - Clarified boundary vs #[island] macro distinction (2026-04-11)
- 1.1.0 - Clarified SSR-only boundary (2026-04-10)
- 1.0.0 - Initial definition (2026-04-07)
