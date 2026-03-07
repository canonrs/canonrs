# CanonRS Architecture

## Overview

CanonRS is a Cargo workspace with 4 runtime crates and 1 build tooling crate, forming an enterprise-grade UI component library for Leptos.
```
canonrs          ← public facade (only this in your Cargo.toml)
├── canonrs-core     ← types, providers, primitives
├── canonrs-server   ← all 80+ UI components, layouts, blocks
└── canonrs-client   ← WASM behaviors and interactive components
```

---

## Crate Responsibilities

### `canonrs` — Facade
Single public entry point. Re-exports everything under a unified namespace. Developers never reference internal crates directly.
```toml
canonrs = { version = "0.1", features = ["ssr"] }
# or
canonrs = { version = "0.1", features = ["hydrate"] }
```

### `canonrs-core` — Foundation
Pure Rust. No heavy dependencies. Compiles on all targets.
Contains: type contracts for all components, theme/density/language providers, `CanonRSRoot`, design tokens, shared enums and utilities.

### `canonrs-server` — Component Library
All 80+ Leptos components. Compiles on SSR and WASM. Heavy dependencies (`syntect`, `pulldown-cmark`) are optional and only activated with `ssr`.

The markdown pipeline is the only SSR-heavy subsystem. Its public API is stable across targets — `render_markdown()` always exists, returning `RenderedMarkdown::default()` on WASM.

### `canonrs-client` — Browser Layer
Only compiled in `hydrate` builds. Contains browser behaviors (scroll, drag, sidebar, etc.), interactive component implementations, and the client-side theme engine. Behaviors require `wasm32` target.

### `canonrs-tokens` — Build Tooling
Not a runtime dependency. Generates `canonrs.bundle.css` from design token definitions. Called by `canonrs/build.rs` at compile time.

---

## Feature Flags

| Feature | Target | Activates |
|---------|--------|-----------|
| `ssr` | x86_64 | Full server stack: axum, tokio, syntect, pulldown-cmark, markdown renderer |
| `hydrate` | wasm32 | Leptos hydration, canonrs-client, browser behaviors |

`ssr` and `hydrate` are mutually exclusive.

---

## Build Targets
```bash
# SSR
cargo check --features ssr

# Hydrate / WASM
cargo check --features hydrate
```

---

## Critical Rules

**1. API never changes across targets.**
Same function signatures in SSR and hydrate. Implementation differs, API does not.

**2. Heavy deps are always optional.**
`syntect`, `pulldown-cmark` only enter the build graph when `ssr` is active.

**3. `canonrs-server` compiles in hydrate.**
UI components are Leptos — they run in both targets. Only the rendering pipeline is SSR-only.

**4. No target_arch in facade.**
`#[cfg(target_arch = "wasm32")]` is only used in `canonrs-client` internals. Everywhere else, features control compilation.

---

## Dependency Graph
```
canonrs
├── canonrs-core          (always)
├── canonrs-server        (ssr + hydrate)
└── canonrs-client        (hydrate only)
    └── canonrs-core
```

No circular dependencies. `canonrs-core` has no internal CanonRS dependencies.

---

## Validation
```bash
cargo check --features ssr
cargo check --features hydrate
cargo check -p canonrs-server --features hydrate
```
