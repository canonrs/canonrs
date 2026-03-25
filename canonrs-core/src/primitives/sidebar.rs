//! @canon-level: strict
//! @canon-owner: primitives-team
//! Sidebar Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn SidebarPrimitive(
    children: Children,
    #[prop(default = false)] collapsed: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
    #[prop(default = false)] rail: bool,
) -> impl IntoView {
    view! {
        <aside
            data-rs-sidebar=""
            data-rs-rail={if rail { Some("true") } else { None }}
            data-rs-state={if collapsed { "collapsed" } else { "expanded" }}
            role="complementary"
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </aside>
    }
}

#[component]
pub fn SidebarHeaderPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-sidebar-header="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-sidebar-content="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarFooterPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-sidebar-footer="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarMenuPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <nav data-rs-sidebar-menu="" role="navigation" class=class>
            {children()}
        </nav>
    }
}

#[component]
pub fn SidebarMenuItemPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = false)] active: bool,
) -> impl IntoView {
    view! {
        <div
            data-rs-sidebar-menu-item=""
            data-rs-state={if active { "open" } else { "closed" }}
            data-rs-active={if active { Some("true") } else { None }}
            data-href=href
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarMenuGroupPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <div
            data-rs-sidebar-menu-group=""
            class=class
            id=id.filter(|s| !s.is_empty())
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarSeparatorPrimitive(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-sidebar-separator=""
            role="separator"
            aria-orientation="horizontal"
            class=class
        />
    }
}

#[component]
pub fn SidebarGroupLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-sidebar-group-label="" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarTriggerPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] style: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-sidebar-toggle="1"
            class=class
            style=style
        >
            {children()}
        </button>
    }
}
