//! @canon-level: strict
//! @canon-owner: primitives-team
//! VirtualList Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn VirtualListPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-virtual-list=""
            data-rs-uid=crate::infra::uid::generate("vl")
            data-rs-interaction="data"
            data-rs-component="VirtualList"
            role="list"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn VirtualListViewportPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] aria_label: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-virtual-list-viewport=""
            role="presentation"
            aria-label=aria_label
            tabindex="0"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn VirtualListContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-virtual-list-content=""
            role="presentation"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn VirtualListItemPrimitive(
    children: Children,
    #[prop(default = 0usize)] index: usize,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-virtual-list-item=""
            data-rs-index=index.to_string()
            role="listitem"
            aria-setsize="-1"
            aria-posinset={(index + 1).to_string()}
            class=class
        >
            {children()}
        </div>
    }
}
