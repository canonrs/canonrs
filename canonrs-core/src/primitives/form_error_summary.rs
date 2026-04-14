//! @canon-level: strict
//! @canon-owner: primitives-team
//! FormErrorSummary Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn FormErrorSummaryPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-form-error-summary=""
            data-rs-uid=crate::infra::uid::generate("fes")
            role="alert"
            aria-live="assertive"
            aria-atomic="true"
            tabindex="-1"
            class=class
        >
            {children()}
        </div>
    }
}
