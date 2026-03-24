//! @canon-level: strict
//! @canon-owner: primitives-team
//! VirtualList Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn VirtualListPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-virtual-list=""
            role="list"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn VirtualListViewportPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-virtual-list-viewport=""
            role="presentation"
            aria-label="Scrollable content"
            tabindex="0"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn VirtualListContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-virtual-list-content=""
            role="presentation"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn VirtualListItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = 0usize)] index: usize,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-virtual-list-item=""
            data-rs-index=index.to_string()
            role="listitem"
            aria-setsize="-1"
            aria-posinset={(index + 1).to_string()}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}
