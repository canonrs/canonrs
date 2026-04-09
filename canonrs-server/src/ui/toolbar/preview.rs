use leptos::prelude::*;
use super::toolbar_island::{ToolbarIsland, ToolbarItemIsland, ToolbarSeparatorIsland};
use canonrs_core::primitives::ToolbarOrientation;

#[component]
pub fn ToolbarShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="" style="width:100%;">
                <ToolbarIsland aria_label="Text formatting">
                    <ToolbarItemIsland value="bold">"B"</ToolbarItemIsland>
                    <ToolbarItemIsland value="italic">"I"</ToolbarItemIsland>
                    <ToolbarItemIsland value="underline">"U"</ToolbarItemIsland>
                    <ToolbarSeparatorIsland />
                    <ToolbarItemIsland value="left">"←"</ToolbarItemIsland>
                    <ToolbarItemIsland value="center">"↔"</ToolbarItemIsland>
                    <ToolbarItemIsland value="right">"→"</ToolbarItemIsland>
                    <ToolbarSeparatorIsland />
                    <ToolbarItemIsland value="link">"🔗"</ToolbarItemIsland>
                </ToolbarIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Toolbar role and orientation enforced via contract."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Vertical"</span>
                <div data-rs-showcase-preview-row="">
                    <ToolbarIsland aria_label="Actions" orientation=ToolbarOrientation::Vertical>
                        <ToolbarItemIsland value="cut">"✂"</ToolbarItemIsland>
                        <ToolbarItemIsland value="copy">"⎘"</ToolbarItemIsland>
                        <ToolbarItemIsland value="paste">"📋"</ToolbarItemIsland>
                        <ToolbarSeparatorIsland />
                        <ToolbarItemIsland value="delete">"🗑"</ToolbarItemIsland>
                    </ToolbarIsland>
                </div>
            </div>
        </div>
    }
}
