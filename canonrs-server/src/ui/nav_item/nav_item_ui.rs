#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{NavItemPrimitive, NavGroupPrimitive};
use canonrs_core::meta::{ActivityState, DisabledState};

#[component]
pub fn NavItem(
    #[prop(into)] label: String,
    #[prop(optional, into)] href: Option<String>,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(optional)] icon: Option<Children>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <NavItemPrimitive
            href=href.unwrap_or_else(|| "".to_string())
            active=active
            disabled=disabled
            class=class
        >
            {icon.map(|i| view! { <span data-rs-nav-item-icon="">{i()}</span> })}
            <span data-rs-nav-item-label="">{label}</span>
        </NavItemPrimitive>
    }
}

#[component]
pub fn NavItemPreview() -> impl IntoView {
    view! { <NavItem label="Nav Item".to_string() /> }
}

#[component]
pub fn NavGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::from("vertical"))] direction: String,
) -> impl IntoView {
    view! {
        <NavGroupPrimitive
            class=class
            aria_label=aria_label.unwrap_or_default()
            direction=direction
        >
            {children()}
        </NavGroupPrimitive>
    }
}
