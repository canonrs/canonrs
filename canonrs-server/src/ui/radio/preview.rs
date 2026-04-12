use leptos::prelude::*;
use super::radio_boundary::{RadioGroup, RadioGroupItem};
use canonrs_core::meta::{SelectionState, DisabledState};

#[component]
pub fn RadioShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <RadioGroup>
                    <RadioGroupItem value="leptos" name="framework">"Leptos"</RadioGroupItem>
                    <RadioGroupItem value="dioxus" name="framework" selected=SelectionState::Selected>"Dioxus"</RadioGroupItem>
                    <RadioGroupItem value="yew" name="framework">"Yew"</RadioGroupItem>
                </RadioGroup>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Selection state mapped directly to DOM and ARIA."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <RadioGroup disabled=DisabledState::Disabled>
                        <RadioGroupItem value="a" name="disabled">"Option A"</RadioGroupItem>
                        <RadioGroupItem value="b" name="disabled">"Option B"</RadioGroupItem>
                    </RadioGroup>
                </div>
            </div>
        </div>
    }
}
