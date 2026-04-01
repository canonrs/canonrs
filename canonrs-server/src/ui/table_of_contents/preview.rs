use leptos::prelude::*;
use super::table_of_contents_ui::TableOfContents;
use canonrs_core::TocItem;
use canonrs_core::primitives::table_of_contents::TocMode;
use crate::blocks::split::split_block::{Split, SplitRatio};
use crate::ui::scroll_area::scroll_area_ui::ScrollArea;

fn sample_items() -> Vec<TocItem> {
    vec![
        TocItem { id: "toc-intro".to_string(),    text: "Introduction".to_string(),  level: 1 },
        TocItem { id: "toc-setup".to_string(),    text: "Setup".to_string(),         level: 1 },
        TocItem { id: "toc-install".to_string(),  text: "Installation".to_string(),  level: 2 },
        TocItem { id: "toc-config".to_string(),   text: "Configuration".to_string(), level: 2 },
        TocItem { id: "toc-usage".to_string(),    text: "Usage".to_string(),         level: 1 },
        TocItem { id: "toc-advanced".to_string(), text: "Advanced".to_string(),      level: 2 },
        TocItem { id: "toc-api".to_string(),      text: "API Reference".to_string(), level: 1 },
    ]
}

#[component]
pub fn TableOfContentsShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <p data-rs-showcase-preview-anchor="">
                "TOC hierarchy and state derived from structured data model."
            </p>
            <div data-rs-showcase-preview-stage="">
                <Split
                    ratio=SplitRatio::OneThird
                    aside=leptos::children::ToChildren::to_children(|| view! {
                        <div style="display:flex;flex-direction:row;gap:1rem;align-items:flex-start;">
                            <div style="flex:1;min-width:0;">
                                <span data-rs-showcase-preview-label="">"Simple"</span>
                                <TableOfContents items=sample_items() mode=TocMode::Simple title="On this page" />
                            </div>
                            <div style="flex:1;min-width:0;">
                                <span data-rs-showcase-preview-label="">"Expand"</span>
                                <TableOfContents items=sample_items() mode=TocMode::Expand title="On this page" />
                            </div>
                            <div style="flex:1;min-width:0;">
                                <span data-rs-showcase-preview-label="">"Nested"</span>
                                <TableOfContents items=sample_items() mode=TocMode::Nested title="On this page" />
                            </div>
                        </div>
                    })
                    main=leptos::children::ToChildren::to_children(|| view! {
                        <div id="toc-scroll-root" style="height:400px;">
                            <ScrollArea>
                                <div style="padding:1rem;display:flex;flex-direction:column;gap:2rem;">
                                    <section><h2 id="toc-intro">"Introduction"</h2><p>"Introduction content. Scroll to navigate between sections and watch the TOC highlight update in real time."</p></section>
                                    <section><h2 id="toc-setup">"Setup"</h2><p>"Setup section. This section covers everything you need to get started."</p></section>
                                    <section><h3 id="toc-install">"Installation"</h3><p>"Install via cargo add canonrs or add the dependency manually to your Cargo.toml."</p></section>
                                    <section><h3 id="toc-config">"Configuration"</h3><p>"Configure tokens, themes and behavior via the canonrs-tokens package."</p></section>
                                    <section><h2 id="toc-usage">"Usage"</h2><p>"Import components directly from canonrs::ui and compose your interface."</p></section>
                                    <section><h3 id="toc-advanced">"Advanced"</h3><p>"Advanced patterns include server-side rendering, hydration and custom behaviors."</p></section>
                                    <section><h2 id="toc-api">"API Reference"</h2><p>"Full API reference with all props, variants and examples for every component."</p></section>
                                </div>
                            </ScrollArea>
                        </div>
                    })
                />
            </div>
        </div>
    }
}
