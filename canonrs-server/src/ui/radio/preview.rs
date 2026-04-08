use leptos::prelude::*;
use super::radio_island::{RadioGroupIsland, RadioGroupItemIsland};
use canonrs_core::meta::{SelectionState, DisabledState};

#[component]
pub fn RadioShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <RadioGroupIsland>
                    <RadioGroupItemIsland value="leptos" name="framework">"Leptos"</RadioGroupItemIsland>
                    <RadioGroupItemIsland value="dioxus" name="framework" selected=SelectionState::Selected>"Dioxus"</RadioGroupItemIsland>
                    <RadioGroupItemIsland value="yew" name="framework">"Yew"</RadioGroupItemIsland>
                </RadioGroupIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Selection state mapped directly to DOM and ARIA."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <RadioGroupIsland disabled=DisabledState::Disabled>
                        <RadioGroupItemIsland value="a" name="disabled">"Option A"</RadioGroupItemIsland>
                        <RadioGroupItemIsland value="b" name="disabled">"Option B"</RadioGroupItemIsland>
                    </RadioGroupIsland>
                </div>
            </div>
        </div>
    }
}
