//! @canon-level: strict
//! InputGroup Primitive - HTML puro

use leptos::prelude::*;
use crate::meta::ToggleState;

#[component]
pub fn InputGroupPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = ToggleState::Off)] merge_radius: ToggleState,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
) -> impl IntoView {
    view! {
        <div
            data-rs-input-group=""
            data-rs-uid=crate::infra::uid::generate("ig")
            data-rs-interaction="init"
            data-rs-state={if merge_radius == ToggleState::On { Some("merge-radius") } else { None }}
            class=class
            node_ref=node_ref.unwrap_or_default()
        >
            {children()}
        </div>
    }
}
