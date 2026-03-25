//! @canon-level: strict
//! @canon-owner: primitives-team
//! Menubar Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn MenubarPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-menubar="" role="menubar" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn MenubarMenuPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-menubar-menu="" data-rs-state="closed" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn MenubarTriggerPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-menubar-trigger=""
            role="menuitem"
            aria-haspopup="menu"
            aria-expanded="false"
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn MenubarContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-menubar-content="" role="menu" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn MenubarItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-menubar-item=""
            role="menuitem"
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn MenubarSeparatorPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-menubar-separator="" role="separator" class=class />
    }
}
