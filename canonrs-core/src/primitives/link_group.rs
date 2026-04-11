//! LinkGroup Primitive

use leptos::prelude::*;

#[component]
pub fn LinkGroupPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <nav
            data-rs-link-group=""
            data-rs-uid=crate::infra::uid::generate("lg")
            data-rs-interaction="nav"
            data-rs-component="LinkGroup"
            data-rs-behavior="navigation"
            class=class
        >
            {children()}
        </nav>
    }
}

#[component]
pub fn LinkGroupLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-link-group-label="" class=class>
            {children()}
        </div>
    }
}
