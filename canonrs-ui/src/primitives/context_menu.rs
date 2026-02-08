//! @canon-level: strict
//! @canon-owner: primitives-team
//! ContextMenu Primitive - HTML puro + ARIA
//! Base: DropdownMenu com evento contextmenu
//!
//! Canon: data-value é fornecido via atributo DOM, não prop Rust

use leptos::prelude::*;

#[component]
pub fn ContextMenuPrimitive(
    #[prop(optional)] children: Option<Children>,
    open: Signal<bool>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-context-menu=""
            attr:data-state={move || if open.get() { "open" } else { "closed" }}
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
    #[prop(default = String::new())] controls_id: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            attr:data-context-menu-trigger=""
            type="button"
            attr:aria-haspopup="menu"
            attr:aria-controls={controls_id}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn ContextMenuContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] content_id: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-context-menu-content=""
            tabindex="-1"
            role="menu"
            id={content_id}
            class=class
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
            attr:data-context-menu-item=""
            type="button"
            role="menuitem"
            tabindex="-1"
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
            attr:data-context-menu-separator=""
            role="separator"
            class=class
            id=id
        />
    }
}
