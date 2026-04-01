use leptos::prelude::*;
use super::input_ui::Input;
use canonrs_core::primitives::{InputVariant, InputSize};
use canonrs_core::meta::DisabledState;

#[component]
pub fn InputShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Input placeholder="Type something..." />
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Variant and size strictly constrained via typed enums."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <Input placeholder="Default" variant=InputVariant::Default />
                    <Input placeholder="Success" variant=InputVariant::Success />
                    <Input placeholder="Warning" variant=InputVariant::Warning />
                    <Input placeholder="Error" variant=InputVariant::Error />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="" style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <Input placeholder="Small" size=InputSize::Sm />
                    <Input placeholder="Medium" size=InputSize::Md />
                    <Input placeholder="Large" size=InputSize::Lg />
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <div data-rs-showcase-preview-row="">
                    <Input placeholder="Disabled" disabled=DisabledState::Disabled />
                </div>
            </div>
        </div>
    }
}
