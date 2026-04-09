//! @canon-level: strict
//! Sidebar Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::sidebar_ui::{
    Sidebar, SidebarHeader, SidebarContent, SidebarFooter,
    SidebarMenu, SidebarMenuItem, SidebarMenuGroup,
    SidebarSeparator, SidebarGroupLabel,
};
use canonrs_core::primitives::SidebarVariant;
use canonrs_core::meta::{VisibilityState, ActivityState, DisabledState};



#[component]
pub fn SidebarIsland(
    children: Children,
    #[prop(default = VisibilityState::Open)] state: VisibilityState,
    #[prop(default = SidebarVariant::Default)] variant: SidebarVariant,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <Sidebar state=state variant=variant class=class>{children()}</Sidebar>
    }
}

#[component]
pub fn SidebarHeaderIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarHeader class=class>{children()}</SidebarHeader> }
}

#[component]
pub fn SidebarContentIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarContent class=class>{children()}</SidebarContent> }
}

#[component]
pub fn SidebarFooterIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarFooter class=class>{children()}</SidebarFooter> }
}

#[component]
pub fn SidebarMenuIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarMenu class=class>{children()}</SidebarMenu> }
}

#[component]
pub fn SidebarMenuItemIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] href: String,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
) -> impl IntoView {
    view! { <SidebarMenuItem class=class href=href active=active disabled=disabled>{children()}</SidebarMenuItem> }
}

#[component]
pub fn SidebarMenuGroupIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional, into)] label: Option<String>,
) -> impl IntoView {
    view! { <SidebarMenuGroup class=class label=label.unwrap_or_default()>{children()}</SidebarMenuGroup> }
}

#[component]
pub fn SidebarSeparatorIsland(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarSeparator class=class /> }
}

#[component]
pub fn SidebarGroupLabelIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarGroupLabel class=class>{children()}</SidebarGroupLabel> }
}
