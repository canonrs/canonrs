use leptos::prelude::*;
use super::markdown_ui::{MarkdownSurface, TocPosition};
use super::renderer::render_markdown_with_prefix;

fn enterprise_markdown() -> &'static str {
    r#"
# CanonRS Framework

CanonRS is an enterprise-grade UI framework built with **Rust** and **Leptos**. SSR-first, behavior-driven, and 100% token-based.

## Getting Started

Add CanonRS to your project and configure the build pipeline.

### Installation

Add the dependency to your `Cargo.toml`:
```toml
[dependencies]
canonrs = { path = "../canonrs" }
```

### Configuration

Configure your workspace profiles for optimized WASM builds:
```toml
[profile.wasm-release]
inherits = "release"
opt-level = "z"
lto = "fat"
codegen-units = 1
panic = "abort"
```

#### Environment Variables

Set up your development environment:
```bash
export LEPTOS_ENV=DEV
export LEPTOS_SITE_ADDR=0.0.0.0:3000
```

## Architecture

CanonRS follows a strict separation of concerns across 4 layers.

### Core Concepts

Every component is built from the same 4 primitives.

#### Primitives

Pure HTML with `data-*` attributes. Zero logic, zero state.
```rust
#[component]
pub fn ButtonPrimitive(
    children: Children,
) -> impl IntoView {
    view! {
        <button data-button="" type="button">
            {children()}
        </button>
    }
}
```

#### Behaviors

Client-side JS that attaches after hydration via `IntersectionObserver` and DOM events.

#### Design Tokens

CSS custom properties generated from Rust token definitions at compile time.

#### SSR Pipeline
```
Request
  → Server renders HTML
  → Tokens applied via CSS
  → Behaviors hydrate
  → Zero layout shift
```

## API Reference

Complete component documentation with examples.

### Components

All components follow the Canon pattern: primitive → UI → behavior.

#### DataTable

Enterprise data grid with sort, filter, pagination, and density controls.

#### CodeBlock

SSR syntax highlighting via syntect. Copy button via behavior.

#### Markdown

This very component. SSR rendering with TOC extraction and scroll-spy.

## Contributing

We welcome contributions. Please read the architecture guide first.
"#
}

pub fn basic_example() -> impl IntoView {
    let rendered = render_markdown_with_prefix(enterprise_markdown(), "md-top");
    view! {
        <MarkdownSurface
            rendered=rendered
            show_toc=true
            show_toolbar=true
            toc_position=TocPosition::Top
            id="markdown-top-example"
        />
    }
}

pub fn sidebar_example() -> impl IntoView {
    let rendered = render_markdown_with_prefix(enterprise_markdown(), "md-sidebar");
    view! {
        <MarkdownSurface
            rendered=rendered
            show_toc=true
            show_toolbar=false
            toc_position=TocPosition::Sidebar
            id="markdown-sidebar-example"
        />
    }
}
