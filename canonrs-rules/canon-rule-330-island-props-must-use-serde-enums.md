# Canon Rule #330: Island Props Must Use Serde Enums

**Status:** ENFORCED
**Severity:** CRITICAL
**Version:** 1.0.0
**Date:** 2026-04-04

**Category:** islands-architecture
**Tags:** islands, props, serde, serialization
**Language:** EN

---

**Intro:**
Props of Leptos islands that need to be serialized across the SSR/hydration boundary must use enums with serde derives. Primitive optional types silently fail serialization.

**Problem:**
`Option<String>` and `Option<bool>` with `#[prop(optional)]` always serialize as `null` in `data-props`, making the value invisible to the client after hydration.

**Solution:**
Define an enum with `#[derive(serde::Serialize, serde::Deserialize)]` for every prop that carries meaningful state across the SSR boundary.

**Signals:**
- prop always appears as `null` in `data-props` HTML attribute
- client island never receives the intended value
- state appears correct in SSR but resets after hydration

**Search Intent:**
leptos island prop null after hydration, option string not serialized island

**Keywords:**
leptos island props serde, island serialization, data-props null, option bool island

---

## Principle

The Leptos island macro serializes component props into the `data-props` HTML attribute using serde. Only types that implement `Serialize + Deserialize` are correctly encoded. `Option<String>` with `into` and `Option<bool>` with `optional` collapse to `null` silently.

---

## Problem

An island prop declared as `Option<String>` or `Option<bool>` with `#[prop(optional)]` will always appear as `null` in the rendered HTML, even when a value is explicitly passed at the call site.

---

## Patterns

### Forbidden Pattern
```rust
#[island]
pub fn ButtonIsland(
    #[prop(optional, into)] state_hint: Option<String>,
    #[prop(optional)] flag: Option<bool>,
) -> impl IntoView { ... }
```

### Canonical Pattern
```rust
#[derive(Clone, Copy, Debug, PartialEq, Default, serde::Serialize, serde::Deserialize)]
pub enum ButtonStateHint {
    #[default] None,
    First,
    Last,
    Hover,
    Focus,
}

#[island]
pub fn ButtonIsland(
    #[prop(optional)] state_hint: Option<ButtonStateHint>,
) -> impl IntoView { ... }
```

---

## Contract

### Enforcement

- every island prop that carries state across SSR must use a serde enum
- `Option<String>` and `Option<bool>` are forbidden as island props
- enums must derive `Serialize`, `Deserialize`, `Default`, and `PartialEq`

### Exceptions

- `Option<String>` is acceptable for purely cosmetic props (class, aria_label) that do not affect reactive state

---

## Version History

- 1.0.0 - Initial definition
