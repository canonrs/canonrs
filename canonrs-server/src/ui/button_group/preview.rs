use leptos::prelude::*;
use super::button_group_ui::ButtonGroup;
use crate::ui::button::button_ui::{Button, ButtonVariant, ButtonSize};
use canonrs_core::ToggleState;
use canonrs_core::primitives::{ButtonPrimitive, ButtonVariant as CoreVariant};

#[component]
pub fn ButtonGroupShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ButtonGroup attached=ToggleState::On aria_label="Actions".to_string()>
                    <Button variant=ButtonVariant::Secondary size=ButtonSize::Md>"Left"</Button>
                    <Button variant=ButtonVariant::Secondary size=ButtonSize::Md>"Center"</Button>
                    <Button variant=ButtonVariant::Secondary size=ButtonSize::Md>"Right"</Button>
                </ButtonGroup>
            </div>
            <p data-rs-showcase-preview-anchor="">"Grouped actions. One contract."</p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Detached"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroup attached=ToggleState::Off>
                        <Button variant=ButtonVariant::Secondary>"A"</Button>
                        <Button variant=ButtonVariant::Secondary>"B"</Button>
                        <Button variant=ButtonVariant::Secondary>"C"</Button>
                    </ButtonGroup>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Attached"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroup attached=ToggleState::On>
                        <Button variant=ButtonVariant::Primary>"Save"</Button>
                        <Button variant=ButtonVariant::Secondary>"Cancel"</Button>
                    </ButtonGroup>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Attached — first/last radius"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroup attached=ToggleState::On aria_label="First last demo">
                        <ButtonPrimitive variant=CoreVariant::Secondary attr:data-rs-state="first">"First"</ButtonPrimitive>
                        <ButtonPrimitive variant=CoreVariant::Secondary>"Middle"</ButtonPrimitive>
                        <ButtonPrimitive variant=CoreVariant::Secondary attr:data-rs-state="last">"Last"</ButtonPrimitive>
                    </ButtonGroup>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Attached — hover z-index"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroup attached=ToggleState::On aria_label="Hover z-index demo">
                        <ButtonPrimitive variant=CoreVariant::Secondary>"One"</ButtonPrimitive>
                        <ButtonPrimitive variant=CoreVariant::Secondary attr:data-rs-state="hover">"Hover"</ButtonPrimitive>
                        <ButtonPrimitive variant=CoreVariant::Secondary>"Three"</ButtonPrimitive>
                    </ButtonGroup>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Attached — focus z-index"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroup attached=ToggleState::On aria_label="Focus z-index demo">
                        <ButtonPrimitive variant=CoreVariant::Secondary>"One"</ButtonPrimitive>
                        <ButtonPrimitive variant=CoreVariant::Secondary attr:data-rs-state="focus">"Focus"</ButtonPrimitive>
                        <ButtonPrimitive variant=CoreVariant::Secondary>"Three"</ButtonPrimitive>
                    </ButtonGroup>
                </div>
            </div>

        </div>
    }
}
