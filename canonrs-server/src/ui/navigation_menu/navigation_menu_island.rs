//! @canon-level: strict
//! NavigationMenu Island — Canon Rule #340 (zero-logic boundary)

use leptos::prelude::*;
use super::navigation_menu_ui::{
    NavigationMenu, NavigationMenuItem, NavigationMenuTrigger,
    NavigationMenuContent, NavigationMenuLink
};

#[component]
pub fn NavigationMenuIsland(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! { <NavigationMenu class=class>{children()}</NavigationMenu> }
}

#[component]
pub fn NavigationMenuItemIsland(children: Children) -> impl IntoView {
    view! { <NavigationMenuItem>{children()}</NavigationMenuItem> }
}

#[component]
pub fn NavigationMenuTriggerIsland(children: Children) -> impl IntoView {
    view! { <NavigationMenuTrigger>{children()}</NavigationMenuTrigger> }
}

#[component]
pub fn NavigationMenuContentIsland(children: Children) -> impl IntoView {
    view! { <NavigationMenuContent>{children()}</NavigationMenuContent> }
}

#[component]
pub fn NavigationMenuLinkIsland(
    children: Children,
    #[prop(into, default = String::new())] href: String,
) -> impl IntoView {
    view! { <NavigationMenuLink href=href>{children()}</NavigationMenuLink> }
}
