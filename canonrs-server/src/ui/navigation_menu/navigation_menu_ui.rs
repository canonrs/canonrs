
use leptos::prelude::*;
use canonrs_core::VisibilityState;
use canonrs_core::primitives::{
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
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <NavigationMenuPrimitive class=class>
            {children()}
        </NavigationMenuPrimitive>
    }
}

#[component]
pub fn NavigationMenuList(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <NavigationMenuListPrimitive class=class>
            {children()}
        </NavigationMenuListPrimitive>
    }
}

#[component]
pub fn NavigationMenuItem(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <NavigationMenuItemPrimitive class=class>
            {children()}
        </NavigationMenuItemPrimitive>
    }
}

/// Trigger sem id wiring — behavior JS conecta via DOM closest
#[component]
pub fn NavigationMenuTrigger(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <NavigationMenuTriggerPrimitive state=VisibilityState::Closed class=class>
            {children()}
        </NavigationMenuTriggerPrimitive>
    }
}

/// Content sem id wiring — abre via CSS :hover/:focus-within + behavior keyboard
#[component]
pub fn NavigationMenuContent(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <NavigationMenuContentPrimitive class=class>
            {children()}
        </NavigationMenuContentPrimitive>
    }
}

#[component]
pub fn NavigationMenuLink(
    children: Children,
    #[prop(into, default = String::new())] href: String,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <NavigationMenuLinkPrimitive href=href class=class>
            {children()}
        </NavigationMenuLinkPrimitive>
    }
}

#[component]
pub fn NavigationMenuSubItem(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <NavigationMenuSubItemPrimitive class=class>
            {children()}
        </NavigationMenuSubItemPrimitive>
    }
}

#[component]
pub fn NavigationMenuPreview() -> impl IntoView {
    view! {
        <NavigationMenu>
            <NavigationMenuList>
                <NavigationMenuItem>
                    <NavigationMenuTrigger>"Products"</NavigationMenuTrigger>
                    <NavigationMenuContent>
                        <NavigationMenuSubItem>
                            <NavigationMenuLink href="/product-1">"Product 1"</NavigationMenuLink>
                        </NavigationMenuSubItem>
                        <NavigationMenuSubItem>
                            <NavigationMenuLink href="/product-2">"Product 2"</NavigationMenuLink>
                        </NavigationMenuSubItem>
                    </NavigationMenuContent>
                </NavigationMenuItem>
                <NavigationMenuItem>
                    <NavigationMenuLink href="/pricing">"Pricing"</NavigationMenuLink>
                </NavigationMenuItem>
            </NavigationMenuList>
        </NavigationMenu>
    }
}
