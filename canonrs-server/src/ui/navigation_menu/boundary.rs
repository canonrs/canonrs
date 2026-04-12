//! @canon-level: strict
//! NavigationMenu Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::navigation_menu_ui::{
    NavigationMenu as NavigationMenuUi,
    NavigationMenuItem as NavigationMenuItemUi,
    NavigationMenuTrigger as NavigationMenuTriggerUi,
    NavigationMenuContent as NavigationMenuContentUi,
    NavigationMenuLink as NavigationMenuLinkUi
};

#[component]
pub fn NavigationMenu(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <NavigationMenuUi class=class>{children()}</NavigationMenuUi> }
}

#[component]
pub fn NavigationMenuItem(children: Children) -> impl IntoView {
    view! { <NavigationMenuItemUi>{children()}</NavigationMenuItemUi> }
}

#[component]
pub fn NavigationMenuTrigger(children: Children) -> impl IntoView {
    view! { <NavigationMenuTriggerUi>{children()}</NavigationMenuTriggerUi> }
}

#[component]
pub fn NavigationMenuContent(children: Children) -> impl IntoView {
    view! { <NavigationMenuContentUi>{children()}</NavigationMenuContentUi> }
}

#[component]
pub fn NavigationMenuLink(
    children: Children,
    #[prop(into, default = String::new())] href: String,
) -> impl IntoView {
    view! { <NavigationMenuLinkUi href=href>{children()}</NavigationMenuLinkUi> }
}
