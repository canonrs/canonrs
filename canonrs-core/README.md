# canonrs-core

Foundation layer of the CanonRS design system.

No heavy dependencies. Compiles on all targets: SSR, WASM, and native.

## Responsibility

Provides the shared types, design contracts, providers, and primitive type definitions used by all other CanonRS crates. This crate never pulls in `syntect`, `pulldown-cmark`, `axum`, or any browser API.

## Modules

### `primitives/`
Type definitions and prop contracts for all 80+ UI components. These are the canonical shapes — not the implementations. Each file defines the types, enums, and props that the actual component in `canonrs-server` implements.

### `utils/`
Utility functions shared across the system. Includes `id_gen` for deterministic component ID generation.

### `shared/`
Cross-cutting domain types used by multiple components:
- `behavior_core` — base behavior contracts
- `behavior_error` / `behavior_telemetry` — error and observability types
- `navigation_context` — shared navigation state
- `orientation`, `status_variant`, `drawer_variant` — common enums
- `toc_types` — table of contents data structures

### `design/`
Design system contracts, governance rules, token definitions, and legacy migration guides. This is the source of truth for the token architecture.

### `theme/`
Theme provider, theme types, and `ThemeMode` enum. Used by both SSR and client targets.

### `density/`
Density context provider and types. Controls UI density (compact, default, comfortable).

### `language/`
Language and i18n context provider.

### `hydration/`
Hydration state management.

### `root/`
`CanonRSRoot` — the top-level provider component that wraps all other providers. Must be placed at the root of every CanonRS app.

### `prelude.rs`
Convenience re-exports. Import with `use canonrs_core::prelude::*` or `use canonrs::providers::prelude::*`.

## Features

| Feature | Effect |
|---------|--------|
| `ssr` | Enables SSR-specific provider behavior |
| `hydrate` | Enables hydration mode |
