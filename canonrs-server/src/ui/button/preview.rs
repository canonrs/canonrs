use leptos::prelude::*;
use super::button_boundary::{Button, ButtonVariant, ButtonSize};
use crate::ui::button_group::button_group_boundary::ButtonGroup;
use canonrs_core::ToggleState;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn ButtonPreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Button variant=ButtonVariant::Primary size=ButtonSize::Lg>"Confirm Action"</Button>
            <p data-rs-showcase-preview-anchor="">"Cannot drift. Cannot break. Cannot diverge."</p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Sm>
                    <Button variant=ButtonVariant::Primary>"Primary"</Button>
                    <Button variant=ButtonVariant::Secondary>"Secondary"</Button>
                    <Button variant=ButtonVariant::Outline>"Outline"</Button>
                    <Button variant=ButtonVariant::Ghost>"Ghost"</Button>
                    <Button variant=ButtonVariant::Destructive>"Destructive"</Button>
                    <Button variant=ButtonVariant::Link>"Link"</Button>
                    <Button variant=ButtonVariant::Default>"Default"</Button>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Sm>
                    <Button variant=ButtonVariant::Primary size=ButtonSize::Xs>"Xs"</Button>
                    <Button variant=ButtonVariant::Primary size=ButtonSize::Sm>"Sm"</Button>
                    <Button variant=ButtonVariant::Primary size=ButtonSize::Md>"Md"</Button>
                    <Button variant=ButtonVariant::Primary size=ButtonSize::Lg>"Lg"</Button>
                    <Button variant=ButtonVariant::Primary size=ButtonSize::Xl>"Xl"</Button>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"States"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Sm>
                    <Button variant=ButtonVariant::Primary>"Default"</Button>
                    <Button variant=ButtonVariant::Primary disabled=true>"Disabled"</Button>
                    <Button variant=ButtonVariant::Ghost attr:data-rs-state="hover">"Ghost Hover"</Button>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Validation"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Sm>
                    <Button variant=ButtonVariant::Primary validation="error">"Error"</Button>
                    <Button variant=ButtonVariant::Primary validation="warning">"Warning"</Button>
                    <Button variant=ButtonVariant::Primary validation="success">"Success"</Button>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Button Group — detached"</span>
                <ButtonGroup aria_label="Group detached">
                    <Button variant=ButtonVariant::Primary>"One"</Button>
                    <Button variant=ButtonVariant::Primary>"Two"</Button>
                    <Button variant=ButtonVariant::Primary>"Three"</Button>
                </ButtonGroup>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Button Group — attached"</span>
                <ButtonGroup attached=ToggleState::On aria_label="Group attached">
                    <Button variant=ButtonVariant::Primary>"One"</Button>
                    <Button variant=ButtonVariant::Primary>"Two"</Button>
                    <Button variant=ButtonVariant::Primary>"Three"</Button>
                </ButtonGroup>
            </Stack>
        </Stack>
    }
}
