use leptos::prelude::*;
use super::navigation_menu_boundary::{
    NavigationMenu, NavigationMenuItem, NavigationMenuTrigger,
    NavigationMenuContent, NavigationMenuLink,
};
use canonrs_core::primitives::layout::stack::{StackPrimitive as Stack, StackDirection, StackGap};

#[component]
pub fn NavigationMenuShowcasePreview() -> impl IntoView {
    view! {
        <Stack direction=StackDirection::Vertical gap=StackGap::Lg>
            <p data-rs-showcase-preview-anchor="">
                "Trigger-content relationship governed by DOM — no id wiring needed."
            </p>
            <NavigationMenu>
                <NavigationMenuItem>
                    <NavigationMenuTrigger>"Products"</NavigationMenuTrigger>
                    <NavigationMenuContent>
                        <NavigationMenuLink href="#">"CanonRS Core"</NavigationMenuLink>
                        <NavigationMenuLink href="#">"CanonRS UI"</NavigationMenuLink>
                        <NavigationMenuLink href="#">"CanonRS Tokens"</NavigationMenuLink>
                    </NavigationMenuContent>
                </NavigationMenuItem>
                <NavigationMenuItem>
                    <NavigationMenuTrigger>"Docs"</NavigationMenuTrigger>
                    <NavigationMenuContent>
                        <NavigationMenuLink href="#">"Getting Started"</NavigationMenuLink>
                        <NavigationMenuLink href="#">"API Reference"</NavigationMenuLink>
                    </NavigationMenuContent>
                </NavigationMenuItem>
                <NavigationMenuItem>
                    <NavigationMenuLink href="#">"Pricing"</NavigationMenuLink>
                </NavigationMenuItem>
                <NavigationMenuItem>
                    <NavigationMenuLink href="#">"Blog"</NavigationMenuLink>
                </NavigationMenuItem>
            </NavigationMenu>
        </Stack>
    }
}
