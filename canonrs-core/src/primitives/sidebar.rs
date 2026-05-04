//! @canon-level: strict
//! @canon-owner: primitives-team
//! Sidebar Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{VisibilityState, ActivityState, DisabledState};

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum SidebarVariant {
    #[default]
    Default,
    Rail,
}
impl SidebarVariant {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Default => "default", Self::Rail => "rail" }
    }
}

#[component]
pub fn SidebarPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Open)] state: VisibilityState,
    #[prop(default = SidebarVariant::Default)] variant: SidebarVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let uid_sb = crate::infra::uid::generate("sb");
    view! {
        <aside
            data-rs-sidebar=""
            data-rs-uid=uid_sb
            data-rs-interaction="nav"
            data-rs-visibility=state.as_str()
            data-rs-variant=variant.as_str()
            aria-hidden=state.aria_hidden()
            role="complementary"
            class=class
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
    #[prop(into, default = String::new())] href: String,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let is_active = active == ActivityState::Active;
    view! {
        <a
            data-rs-sidebar-menu-item=""
            data-rs-activity=active.as_str()
            data-rs-disabled=if disabled.disabled() { Some("disabled") } else { None }
            href=if disabled.disabled() { "#".to_string() } else { href }
            aria-current=if is_active { Some("page") } else { None }
            aria-disabled=disabled.aria_disabled()
            tabindex=if disabled.disabled() { "-1" } else { "0" }
            class=class
        >
            {children()}
        </a>
    }
}

#[component]
pub fn SidebarMenuGroupPrimitive(
    children: Children,
    #[prop(into, optional)] label: Option<String>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-sidebar-menu-group=""
            role="group"
            aria-label=label
            class=class
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
        <div
            data-rs-sidebar-group-label=""
            role="presentation"
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn SidebarTriggerPrimitive(
    children: Children,
    #[prop(default = VisibilityState::Open)] state: VisibilityState,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] style: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            data-rs-sidebar-toggle=""
            data-rs-visibility=state.as_str()
            aria-expanded=state.aria_expanded()
            class=class
            style=style
        >
            {children()}
        </button>
    }
}
