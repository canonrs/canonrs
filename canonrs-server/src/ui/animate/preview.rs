use leptos::prelude::*;
use super::animate_boundary::Animate;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};
use canonrs_core::primitives::layout::grid::{GridPrimitive as Grid, GridCols};

#[component]
pub fn AnimateShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <Animate animation="fade-in" duration="1.2s">
                <div data-rs-animate-demo="">"Fade In"</div>
            </Animate>
            <p data-rs-showcase-preview-anchor="">
                "Animation type and easing enforced through typed enums. Respects prefers-reduced-motion."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Animation variants"</span>
                <Grid cols=GridCols::Three>
                    <Animate animation="fade-in"   duration="1.4s"><div data-rs-animate-demo="">"FadeIn"</div></Animate>
                    <Animate animation="fade-out"  duration="1.4s"><div data-rs-animate-demo="">"FadeOut"</div></Animate>
                    <Animate animation="slide-in"  duration="1.4s"><div data-rs-animate-demo="">"SlideIn"</div></Animate>
                    <Animate animation="slide-out" duration="1.4s"><div data-rs-animate-demo="">"SlideOut"</div></Animate>
                    <Animate animation="scale-in"  duration="1.4s"><div data-rs-animate-demo="">"ScaleIn"</div></Animate>
                    <Animate animation="scale-out" duration="1.4s"><div data-rs-animate-demo="">"ScaleOut"</div></Animate>
                </Grid>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Easing"</span>
                <Grid cols=GridCols::Four>
                    <Animate animation="slide-in" easing="ease-in"     duration="1.6s"><div data-rs-animate-demo="">"EaseIn"</div></Animate>
                    <Animate animation="slide-in" easing="ease-out"    duration="1.6s"><div data-rs-animate-demo="">"EaseOut"</div></Animate>
                    <Animate animation="slide-in" easing="ease-in-out" duration="1.6s"><div data-rs-animate-demo="">"EaseInOut"</div></Animate>
                    <Animate animation="slide-in" easing="linear"      duration="1.6s"><div data-rs-animate-demo="">"Linear"</div></Animate>
                </Grid>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Stagger"</span>
                <Animate animation="fade-in" duration="0.6s" stagger=100.0>
                    <div data-rs-animate-demo="">"Item 1"</div>
                    <div data-rs-animate-demo="">"Item 2"</div>
                    <div data-rs-animate-demo="">"Item 3"</div>
                </Animate>
            </Stack>
        </Stack>
    }
}
