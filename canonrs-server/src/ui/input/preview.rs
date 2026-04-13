use leptos::prelude::*;
use super::input_boundary::Input;
use canonrs_core::primitives::{InputVariant, InputSize};
use canonrs_core::meta::DisabledState;
use canonrs_core::primitives::layout::grid::{GridPrimitive as Grid, GridCols};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn InputShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Input placeholder="Type something..." />
            <p data-rs-showcase-preview-anchor="">
                "Variant and size strictly constrained via typed enums."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <Grid cols=GridCols::Two>
                    <Stack direction=StackDirection::Vertical gap=StackGap::Xs>
                        <span data-rs-showcase-preview-label="">"Default"</span>
                        <Input placeholder="Default" variant=InputVariant::Default />
                    </Stack>
                    <Stack direction=StackDirection::Vertical gap=StackGap::Xs>
                        <span data-rs-showcase-preview-label="">"Success"</span>
                        <Input placeholder="Success" variant=InputVariant::Success />
                    </Stack>
                    <Stack direction=StackDirection::Vertical gap=StackGap::Xs>
                        <span data-rs-showcase-preview-label="">"Warning"</span>
                        <Input placeholder="Warning" variant=InputVariant::Warning />
                    </Stack>
                    <Stack direction=StackDirection::Vertical gap=StackGap::Xs>
                        <span data-rs-showcase-preview-label="">"Error"</span>
                        <Input placeholder="Error" variant=InputVariant::Error />
                    </Stack>
                </Grid>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Sizes"</span>
                <Grid cols=GridCols::Three>
                    <Stack direction=StackDirection::Vertical gap=StackGap::Xs>
                        <span data-rs-showcase-preview-label="">"Small"</span>
                        <Input placeholder="Small" size=InputSize::Sm />
                    </Stack>
                    <Stack direction=StackDirection::Vertical gap=StackGap::Xs>
                        <span data-rs-showcase-preview-label="">"Medium"</span>
                        <Input placeholder="Medium" size=InputSize::Md />
                    </Stack>
                    <Stack direction=StackDirection::Vertical gap=StackGap::Xs>
                        <span data-rs-showcase-preview-label="">"Large"</span>
                        <Input placeholder="Large" size=InputSize::Lg />
                    </Stack>
                </Grid>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Disabled"</span>
                <Input placeholder="Disabled" disabled=DisabledState::Disabled />
            </Stack>
        </Stack>
    }
}
