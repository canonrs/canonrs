use leptos::prelude::*;
use super::boundary::ButtonGroup;
use canonrs_core::ToggleState;
use crate::ui::button::Button;
use canonrs_core::primitives::{ButtonVariant, ButtonSize, ButtonStateHint};

#[component]
pub fn ButtonGroupShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ButtonGroup attached=ToggleState::On aria_label="Actions">
                    <Button variant=ButtonVariant::Secondary size=ButtonSize::Md>"Left"</Button>
                    <Button variant=ButtonVariant::Secondary size=ButtonSize::Md>"Center"</Button>
                    <Button variant=ButtonVariant::Secondary size=ButtonSize::Md>"Right"</Button>
                </ButtonGroup>
            </div>
            <p data-rs-showcase-preview-anchor="">"Grouped actions. One contract."</p>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Detached"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroup>
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
                        <Button variant=ButtonVariant::Secondary state_hint=ButtonStateHint::First>"First"</Button>
                        <Button variant=ButtonVariant::Secondary>"Middle"</Button>
                        <Button variant=ButtonVariant::Secondary state_hint=ButtonStateHint::Last>"Last"</Button>
                    </ButtonGroup>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Attached — hover z-index"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroup attached=ToggleState::On aria_label="Hover z-index demo">
                        <Button variant=ButtonVariant::Secondary>"One"</Button>
                        <Button variant=ButtonVariant::Secondary state_hint=ButtonStateHint::Hover>"Hover"</Button>
                        <Button variant=ButtonVariant::Secondary>"Three"</Button>
                    </ButtonGroup>
                </div>
            </div>

            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Attached — focus z-index"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroup attached=ToggleState::On aria_label="Focus z-index demo">
                        <Button variant=ButtonVariant::Secondary>"One"</Button>
                        <Button variant=ButtonVariant::Secondary state_hint=ButtonStateHint::Focus>"Focus"</Button>
                        <Button variant=ButtonVariant::Secondary>"Three"</Button>
                    </ButtonGroup>
                </div>
            </div>

        </div>
    }
}
