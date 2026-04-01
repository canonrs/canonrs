use leptos::prelude::*;
use super::navigation_menu_ui::{
    NavigationMenu, NavigationMenuList, NavigationMenuItem,
    NavigationMenuTrigger, NavigationMenuContent,
    NavigationMenuLink, NavigationMenuSubItem,
};

#[component]
pub fn NavigationMenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <NavigationMenu>
                    <NavigationMenuList>
                        <NavigationMenuItem>
                            <NavigationMenuTrigger>"Products"</NavigationMenuTrigger>
                            <NavigationMenuContent>
                                <NavigationMenuSubItem>
                                    <NavigationMenuLink href="#">"CanonRS Core"</NavigationMenuLink>
                                </NavigationMenuSubItem>
                                <NavigationMenuSubItem>
                                    <NavigationMenuLink href="#">"CanonRS UI"</NavigationMenuLink>
                                </NavigationMenuSubItem>
                                <NavigationMenuSubItem>
                                    <NavigationMenuLink href="#">"CanonRS Tokens"</NavigationMenuLink>
                                </NavigationMenuSubItem>
                            </NavigationMenuContent>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                            <NavigationMenuTrigger>"Docs"</NavigationMenuTrigger>
                            <NavigationMenuContent>
                                <NavigationMenuSubItem>
                                    <NavigationMenuLink href="#">"Getting Started"</NavigationMenuLink>
                                </NavigationMenuSubItem>
                                <NavigationMenuSubItem>
                                    <NavigationMenuLink href="#">"API Reference"</NavigationMenuLink>
                                </NavigationMenuSubItem>
                            </NavigationMenuContent>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                            <NavigationMenuLink href="#">"Pricing"</NavigationMenuLink>
                        </NavigationMenuItem>
                        <NavigationMenuItem>
                            <NavigationMenuLink href="#">"Blog"</NavigationMenuLink>
                        </NavigationMenuItem>
                    </NavigationMenuList>
                </NavigationMenu>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Trigger-content relationship enforced without id wiring."
            </p>
        </div>
    }
}
