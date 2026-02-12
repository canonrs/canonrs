use leptos::prelude::*;
use crate::primitives::{
    SidebarPrimitive, SidebarHeaderPrimitive, SidebarContentPrimitive,
    SidebarFooterPrimitive, SidebarMenuPrimitive, SidebarMenuItemPrimitive,
    SidebarMenuGroupPrimitive, SidebarSeparatorPrimitive, SidebarGroupLabelPrimitive,
};

#[component]
pub fn Sidebar(
    #[prop(optional)] children: Option<Children>,
    #[prop(into, default = MaybeSignal::Static(false))] collapsed: MaybeSignal<bool>,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <SidebarPrimitive 
            class={class}
            class:sidebar-collapsed={move || collapsed.get()}
            id={id.unwrap_or_default()}
        >
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
        <SidebarHeaderPrimitive class={class}>
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
        <SidebarContentPrimitive class={class}>
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
        <SidebarFooterPrimitive class={class}>
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
        <SidebarMenuPrimitive class={class}>
            {children.map(|c| c())}
        </SidebarMenuPrimitive>
    }
}

#[component]
pub fn SidebarMenuItem(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] href: String,
    #[prop(default = false)] active: bool,
) -> impl IntoView {
    view! {
        <SidebarMenuItemPrimitive class={class} href={href} active={active}>
            {children.map(|c| c())}
        </SidebarMenuItemPrimitive>
    }
}

#[component]
pub fn SidebarMenuGroup(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
    #[prop(into, optional)] id: Option<String>,
) -> impl IntoView {
    view! {
        <SidebarMenuGroupPrimitive class={class} id={id.unwrap_or_default()}>
            {children.map(|c| c())}
        </SidebarMenuGroupPrimitive>
    }
}

#[component]
pub fn SidebarSeparator(
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarSeparatorPrimitive class={class} />
    }
}

#[component]
pub fn SidebarGroupLabel(
    #[prop(optional)] children: Option<Children>,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <SidebarGroupLabelPrimitive class={class}>
            {children.map(|c| c())}
        </SidebarGroupLabelPrimitive>
    }
}
