//! @canon-level: strict
//! @canon-owner: primitives-team
//! NavigationMenu Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::VisibilityState;
use crate::infra::state_engine::{visibility_attrs, trigger_attrs};

#[component]
pub fn NavigationMenuPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <nav
            data-rs-navigation-menu=""
            data-rs-uid=crate::infra::uid::generate("nm")
            data-rs-interaction="init"
            class=class
        >
            {children()}
        </nav>
    }
}

#[component]
pub fn NavigationMenuListPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <ul
            data-rs-navigation-menu-list=""
            role="menubar"
            aria-orientation="horizontal"
            class=class
        >
            {children()}
        </ul>
    }
}

#[component]
pub fn NavigationMenuItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let item_uid = crate::infra::uid::generate("ni");
    view! {
        <li data-rs-navigation-menu-item="" data-rs-uid=item_uid class=class>
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
) -> impl IntoView {
    let t = trigger_attrs(state);
    view! {
        <button
            type="button"
            data-rs-navigation-menu-trigger=""
            data-rs-uid=crate::infra::uid::generate("nt")
            data-rs-state=t.data_rs_state
            aria-haspopup="menu"
            aria-expanded=t.aria_expanded
            class=class
        >
            {children()}
        </button>
    }
}

/// Content — visibilidade controlada por CSS via data-rs-state no trigger pai
#[component]
pub fn NavigationMenuContentPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Closed)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let s = visibility_attrs(state);
    view! {
        <div
            data-rs-navigation-menu-content=""
            data-rs-state=s.data_rs_state
            aria-hidden=s.aria_hidden
            role="menu"
            class=class
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
) -> impl IntoView {
    view! {
        <a
            data-rs-navigation-menu-link=""
            role="menuitem"
            href=href
            class=class
        >
            {children()}
        </a>
    }
}

#[component]
pub fn NavigationMenuSubItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-navigation-menu-subitem="" class=class>
            {children()}
        </div>
    }
}
