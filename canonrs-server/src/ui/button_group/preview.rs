use leptos::prelude::*;
use super::button_group_boundary::ButtonGroup;
use canonrs_core::ToggleState;
use crate::ui::button::button_boundary::Button;
use canonrs_core::primitives::{ButtonVariant, ButtonSize, ButtonStateHint};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn ButtonGroupShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <ButtonGroup attached=ToggleState::On aria_label="Actions">
                <Button variant=ButtonVariant::Secondary size=ButtonSize::Md>"Left"</Button>
                <Button variant=ButtonVariant::Secondary size=ButtonSize::Md>"Center"</Button>
                <Button variant=ButtonVariant::Secondary size=ButtonSize::Md>"Right"</Button>
            </ButtonGroup>
            <p data-rs-showcase-preview-anchor="">"Grouped actions. One contract."</p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Detached"</span>
                <ButtonGroup>
                    <Button variant=ButtonVariant::Secondary>"A"</Button>
                    <Button variant=ButtonVariant::Secondary>"B"</Button>
                    <Button variant=ButtonVariant::Secondary>"C"</Button>
                </ButtonGroup>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Attached"</span>
                <ButtonGroup attached=ToggleState::On>
                    <Button variant=ButtonVariant::Primary>"Save"</Button>
                    <Button variant=ButtonVariant::Secondary>"Cancel"</Button>
                </ButtonGroup>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"First/last radius"</span>
                <ButtonGroup attached=ToggleState::On aria_label="First last demo">
                    <Button variant=ButtonVariant::Secondary state_hint=ButtonStateHint::First>"First"</Button>
                    <Button variant=ButtonVariant::Secondary>"Middle"</Button>
                    <Button variant=ButtonVariant::Secondary state_hint=ButtonStateHint::Last>"Last"</Button>
                </ButtonGroup>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Hover z-index"</span>
                <ButtonGroup attached=ToggleState::On aria_label="Hover z-index demo">
                    <Button variant=ButtonVariant::Secondary>"One"</Button>
                    <Button variant=ButtonVariant::Secondary state_hint=ButtonStateHint::Hover>"Hover"</Button>
                    <Button variant=ButtonVariant::Secondary>"Three"</Button>
                </ButtonGroup>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Focus z-index"</span>
                <ButtonGroup attached=ToggleState::On aria_label="Focus z-index demo">
                    <Button variant=ButtonVariant::Secondary>"One"</Button>
                    <Button variant=ButtonVariant::Secondary state_hint=ButtonStateHint::Focus>"Focus"</Button>
                    <Button variant=ButtonVariant::Secondary>"Three"</Button>
                </ButtonGroup>
            </Stack>
        </Stack>
    }
}
