//! @canon-level: strict
//! @canon-owner: primitives-team
//! NavigationMenu Primitive - HTML puro + ARIA
//! Base: Menubar com submenu overlay

use leptos::prelude::*;

#[component]
pub fn NavigationMenuPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <nav
            attr:data-navigation-menu=""
            role="navigation"
            class=class
            id=id
        >
            {children()}
        </nav>
    }
}

#[component]
pub fn NavigationMenuListPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <ul
            attr:data-navigation-menu-list=""
            role="menubar"
            attr:aria-orientation="horizontal"
            class=class
            id=id
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn NavigationMenuItemPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <li
            attr:data-navigation-menu-item=""
            class=class
            id=id
        >
            {children()}
        </li>
    }
}

#[component]
pub fn NavigationMenuTriggerPrimitive(
    children: Children,
    #[prop(default = -1)] tabindex: i32,
    #[prop(default = String::new())] controls_id: String,
    #[prop(default = false)] expanded: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            attr:data-navigation-menu-trigger=""
            type="button"
            tabindex={tabindex}
            attr:aria-haspopup="menu"
            attr:aria-controls={controls_id}
            attr:aria-expanded={if expanded { "true" } else { "false" }}
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
    #[prop(default = String::new())] content_id: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            role="menu"
            id={content_id}
            attr:data-navigation-menu-content=""
            tabindex="-1"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn NavigationMenuLinkPrimitive(
    children: Children,
    #[prop(default = String::new())] href: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <a
        
            attr:data-navigation-menu-link=""
            href={href}
            class=class
            id=id
        >
            {children()}
        </a>
    }
}

#[component]
pub fn NavigationMenuSubItemPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-navigation-menu-subitem=""
            role="menuitem"
            tabindex="-1"
            class=class
            id=id
        >
            {children()}
        </div>
    }
}
