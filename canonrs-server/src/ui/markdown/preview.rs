use leptos::prelude::*;
use super::markdown_boundary::{MarkdownSurface, TocPosition};
use super::render_markdown;

#[component]
pub fn MarkdownShowcasePreview() -> impl IntoView {
    let sample = concat!(
        "# CanonRS Design System\n\n",
        "CanonRS is a design system built with **Leptos** and **Rust**. \n",
        "Every component follows a strict contract between primitives, tokens, and interaction modules.\n\n",
        "## Architecture\n\n",
        "The system is built on three layers: primitives define the contract, \n",
        "UI components compose them, and interaction modules handle behavior exclusively on the client.\n\n",
        "### Primitives\n\n",
        "Primitives are the foundation. They render SSR-safe HTML with `data-rs-*` attributes \n",
        "that declare behavior without executing it. No logic lives inside a primitive.\n\n",
        "### Tokens\n\n",
        "All visual decisions are encoded as CSS custom properties. \n",
        "Tokens are organized by family — layout, color, motion, typography — \n",
        "and are the single source of truth for theming.\n\n",
        "## Features\n\n",
        "- SSR-safe rendering with deterministic DOM output\n",
        "- Token-driven theming via CSS custom properties\n",
        "- Behavior layer that runs exclusively on the client\n",
        "- Full ARIA compliance via structured contracts\n",
        "- Zero hydration mismatch by design\n\n",
        "## Component Table\n\n",
        "| Component | Category | Status |\n",
        "|-----------|----------|--------|\n",
        "| Button | Actions | Stable |\n",
        "| Avatar | Identity | Stable |\n",
        "| Progress | Feedback | Stable |\n",
        "| CodeBlock | Content | Stable |\n",
        "| Markdown | Content | Stable |\n\n",
        "## Code Example\n\n",
        "```rust\n",
        "fn main() {\n",
        "    println!(\"Hello, CanonRS!\");\n",
        "}\n",
        "```\n\n",
        "## Interaction Model\n\n",
        "All behavior is delegated to WASM modules that operate directly on the DOM. \n",
        "The boundary layer is zero-logic — it declares, never executes.\n\n",
        "### Init vs Interaction\n\n",
        "Simple stateless behaviors use the init layer. \n",
        "Complex coordinated behaviors — sorting, selection engines, virtualization — \n",
        "belong to the interaction layer and run as dedicated WASM modules.\n\n",
        "## Summary\n\n",
        "All components follow the Canon contract. \n",
        "The DOM is the source of truth. CSS reacts to state. WASM drives behavior.\n"
    );

    let rendered = render_markdown(sample);

    view! {
        <MarkdownSurface
            rendered=rendered
            show_toc=true
            show_toolbar=false
            toc_position=TocPosition::Sidebar
            id="md-preview"
        />
    }
}
