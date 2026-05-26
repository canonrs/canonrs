#![allow(unused_variables)]
//! @canon-level: strict
//! Icon Primitive - SVG wrapper

use leptos::prelude::*;

#[component]
pub fn IconPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(optional, into)] id: Option<String>,
) -> impl IntoView {
    let _uid_ico = crate::infra::uid::generate("ico");
    view! {
        <span data-rs-icon="" class={class} id=id>
        data-rs-uid=uid_ico
            {children()}
        </span>
    }
}

#[component]
pub fn IconInnerPrimitive(
    children: Children,
) -> impl IntoView {
    view! { <span data-rs-icon-inner="">{children()}</span> }
}
