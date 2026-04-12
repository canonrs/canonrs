//! @canon-level: strict
//! Menubar Island — bootstrap only, delegates to interaction engine

use leptos::prelude::*;
use super::menubar_ui::{
    Menubar as MenubarUi,
    MenubarMenu as MenubarMenuUi,
    MenubarTrigger as MenubarTriggerUi,
    MenubarContent as MenubarContentUi,
    MenubarItem as MenubarItemUi,
    MenubarSeparator as MenubarSeparatorUi
};



#[component]
pub fn Menubar(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! {
        <MenubarUi class=class.unwrap_or_default()>{children()}</MenubarUi>
    }
}

#[component]
pub fn MenubarMenu(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <MenubarMenuUi class=class.unwrap_or_default()>{children()}</MenubarMenuUi> }
}

#[component]
pub fn MenubarTrigger(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <MenubarTriggerUi class=class.unwrap_or_default()>{children()}</MenubarTriggerUi> }
}

#[component]
pub fn MenubarContent(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <MenubarContentUi class=class.unwrap_or_default()>{children()}</MenubarContentUi> }
}

#[component]
pub fn MenubarItem(
    children: Children,
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <MenubarItemUi class=class.unwrap_or_default()>{children()}</MenubarItemUi> }
}

#[component]
pub fn MenubarSeparator(
    #[prop(optional, into)] class: Option<String>,
) -> impl IntoView {
    view! { <MenubarSeparatorUi class=class.unwrap_or_default() /> }
}
