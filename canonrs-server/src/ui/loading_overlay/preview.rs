use leptos::prelude::*;
use super::loading_overlay_boundary::LoadingOverlay;
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn LoadingOverlayShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <LoadingOverlay state="loading">
                <div data-rs-loading-demo="">
                    <span>"Título do card"</span>
                    <span>"Descrição do conteúdo"</span>
                </div>
            </LoadingOverlay>
            <p data-rs-showcase-preview-anchor="">
                "Loading visibility and aria-busy managed automatically."
            </p>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"States"</span>
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <LoadingOverlay>
                        <div data-rs-loading-demo="">"Idle — conteúdo visível"</div>
                    </LoadingOverlay>
                    <LoadingOverlay state="loading">
                        <div data-rs-loading-demo="">"Loading — conteúdo bloqueado"</div>
                    </LoadingOverlay>
                </Stack>
            </Stack>
            <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                <span data-rs-showcase-preview-label="">"Modes"</span>
                <Stack direction=StackDirection::Vertical gap=StackGap::Sm>
                    <LoadingOverlay state="loading" mode="blocking">
                        <div data-rs-loading-demo="">"Blocking"</div>
                    </LoadingOverlay>
                    <LoadingOverlay state="loading" mode="subtle">
                        <div data-rs-loading-demo="">"Transparent"</div>
                    </LoadingOverlay>
                </Stack>
            </Stack>
        </Stack>
    }
}
