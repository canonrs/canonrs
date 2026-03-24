//! @canon-level: strict
//! @canon-owner: primitives-team
//! FormErrorSummary Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn FormErrorSummaryPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-form-error-summary=""
            data-rs-state="closed"
            role="alert"
            aria-live="polite"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}
