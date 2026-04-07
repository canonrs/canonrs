# Canon Rule #340: Passthrough Island Must Be Zero Logic

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-07

**Category:** island-architecture
**Tags:** island, passthrough, api, typing, ui, contract, rust
**Language:** EN

---

**Intro:**
Passthrough islands exist only as SSR/hydration boundaries and must not contain any logic, transformation, or interpretation of props. They must forward typed data directly to UI components without modification.

**Problem:**
When passthrough islands perform transformations such as string parsing, enum mapping, default resolution, or conditional rendering, they break the CanonRS architecture by introducing logic into a layer that must remain purely mechanical.

**Solution:**
Passthrough islands must accept fully typed props (enums, booleans, structured data) and forward them directly to UI components. All transformations must occur before the island (call site) or inside the UI layer.

**Signals:**
- usage of `match` inside island
- usage of `unwrap_or`, `unwrap_or_default`, or fallback logic
- string → enum conversion inside island
- conditional rendering branches (`if`, `match`) in island
- presence of parsing or normalization logic
- island API accepts `String` where enum exists

**Search Intent:**
passthrough island logic, island should not transform props, enum vs string in UI architecture, rust island pattern clean architecture

**Keywords:**
passthrough island, zero logic island, typed props rust ui, enum api design, canonical island pattern

---

## Principle

Passthrough islands are **mechanical boundaries only**. They do not interpret, decide, or transform. They only forward already-valid, typed data to UI components.

---

## Patterns

### Forbidden Pattern
```rust
#[component]
pub fn BadgeIsland(
    #[prop(optional, into)] variant: Option<String>,
) -> impl IntoView {
    let variant_val = match variant.as_deref() {
        Some("primary") => BadgeVariant::Primary,
        _ => BadgeVariant::Default,
    };

    view! {
        <Badge variant=variant_val />
    }
}
```

### Canonical Pattern
```rust
#[component]
pub fn BadgeIsland(
    #[prop(default = BadgeVariant::Default)] variant: BadgeVariant,
) -> impl IntoView {
    view! {
        <Badge variant=variant />
    }
}
```

---

## Contract

### Enforcement

- Passthrough islands MUST use `#[component]`, never `#[island]`
- Props MUST be fully typed (enums, bool, structured types)
- NO `match`, `if`, or branching logic allowed
- NO `unwrap_or`, `unwrap_or_default`, or fallback logic
- NO string parsing or conversion inside island
- NO DOM construction beyond forwarding to UI
- MUST be 1:1 proxy to UI component

### Exceptions

None.

---

## Version History

- 1.0.0 - Initial definition — establishes zero-logic contract for passthrough islands (2026-04-07)
