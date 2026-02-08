//! @canon-level: strict
//! @canon-owner: primitives-team
//! VirtualList Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn VirtualList(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-virtual-list=""
            role="list"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn VirtualListViewport(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-virtual-list-viewport=""
            role="presentation"
            tabindex="0"
            attr:aria-label="Scrollable content"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn VirtualListContent(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-virtual-list-content=""
            role="presentation"
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn VirtualListItem(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = 0)] index: usize,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-virtual-list-item=""
            attr:data-index={index.to_string()}
            role="listitem"
            attr:aria-setsize="-1"
            attr:aria-posinset={(index + 1).to_string()}
            class={class}
            id={id}
        >
            {children.map(|c| c())}
        </div>
    }
}
