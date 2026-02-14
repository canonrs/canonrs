use leptos::prelude::*;
use super::{Toast, ToastTitle, ToastDescription, ToastClose, ToastVariant};

/// BasicExample - Stateless structure demonstration
/// Shows all variants without interactive behavior
#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <Toast variant=ToastVariant::Default open=true>
                <ToastTitle>"Notification"</ToastTitle>
                <ToastDescription>"Your settings have been updated."</ToastDescription>
                <ToastClose>"×"</ToastClose>
            </Toast>

            <Toast variant=ToastVariant::Success open=true>
                <ToastTitle>"Success"</ToastTitle>
                <ToastDescription>"Your changes have been saved successfully."</ToastDescription>
                <ToastClose>"×"</ToastClose>
            </Toast>

            <Toast variant=ToastVariant::Warning open=true>
                <ToastTitle>"Warning"</ToastTitle>
                <ToastDescription>"Your session will expire in 5 minutes."</ToastDescription>
                <ToastClose>"×"</ToastClose>
            </Toast>

            <Toast variant=ToastVariant::Error open=true>
                <ToastTitle>"Error"</ToastTitle>
                <ToastDescription>"Failed to save changes. Please try again."</ToastDescription>
                <ToastClose>"×"</ToastClose>
            </Toast>

            <Toast variant=ToastVariant::Info open=true>
                <ToastTitle>"Update Available"</ToastTitle>
                <ToastDescription>"A new version is available. Click to update."</ToastDescription>
                <ToastClose>"×"</ToastClose>
            </Toast>
        </div>
    }
}
