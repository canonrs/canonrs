//! @canon-level: strict
//! Sidebar Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::sidebar_ui::{
    Sidebar as SidebarUi,
    SidebarHeader as SidebarHeaderUi,
    SidebarContent as SidebarContentUi,
    SidebarFooter as SidebarFooterUi,
    SidebarMenu as SidebarMenuUi,
    SidebarMenuItem as SidebarMenuItemUi,
    SidebarMenuGroup as SidebarMenuGroupUi,
    SidebarSeparator as SidebarSeparatorUi,
    SidebarGroupLabel as SidebarGroupLabelUi
};
use canonrs_core::primitives::SidebarVariant;
use canonrs_core::meta::{VisibilityState, ActivityState, DisabledState};



#[component]
pub fn Sidebar(
    children: Children,
    #[prop(default = VisibilityState::Open)] state: VisibilityState,
    #[prop(default = SidebarVariant::Default)] variant: SidebarVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarUi state=state variant=variant class=class>{children()}</SidebarUi>
    }
}

#[component]
pub fn SidebarHeader(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarHeaderUi class=class>{children()}</SidebarHeaderUi> }
}

#[component]
pub fn SidebarContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarContentUi class=class>{children()}</SidebarContentUi> }
}

#[component]
pub fn SidebarFooter(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarFooterUi class=class>{children()}</SidebarFooterUi> }
}

#[component]
pub fn SidebarMenu(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarMenuUi class=class>{children()}</SidebarMenuUi> }
}

#[component]
pub fn SidebarMenuItem(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
) -> impl IntoView {
    view! { <SidebarMenuItemUi class=class href=href active=active disabled=disabled>{children()}</SidebarMenuItemUi> }
}

#[component]
pub fn SidebarMenuGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    view! { <SidebarMenuGroupUi class=class label=label.unwrap_or_default()>{children()}</SidebarMenuGroupUi> }
}

#[component]
pub fn SidebarSeparator(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarSeparatorUi class=class /> }
}

#[component]
pub fn SidebarGroupLabel(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarGroupLabelUi class=class>{children()}</SidebarGroupLabelUi> }
}
