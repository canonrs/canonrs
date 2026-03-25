//! @canon-level: strict
//! List Primitives - Accessible list container + items

use leptos::prelude::*;

#[component]
pub fn ListPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-list=""
            role="listbox"
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ListItemPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-list-item=""
            role="option"
            class={class}
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ListItemTitlePrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-list-item-title="" class={class}>
            {children()}
        </div>
    }
}

#[component]
pub fn ListItemDescriptionPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-list-item-description="" class={class}>
            {children()}
        </div>
    }
}
