use leptos::prelude::*;
use super::loading_overlay_boundary::LoadingOverlay;
use crate::blocks::card::CardBlock;
use crate::ui::card::{CardHeader, CardTitle, CardContent};
use canonrs_core::slot;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn LoadingOverlayShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <CardBlock
                header=slot!(|| view! {
                    <CardHeader><CardTitle>"Loading"</CardTitle></CardHeader>
                }.into_any())
                content=slot!(|| view! {
                    <CardContent>
                        <LoadingOverlay state="loading">
                            <div data-rs-loading-demo="">
                                <span>"Card title"</span>
                                <span>"Card description content"</span>
                            </div>
                        </LoadingOverlay>
                    </CardContent>
                }.into_any())
            />
            <p data-rs-showcase-preview-anchor="">
                "Loading visibility and aria-busy managed automatically."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"States"</span>
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <CardBlock content=slot!(|| view! {
                        <CardContent>
                            <LoadingOverlay>"Idle — content visible"</LoadingOverlay>
                        </CardContent>
                    }.into_any()) />
                    <CardBlock content=slot!(|| view! {
                        <CardContent>
                            <LoadingOverlay state="loading">"Loading — content blocked"</LoadingOverlay>
                        </CardContent>
                    }.into_any()) />
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Modes"</span>
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <CardBlock content=slot!(|| view! {
                        <CardContent>
                            <LoadingOverlay state="loading" mode="blocking">"Blocking"</LoadingOverlay>
                        </CardContent>
                    }.into_any()) />
                    <CardBlock content=slot!(|| view! {
                        <CardContent>
                            <LoadingOverlay state="loading" mode="subtle">"Subtle"</LoadingOverlay>
                        </CardContent>
                    }.into_any()) />
                </Stack>
            </Stack>
        </Stack>
    }
}
