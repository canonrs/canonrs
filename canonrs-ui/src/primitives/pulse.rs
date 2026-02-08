//! @canon-level: strict
//! @canon-owner: primitives-team
//! Pulse Primitive - CSS animation indicator

use leptos::prelude::*;

#[component]
pub fn PulsePrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <span
            data-pulse=""
            attr:aria-hidden="true"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </span>
    }
}
