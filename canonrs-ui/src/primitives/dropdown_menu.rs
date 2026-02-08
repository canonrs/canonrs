//! @canon-level: strict
//! @canon-owner: primitives-team
//! DropdownMenu Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn DropdownMenuPrimitive(
    #[prop(optional)] children: Option<Children>,
    open: Signal<bool>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView
{
    view! {
        <div
            attr:data-dropdown=""
            attr:data-state={move || if open.get() { "open" } else { "closed" }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DropdownMenuTriggerPrimitive(
    #[prop(optional)] children: Option<Children>,
    open: Signal<bool>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView
{
    view! {
        <button
            attr:data-dropdown-trigger=""
            type="button"
            attr:aria-haspopup="menu"
            attr:aria-expanded={move || if open.get() { "true" } else { "false" }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn DropdownMenuContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-dropdown-content=""
            role="menu"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DropdownMenuGroupPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-dropdown-group=""
            role="group"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn DropdownMenuItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            attr:data-dropdown-item=""
            type="button"
            role="menuitem"
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}

#[component]
pub fn DropdownMenuCheckboxItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = false)] checked: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            attr:data-dropdown-checkbox-item=""
            attr:data-checked={if checked { "true" } else { "false" }}
            type="button"
            role="menuitemcheckbox"
            attr:aria-checked={if checked { "true" } else { "false" }}
            class=class
            id=id
        >
            {children.map(|c| c())}
        </button>
    }
}
