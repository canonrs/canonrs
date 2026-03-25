//! @canon-level: strict
//! @canon-owner: primitives-team
//! DropdownMenu Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn DropdownMenuPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(default = false)] open: bool,
) -> impl IntoView {
    view! {
        <div
            data-rs-dropdown-menu=""
            data-rs-state={if open { "open" } else { "closed" }}
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuTriggerPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(default = false)] open: bool,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-dropdown-menu-trigger=""
            aria-haspopup="menu"
            aria-expanded={if open { "true" } else { "false" }}
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DropdownMenuContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-dropdown-menu-content=""
            role="menu"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuGroupPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-dropdown-menu-group=""
            role="group"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn DropdownMenuItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-dropdown-menu-item=""
            role="menuitem"
            aria-disabled={if disabled { Some("true") } else { None }}
            data-rs-disabled={if disabled { Some("true") } else { None }}
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DropdownMenuSeparatorPrimitive(
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-dropdown-menu-separator=""
            role="separator"
            class=class
            id=id.filter(|s| !s.is_empty())
        />
    }
}

#[component]
pub fn DropdownMenuCheckboxItemPrimitive(
    children: Children,
    #[prop(default = false)] checked: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-dropdown-menu-checkbox-item=""
            role="menuitemcheckbox"
            aria-checked={if checked { "true" } else { "false" }}
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </button>
    }
}

#[component]
pub fn DropdownMenuLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-dropdown-menu-label=""
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}
