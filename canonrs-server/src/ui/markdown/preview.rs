use leptos::prelude::*;
use super::markdown_island::MarkdownSurfaceIsland;
use super::render_markdown;
use crate::ui::scroll_area::scroll_area_island::ScrollAreaIsland;

#[component]
pub fn MarkdownShowcasePreview() -> impl IntoView {
    let sample = concat!(
        "## Introduction\n\n",
        "CanonRS is a design system built with **Leptos** and **Rust**.\n\n",
        "## Features\n\n",
        "- SSR-safe rendering with deterministic DOM output\n",
        "- Token-driven theming via CSS custom properties\n",
        "- Behavior layer that runs exclusively on the client\n",
        "- Full ARIA compliance via structured contracts\n\n",
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
        "## Summary\n\n",
        "All components follow the Canon contract.\n"
    );

    let rendered_toc   = render_markdown(sample);
    let rendered_plain = render_markdown(sample);

    view! {
        <div data-rs-showcase-preview-hero="">
            <p data-rs-showcase-preview-anchor="">
                "SSR-safe markdown rendering with deterministic DOM output."
            </p>
            <div data-rs-showcase-preview-stage="" style="width:100%;padding:0;">
                <div style="display:grid;grid-template-columns:200px 1fr;width:100%;height:400px;">
                    <div style="display:flex;flex-direction:column;overflow:hidden;height:400px;">
                        <span data-rs-showcase-preview-label="">"With Table of Contents"</span>
                        <div style="flex:1;overflow:hidden;">
                            <ScrollAreaIsland>
                                <MarkdownSurfaceIsland
                                    rendered=rendered_toc
                                    show_toc=true
                                    show_toolbar=false
                                    toc_position="sidebar"
                                    id="md-preview-toc"
                                />
                            </ScrollAreaIsland>
                        </div>
                    </div>
                    <div style="display:flex;flex-direction:column;overflow:hidden;height:400px;">
                        <span data-rs-showcase-preview-label="">"Only Markdown"</span>
                        <div style="flex:1;overflow:hidden;">
                            <ScrollAreaIsland>
                                <MarkdownSurfaceIsland
                                    rendered=rendered_plain
                                    show_toc=false
                                    show_toolbar=false
                                    id="md-preview-plain"
                                />
                            </ScrollAreaIsland>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
