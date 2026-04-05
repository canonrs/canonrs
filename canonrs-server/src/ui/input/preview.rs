use leptos::prelude::*;
use super::input_ui::Input;
use super::input_island::InputIsland;
use canonrs_core::primitives::{InputVariant, InputSize};
use canonrs_core::meta::DisabledState;

#[component]
pub fn InputShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <InputIsland placeholder="Type something..." />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Variant and size strictly constrained via typed enums."
            </p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-preview-dev-grid="">
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Default"</span>
                        <Input placeholder="Default" variant=InputVariant::Default />
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Success"</span>
                        <Input placeholder="Success" variant=InputVariant::Success />
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Warning"</span>
                        <Input placeholder="Warning" variant=InputVariant::Warning />
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Error"</span>
                        <Input placeholder="Error" variant=InputVariant::Error />
                    </div>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-preview-dev-grid="">
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Small"</span>
                        <Input placeholder="Small" size=InputSize::Sm />
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Medium"</span>
                        <Input placeholder="Medium" size=InputSize::Md />
                    </div>
                    <div data-rs-showcase-preview-section="">
                        <span data-rs-showcase-preview-label="">"Large"</span>
                        <Input placeholder="Large" size=InputSize::Lg />
                    </div>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <Input placeholder="Disabled" disabled=DisabledState::Disabled />
            </div>

        </div>
    }
}
