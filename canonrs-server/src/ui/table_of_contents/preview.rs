use leptos::prelude::*;
use super::table_of_contents_island::TableOfContentsIsland;
use crate::ui::scroll_area::scroll_area_island::ScrollAreaIsland;

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
                        <TableOfContentsIsland mode="simple" title="On this page">
                            <li data-rs-toc-item="" data-rs-target="toc-intro"   data-rs-level="1" data-rs-state="idle" data-child="false"><a data-rs-toc-link="" href="#toc-intro">"Introduction"</a></li>
                            <li data-rs-toc-item="" data-rs-target="toc-setup"   data-rs-level="1" data-rs-state="idle" data-child="false"><a data-rs-toc-link="" href="#toc-setup">"Setup"</a></li>
                            <li data-rs-toc-item="" data-rs-target="toc-install" data-rs-level="2" data-rs-state="idle" data-child="true"><a data-rs-toc-link="" href="#toc-install">"Installation"</a></li>
                            <li data-rs-toc-item="" data-rs-target="toc-config"  data-rs-level="2" data-rs-state="idle" data-child="true"><a data-rs-toc-link="" href="#toc-config">"Configuration"</a></li>
                            <li data-rs-toc-item="" data-rs-target="toc-usage"   data-rs-level="1" data-rs-state="idle" data-child="false"><a data-rs-toc-link="" href="#toc-usage">"Usage"</a></li>
                            <li data-rs-toc-item="" data-rs-target="toc-api"     data-rs-level="1" data-rs-state="idle" data-child="false"><a data-rs-toc-link="" href="#toc-api">"API Reference"</a></li>
                        </TableOfContentsIsland>
                    </div>
                    <div style="display:flex;flex-direction:column;gap:var(--space-xs);">
                        <span data-rs-showcase-preview-label="">"Expand"</span>
                        <TableOfContentsIsland mode="expand" title="On this page">
                            <li data-rs-toc-item="" data-rs-target="toc-intro"   data-rs-level="1" data-rs-state="idle" data-child="false"><a data-rs-toc-link="" href="#toc-intro">"Introduction"</a></li>
                            <li data-rs-toc-item="" data-rs-target="toc-setup"   data-rs-level="1" data-rs-state="idle" data-child="false"><a data-rs-toc-link="" href="#toc-setup">"Setup"</a></li>
                            <li data-rs-toc-item="" data-rs-target="toc-install" data-rs-level="2" data-rs-state="idle" data-child="true"><a data-rs-toc-link="" href="#toc-install">"Installation"</a></li>
                            <li data-rs-toc-item="" data-rs-target="toc-usage"   data-rs-level="1" data-rs-state="idle" data-child="false"><a data-rs-toc-link="" href="#toc-usage">"Usage"</a></li>
                        </TableOfContentsIsland>
                    </div>
                    <div style="display:flex;flex-direction:column;gap:var(--space-xs);">
                        <span data-rs-showcase-preview-label="">"Nested"</span>
                        <TableOfContentsIsland mode="nested" title="On this page">
                            <li data-rs-toc-item="" data-rs-target="toc-intro"   data-rs-level="1" data-rs-state="idle" data-child="false"><a data-rs-toc-link="" href="#toc-intro">"Introduction"</a></li>
                            <li data-rs-toc-item="" data-rs-target="toc-setup"   data-rs-level="1" data-rs-state="idle" data-child="false"><a data-rs-toc-link="" href="#toc-setup">"Setup"</a></li>
                            <li data-rs-toc-item="" data-rs-target="toc-install" data-rs-level="2" data-rs-state="idle" data-child="true"><a data-rs-toc-link="" href="#toc-install">"Installation"</a></li>
                            <li data-rs-toc-item="" data-rs-target="toc-usage"   data-rs-level="1" data-rs-state="idle" data-child="false"><a data-rs-toc-link="" href="#toc-usage">"Usage"</a></li>
                        </TableOfContentsIsland>
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
