use canonrs_core::primitives::table_of_contents::TocMode;
use canonrs_core::TocItem;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};
use leptos::prelude::*;
use super::table_of_contents_boundary::TableOfContents;
use crate::ui::scroll_area::scroll_area_boundary::ScrollArea;

fn demo_items() -> Vec<TocItem> {
    vec![
        TocItem::new("toc-intro".into(),   "Introduction".into(),  1),
        TocItem::new("toc-setup".into(),   "Setup".into(),         1),
        TocItem::new("toc-install".into(), "Installation".into(),  2),
        TocItem::new("toc-config".into(),  "Configuration".into(), 2),
        TocItem::new("toc-env".into(),     "Environment".into(),   3),
        TocItem::new("toc-usage".into(),   "Usage".into(),         1),
        TocItem::new("toc-examples".into(),"Examples".into(),      2),
        TocItem::new("toc-api".into(),     "API Reference".into(), 1),
        TocItem::new("toc-props".into(),   "Props".into(),         2),
        TocItem::new("toc-events".into(),  "Events".into(),        2),
    ]
}

#[component]
pub fn TableOfContentsShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <p data-rs-showcase-preview-anchor="">
                "TOC hierarchy and state derived from structured data model."
            </p>
            <div data-rs-showcase-preview-stage="" style="padding:0">
                <div style="display:grid;grid-template-columns:1fr 1fr 1fr 2fr;gap:var(--space-md);height:400px;width:100%">
                    <div style="display:flex;flex-direction:column;gap:var(--space-xs)">
                        <span data-rs-showcase-preview-label="">"Simple"</span>
                        <TableOfContents mode=TocMode::Simple title="On this page" items=demo_items() />
                    </div>
                    <div style="display:flex;flex-direction:column;gap:var(--space-xs)">
                        <span data-rs-showcase-preview-label="">"Expand"</span>
                        <TableOfContents mode=TocMode::Expand title="On this page" items=demo_items() />
                    </div>
                    <div style="display:flex;flex-direction:column;gap:var(--space-xs)">
                        <span data-rs-showcase-preview-label="">"Nested"</span>
                        <TableOfContents mode=TocMode::Nested title="On this page" items=demo_items() />
                    </div>
                    <div style="height:400px;display:flex;flex-direction:column">
                        <span data-rs-showcase-preview-label="">"Content"</span>
                        <div style="flex:1;min-height:0">
                            <ScrollArea>
                                <Stack direction=StackDirection::Vertical gap=StackGap::Xl>
                                    <section>
                                        <h2 id="toc-intro">"Introduction"</h2>
                                        <p>"Introduction content goes here. This section covers the basic concepts and goals of the library. Understanding the foundation will help you get the most out of CanonRS."</p>
                                        <p>"CanonRS is built on a strict contract between primitives, tokens, and interaction modules. Each layer has a single responsibility and clear boundaries."</p>
                                    </section>
                                    <section>
                                        <h2 id="toc-setup">"Setup"</h2>
                                        <p>"Before you begin, make sure you have Rust and the WASM toolchain installed. The setup process is straightforward and takes only a few minutes."</p>
                                        <p>"You will also need to configure your build pipeline to output WASM artifacts. Follow the steps below carefully to avoid common pitfalls."</p>
                                    </section>
                                    <section>
                                        <h3 id="toc-install">"Installation"</h3>
                                        <p>"Install via cargo add canonrs. You can also add it manually to your Cargo.toml. Make sure to enable the ssr feature for server-side rendering support."</p>
                                        <p>"After installation, run cargo build to verify everything compiles correctly."</p>
                                    </section>
                                    <section>
                                        <h3 id="toc-config">"Configuration"</h3>
                                        <p>"Configure tokens, themes and behavior. The token system is the foundation of all visual decisions. Start by selecting a base theme and customize from there."</p>
                                        <p>"All configuration is done through CSS custom properties. No JavaScript configuration is required."</p>
                                    </section>
                                    <section>
                                        <h4 id="toc-env">"Environment"</h4>
                                        <p>"Set the CANON_ENV variable to production before deploying. This enables optimized output and disables debug logging. Make sure all required environment variables are set."</p>
                                    </section>
                                    <section>
                                        <h2 id="toc-usage">"Usage"</h2>
                                        <p>"Import components from canonrs::ui. Each component follows the same pattern: primitive, UI, boundary. Use the boundary in your application code."</p>
                                        <p>"Components are designed to be composable. You can nest them freely as long as you follow the contract defined by each primitive."</p>
                                    </section>
                                    <section>
                                        <h3 id="toc-examples">"Examples"</h3>
                                        <p>"See the examples directory for working demos of each component. Each example is self-contained and can be run independently. Use them as a starting point for your own implementation."</p>
                                    </section>
                                    <section>
                                        <h2 id="toc-api">"API Reference"</h2>
                                        <p>"Full API reference documentation. Every component, primitive, and token is documented here. Use the search to find what you need quickly."</p>
                                        <p>"The API is versioned and follows semantic versioning. Breaking changes are announced in the changelog."</p>
                                    </section>
                                    <section>
                                        <h3 id="toc-props">"Props"</h3>
                                        <p>"Each component accepts a set of typed props defined by its primitive. Props are validated at compile time by the Rust type system. Optional props have sensible defaults."</p>
                                    </section>
                                    <section>
                                        <h3 id="toc-events">"Events"</h3>
                                        <p>"Components emit events via data-rs-action attributes. Listen for these in your interaction modules. Events bubble up through the DOM following standard browser conventions."</p>
                                    </section>
                                </Stack>
                            </ScrollArea>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}
