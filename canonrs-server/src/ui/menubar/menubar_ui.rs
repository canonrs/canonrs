//! @canon-id: menubar
//! @canon-label: Menubar
//! @canon-family: interactive
//! @canon-category: Navigation
//! @canon-intent: Horizontal application menu bar
//! @canon-description: Menu bar navigation
//! @canon-composable: true
//! @canon-capabilities: OpenClose
//! @canon-required-parts: MenubarMenu, MenubarTrigger
//! @canon-optional-parts: MenubarContent, MenubarItem, MenubarSeparator
//! @canon-tags: menubar, navigation, desktop, app

use leptos::prelude::*;
use canonrs_core::primitives::{
    MenubarPrimitive,
    MenubarMenuPrimitive,
    MenubarTriggerPrimitive,
    MenubarContentPrimitive,
    MenubarItemPrimitive,
    MenubarSeparatorPrimitive,
};

#[component]
pub fn Menubar(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <MenubarPrimitive class=class>
            {children()}
        </MenubarPrimitive>
    }
}

#[component]
pub fn MenubarMenu(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <MenubarMenuPrimitive class=class>
            {children()}
        </MenubarMenuPrimitive>
    }
}

#[component]
pub fn MenubarTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <MenubarTriggerPrimitive class=class>
            {children()}
        </MenubarTriggerPrimitive>
    }
}

#[component]
pub fn MenubarContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <MenubarContentPrimitive class=class>
            {children()}
        </MenubarContentPrimitive>
    }
}

#[component]
pub fn MenubarItem(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <MenubarItemPrimitive class=class>
            {children()}
        </MenubarItemPrimitive>
    }
}

#[component]
pub fn MenubarSeparator(
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <MenubarSeparatorPrimitive class=class />
    }
}

#[component]
pub fn MenubarPreview() -> impl IntoView {
    view! {
        <Menubar>
            <MenubarMenu>
                <MenubarTrigger>"File"</MenubarTrigger>
                <MenubarContent>
                    <MenubarItem>"New"</MenubarItem>
                    <MenubarItem>"Open"</MenubarItem>
                    <MenubarSeparator />
                    <MenubarItem>"Exit"</MenubarItem>
                </MenubarContent>
            </MenubarMenu>
            <MenubarMenu>
                <MenubarTrigger>"Edit"</MenubarTrigger>
                <MenubarContent>
                    <MenubarItem>"Cut"</MenubarItem>
                    <MenubarItem>"Copy"</MenubarItem>
                    <MenubarItem>"Paste"</MenubarItem>
                </MenubarContent>
            </MenubarMenu>
        </Menubar>
    }
}
