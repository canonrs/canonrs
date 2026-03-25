//! @canon-level: strict
//! @canon-owner: primitives-team
//! ContextMenu Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn ContextMenuPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(default = false)] open: bool,
) -> impl IntoView {
    view! {
        <div
            data-rs-context-menu=""
            data-rs-state={if open { "open" } else { "closed" }}
            role="menu"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ContextMenuTriggerPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-context-menu-trigger=""
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ContextMenuContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-context-menu-content=""
            role="menu"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn ContextMenuItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-context-menu-item=""
            role="menuitem"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn ContextMenuSeparatorPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-context-menu-separator=""
            role="separator"
            class=class
            id=id.filter(|s| !s.is_empty())
        />
    }
}
