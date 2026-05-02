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
    let uid_ig = crate::infra::uid::generate("ig");
    view! {
        <div
            data-rs-input-group=""
            data-rs-uid=uid_ig
            data-rs-interaction="init"
            role="group"
            data-rs-state={if merge_radius == ToggleState::On { Some("merge-radius") } else { None }}
            class=class
            node_ref=node_ref.unwrap_or_default()
        >
            {children()}
        </div>
    }
}

#[component]
pub fn InputGroupPrefix(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_igp = crate::infra::uid::generate("igp");
    view! {
        <div
            data-rs-input-group-prefix=""

                        data-rs-uid=uid_igp
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn InputGroupSuffix(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_igs = crate::infra::uid::generate("igs");
    view! {
        <div
            data-rs-input-group-suffix=""

                        data-rs-uid=uid_igs
            class=class
        >
            {children()}
        </div>
    }
}
