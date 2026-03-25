//! @canon-level: strict
//! @canon-owner: primitives-team
//! NavigationMenu Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::state_engine::visibility_attrs;

#[component]
pub fn NavigationMenuPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <nav
            data-rs-navigation-menu=""
            data-rs-component="NavigationMenu"
            data-rs-behavior="navigation"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
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
        <ul
            data-rs-navigation-menu-list=""
            role="menubar"
            aria-orientation="horizontal"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
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
        <li data-rs-navigation-menu-item="" class=class id=id.filter(|s| !s.is_empty())>
            {children()}
        </li>
    }
}

/// Trigger sem id wiring — relação com content via DOM closest/sibling
#[component]
pub fn NavigationMenuTriggerPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <button
            type="button"
            data-rs-navigation-menu-trigger=""
            data-rs-state=s.data_rs_state
            aria-haspopup="menu"
            aria-expanded=s.aria_expanded
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </button>
    }
}

/// Content sem id wiring — aberto via CSS :hover/:focus-within + behavior keyboard
#[component]
pub fn NavigationMenuContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-navigation-menu-content=""
            role="menu"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
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
        <a
            data-rs-navigation-menu-link=""
            role="menuitem"
            href=href
            class=class
            id=id.filter(|s| !s.is_empty())
        >
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
        <div data-rs-navigation-menu-subitem="" class=class id=id.filter(|s| !s.is_empty())>
            {children()}
        </div>
    }
}
