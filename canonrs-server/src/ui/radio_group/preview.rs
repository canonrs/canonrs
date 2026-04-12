use leptos::prelude::*;
use super::boundary::{RadioGroup, RadioGroupItem};
use canonrs_core::meta::{SelectionState, DisabledState};

#[component]
pub fn RadioGroupShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <RadioGroup>
                    <RadioGroupItem value="leptos" name="framework">"Leptos"</RadioGroupItem>
                    <RadioGroupItem value="dioxus" name="framework">"Dioxus"</RadioGroupItem>
                    <RadioGroupItem value="yew" name="framework">"Yew"</RadioGroupItem>
                </RadioGroup>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Selection state mapped directly to DOM and ARIA."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Pre-selected"</span>
                <div data-rs-showcase-preview-row="">
                    <RadioGroup>
                        <RadioGroupItem value="sm" name="size">"Small"</RadioGroupItem>
                        <RadioGroupItem value="md" name="size" selected=SelectionState::Selected>"Medium"</RadioGroupItem>
                        <RadioGroupItem value="lg" name="size">"Large"</RadioGroupItem>
                    </RadioGroup>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <RadioGroup disabled=DisabledState::Disabled>
                        <RadioGroupItem value="a" name="dis">"Option A"</RadioGroupItem>
                        <RadioGroupItem value="b" name="dis">"Option B"</RadioGroupItem>
                    </RadioGroup>
                </div>
            </div>
        </div>
    }
}
