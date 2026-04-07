# Canon Rule #338: Island Boundary Rule

**Status:** ENFORCED
**Severity:** HIGH
**Version:** 1.0.0
**Date:** 2026-04-07

**Category:** leptos-architecture
**Tags:** island, ssr, leptos, layout, wasm, interaction
**Language:** EN

---

**Intro:**
Islands in CanonRS must never wrap layout structure. An island is a JS initialization boundary, not a layout container. Every component must render its full DOM in SSR via `#[component]`, and delegate JS initialization to a minimal `#[island]`.

**Problem:**
A `#[island]` wrapping layout structure causes Leptos to emit an empty shell in SSR. The DOM never reaches the browser on first paint. Flex chains, height inheritance, and CSS selectors all fail silently.

**Solution:**
Split every interactive component into three layers: SSR layout component, minimal init island, and optional interaction module.

**Signals:**
- component renders empty in SSR curl output
- flex/grid layout collapses despite correct CSS
- `height: 100%` has no effect on island children
- `display: contents` on `leptos-island` is used as a workaround (forbidden)

**Search Intent:**
leptos island empty SSR, island layout broken, flex chain broken leptos

**Keywords:**
leptos island ssr, island boundary, flex layout island, wasm init pattern

---

## Principle

Leptos `#[island]` marks a hydration boundary. The server renders only the shell. Any layout structure inside an island is invisible on first paint.

The correct pattern separates concerns into three explicit layers:

- Layer 1 — SSR Layout: `#[component]` renders full DOM in SSR
- Layer 2 — Init Island: `#[island]` minimal, no children, calls init()
- Layer 3 — Interaction: `interactions/*.rs` pointer/drag/keyboard (optional)

---

## Island Types

### Type 1 — Passthrough Island
Component with no JS. Island is a re-export of the SSR component.

### Type 2 — Init Island
Component with JS initialization. Layout stays in `#[component]`, init moves to minimal `#[island]`.

### Type 3 — Interaction Island
Component with pointer/drag/keyboard complexity. Adds an `interactions/*.rs` module.

---

## Patterns

### Forbidden Pattern
`#[island]` wrapping layout — SSR emits empty shell, DOM never reaches browser.

### Canonical Pattern
Layout in `#[component]` renders full DOM in SSR. Init island is minimal with no children.

---

## Contract

### Enforcement

- `#[island]` must never contain children that are part of layout structure
- `#[island]` must never be the direct parent of flex or grid children
- init island must render `view! { <></> }` — no DOM output
- `display: contents` on `leptos-island` is forbidden as a layout workaround
- every component that requires JS must have an explicit Init island
- init logic must never live inside a layout component

### Exceptions

None.

---

## Version History

- 1.0.0 - Initial definition — derived from resizable layout failure (2026-04-07)
