use leptos::prelude::*;
use super::navigation_menu_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <NavigationMenu>
            <NavigationMenuList>
                <NavigationMenuItem>
                    <NavigationMenuTrigger controls_id="menu-products">"Products"</NavigationMenuTrigger>
                    <NavigationMenuContent content_id="menu-products">
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
                <NavigationMenuItem>
                    <NavigationMenuLink href="/docs">"Docs"</NavigationMenuLink>
                </NavigationMenuItem>
            </NavigationMenuList>
        </NavigationMenu>
    }
}
