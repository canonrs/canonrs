//! LinkGroup Primitive

use leptos::prelude::*;

#[component]
pub fn LinkGroupPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_lg = crate::infra::uid::generate("lg");
    view! {
        <nav
            data-rs-link-group=""
            data-rs-uid=uid_lg
            data-rs-interaction="nav"
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
