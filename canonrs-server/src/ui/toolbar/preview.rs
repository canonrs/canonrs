use leptos::prelude::*;
use super::toolbar_boundary::{Toolbar, ToolbarItem, ToolbarSeparator};
use canonrs_core::primitives::ToolbarOrientation;

#[component]
pub fn ToolbarShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="" style="width:100%;">
                <Toolbar aria_label="Text formatting">
                    <ToolbarItem value="bold">"B"</ToolbarItem>
                    <ToolbarItem value="italic">"I"</ToolbarItem>
                    <ToolbarItem value="underline">"U"</ToolbarItem>
                    <ToolbarSeparator />
                    <ToolbarItem value="left">"←"</ToolbarItem>
                    <ToolbarItem value="center">"↔"</ToolbarItem>
                    <ToolbarItem value="right">"→"</ToolbarItem>
                    <ToolbarSeparator />
                    <ToolbarItem value="link">"🔗"</ToolbarItem>
                </Toolbar>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Toolbar role and orientation enforced via contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Vertical"</span>
                <div data-rs-showcase-preview-row="">
                    <Toolbar aria_label="Actions" orientation=ToolbarOrientation::Vertical>
                        <ToolbarItem value="cut">"✂"</ToolbarItem>
                        <ToolbarItem value="copy">"⎘"</ToolbarItem>
                        <ToolbarItem value="paste">"📋"</ToolbarItem>
                        <ToolbarSeparator />
                        <ToolbarItem value="delete">"🗑"</ToolbarItem>
                    </Toolbar>
                </div>
            </div>
        </div>
    }
}
