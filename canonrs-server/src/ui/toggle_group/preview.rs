use leptos::prelude::*;
use super::toggle_group_island::{ToggleGroupIsland, ToggleGroupItemIsland};

#[component]
pub fn ToggleGroupShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ToggleGroupIsland>
                    <ToggleGroupItemIsland value="left" on=true>"Left"</ToggleGroupItemIsland>
                    <ToggleGroupItemIsland value="center">"Center"</ToggleGroupItemIsland>
                    <ToggleGroupItemIsland value="right">"Right"</ToggleGroupItemIsland>
                </ToggleGroupIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Group behavior and selection mode enforced structurally."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Multiple selection"</span>
                <div data-rs-showcase-preview-row="">
                    <ToggleGroupIsland multiple=true>
                        <ToggleGroupItemIsland value="bold" on=true>"Bold"</ToggleGroupItemIsland>
                        <ToggleGroupItemIsland value="italic" on=true>"Italic"</ToggleGroupItemIsland>
                        <ToggleGroupItemIsland value="underline">"Underline"</ToggleGroupItemIsland>
                        <ToggleGroupItemIsland value="strike">"Strike"</ToggleGroupItemIsland>
                    </ToggleGroupIsland>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <ToggleGroupIsland disabled=true>
                        <ToggleGroupItemIsland value="a">"Option A"</ToggleGroupItemIsland>
                        <ToggleGroupItemIsland value="b">"Option B"</ToggleGroupItemIsland>
                    </ToggleGroupIsland>
                </div>
            </div>
        </div>
    }
}
