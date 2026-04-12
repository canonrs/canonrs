use leptos::prelude::*;
use super::boundary::Drawer;
use canonrs_core::primitives::DrawerSide;

#[component]
pub fn DrawerShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Drawer
                    trigger_label="Open Drawer"
                    title="Drawer Title"
                    description="Slides in from the side. State governed via DOM."
                    close_label="Close"
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Drawer visibility and overlay fully governed via shared state."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Left side"</span>
                <div data-rs-showcase-preview-row="">
                    <Drawer
                        trigger_label="Open left"
                        title="Left Drawer"
                        close_label="Close"
                        side=DrawerSide::Left
                    />
                </div>
            </div>
        </div>
    }
}
