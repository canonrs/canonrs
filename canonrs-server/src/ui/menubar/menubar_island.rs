//! @canon-level: strict
//! Menubar Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::menubar_ui::{Menubar, MenubarMenu, MenubarTrigger, MenubarContent, MenubarItem, MenubarSeparator};



#[component]
pub fn MenubarIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <Menubar class=class.unwrap_or_default()>{children()}</Menubar>
    }
}

#[component]
pub fn MenubarMenuIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <MenubarMenu class=class.unwrap_or_default()>{children()}</MenubarMenu> }
}

#[component]
pub fn MenubarTriggerIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <MenubarTrigger class=class.unwrap_or_default()>{children()}</MenubarTrigger> }
}

#[component]
pub fn MenubarContentIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <MenubarContent class=class.unwrap_or_default()>{children()}</MenubarContent> }
}

#[component]
pub fn MenubarItemIsland(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <MenubarItem class=class.unwrap_or_default()>{children()}</MenubarItem> }
}

#[component]
pub fn MenubarSeparatorIsland(
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <MenubarSeparator class=class.unwrap_or_default() /> }
}
