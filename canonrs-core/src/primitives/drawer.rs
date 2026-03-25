//! @canon-level: strict
//! @canon-owner: primitives-team
//! Drawer Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn DrawerPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(default = false)] open: bool,
) -> impl IntoView {
    view! {
        <div
            data-rs-drawer=""
            data-rs-component="Drawer"
            data-rs-behavior="overlay"
            data-rs-state={if open { "open" } else { "closed" }}
            aria-hidden={if open { "false" } else { "true" }}
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DrawerTriggerPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(default = false)] open: bool,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-drawer-trigger=""
            aria-haspopup="dialog"
            aria-expanded={if open { "true" } else { "false" }}
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DrawerOverlayPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-drawer-overlay=""
            aria-hidden="true"
            class=class
            id=id.filter(|s| !s.is_empty())
        />
    }
}

#[component]
pub fn DrawerContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(optional)] aria_labelledby: Option<String>,
    #[prop(optional)] aria_describedby: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-drawer-content=""
            role="dialog"
            aria-modal="true"
            aria-labelledby=aria_labelledby.filter(|s| !s.is_empty())
            aria-describedby=aria_describedby.filter(|s| !s.is_empty())
            tabindex="-1"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}
