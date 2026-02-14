use leptos::prelude::*;
use super::InteractiveToast;
use canonrs_ui::ui::toast::ToastVariant;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div style="display: flex; flex-direction: column; gap: 1rem;">
            <InteractiveToast
                title="Manual Dismiss".to_string()
                description="Click X to close this toast.".to_string()
                variant=ToastVariant::Default
            />

            <InteractiveToast
                title="Auto Dismiss (3s)".to_string()
                description="This toast will auto-close in 3 seconds.".to_string()
                variant=ToastVariant::Success
                auto_dismiss_ms=3000
            />

            <InteractiveToast
                title="Auto Dismiss (5s)".to_string()
                description="Warning that auto-closes in 5 seconds.".to_string()
                variant=ToastVariant::Warning
                auto_dismiss_ms=5000
            />
        </div>
    }
}
