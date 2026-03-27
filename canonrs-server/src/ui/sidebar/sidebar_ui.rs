//! @canon-id: sidebar
//! @canon-label: Sidebar
//! @canon-family: navigation
//! @canon-category: Navigation
//! @canon-intent: Vertical navigation panel
//! @canon-description: Sidebar navigation component
//! @canon-composable: true
//! @canon-capabilities: OpenClose
//! @canon-required-parts: SidebarContent
//! @canon-optional-parts: SidebarHeader, SidebarFooter, SidebarMenu, SidebarMenuItem
//! @canon-tags: sidebar, nav, navigation, links, left-panel

use leptos::prelude::*;
use canonrs_core::primitives::{
    SidebarPrimitive, SidebarVariant, SidebarHeaderPrimitive, SidebarContentPrimitive,
    SidebarFooterPrimitive, SidebarMenuPrimitive, SidebarMenuItemPrimitive,
    SidebarMenuGroupPrimitive, SidebarSeparatorPrimitive, SidebarGroupLabelPrimitive,
};
use canonrs_core::meta::{VisibilityState, ActivityState, DisabledState};

#[component]
pub fn Sidebar(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = VisibilityState::Open)] state: VisibilityState,
    #[prop(default = SidebarVariant::Default)] variant: SidebarVariant,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarPrimitive state=state variant=variant class=class>
            {children.map(|c| c())}
        </SidebarPrimitive>
    }
}

#[component]
pub fn SidebarHeader(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarHeaderPrimitive class=class>
            {children.map(|c| c())}
        </SidebarHeaderPrimitive>
    }
}

#[component]
pub fn SidebarContent(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarContentPrimitive class=class>
            {children.map(|c| c())}
        </SidebarContentPrimitive>
    }
}

#[component]
pub fn SidebarFooter(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarFooterPrimitive class=class>
            {children.map(|c| c())}
        </SidebarFooterPrimitive>
    }
}

#[component]
pub fn SidebarMenu(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarMenuPrimitive class=class>
            {children.map(|c| c())}
        </SidebarMenuPrimitive>
    }
}

#[component]
pub fn SidebarMenuItem(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] href: String,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
) -> impl IntoView {
    view! {
        <SidebarMenuItemPrimitive class=class href=href active=active disabled=disabled>
            {children.map(|c| c())}
        </SidebarMenuItemPrimitive>
    }
}

#[component]
pub fn SidebarMenuGroup(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] label: Option<String>,
) -> impl IntoView {
    view! {
        <SidebarMenuGroupPrimitive class=class label=label.unwrap_or_default()>
            {children.map(|c| c())}
        </SidebarMenuGroupPrimitive>
    }
}

#[component]
pub fn SidebarSeparator(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarSeparatorPrimitive class=class /> }
}

#[component]
pub fn SidebarGroupLabel(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarGroupLabelPrimitive class=class>
            {children.map(|c| c())}
        </SidebarGroupLabelPrimitive>
    }
}

#[component]
pub fn SidebarPreview() -> leptos::prelude::AnyView {
    view! { <Sidebar>"Sidebar"</Sidebar> }.into_any()
}
