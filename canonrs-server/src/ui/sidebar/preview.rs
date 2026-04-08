use leptos::prelude::*;
use super::sidebar_island::{
    SidebarIsland, SidebarHeaderIsland, SidebarContentIsland,
    SidebarMenuIsland, SidebarMenuItemIsland, SidebarMenuGroupIsland,
    SidebarGroupLabelIsland,
};
use canonrs_core::meta::ActivityState;

#[component]
pub fn SidebarShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <SidebarIsland>
                    <SidebarHeaderIsland>"CanonRS"</SidebarHeaderIsland>
                    <SidebarContentIsland>
                        <SidebarMenuGroupIsland label="Main">
                            <SidebarGroupLabelIsland>"Main"</SidebarGroupLabelIsland>
                            <SidebarMenuIsland>
                                <SidebarMenuItemIsland href="#" active=ActivityState::Active>"Dashboard"</SidebarMenuItemIsland>
                                <SidebarMenuItemIsland href="#">"Components"</SidebarMenuItemIsland>
                                <SidebarMenuItemIsland href="#">"Tokens"</SidebarMenuItemIsland>
                            </SidebarMenuIsland>
                        </SidebarMenuGroupIsland>
                        <SidebarMenuGroupIsland label="Settings">
                            <SidebarGroupLabelIsland>"Settings"</SidebarGroupLabelIsland>
                            <SidebarMenuIsland>
                                <SidebarMenuItemIsland href="#">"Preferences"</SidebarMenuItemIsland>
                                <SidebarMenuItemIsland href="#">"Team"</SidebarMenuItemIsland>
                            </SidebarMenuIsland>
                        </SidebarMenuGroupIsland>
                    </SidebarContentIsland>
                </SidebarIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Navigation state and structure governed by DOM — SSR-safe, hydration-safe."
            </p>
        </div>
    }
}
