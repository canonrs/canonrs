use leptos::prelude::*;
use super::progress_boundary::Progress;
use canonrs_core::primitives::progress::ProgressState;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn ProgressShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Value always clamped between 0-100 and ARIA-compliant."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Values"</span>
                <Progress value=0.0 />
                <Progress value=25.0 />
                <Progress value=50.0 />
                <Progress value=75.0 />
                <Progress value=100.0 />
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Indeterminate"</span>
                <Progress state=ProgressState::Indeterminate />
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Loading"</span>
                <Progress state=ProgressState::Loading />
            </Stack>
        </Stack>
    }
}
