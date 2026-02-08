use leptos::prelude::*;
use super::navigation_menu_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <NavigationMenu>
            <NavigationMenuItem>
                <NavigationMenuTrigger>"Products"</NavigationMenuTrigger>
                <NavigationMenuContent>
                    <NavigationMenuLink href="#".to_string()>"Product 1"</NavigationMenuLink>
                    <NavigationMenuLink href="#".to_string()>"Product 2"</NavigationMenuLink>
                </NavigationMenuContent>
            </NavigationMenuItem>
            <NavigationMenuItem>
                <NavigationMenuLink href="#".to_string()>"Pricing"</NavigationMenuLink>
            </NavigationMenuItem>
        </NavigationMenu>
    }
}
