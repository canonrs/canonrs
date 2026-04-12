use leptos::prelude::*;
use super::boundary::Toast;
use canonrs_core::primitives::ToastVariant;

#[component]
pub fn ToastShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <div style="display:flex;flex-direction:column;gap:var(--space-sm);">
                    <Toast title="Notification" description="Your settings have been saved." duration_ms=60000 />
                    <Toast variant=ToastVariant::Success title="Success" description="File uploaded." duration_ms=60000 />
                    <Toast variant=ToastVariant::Error title="Error" description="Something went wrong." duration_ms=60000 />
                    <Toast variant=ToastVariant::Warning title="Warning" description="Session expires soon." duration_ms=60000 />
                </div>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Variant enforces correct role and aria-live automatically."
            </p>
        </div>
    }
}
