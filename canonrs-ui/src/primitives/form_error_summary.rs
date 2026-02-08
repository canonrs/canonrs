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
            attr:data-form-error-summary=""
            role="alert"
            attr:aria-live="polite"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}
