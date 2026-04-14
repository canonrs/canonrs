//! @canon-level: strict
//! @canon-owner: primitives-team
//! ListItem Primitives

use leptos::prelude::*;

#[component]
pub fn ListPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ul
            data-rs-list=""
            class=class
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn ListItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <li data-rs-list-item=""
            data-rs-uid=crate::infra::uid::generate("li")
            data-rs-interaction="data"
            data-rs-interaction="data" class=class>
            {children()}
        </li>
    }
}

#[component]
pub fn ListItemTitlePrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-list-item-title="" class=class>
            {children()}
        </span>
    }
}

#[component]
pub fn ListItemDescriptionPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <span data-rs-list-item-description="" class=class>
            {children()}
        </span>
    }
}
