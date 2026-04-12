use leptos::prelude::*;
use super::doc_progress_boundary::DocProgressSlot;
use crate::ui::scroll_area::scroll_area_ui::ScrollArea;

#[component]
pub fn DocProgressShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <p data-rs-showcase-preview-anchor="">
                "Progress tracking injected automatically via behavior layer."
            </p>
            <div data-rs-showcase-preview-stage="">
                <div style="position:relative;width:100%;height:400px;border:1px solid rgba(255,255,255,0.08);border-radius:var(--radius-md);overflow:hidden;">
                    <DocProgressSlot
                        scroll_target="doc-progress-viewport"
                        position="top"
                    />
                    <div style="height:100%;">
                        <ScrollArea viewport_id="doc-progress-viewport">
                            <div style="padding:var(--space-xl);display:flex;flex-direction:column;gap:var(--space-2xl);">
                                <section>
                                    <h2 style="margin-bottom:var(--space-sm);">"Introduction"</h2>
                                    <p>"This document tracks your reading progress as you scroll."</p>
                                </section>
                                <section>
                                    <h2 style="margin-bottom:var(--space-sm);">"Getting Started"</h2>
                                    <p>"DocProgress uses a behavior layer that observes scroll position."</p>
                                </section>
                                <section>
                                    <h2 style="margin-bottom:var(--space-sm);">"Installation"</h2>
                                    <p>"Add the component to your layout and the behavior handles the rest."</p>
                                </section>
                                <section>
                                    <h2 style="margin-bottom:var(--space-sm);">"Configuration"</h2>
                                    <p>"Tokens control height, color and transition speed."</p>
                                </section>
                                <section>
                                    <h2 style="margin-bottom:var(--space-sm);">"Advanced Usage"</h2>
                                    <p>"Use DocProgressSlot to embed the bar inside a custom header."</p>
                                </section>
                                <section>
                                    <h2 style="margin-bottom:var(--space-sm);">"API Reference"</h2>
                                    <p>"DocProgress — standalone fixed bar. DocProgressSlot — portal for custom placement."</p>
                                </section>
                            </div>
                        </ScrollArea>
                    </div>
                </div>
            </div>
        </div>
    }
}
