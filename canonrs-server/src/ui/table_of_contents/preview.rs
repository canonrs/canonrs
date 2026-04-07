use leptos::prelude::*;
use super::table_of_contents_island::{TableOfContentsIsland, TocIslandItem};
use crate::ui::scroll_area::scroll_area_island::ScrollAreaIsland;

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
            <div data-rs-showcase-preview-stage="" style="padding:0;">
                <div style="display:grid;grid-template-columns:1fr 1fr 1fr 2fr;gap:var(--space-md);height:400px;width:100%;">
                    <div style="display:flex;flex-direction:column;gap:var(--space-xs);">
                        <span data-rs-showcase-preview-label="">"Simple"</span>
                        <TableOfContentsIsland items=sample_items() mode="simple" title="On this page" />
                    </div>
                    <div style="display:flex;flex-direction:column;gap:var(--space-xs);">
                        <span data-rs-showcase-preview-label="">"Expand"</span>
                        <TableOfContentsIsland items=sample_items() mode="expand" title="On this page" />
                    </div>
                    <div style="display:flex;flex-direction:column;gap:var(--space-xs);">
                        <span data-rs-showcase-preview-label="">"Nested"</span>
                        <TableOfContentsIsland items=sample_items() mode="nested" title="On this page" />
                    </div>
                    <div style="height:400px;display:flex;flex-direction:column;">
                        <span data-rs-showcase-preview-label="">"Content"</span>
                        <div style="flex:1;min-height:0;">
                            <ScrollAreaIsland>
                                <div style="padding:var(--space-md);display:flex;flex-direction:column;gap:var(--space-xl);">
                                    <section><h2 id="toc-intro">"Introduction"</h2><p>"Introduction content goes here."</p></section>
                                    <section><h2 id="toc-setup">"Setup"</h2><p>"Setup section content."</p></section>
                                    <section><h3 id="toc-install">"Installation"</h3><p>"Install via cargo add canonrs."</p></section>
                                    <section><h3 id="toc-config">"Configuration"</h3><p>"Configure tokens, themes and behavior."</p></section>
                                    <section><h2 id="toc-usage">"Usage"</h2><p>"Import components from canonrs::ui."</p></section>
                                    <section><h3 id="toc-advanced">"Advanced"</h3><p>"Advanced SSR and hydration patterns."</p></section>
                                    <section><h2 id="toc-api">"API Reference"</h2><p>"Full API reference documentation."</p></section>
                                </div>
                            </ScrollAreaIsland>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
