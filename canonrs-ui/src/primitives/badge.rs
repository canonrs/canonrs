//! @canon-level: strict
//! @canon-owner: primitives-team
//! Badge Primitive - HTML puro

use leptos::prelude::*;

#[component]
pub fn BadgePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-badge=""
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </span>
    }
}
