//! @canon-level: strict
//! @canon-owner: primitives-team
//! FormErrorSummary Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn FormErrorSummaryPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-form-error-summary=""
            role="alert"
            aria-live="polite"
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children.map(|c| c())}
        </div>
    }
}
