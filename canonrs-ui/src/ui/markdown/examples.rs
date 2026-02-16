use leptos::prelude::*;
use super::{MarkdownSurface, TocPosition, RenderedMarkdown};
use super::renderer::{render_markdown, render_markdown_with_prefix as render_with_prefix};


fn enterprise_markdown() -> &'static str {
    r#"
# CanonRS Framework

A modern, enterprise-grade component library.

## Getting Started

Learn how to install and configure CanonRS.

### Installation
```bash
cargo add canonrs
```

### Configuration

Set up your project with the CanonRS design system.

## Architecture

Understanding the core concepts.

### Component Model

SSR-first, behavior-driven components.

### Token System

Design tokens for consistency.

## Examples

Practical examples and patterns.
"#
}

pub fn basic_example() -> impl IntoView {
    let rendered = render_with_prefix(enterprise_markdown(), "md-top");
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
    let rendered = render_with_prefix(enterprise_markdown(), "md-sidebar");
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

pub fn toc_with_breadcrumb_example() -> impl IntoView {
    use crate::ui::breadcrumb::{NavigationProvider, BreadcrumbAuto};
    
    let rendered = render_with_prefix(enterprise_markdown(), "md-nav");
    
    view! {
        <NavigationProvider>
            <div style="display: flex; flex-direction: column;">
                <div class="breadcrumb-sticky">
                    <BreadcrumbAuto />
                </div>
                <MarkdownSurface
                    rendered=rendered
                    show_toc=true
                    show_toolbar=false
                    toc_position=TocPosition::Sidebar
                    id="markdown-nav-example"
                />
            </div>
        </NavigationProvider>
    }
}
