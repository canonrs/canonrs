//! @canon-level: strict
//! @canon-owner: primitives-team
//! ContextMenu Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn ContextMenuPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-context-menu=""
            data-state="closed"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-trigger=""
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-context-menu-content=""
            role="menu"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn ContextMenuItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            data-rs-context-menu-item=""
            role="menuitem"
            type="button"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn ContextMenuSeparatorPrimitive(
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-context-menu-separator=""
            role="separator"
            class=class
            id=id
        />
    }
}
