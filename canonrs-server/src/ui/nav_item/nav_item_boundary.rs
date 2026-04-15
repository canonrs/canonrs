//! @canon-level: strict
//! NavItem Boundary — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::nav_item_ui::{NavItem as NavItemUi, NavGroup as NavGroupUi};
use canonrs_core::meta::{ActivityState, DisabledState};

#[component]
pub fn NavItem(
    #[prop(into)] label: String,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <NavItemUi label=label href=href active=active disabled=disabled class=class /> }
}

#[component]
pub fn NavGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, optional)] aria_label: Option<String>,
    #[prop(into, default = String::from("vertical"))] direction: String,
) -> impl IntoView {
    view! { <NavGroupUi class=class aria_label=aria_label.unwrap_or_default() direction=direction>{children()}</NavGroupUi> }
}
