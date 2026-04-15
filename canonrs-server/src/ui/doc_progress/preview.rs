use leptos::prelude::*;
use super::doc_progress_boundary::DocProgressSlot;
use crate::ui::scroll_area::scroll_area_boundary::ScrollArea;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};
use canonrs_core::primitives::layout::container::{ContainerPrimitive as Container, ContainerSize};

#[component]
pub fn DocProgressShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Progress tracking injected automatically via behavior layer."
            </p>
            <Container size=ContainerSize::Md>
            <div data-rs-doc-progress-demo="" style="position:relative; height: 320px; border: var(--border-thin) solid var(--theme-surface-border); border-radius: var(--radius-md); overflow: hidden;">
                <DocProgressSlot scroll_target="doc-progress-viewport" position="top" />
                <ScrollArea viewport_id="doc-progress-viewport">
                    <Stack direction=StackDirection::Vertical gap=StackGap::Xl>
                        <section>
                            <h2>"Introduction"</h2>
                            <p>"This document tracks your reading progress as you scroll through the content. The progress bar updates in real time based on scroll position, giving readers a clear visual indicator of how far they have come."</p>
                            <p>"The implementation uses a behavior layer that observes scroll events and maps them to a percentage value. This value is stored as a data attribute on the root element, keeping the DOM as the single source of truth."</p>
                        </section>
                        <section>
                            <h2>"Getting Started"</h2>
                            <p>"DocProgress uses a behavior layer that observes scroll position and computes a reading percentage. The component is SSR-safe and hydration-safe — the initial state is rendered on the server and the behavior layer takes over on the client."</p>
                            <p>"To use DocProgress, simply add it to your layout. No configuration is required for basic usage. The component will automatically track the scroll position of the window or a custom scroll container."</p>
                        </section>
                        <section>
                            <h2>"Installation"</h2>
                            <p>"Add the DocProgress component to your layout and the behavior handles the rest. The component exposes two variants: a standalone fixed bar that sits at the top of the viewport, and a portal variant that can be embedded inside a custom header or container."</p>
                            <p>"The portal variant is useful when you want the progress bar to appear inside a sticky header or a custom scrollable area rather than at the top of the page."</p>
                        </section>
                        <section>
                            <h2>"Configuration"</h2>
                            <p>"Tokens control height, color and transition speed. The following tokens are available: doc-progress-height, doc-progress-bg, doc-progress-bar-bg, doc-progress-z-index, and doc-progress-transition."</p>
                            <p>"All tokens follow the CanonRS token system and can be overridden at any level of the component tree. This makes it easy to customize the appearance of the progress bar without touching the component source code."</p>
                        </section>
                        <section>
                            <h2>"Advanced Usage"</h2>
                            <p>"Use DocProgressSlot to embed the bar inside a custom header. This is useful when you want the progress bar to appear inside a sticky navigation bar or a custom layout component."</p>
                            <p>"The scroll_target attribute allows you to specify a custom scroll container. This is useful when the scrollable content is not the window but a specific element on the page, such as a modal or a side panel."</p>
                        </section>
                        <section>
                            <h2>"API Reference"</h2>
                            <p>"DocProgress — standalone fixed bar that tracks window scroll. DocProgressSlot — portal for custom placement inside headers or containers. Both components share the same behavior layer and token system."</p>
                            <p>"The aria-valuenow attribute is updated in real time to reflect the current progress percentage. Screen readers can announce the progress to users who rely on assistive technology."</p>
                        </section>
                        <section>
                            <h2>"Accessibility"</h2>
                            <p>"The progress bar uses role=progressbar with aria-valuemin, aria-valuemax, aria-valuenow, and aria-valuetext attributes. The aria-valuetext attribute provides a human-readable description of the progress value for screen readers."</p>
                        </section>
                    </Stack>
                </ScrollArea>
            </div>
            </Container>
        </Stack>
    }
}
