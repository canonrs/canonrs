//! @canon-level: strict
//! @canon-owner: primitives-team
//! NavigationMenu Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn NavigationMenuPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <nav data-navigation-menu="" class=class id=id>
            {children()}
        </nav>
    }
}

#[component]
pub fn NavigationMenuListPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <ul data-navigation-menu-list="" role="menubar" aria-orientation="horizontal" class=class id=id>
            {children()}
        </ul>
    }
}

#[component]
pub fn NavigationMenuItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <li data-navigation-menu-item="" class=class id=id>
            {children()}
        </li>
    }
}

#[component]
pub fn NavigationMenuTriggerPrimitive(
    children: Children,
    #[prop(into, default = String::new())] controls_id: String,
    #[prop(default = false)] expanded: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let aria_controls = if controls_id.is_empty() { None } else { Some(controls_id) };
    view! {
        <button
            data-navigation-menu-trigger=""
            type="button"
            attr:aria-haspopup="menu"
            attr:aria-controls=aria_controls
            attr:aria-expanded=if expanded { "true" } else { "false" }
            class=class
            id=id
        >
            {children()}
        </button>
    }
}

#[component]
pub fn NavigationMenuContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] content_id: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let id_val = if content_id.is_empty() { None } else { Some(content_id) };
    view! {
        <div data-navigation-menu-content="" role="menu" id=id_val class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn NavigationMenuLinkPrimitive(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <a data-navigation-menu-link="" role="menuitem" href=href class=class id=id>
            {children()}
        </a>
    }
}

#[component]
pub fn NavigationMenuSubItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div data-navigation-menu-subitem="" class=class id=id>
            {children()}
        </div>
    }
}
