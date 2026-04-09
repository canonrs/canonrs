use leptos::prelude::*;
use super::navigation_menu_island::{NavigationMenuIsland, NavigationMenuItemIsland, NavigationMenuTriggerIsland, NavigationMenuContentIsland, NavigationMenuLinkIsland};

#[component]
pub fn NavigationMenuShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <NavigationMenuIsland>
                    <NavigationMenuItemIsland>
                        <NavigationMenuTriggerIsland>"Products"</NavigationMenuTriggerIsland>
                        <NavigationMenuContentIsland>
                            <NavigationMenuLinkIsland href="#">"CanonRS Core"</NavigationMenuLinkIsland>
                            <NavigationMenuLinkIsland href="#">"CanonRS UI"</NavigationMenuLinkIsland>
                            <NavigationMenuLinkIsland href="#">"CanonRS Tokens"</NavigationMenuLinkIsland>
                        </NavigationMenuContentIsland>
                    </NavigationMenuItemIsland>
                    <NavigationMenuItemIsland>
                        <NavigationMenuTriggerIsland>"Docs"</NavigationMenuTriggerIsland>
                        <NavigationMenuContentIsland>
                            <NavigationMenuLinkIsland href="#">"Getting Started"</NavigationMenuLinkIsland>
                            <NavigationMenuLinkIsland href="#">"API Reference"</NavigationMenuLinkIsland>
                        </NavigationMenuContentIsland>
                    </NavigationMenuItemIsland>
                    <NavigationMenuItemIsland>
                        <a data-rs-navigation-menu-link="" href="#">"Pricing"</a>
                    </NavigationMenuItemIsland>
                    <NavigationMenuItemIsland>
                        <a data-rs-navigation-menu-link="" href="#">"Blog"</a>
                    </NavigationMenuItemIsland>
                </NavigationMenuIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Trigger-content relationship governed by DOM — no id wiring needed."
            </p>
        </div>
    }
}
