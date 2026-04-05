use leptos::prelude::*;
use super::tooltip_island::TooltipIsland;

#[component]
pub fn TooltipShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <TooltipIsland
                    label="Hover me"
                    content="This is a tooltip"
                />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Trigger-content relationship enforced without manual wiring."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sides"</span>
                <div data-rs-showcase-preview-row="">
                    <TooltipIsland label="Top"    content="Top tooltip"    side="top" />
                    <TooltipIsland label="Bottom" content="Bottom tooltip" side="bottom" />
                    <TooltipIsland label="Left"   content="Left tooltip"   side="left" />
                    <TooltipIsland label="Right"  content="Right tooltip"  side="right" />
                </div>
            </div>
        </div>
    }
}
