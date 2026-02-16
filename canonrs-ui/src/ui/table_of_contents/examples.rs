use leptos::prelude::*;
use canonrs_shared::TocItem;
use super::table_of_contents::{TableOfContents, TocMode};

fn enterprise_items() -> Vec<TocItem> {
    vec![
        TocItem::new("introduction".into(),          "Introduction".into(),          1),
        TocItem::new("getting-started".into(),       "Getting Started".into(),       2),
        TocItem::new("installation".into(),          "Installation".into(),          3),
        TocItem::new("configuration".into(),         "Configuration".into(),         3),
        TocItem::new("environment-variables".into(), "Environment Variables".into(), 4),
        TocItem::new("architecture".into(),          "Architecture".into(),          2),
        TocItem::new("core-concepts".into(),         "Core Concepts".into(),         3),
        TocItem::new("primitives".into(),            "Primitives".into(),            4),
        TocItem::new("behaviors".into(),             "Behaviors".into(),             4),
        TocItem::new("tokens".into(),                "Design Tokens".into(),         4),
        TocItem::new("api-reference".into(),         "API Reference".into(),         2),
        TocItem::new("contributing".into(),          "Contributing".into(),          2),
    ]
}

fn fake_content() -> impl IntoView {
    view! {
        <div data-toc-demo-content="" style="display:flex; flex-direction:column; gap:var(--space-lg); padding:var(--space-md);">
            <h1 id="introduction" style="font-size:var(--font-size-2xl); font-weight:var(--font-weight-bold);">"Introduction"</h1>
            <p style="color:var(--theme-surface-fg-muted);">"CanonRS is an enterprise-grade UI framework built with Rust and Leptos. SSR-first, behavior-driven, token-based."</p>

            <h2 id="getting-started" style="font-size:var(--font-size-xl); font-weight:var(--font-weight-semibold); margin-top:var(--space-xl);">"Getting Started"</h2>
            <p style="color:var(--theme-surface-fg-muted);">"Everything you need to get up and running with CanonRS in your project."</p>

            <h3 id="installation" style="font-size:var(--font-size-lg); font-weight:var(--font-weight-semibold); margin-top:var(--space-lg);">"Installation"</h3>
            <p style="color:var(--theme-surface-fg-muted);">"Add CanonRS to your Cargo.toml and configure the build pipeline."</p>

            <h3 id="configuration" style="font-size:var(--font-size-lg); font-weight:var(--font-weight-semibold); margin-top:var(--space-lg);">"Configuration"</h3>
            <p style="color:var(--theme-surface-fg-muted);">"Configure your workspace with the correct profiles and feature flags."</p>

            <h4 id="environment-variables" style="font-size:var(--font-size-md); font-weight:var(--font-weight-semibold); margin-top:var(--space-md);">"Environment Variables"</h4>
            <p style="color:var(--theme-surface-fg-muted);">"Set up the required environment variables for development and production."</p>

            <h2 id="architecture" style="font-size:var(--font-size-xl); font-weight:var(--font-weight-semibold); margin-top:var(--space-xl);">"Architecture"</h2>
            <p style="color:var(--theme-surface-fg-muted);">"CanonRS follows a strict separation of concerns: primitives, behaviors, tokens, and UI components."</p>

            <h3 id="core-concepts" style="font-size:var(--font-size-lg); font-weight:var(--font-weight-semibold); margin-top:var(--space-lg);">"Core Concepts"</h3>
            <p style="color:var(--theme-surface-fg-muted);">"Understanding the 4 pillars of CanonRS architecture."</p>

            <h4 id="primitives" style="font-size:var(--font-size-md); font-weight:var(--font-weight-semibold); margin-top:var(--space-md);">"Primitives"</h4>
            <p style="color:var(--theme-surface-fg-muted);">"Pure HTML components with data attributes. No logic, no state."</p>

            <h4 id="behaviors" style="font-size:var(--font-size-md); font-weight:var(--font-weight-semibold); margin-top:var(--space-md);">"Behaviors"</h4>
            <p style="color:var(--theme-surface-fg-muted);">"Client-side JS that attaches to data attributes after hydration."</p>

            <h4 id="tokens" style="font-size:var(--font-size-md); font-weight:var(--font-weight-semibold); margin-top:var(--space-md);">"Design Tokens"</h4>
            <p style="color:var(--theme-surface-fg-muted);">"CSS custom properties generated from Rust token definitions."</p>

            <h2 id="api-reference" style="font-size:var(--font-size-xl); font-weight:var(--font-weight-semibold); margin-top:var(--space-xl);">"API Reference"</h2>
            <p style="color:var(--theme-surface-fg-muted);">"Complete API documentation for all CanonRS components."</p>

            <h2 id="contributing" style="font-size:var(--font-size-xl); font-weight:var(--font-weight-semibold); margin-top:var(--space-xl);">"Contributing"</h2>
            <p style="color:var(--theme-surface-fg-muted);">"How to contribute to the CanonRS project."</p>
        </div>
    }
}

fn demo_layout(toc: impl IntoView, content_id: &'static str) -> impl IntoView {
    view! {
        <div style="display:grid; grid-template-columns:220px 1fr; gap:var(--space-lg); max-height:400px; overflow:hidden; border:1px solid var(--theme-surface-border); border-radius:var(--radius-md);">
            <div data-toc-sticky="" style="position:sticky; top:0; height:400px; overflow-y:auto; border-right:1px solid var(--theme-surface-border-muted); padding:var(--space-md);">
                {toc}
            </div>
            <div id=content_id style="overflow-y:auto; height:400px; padding:var(--space-md);">
                {fake_content()}
            </div>
        </div>
    }
}

pub fn simple_example() -> impl IntoView {
    demo_layout(
        view! {
            <TableOfContents
                items=enterprise_items()
                mode=TocMode::Simple
                id="toc-simple"
                title="On this page"
            />
        },
        "toc-simple-content"
    )
}

pub fn expand_example() -> impl IntoView {
    demo_layout(
        view! {
            <TableOfContents
                items=enterprise_items()
                mode=TocMode::Expand
                id="toc-expand"
                title="Contents"
            />
        },
        "toc-expand-content"
    )
}

pub fn nested_example() -> impl IntoView {
    demo_layout(
        view! {
            <TableOfContents
                items=enterprise_items()
                mode=TocMode::Nested
                id="toc-nested"
                title="Documentation"
            />
        },
        "toc-nested-content"
    )
}

pub fn basic_example() -> impl IntoView {
    simple_example()
}
