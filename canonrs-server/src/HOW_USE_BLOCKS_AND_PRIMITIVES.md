# HOW TO USE BLOCKS AND PRIMITIVES — CanonRS (Runtime + SSR Complete)

## The Flow

Primitive → UI → Boundary → Block → Layout → Page

Each layer has one responsibility. No layer knows the layers above it.

---

## Layers

### Primitive (`canonrs_core::primitives`)
Pure HTML + ARIA + `data-rs-*` attributes.

- Zero logic
- Zero style
- Defines semantic contract and DOM shape
- Defines `data-rs-interaction` when behavior exists

Use only when building a new UI component from scratch.

```rust
use canonrs_core::primitives::CardPrimitive;
```

---

### UI (`canonrs::ui`)
1:1 mapping over the primitive.

- Typed props
- No logic
- No state
- No transformation

Use when composing inside a Boundary.

```rust
use canonrs::ui::card::{Card, CardHeader, CardContent, CardFooter};
```

---

### Boundary (`canonrs::ui`)
Public API of the component.

- Applies typed defaults
- Delegates directly to UI
- No signals
- No closures reativas
- No slots

This is what pages and blocks should import for atomic components.

```rust
<Card>"content"</Card>
<CardHeader><CardTitle>"Title"</CardTitle></CardHeader>
```

---

### Block (`canonrs::blocks`)
Semantic composition of Boundaries.

- Slots are always `Option<ChildrenFn>`
- No business logic
- No reactive state
- No DOM mutation

Use when a pattern composes multiple UI regions into a reusable context.

```rust
use canonrs::blocks::card::{CardBlock, CardVariant};

<CardBlock
    variant=CardVariant::Outlined
    header=slot!(|| view! {
        <CardHeader><CardTitle>"Title"</CardTitle></CardHeader>
    }.into_any())
    content=slot!(|| view! {
        <CardContent>"Body"</CardContent>
    }.into_any())
    footer=slot!(|| view! {
        <CardFooter>"Action"</CardFooter>
    }.into_any())
/>
```

---

### Layout Primitive (`canonrs::primitives::layout`)
Structural layout containers.

- No logic
- No state
- Only arrangement

Use inside blocks and pages.

```rust
use canonrs::primitives::layout::grid::{GridPrimitive as Grid, GridCols};

<Grid cols=GridCols::Three>
    <CardBlock ... />
</Grid>
```

---

### Page
Final composition layer.

- Orchestrates data loading
- Owns signals
- Owns async
- Passes static snapshots to components

---

## Runtime & DOM Rules (CR-4xx)

### DOM Ownership

Components with `data-rs-interaction` are DOM-driven.

- State lives in `data-rs-*`
- Runtime mutates DOM directly
- Page MUST NOT control visual state via signals

```text
DOM = single source of truth
```

---

### Interaction Stability

Components with `data-rs-interaction` MUST be stable after first render.

Forbidden:

- Inside components that call `.update()`
- Inside `Resource` / `Suspense`
- Inside reactive closures

Correct:

- Render once
- Runtime controls behavior

---

### Reactivity Rules

Signals MUST NOT control interaction components.

Forbidden:

```rust
move || signal.get()
prop:value=signal
prop:open=signal
```

Allowed:

```rust
let snapshot = signal.get_untracked();
<Component data=snapshot />
```

---

### Async Rules

Interaction components MUST NOT depend on async directly.

Correct:

```rust
spawn_local(fetch);
let data = signal.get_untracked();
<Component data=data />
```

Incorrect:

```rust
<Suspense>
    <Component ... />
</Suspense>
```

---

### UID Contract

`data-rs-uid` must be:

- Stable across SSR and hydration
- Unique per instance

If UID changes:

```text
component breaks
```

---

### DOM Access

Runtime MUST NOT store Element references.

Always resolve at execution time:

```rust
doc.query_selector("[data-rs-uid='...']")
```

---

## Rules Table

| Layer     | Slot type             | Rule                              |
|-----------|----------------------|-----------------------------------|
| Primitive | —                    | HTML + ARIA + data-rs-* only      |
| UI        | `Children`           | 1:1 mapping, no logic             |
| Boundary  | `Children`           | typed defaults, no signals        |
| Block     | `Option<ChildrenFn>` | semantic composition only         |
| Layout    | `Option<ChildrenFn>` | structure only                    |
| Page      | `ChildrenFn`         | owns state and async              |

---

## slot! Macro

Used to define slots ergonomically.

```rust
slot!(|| view! { <Foo /> })
slot!(move || view! { <Foo value=val /> })
```

Implementation:

```rust
#[macro_export]
macro_rules! slot {
    ($closure:expr) => {
        std::sync::Arc::new($closure) as leptos::prelude::ChildrenFn
    };
}
```

---

## Anti-patterns

```rust
// ❌ inline style
<div style="display:flex">

// ❌ variant in Primitive
CardPrimitive { variant: CardVariant }

// ❌ slots in Boundary
Card { header: Option<ChildrenFn> }

// ❌ HTML in Block slots
header=slot!(|| view! { <span>"Title"</span> }.into_any())

// ❌ signal controlling interaction
prop:value=signal
prop:open=signal

// ❌ async inside interaction
<Suspense><Select /></Suspense>
```

---

## Correct Patterns

```rust
// ✅ UI inside slot
header=slot!(|| view! {
    <CardHeader><CardTitle>"Title"</CardTitle></CardHeader>
}.into_any())

// ✅ snapshot before render
let data = signal.get_untracked();
<Component data=data />

// ✅ DOM-driven interaction
runtime controls state via data-rs-*
```

---

## Final Principle

```text
Structure is declarative.
Behavior is DOM-driven.
Runtime owns interaction.
```