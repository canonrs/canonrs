//! @canon-level: strict
//! @canon-owner: primitives-team
//! Sidebar Primitive - HTML puro + ARIA

use leptos::prelude::*;

#[component]
pub fn SidebarPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = Signal::derive(|| false))] collapsed: Signal<bool>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <aside
            data-rs-sidebar=""
            data-rs-state={move || if collapsed.get() { "collapsed" } else { "expanded" }}
            role="complementary"
            class=class
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children.map(|c| c())}
        </aside>
    }
}

#[component]
pub fn SidebarHeaderPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-sidebar-header="" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SidebarContentPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-sidebar-content="" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SidebarFooterPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-sidebar-footer="" class=class>
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SidebarMenuPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <nav data-rs-sidebar-menu="" role="navigation" class=class>
            {children.map(|c| c())}
        </nav>
    }
}

#[component]
pub fn SidebarMenuItemPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] href: String,
    #[prop(default = false)] active: bool,
) -> impl IntoView {
    let state = if active { "active" } else { "inactive" };
    view! {
        <div
            data-rs-sidebar-menu-item=""
            data-rs-state=state
            data-href=href
            class=class
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SidebarMenuGroupPrimitive(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-sidebar-menu-group=""
            class=class
            id={if id.is_empty() { None } else { Some(id) }}
        >
            {children.map(|c| c())}
        </div>
    }
}

#[component]
pub fn SidebarSeparatorPrimitive(
    #[prop(default = String::new())] class: String,
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
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-sidebar-group-label="" class=class>
            {children.map(|c| c())}
        </div>
    }
}
