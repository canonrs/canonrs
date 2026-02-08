use leptos::prelude::*;
use crate::primitives::{
    NavigationMenuPrimitive,
    NavigationMenuItemPrimitive,
    NavigationMenuTriggerPrimitive,
    NavigationMenuContentPrimitive,
    NavigationMenuLinkPrimitive,
    NavigationMenuSubItemPrimitive,
};

#[component]
pub fn NavigationMenu(
    children: Children,
    #[prop(default = String::new())] class_name: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <NavigationMenuPrimitive
            attr:class={class_name}
            id=id
        >
            {children()}
        </NavigationMenuPrimitive>
    }
}

#[component]
pub fn NavigationMenuList(
    children: Children,
    #[prop(default = String::new())] class_name: String,
    #[prop(default = String::new())] _id: String,
) -> impl IntoView {
    view! {
            attr:class={class_name}
            id=id
        >
            {children()}
    }
}

#[component]
pub fn NavigationMenuItem(
    children: Children,
    #[prop(default = String::new())] class_name: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <NavigationMenuItemPrimitive
            attr:class={class_name}
            id=id
        >
            {children()}
        </NavigationMenuItemPrimitive>
    }
}

#[component]
pub fn NavigationMenuTrigger(
    children: Children,
    #[prop(default = -1)] tabindex: i32,
    #[prop(default = String::new())] controls_id: String,
    #[prop(default = false)] expanded: bool,
    #[prop(default = String::new())] class_name: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <NavigationMenuTriggerPrimitive
            tabindex=tabindex
            controls_id=controls_id
            expanded=expanded
            attr:class={class_name}
            id=id
        >
            {children()}
        </NavigationMenuTriggerPrimitive>
    }
}

#[component]
pub fn NavigationMenuContent(
    children: Children,
    #[prop(default = String::new())] content_id: String,
    #[prop(default = String::new())] class_name: String,
) -> impl IntoView {
    view! {
        <NavigationMenuContentPrimitive
            content_id=content_id
            attr:class={class_name}
        >
            {children()}
        </NavigationMenuContentPrimitive>
    }
}

#[component]
pub fn NavigationMenuLink(
    children: Children,
    #[prop(default = String::new())] href: String,
    #[prop(default = String::new())] class_name: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <NavigationMenuLinkPrimitive
            href=href
            attr:class={class_name}
            id=id
        >
            {children()}
        </NavigationMenuLinkPrimitive>
    }
}

#[component]
pub fn NavigationMenuSubItem(
    children: Children,
    #[prop(default = String::new())] class_name: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <NavigationMenuSubItemPrimitive
            attr:class={class_name}
            id=id
        >
            {children()}
        </NavigationMenuSubItemPrimitive>
    }
}
