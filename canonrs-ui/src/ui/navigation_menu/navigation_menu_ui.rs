//! @canon-level: ui
//! NavigationMenu - Declarative UI wrapper

use leptos::prelude::*;

fn oc(s: Option<String>) -> String { s.unwrap_or_default() }
use crate::primitives::{
    NavigationMenuPrimitive,
    NavigationMenuListPrimitive,
    NavigationMenuItemPrimitive,
    NavigationMenuTriggerPrimitive,
    NavigationMenuContentPrimitive,
    NavigationMenuLinkPrimitive,
    NavigationMenuSubItemPrimitive,
};

#[component]
pub fn NavigationMenu(
    children: Children,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    view! {
        <NavigationMenuPrimitive class=oc(class_name)>
            {children()}
        </NavigationMenuPrimitive>
    }
}

#[component]
pub fn NavigationMenuList(
    children: Children,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    view! {
        <NavigationMenuListPrimitive class=oc(class_name)>
            {children()}
        </NavigationMenuListPrimitive>
    }
}

#[component]
pub fn NavigationMenuItem(
    children: Children,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    view! {
        <NavigationMenuItemPrimitive class=oc(class_name)>
            {children()}
        </NavigationMenuItemPrimitive>
    }
}

#[component]
pub fn NavigationMenuTrigger(
    children: Children,
    #[prop(into, default = String::new())] controls_id: String,
    #[prop(default = false)] expanded: bool,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    view! {
        <NavigationMenuTriggerPrimitive
            controls_id=controls_id
            expanded=expanded
            class=oc(class_name)
        >
            {children()}
        </NavigationMenuTriggerPrimitive>
    }
}

#[component]
pub fn NavigationMenuContent(
    children: Children,
    #[prop(into, default = String::new())] content_id: String,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    view! {
        <NavigationMenuContentPrimitive content_id=content_id class=oc(class_name)>
            {children()}
        </NavigationMenuContentPrimitive>
    }
}

#[component]
pub fn NavigationMenuLink(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    view! {
        <NavigationMenuLinkPrimitive href=href class=oc(class_name)>
            {children()}
        </NavigationMenuLinkPrimitive>
    }
}

#[component]
pub fn NavigationMenuSubItem(
    children: Children,
    #[prop(optional, into)] class_name: Option<String>,
) -> impl IntoView {
    view! {
        <NavigationMenuSubItemPrimitive class=oc(class_name)>
            {children()}
        </NavigationMenuSubItemPrimitive>
    }
}
