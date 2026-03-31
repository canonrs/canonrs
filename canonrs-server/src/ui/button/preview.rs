use leptos::prelude::*;
use super::button_ui::{Button, ButtonVariant, ButtonSize};
use canonrs_core::DisabledState;

#[component]
pub fn ButtonPreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Button variant=ButtonVariant::Primary size=ButtonSize::Lg>
                    "Confirm Action"
                </Button>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Cannot drift. Cannot break. Cannot diverge."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="">
                    <Button variant=ButtonVariant::Primary>"Primary"</Button>
                    <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                    <Button variant=ButtonVariant::Outline>"Outline"</Button>
                    <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                    <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
                    <Button variant=ButtonVariant::Link>"Link"</Button>
                    <Button variant=ButtonVariant::Default>"Default"</Button>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="">
                    <Button variant=ButtonVariant::Primary size=ButtonSize::Xs>"Xs"</Button>
                    <Button variant=ButtonVariant::Primary size=ButtonSize::Sm>"Sm"</Button>
                    <Button variant=ButtonVariant::Primary size=ButtonSize::Md>"Md"</Button>
                    <Button variant=ButtonVariant::Primary size=ButtonSize::Lg>"Lg"</Button>
                    <Button variant=ButtonVariant::Primary size=ButtonSize::Xl>"Xl"</Button>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="">
                    <Button variant=ButtonVariant::Primary>"Default"</Button>
                    <Button variant=ButtonVariant::Primary disabled=DisabledState::Disabled>"Disabled"</Button>
                </div>
            </div>
        </div>
    }
}
