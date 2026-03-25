//! @canon-level: strict
//! @canon-owner: primitives-team
//! Label Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn LabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] for_id: String,
) -> impl IntoView {
    view! {
        <label
            data-rs-label=""
            for=if for_id.is_empty() { None } else { Some(for_id) }
            class=class
        >
            {children()}
        </label>
    }
}
