use leptos::prelude::*;
use super::navigation_menu_ui::*;

#[component]
pub fn BasicExample() -> impl IntoView {
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
                <NavigationMenuItem>
                    <NavigationMenuLink href="/docs">"Docs"</NavigationMenuLink>
                </NavigationMenuItem>
            </NavigationMenuList>
        </NavigationMenu>
    }
}
