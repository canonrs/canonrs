use leptos::prelude::*;
use super::aspect_ratio_boundary::AspectRatio;
use canonrs_core::primitives::layout::grid::{GridPrimitive as Grid, GridCols};

#[component]
pub fn AspectRatioShowcasePreview() -> impl IntoView {
    view! {
        <Grid cols=GridCols::One>
            <AspectRatio ratio_w=16.0f32 ratio_h=9.0f32>
                <div data-rs-aspect-demo="">"16 / 9"</div>
            </AspectRatio>
            <p data-rs-showcase-preview-anchor="">
                "Aspect ratio enforced structurally with no layout drift."
            </p>
            <span data-rs-showcase-preview-label="">"Ratios"</span>
            <Grid cols=GridCols::Three>
                <AspectRatio ratio_w=4.0f32  ratio_h=3.0f32><div data-rs-aspect-demo="">"4 / 3"</div></AspectRatio>
                <AspectRatio ratio_w=1.0f32  ratio_h=1.0f32><div data-rs-aspect-demo="">"1 / 1"</div></AspectRatio>
                <AspectRatio ratio_w=21.0f32 ratio_h=9.0f32><div data-rs-aspect-demo="">"21 / 9"</div></AspectRatio>
            </Grid>
        </Grid>
    }
}
