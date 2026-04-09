use leptos::prelude::*;
use super::tabs_island::{TabsRootIsland, TabsTriggerIsland, TabsContentIsland};
use super::tabs_ui::TabsList;

#[component]
pub fn TabsShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <TabsRootIsland>
                    <TabsList>
                        <TabsTriggerIsland value="overview">"Overview"</TabsTriggerIsland>
                        <TabsTriggerIsland value="api">"API"</TabsTriggerIsland>
                        <TabsTriggerIsland value="examples">"Examples"</TabsTriggerIsland>
                    </TabsList>
                    <TabsContentIsland value="overview">"Overview content — structure drives state."</TabsContentIsland>
                    <TabsContentIsland value="api">"API reference content."</TabsContentIsland>
                    <TabsContentIsland value="examples">"Examples content."</TabsContentIsland>
                </TabsRootIsland>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Tab selection governed by DOM — SSR-safe, hydration-safe."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With disabled tab"</span>
                <div data-rs-showcase-preview-row="">
                    <TabsRootIsland>
                        <TabsList>
                            <TabsTriggerIsland value="x">"Active"</TabsTriggerIsland>
                            <TabsTriggerIsland value="y" disabled=true>"Disabled"</TabsTriggerIsland>
                            <TabsTriggerIsland value="z">"Normal"</TabsTriggerIsland>
                        </TabsList>
                        <TabsContentIsland value="x">"Active content."</TabsContentIsland>
                        <TabsContentIsland value="y">"Disabled content."</TabsContentIsland>
                        <TabsContentIsland value="z">"Normal content."</TabsContentIsland>
                    </TabsRootIsland>
                </div>
            </div>
        </div>
    }
}
