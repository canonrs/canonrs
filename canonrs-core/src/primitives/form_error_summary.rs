//! @canon-level: strict
//! FormErrorSummary Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn FormErrorSummaryPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-form-error-summary=""
            role="alert"
            aria-live="polite"
            class=class
        >
            {children()}
        </div>
    }
}
