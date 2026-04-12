use leptos::prelude::*;
use super::navigation_menu_boundary::{NavigationMenu, NavigationMenuItem, NavigationMenuTrigger, NavigationMenuContent, NavigationMenuLink};

#[component]
pub fn NavigationMenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
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
                        <a data-rs-navigation-menu-link="" href="#">"Pricing"</a>
                    </NavigationMenuItem>
                    <NavigationMenuItem>
                        <a data-rs-navigation-menu-link="" href="#">"Blog"</a>
                    </NavigationMenuItem>
                </NavigationMenu>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Trigger-content relationship governed by DOM — no id wiring needed."
            </p>
        </div>
    }
}
