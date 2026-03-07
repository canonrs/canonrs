# SSR-Only Dependencies: pulldown-cmark and syntect

## 1. What are they and where do they live?

Both dependencies are 100% SSR-only. They run exclusively on the server during render. The browser receives pre-rendered HTML and never needs them.

**`pulldown-cmark`** — Markdown parser. Converts raw Markdown text into a structured HTML tree.

**`syntect`** — Syntax highlighter. Applies language-aware HTML highlighting to code blocks inside Markdown.

**Location**: `canonrs-server/Cargo.toml`
```toml
[dependencies]
pulldown-cmark = { version = "0.12.2", optional = true }
syntect = { version = "5.2", default-features = false, features = [
    "default-syntaxes", "default-themes", "html", "regex-fancy"
], optional = true }
html-escape = { version = "0.2", optional = true }

[features]
ssr = ["syntect", "pulldown-cmark", "html-escape", ...]
hydrate = [...]  # neither syntect nor pulldown-cmark appear here
```

Both are `optional = true`. They only enter the build graph when `ssr` is active. A hydrate/WASM build never sees them.

---

## 2. Which components use them and how?

Only two components depend on this pipeline:

**`Markdown`** (`canonrs-server/src/ui/markdown/`) — Full markdown renderer with table of contents, toolbar, and syntax-highlighted code blocks.

**`CodeBlock`** (`canonrs-server/src/ui/code_block/`) — Standalone syntax-highlighted code block.

The pipeline lives in two SSR-only modules:
```
canonrs-server/src/ui/markdown/
├── renderer.rs        ← pulldown-cmark + syntect (cfg ssr)
└── toc_extractor.rs   ← TOC extraction from parsed HTML (cfg ssr)
```

Usage inside the component:
```rust
// SSR: real rendering
#[cfg(feature = "ssr")]
pub fn render_markdown(md: &str) -> RenderedMarkdown {
    renderer::render_markdown(md)
}

// Hydrate: stub — HTML already arrived from server
#[cfg(not(feature = "ssr"))]
pub fn render_markdown(_: &str) -> RenderedMarkdown {
    RenderedMarkdown::default()
}
```

---

## 3. How is a component that uses these features exported?

The public API is **identical across all targets**. The function signature never changes.
```rust
// Always available — same signature in SSR and hydrate
pub fn render_markdown(md: &str) -> RenderedMarkdown
pub fn render_with_prefix(md: &str, prefix: &str) -> RenderedMarkdown
```

The facade re-exports without any cfg:
```rust
// canonrs/src/lib.rs
pub mod ui {
    pub use canonrs_server::ui::*;
}
```

An app imports normally:
```rust
use canonrs::ui::markdown::render_with_prefix;
use canonrs::ui::markdown::MarkdownSurface;
```

No `#[cfg]` needed in app code. The feature controls the implementation. The API never moves.

**Rule**: Never expose SSR-only symbols directly. Always wrap them behind a stable public function with a hydrate stub.
