# CanonRS — Enterprise UI Library for Rust

CanonRS is a correctness-first, enterprise-grade UI component library built on top of [Leptos](https://leptos.dev). It provides a complete design system with SSR, hydration, and WASM support.

---

## Crate Architecture
```
canonrs-core
canonrs-client
canonrs-server
canonrs          ← public facade (use this)
canonrs-tokens   ← build tooling only
```

---

## canonrs

**The only crate you need as a developer.**
```toml
# SSR app
canonrs = { version = "0.1", features = ["ssr"] }

# Hydrate / WASM app
canonrs = { version = "0.1", features = ["hydrate"] }
```

Re-exports everything from `canonrs-core`, `canonrs-server`, and `canonrs-client` under a single unified namespace.

| Module | Description |
|--------|-------------|
| `canonrs::ui::*` | All UI components (Button, Card, Table, Markdown, etc.) |
| `canonrs::layouts::*` | Page layouts (Section, PageHeader, etc.) |
| `canonrs::blocks::*` | Composite blocks (Card, Form, Alert, etc.) |
| `canonrs::providers::*` | Theme, density, language providers |
| `canonrs::primitives::*` | Low-level UI primitives |
| `canonrs::ui_interactive::*` | WASM-only interactive components (hydrate feature) |
| `canonrs::behaviors::*` | WASM-only behaviors (hydrate + wasm32 target) |

---

## canonrs-core

**Foundation layer. No heavy dependencies.**

Contains everything that compiles on both SSR and WASM targets without pulling in `syntect`, `pulldown-cmark`, or any server-only crate.

| Module | Origin | Description |
|--------|--------|-------------|
| `shared/` | canonrs-shared | Shared types and utilities |
| `design/` | canonrs-shared | Design system tokens and types |
| `primitives/` | canonrs-ui-primitives | Low-level UI primitives |
| `utils/` | canonrs-ui-primitives | Utility functions |
| `theme/` | canonrs-providers | Theme context and provider |
| `density/` | canonrs-providers | Density context |
| `language/` | canonrs-providers | Language/i18n context |
| `hydration/` | canonrs-providers | Hydration state |
| `root/` | canonrs-providers | Root component provider |
| `prelude.rs` | canonrs-providers | Re-exports for convenience |

---

## canonrs-server

**SSR and UI layer. The bulk of the component library.**

Contains all 80+ Leptos components. Compiles on both SSR and WASM targets. Heavy dependencies (`syntect`, `pulldown-cmark`) are optional and only activated with the `ssr` feature.

| Module | Description |
|--------|-------------|
| `ui/` | All UI components (Button, Input, Table, Markdown, CodeBlock, etc.) |
| `layouts/` | Page layout components (Section, PageHeader, PageLayout, etc.) |
| `blocks/` | Composite block components (Card, Form, Alert, Field, etc.) |
| `providers/` | SSR-side providers |
| `pages/` | Full page components |

### Features

| Feature | Activates |
|---------|-----------|
| `ssr` | `syntect`, `pulldown-cmark`, `axum`, `tokio`, markdown renderer, TOC extractor |
| `hydrate` | Leptos hydration mode |

### SSR-only internals

The markdown pipeline (`renderer.rs`, `toc_extractor.rs`) and code highlighting (`syntect`) are strictly SSR-only. Public API remains stable across targets — `render_markdown()` and `render_with_prefix()` always exist, returning `RenderedMarkdown::default()` on WASM.

---

## canonrs-client

**WASM / hydration layer.**

Contains interactive components and behaviors that only make sense in the browser. Only compiled when targeting `wasm32` or using the `hydrate` feature.

| Module | Description |
|--------|-------------|
| `ui/` | Interactive UI components (from canonrs-ui-interactive) |
| `behaviors/` | Browser behaviors: scroll, drag, focus, resize, etc. (wasm32 only) |
| `themes/` | Client-side theme switching logic |

---

## canonrs-tokens

**Build tooling. Not a runtime dependency.**

Generates the CSS design token system. Contains the `tokens-engine` binary used by `canonrs`'s `build.rs` to embed the final CSS bundle.
```bash
cargo run --bin tokens-engine
```

---

## Golden Rule

> A developer using CanonRS should never need to know about `syntect`, `pulldown-cmark`, `wasm-bindgen`, or any internal crate.
>
> They add one dependency. They pick one feature. Everything works.

---

## Feature Matrix

| Feature | canonrs-core | canonrs-server | canonrs-client |
|---------|-------------|----------------|----------------|
| `ssr` | ✅ | ✅ (full) | ❌ |
| `hydrate` | ✅ | ✅ (no renderer) | ✅ |
| default | ✅ | ❌ | ❌ |
