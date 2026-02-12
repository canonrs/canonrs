use leptos::prelude::*;
use super::ToastClose;

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <div class="toast" style="padding: 1rem; background: #333; color: white; border-radius: 0.5rem;">
            <p>"This is a toast notification"</p>
            <ToastClose>"×"</ToastClose>
        </div>
    }
}

#[component]
pub fn success_example() -> impl IntoView {
    view! {
        <div class="toast toast-success" style="padding: 1rem; background: #10b981; color: white; border-radius: 0.5rem;">
            <p>"Success! Your changes have been saved."</p>
            <ToastClose>"×"</ToastClose>
        </div>
    }
}

#[component]
pub fn error_example() -> impl IntoView {
    view! {
        <div class="toast toast-error" style="padding: 1rem; background: #ef4444; color: white; border-radius: 0.5rem;">
            <p>"Error! Something went wrong."</p>
            <ToastClose>"×"</ToastClose>
        </div>
    }
}
