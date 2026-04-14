//! @canon-level: strict
//! @canon-owner: primitives-team
//! Sidebar Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{VisibilityState, ActivityState, DisabledState};
use crate::infra::state_engine::{visibility_attrs, trigger_attrs, activity_attrs, disabled_attrs};

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
    let s = visibility_attrs(state);
    view! {
        <aside
            data-rs-sidebar=""
            data-rs-uid=crate::infra::uid::generate("sb")
            data-rs-interaction="nav"
            data-rs-component="Sidebar"
            data-rs-state=s.data_rs_state
            data-rs-variant=variant.as_str()
            aria-hidden=s.aria_hidden
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
    let a = activity_attrs(active);
    let d = disabled_attrs(disabled);
    let is_active = active == ActivityState::Active;
    view! {
        <a
            data-rs-sidebar-menu-item=""
            data-rs-state=a.data_rs_state
            data-rs-disabled=d.data_rs_disabled
            href=if d.disabled { "#".to_string() } else { href }
            aria-current=if is_active { Some("page") } else { None }
            aria-disabled=d.aria_disabled
            tabindex=if d.disabled { "-1" } else { "0" }
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
    let t = trigger_attrs(state);
    view! {
        <button
            type="button"
            data-rs-sidebar-toggle=""
            data-rs-state=t.data_rs_state
            aria-expanded=t.aria_expanded
            class=class
            style=style
        >
            {children()}
        </button>
    }
}
