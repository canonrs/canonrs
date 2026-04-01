use leptos::prelude::*;
use super::toast_ui::{
    Toast, ToastViewport, ToastTitle, ToastDescription,
    ToastAction, ToastClose,
};
use canonrs_core::primitives::ToastVariant;

#[component]
pub fn ToastShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <ToastViewport>
                    <Toast variant=ToastVariant::Default>
                        <ToastTitle>"Notification"</ToastTitle>
                        <ToastDescription>"Your settings have been saved successfully."</ToastDescription>
                        <ToastClose>"×"</ToastClose>
                    </Toast>
                </ToastViewport>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Variant enforces correct role and aria-live automatically."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Variants"</span>
                <div data-rs-showcase-preview-row="">
                    <ToastViewport>
                        <Toast variant=ToastVariant::Success>
                            <ToastTitle>"Success"</ToastTitle>
                            <ToastDescription>"File uploaded successfully."</ToastDescription>
                            <ToastClose>"×"</ToastClose>
                        </Toast>
                        <Toast variant=ToastVariant::Error>
                            <ToastTitle>"Error"</ToastTitle>
                            <ToastDescription>"Something went wrong."</ToastDescription>
                            <ToastClose>"×"</ToastClose>
                        </Toast>
                        <Toast variant=ToastVariant::Warning>
                            <ToastTitle>"Warning"</ToastTitle>
                            <ToastDescription>"Your session expires soon."</ToastDescription>
                            <ToastClose>"×"</ToastClose>
                        </Toast>
                    </ToastViewport>
                </div>
            </div>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With action"</span>
                <div data-rs-showcase-preview-row="">
                    <ToastViewport>
                        <Toast variant=ToastVariant::Default>
                            <ToastTitle>"Update available"</ToastTitle>
                            <ToastDescription>"A new version is ready to install."</ToastDescription>
                            <ToastAction aria_label="Install update">"Install"</ToastAction>
                            <ToastClose>"×"</ToastClose>
                        </Toast>
                    </ToastViewport>
                </div>
            </div>
        </div>
    }
}
