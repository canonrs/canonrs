use leptos::prelude::*;
use super::boundary::Button;
use canonrs_core::primitives::{ButtonVariant, ButtonSize};
use canonrs_core::primitives::ButtonPrimitive;
use canonrs_core::primitives::ButtonVariant as CoreVariant;
use crate::ui::button_group::ButtonGroup;
use canonrs_core::ToggleState;

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
                    <Button variant=ButtonVariant::Primary disabled=true>"Disabled"</Button>
                    <ButtonPrimitive variant=CoreVariant::Ghost attr:data-rs-state="hover">"Ghost Hover"</ButtonPrimitive>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Validation"</span>
                <div data-rs-showcase-preview-row="">
                    <Button variant=ButtonVariant::Primary validation="error">"Error"</Button>
                    <Button variant=ButtonVariant::Primary validation="warning">"Warning"</Button>
                    <Button variant=ButtonVariant::Primary validation="success">"Success"</Button>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Button Group — detached"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroup aria_label="Group detached">
                        <Button variant=ButtonVariant::Primary>"One"</Button>
                        <Button variant=ButtonVariant::Primary>"Two"</Button>
                        <Button variant=ButtonVariant::Primary>"Three"</Button>
                    </ButtonGroup>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Button Group — attached"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroup attached=ToggleState::On aria_label="Group attached">
                        <Button variant=ButtonVariant::Primary>"One"</Button>
                        <Button variant=ButtonVariant::Primary>"Two"</Button>
                        <Button variant=ButtonVariant::Primary>"Three"</Button>
                    </ButtonGroup>
                </div>
            </div>

        </div>
    }
}
