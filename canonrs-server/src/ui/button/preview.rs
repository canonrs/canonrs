use leptos::prelude::*;
use super::button_ui::{Button, ButtonVariant, ButtonSize};
use canonrs_core::DisabledState;
use canonrs_core::primitives::{ButtonGroupPrimitive, ButtonPrimitive, ButtonVariant as CoreVariant};
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
                    <Button variant=ButtonVariant::Primary disabled=DisabledState::Disabled>"Disabled"</Button>
                    <ButtonPrimitive variant=CoreVariant::Ghost attr:data-rs-state="hover">"Ghost Hover"</ButtonPrimitive>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Validation"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonPrimitive variant=CoreVariant::Primary attr:data-rs-validation="error">"Error"</ButtonPrimitive>
                    <ButtonPrimitive variant=CoreVariant::Primary attr:data-rs-validation="warning">"Warning"</ButtonPrimitive>
                    <ButtonPrimitive variant=CoreVariant::Primary attr:data-rs-validation="success">"Success"</ButtonPrimitive>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Button Group — detached"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroupPrimitive aria_label="Group detached">
                        <ButtonPrimitive variant=CoreVariant::Primary>"One"</ButtonPrimitive>
                        <ButtonPrimitive variant=CoreVariant::Primary>"Two"</ButtonPrimitive>
                        <ButtonPrimitive variant=CoreVariant::Primary>"Three"</ButtonPrimitive>
                    </ButtonGroupPrimitive>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Button Group — attached"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroupPrimitive attached=ToggleState::On aria_label="Group attached">
                        <ButtonPrimitive variant=CoreVariant::Primary>"One"</ButtonPrimitive>
                        <ButtonPrimitive variant=CoreVariant::Primary>"Two"</ButtonPrimitive>
                        <ButtonPrimitive variant=CoreVariant::Primary>"Three"</ButtonPrimitive>
                    </ButtonGroupPrimitive>
                </div>
            </div>

        </div>
    }
}
