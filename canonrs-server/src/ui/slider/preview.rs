use leptos::prelude::*;
use super::slider_ui::Slider;
use canonrs_core::meta::DisabledState;

#[component]
pub fn SliderShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Slider min=0.0 max=100.0 value=50.0 />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Drag to set value — state governed by data-rs-value and data-rs-percent."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Steps"</span>
                <div data-rs-showcase-preview-row="">
                    <Slider min=0.0 max=100.0 step=10.0 value=30.0 />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <Slider min=0.0 max=100.0 value=60.0 disabled=DisabledState::Disabled />
                </div>
            </div>
        </div>
    }
}
