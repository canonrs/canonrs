use leptos::prelude::*;
use super::slider_island::{SliderIsland, SliderWithMarksIsland};
use canonrs_core::meta::DisabledState;

#[component]
pub fn SliderShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <SliderIsland value=50.0 />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Drag to set value — interaction-driven, DOM as source of truth."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Steps"</span>
                <div data-rs-showcase-preview-row="">
                    <SliderIsland step=10.0 value=30.0 />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <SliderIsland value=60.0 disabled=DisabledState::Disabled />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With Marks"</span>
                <div data-rs-showcase-preview-row="">
                    <SliderWithMarksIsland step=10.0 value=40.0 />
                </div>
            </div>
        </div>
    }
}
