use leptos::prelude::*;
use super::button_island::{ButtonIsland, ButtonVariant, ButtonSize};
use canonrs_core::primitives::ButtonPrimitive;
use canonrs_core::primitives::ButtonVariant as CoreVariant;
use crate::ui::button_group::button_group_island::{ButtonGroupIsland, ButtonGroupAttached};

#[component]
pub fn ButtonPreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ButtonIsland variant=ButtonVariant::Primary size=ButtonSize::Lg>
                    "Confirm Action"
                </ButtonIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Cannot drift. Cannot break. Cannot diverge."
            </p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonIsland variant=ButtonVariant::Primary>"Primary"</ButtonIsland>
                    <ButtonIsland variant=ButtonVariant::Secondary>"Secondary"</ButtonIsland>
                    <ButtonIsland variant=ButtonVariant::Outline>"Outline"</ButtonIsland>
                    <ButtonIsland variant=ButtonVariant::Ghost>"Ghost"</ButtonIsland>
                    <ButtonIsland variant=ButtonVariant::Destructive>"Destructive"</ButtonIsland>
                    <ButtonIsland variant=ButtonVariant::Link>"Link"</ButtonIsland>
                    <ButtonIsland variant=ButtonVariant::Default>"Default"</ButtonIsland>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonIsland variant=ButtonVariant::Primary size=ButtonSize::Xs>"Xs"</ButtonIsland>
                    <ButtonIsland variant=ButtonVariant::Primary size=ButtonSize::Sm>"Sm"</ButtonIsland>
                    <ButtonIsland variant=ButtonVariant::Primary size=ButtonSize::Md>"Md"</ButtonIsland>
                    <ButtonIsland variant=ButtonVariant::Primary size=ButtonSize::Lg>"Lg"</ButtonIsland>
                    <ButtonIsland variant=ButtonVariant::Primary size=ButtonSize::Xl>"Xl"</ButtonIsland>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"States"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonIsland variant=ButtonVariant::Primary>"Default"</ButtonIsland>
                    <ButtonIsland variant=ButtonVariant::Primary disabled=true>"Disabled"</ButtonIsland>
                    <ButtonPrimitive variant=CoreVariant::Ghost attr:data-rs-state="hover">"Ghost Hover"</ButtonPrimitive>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Validation"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonIsland variant=ButtonVariant::Primary validation="error">"Error"</ButtonIsland>
                    <ButtonIsland variant=ButtonVariant::Primary validation="warning">"Warning"</ButtonIsland>
                    <ButtonIsland variant=ButtonVariant::Primary validation="success">"Success"</ButtonIsland>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Button Group — detached"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroupIsland aria_label="Group detached">
                        <ButtonIsland variant=ButtonVariant::Primary>"One"</ButtonIsland>
                        <ButtonIsland variant=ButtonVariant::Primary>"Two"</ButtonIsland>
                        <ButtonIsland variant=ButtonVariant::Primary>"Three"</ButtonIsland>
                    </ButtonGroupIsland>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Button Group — attached"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroupIsland attached=ButtonGroupAttached::Attached aria_label="Group attached">
                        <ButtonIsland variant=ButtonVariant::Primary>"One"</ButtonIsland>
                        <ButtonIsland variant=ButtonVariant::Primary>"Two"</ButtonIsland>
                        <ButtonIsland variant=ButtonVariant::Primary>"Three"</ButtonIsland>
                    </ButtonGroupIsland>
                </div>
            </div>

        </div>
    }
}
