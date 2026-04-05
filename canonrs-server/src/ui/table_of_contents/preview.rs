use leptos::prelude::*;
use super::table_of_contents_island::{TableOfContentsIsland, TocIslandItem};
use crate::blocks::split::split_block::{Split, SplitRatio};
use crate::ui::scroll_area::scroll_area_ui::ScrollArea;

fn sample_items() -> Vec<TocIslandItem> {
    vec![
        TocIslandItem { id: "toc-intro".to_string(),    text: "Introduction".to_string(),  level: 1 },
        TocIslandItem { id: "toc-setup".to_string(),    text: "Setup".to_string(),         level: 1 },
        TocIslandItem { id: "toc-install".to_string(),  text: "Installation".to_string(),  level: 2 },
        TocIslandItem { id: "toc-config".to_string(),   text: "Configuration".to_string(), level: 2 },
        TocIslandItem { id: "toc-usage".to_string(),    text: "Usage".to_string(),         level: 1 },
        TocIslandItem { id: "toc-advanced".to_string(), text: "Advanced".to_string(),      level: 2 },
        TocIslandItem { id: "toc-api".to_string(),      text: "API Reference".to_string(), level: 1 },
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
                                <TableOfContentsIsland items=sample_items() mode="simple" title="On this page" />
                            </div>
                            <div style="flex:1;min-width:0;">
                                <span data-rs-showcase-preview-label="">"Expand"</span>
                                <TableOfContentsIsland items=sample_items() mode="expand" title="On this page" />
                            </div>
                            <div style="flex:1;min-width:0;">
                                <span data-rs-showcase-preview-label="">"Nested"</span>
                                <TableOfContentsIsland items=sample_items() mode="nested" title="On this page" />
                            </div>
                        </div>
                    })
                    main=leptos::children::ToChildren::to_children(|| view! {
                        <div id="toc-scroll-root" style="height:400px;">
                            <ScrollArea>
                                <div style="padding:1rem;display:flex;flex-direction:column;gap:2rem;">
                                    <section><h2 id="toc-intro">"Introduction"</h2><p>"Introduction content."</p></section>
                                    <section><h2 id="toc-setup">"Setup"</h2><p>"Setup section."</p></section>
                                    <section><h3 id="toc-install">"Installation"</h3><p>"Install via cargo add canonrs."</p></section>
                                    <section><h3 id="toc-config">"Configuration"</h3><p>"Configure tokens, themes and behavior."</p></section>
                                    <section><h2 id="toc-usage">"Usage"</h2><p>"Import components from canonrs::ui."</p></section>
                                    <section><h3 id="toc-advanced">"Advanced"</h3><p>"Advanced patterns include SSR and hydration."</p></section>
                                    <section><h2 id="toc-api">"API Reference"</h2><p>"Full API reference."</p></section>
                                </div>
                            </ScrollArea>
                        </div>
                    })
                />
            </div>
        </div>
    }
}
