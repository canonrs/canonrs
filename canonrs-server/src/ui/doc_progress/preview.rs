use leptos::prelude::*;
use super::doc_progress_boundary::DocProgressSlot;
use crate::ui::scroll_area::scroll_area_boundary::ScrollArea;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn DocProgressShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Progress tracking injected automatically via behavior layer."
            </p>
            <div data-rs-doc-progress-demo="">
                <DocProgressSlot scroll_target="doc-progress-viewport" position="top" />
                <ScrollArea viewport_id="doc-progress-viewport">
                    <Stack direction=StackDirection::Vertical gap=StackGap::Xl>
                        <section>
                            <h2>"Introduction"</h2>
                            <p>"This document tracks your reading progress as you scroll."</p>
                        </section>
                        <section>
                            <h2>"Getting Started"</h2>
                            <p>"DocProgress uses a behavior layer that observes scroll position."</p>
                        </section>
                        <section>
                            <h2>"Installation"</h2>
                            <p>"Add the component to your layout and the behavior handles the rest."</p>
                        </section>
                        <section>
                            <h2>"Configuration"</h2>
                            <p>"Tokens control height, color and transition speed."</p>
                        </section>
                        <section>
                            <h2>"Advanced Usage"</h2>
                            <p>"Use DocProgressSlot to embed the bar inside a custom header."</p>
                        </section>
                        <section>
                            <h2>"API Reference"</h2>
                            <p>"DocProgress — standalone fixed bar. DocProgressSlot — portal for custom placement."</p>
                        </section>
                    </Stack>
                </ScrollArea>
            </div>
        </Stack>
    }
}
