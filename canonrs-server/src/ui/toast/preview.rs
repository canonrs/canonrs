use leptos::prelude::*;
use super::toast_island::{ToastIsland, ToastIslandVariant};

#[component]
pub fn ToastShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <ToastIsland title="Notification" description="Your settings have been saved." duration_ms=60000 />
                    <ToastIsland variant=ToastIslandVariant::Success title="Success" description="File uploaded." duration_ms=60000 />
                    <ToastIsland variant=ToastIslandVariant::Error title="Error" description="Something went wrong." duration_ms=60000 />
                    <ToastIsland variant=ToastIslandVariant::Warning title="Warning" description="Session expires soon." duration_ms=60000 />
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Variant enforces correct role and aria-live automatically."
            </p>
        </div>
    }
}
