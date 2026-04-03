use leptos::prelude::*;
use super::radio_ui::{Radio, RadioGroup, RadioGroupItem};
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
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-preview-dev-grid="">
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Unselected"</span>
                        <Radio value="a" name="state-demo">"Option A"</Radio>
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Selected"</span>
                        <Radio value="b" name="state-demo" selected=SelectionState::Selected>"Option B"</Radio>
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Disabled"</span>
                        <Radio value="c" name="state-demo" disabled=DisabledState::Disabled>"Option C"</Radio>
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Selected + Disabled"</span>
                        <Radio value="d" name="state-demo" selected=SelectionState::Selected disabled=DisabledState::Disabled>"Option D"</Radio>
                    </div>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Group — pre-selected"</span>
                <div data-rs-showcase-preview-row="">
                    <RadioGroup>
                        <RadioGroupItem value="sm" name="size">"Small"</RadioGroupItem>
                        <RadioGroupItem value="md" name="size" selected=SelectionState::Selected>"Medium"</RadioGroupItem>
                        <RadioGroupItem value="lg" name="size">"Large"</RadioGroupItem>
                    </RadioGroup>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Group — disabled"</span>
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
