use leptos::prelude::*;
use super::button_group_ui::ButtonGroup;
use crate::ui::button::button_ui::{Button, ButtonVariant, ButtonSize};
use canonrs_core::ToggleState;

#[component]
pub fn ButtonGroupShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ButtonGroup attached=ToggleState::On aria_label="Actions".to_string()>
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Md>"Left"</Button>
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Md>"Center"</Button>
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Md>"Right"</Button>
                </ButtonGroup>
            </div>
            <p data-rs-showcase-preview-anchor="">"Grouped actions. One contract."</p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Detached"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroup attached=ToggleState::Off>
                        <Button variant=ButtonVariant::Outline>"A"</Button>
                        <Button variant=ButtonVariant::Outline>"B"</Button>
                        <Button variant=ButtonVariant::Outline>"C"</Button>
                    </ButtonGroup>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Attached"</span>
                <div data-rs-showcase-preview-row="">
                    <ButtonGroup attached=ToggleState::On>
                        <Button variant=ButtonVariant::Primary>"Save"</Button>
                        <Button variant=ButtonVariant::Outline>"Cancel"</Button>
                    </ButtonGroup>
                </div>
            </div>
        </div>
    }
}
