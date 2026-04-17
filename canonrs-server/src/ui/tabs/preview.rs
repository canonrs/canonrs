use leptos::prelude::*;
use super::tabs_boundary::{TabsRoot, TabsListBoundary, TabsTrigger, TabsContent};

#[component]
pub fn TabsShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <TabsRoot>
                    <TabsListBoundary>
                        <TabsTrigger value="overview">"Overview"</TabsTrigger>
                        <TabsTrigger value="api">"API"</TabsTrigger>
                        <TabsTrigger value="examples">"Examples"</TabsTrigger>
                    </TabsListBoundary>
                    <TabsContent value="overview">"Overview content — structure drives state."</TabsContent>
                    <TabsContent value="api">"API reference content."</TabsContent>
                    <TabsContent value="examples">"Examples content."</TabsContent>
                </TabsRoot>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Tab selection governed by DOM — SSR-safe, hydration-safe."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"With disabled tab"</span>
                <div data-rs-showcase-preview-row="">
                    <TabsRoot>
                        <TabsListBoundary>
                            <TabsTrigger value="x">"Active"</TabsTrigger>
                            <TabsTrigger value="y" disabled=true>"Disabled"</TabsTrigger>
                            <TabsTrigger value="z">"Normal"</TabsTrigger>
                        </TabsListBoundary>
                        <TabsContent value="x">"Active content."</TabsContent>
                        <TabsContent value="y">"Disabled content."</TabsContent>
                        <TabsContent value="z">"Normal content."</TabsContent>
                    </TabsRoot>
                </div>
            </div>
        </div>
    }
}
