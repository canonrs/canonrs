#![allow(unreachable_pub, dead_code)]

use leptos::prelude::*;
use canonrs_core::primitives::{
    SidebarPrimitive, SidebarVariant, SidebarHeaderPrimitive, SidebarContentPrimitive,
    SidebarFooterPrimitive, SidebarMenuPrimitive, SidebarMenuItemPrimitive,
    SidebarMenuGroupPrimitive, SidebarSeparatorPrimitive, SidebarGroupLabelPrimitive,
    SidebarGroupPrimitive, SidebarGroupTriggerPrimitive, SidebarGroupContentPrimitive,
    SidebarLabelPrimitive, SidebarIconPrimitive, SidebarUserPrimitive,
};
use canonrs_core::meta::{VisibilityState, ActivityState, DisabledState};

#[component]
pub fn Sidebar(
    children: Children,
    #[prop(default = VisibilityState::Open)] state: VisibilityState,
    #[prop(default = SidebarVariant::Default)] variant: SidebarVariant,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarPrimitive state=state variant=variant class=class>
            {children()}
        </SidebarPrimitive>
    }
}

#[component]
pub fn SidebarHeader(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarHeaderPrimitive class=class>
            {children()}
        </SidebarHeaderPrimitive>
    }
}

#[component]
pub fn SidebarContent(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarContentPrimitive class=class>
            {children()}
        </SidebarContentPrimitive>
    }
}

#[component]
pub fn SidebarFooter(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarFooterPrimitive class=class>
            {children()}
        </SidebarFooterPrimitive>
    }
}

#[component]
pub fn SidebarMenu(
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarMenuPrimitive class=class>
            {children()}
        </SidebarMenuPrimitive>
    }
}

#[component]
pub fn SidebarMenuItem(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] href: String,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
) -> impl IntoView {
    view! {
        <SidebarMenuItemPrimitive class=class href=href active=active disabled=disabled>
            {children()}
        </SidebarMenuItemPrimitive>
    }
}

#[component]
pub fn SidebarMenuGroup(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] label: Option<String>,
) -> impl IntoView {
    view! {
        <SidebarMenuGroupPrimitive class=class label=label.unwrap_or_default()>
            {children()}
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
    children: Children,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarGroupLabelPrimitive class=class>
            {children()}
        </SidebarGroupLabelPrimitive>
    }
}

#[component]
pub fn SidebarGroup(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] root: bool,
    #[prop(default = false)] hidden: bool,
    #[prop(default = VisibilityState::Open)] state: VisibilityState,
) -> impl IntoView {
    view! {
        <SidebarGroupPrimitive class=class root=root state=state attr:hidden=hidden.then(|| "")>
            {children()}
        </SidebarGroupPrimitive>
    }
}

#[component]
pub fn SidebarGroupTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarGroupTriggerPrimitive class=class>
            {children()}
        </SidebarGroupTriggerPrimitive>
    }
}

#[component]
pub fn SidebarGroupContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarGroupContentPrimitive class=class>
            {children()}
        </SidebarGroupContentPrimitive>
    }
}

#[component]
pub fn SidebarLabel(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarLabelPrimitive class=class>{children()}</SidebarLabelPrimitive> }
}

#[component]
pub fn SidebarIcon(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarIconPrimitive class=class>{children()}</SidebarIconPrimitive> }
}

#[component]
pub fn SidebarUser(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <SidebarUserPrimitive class=class>{children()}</SidebarUserPrimitive> }
}

#[component]
pub fn SidebarPreview() -> leptos::prelude::AnyView {
    view! { <Sidebar>"Sidebar"</Sidebar> }.into_any()
}
