use leptos::prelude::*;
use super::markdown_boundary::{MarkdownSurface, TocPosition};
use super::render_markdown;
use crate::ui::scroll_area::scroll_area_boundary::ScrollArea;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

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
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "SSR-safe markdown rendering with deterministic DOM output."
            </p>
            <Stack direction=StackDirection::Horizontal gap=StackGap::Md>
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <span data-rs-showcase-preview-label="">"With Table of Contents"</span>
                    <ScrollArea>
                        <MarkdownSurface
                            rendered=rendered_toc
                            show_toc=true
                            show_toolbar=false
                            toc_position=TocPosition::Sidebar
                            id="md-preview-toc"
                        />
                    </ScrollArea>
                </Stack>
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <span data-rs-showcase-preview-label="">"Only Markdown"</span>
                    <ScrollArea>
                        <MarkdownSurface
                            rendered=rendered_plain
                            show_toc=false
                            show_toolbar=false
                            id="md-preview-plain"
                        />
                    </ScrollArea>
                </Stack>
            </Stack>
        </Stack>
    }
}
