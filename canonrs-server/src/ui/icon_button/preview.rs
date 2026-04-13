use leptos::prelude::*;
use super::icon_button_boundary::IconButton;
use canonrs_core::primitives::{IconButtonVariant, IconButtonSize};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};
use canonrs_core::primitives::layout::grid::{GridPrimitive as Grid, GridCols};

#[component]
pub fn IconButtonShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <IconButton aria_label="Close" variant=IconButtonVariant::Solid size=IconButtonSize::Xl>
                "✕"
            </IconButton>
            <p data-rs-showcase-preview-anchor()>
                "Accessibility and loading state enforced in button contract."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Sm>
                    <IconButton aria_label="Default"     variant=IconButtonVariant::Default>"★"</IconButton>
                    <IconButton aria_label="Ghost"       variant=IconButtonVariant::Ghost>"★"</IconButton>
                    <IconButton aria_label="Outline"     variant=IconButtonVariant::Outline>"★"</IconButton>
                    <IconButton aria_label="Solid"       variant=IconButtonVariant::Solid>"★"</IconButton>
                    <IconButton aria_label="Subtle"      variant=IconButtonVariant::Subtle>"★"</IconButton>
                    <IconButton aria_label="Destructive" variant=IconButtonVariant::Destructive>"★"</IconButton>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <Stack direction=StackDirection::Horizontal gap=StackGap::Sm>
                    <IconButton aria_label="Xs" size=IconButtonSize::Xs variant=IconButtonVariant::Solid>"★"</IconButton>
                    <IconButton aria_label="Sm" size=IconButtonSize::Sm variant=IconButtonVariant::Solid>"★"</IconButton>
                    <IconButton aria_label="Md" size=IconButtonSize::Md variant=IconButtonVariant::Solid>"★"</IconButton>
                    <IconButton aria_label="Lg" size=IconButtonSize::Lg variant=IconButtonVariant::Solid>"★"</IconButton>
                    <IconButton aria_label="Xl" size=IconButtonSize::Xl variant=IconButtonVariant::Solid>"★"</IconButton>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"States"</span>
                <Grid cols=GridCols::Four>
                    <Stack direction=StackDirection::Vertical gap=StackGap::Xs>
                        <span data-rs-showcase-preview-label="">"Default"</span>
                        <IconButton aria_label="Default" variant=IconButtonVariant::Solid>"★"</IconButton>
                    </Stack>
                    <Stack direction=StackDirection::Vertical gap=StackGap::Xs>
                        <span data-rs-showcase-preview-label="">"Pressed"</span>
                        <IconButton aria_label="Pressed" variant=IconButtonVariant::Solid pressed=true>"★"</IconButton>
                    </Stack>
                    <Stack direction=StackDirection::Vertical gap=StackGap::Xs>
                        <span data-rs-showcase-preview-label="">"Disabled"</span>
                        <IconButton aria_label="Disabled" variant=IconButtonVariant::Solid disabled=true>"★"</IconButton>
                    </Stack>
                    <Stack direction=StackDirection::Vertical gap=StackGap::Xs>
                        <span data-rs-showcase-preview-label="">"Loading"</span>
                        <IconButton aria_label="Loading" variant=IconButtonVariant::Solid loading=true>"★"</IconButton>
                    </Stack>
                </Grid>
            </Stack>
        </Stack>
    }
}
