use leptos::prelude::*;
use super::tabs_ui::{Tabs, TabsList, TabsTrigger, TabsContent};
use canonrs_core::meta::ActivityState;

#[component]
pub fn TabsShowcasePreview() -> impl IntoView {
    view! {
        <div data-rs-showcase-preview-hero="">
            <div data-rs-showcase-preview-stage="">
                <Tabs>
                    <TabsList>
                        <TabsTrigger value="overview" active=ActivityState::Active>"Overview"</TabsTrigger>
                        <TabsTrigger value="api">"API"</TabsTrigger>
                        <TabsTrigger value="examples">"Examples"</TabsTrigger>
                    </TabsList>
                    <TabsContent value="overview" active=ActivityState::Active>
                        "Overview content — structure drives state."
                    </TabsContent>
                    <TabsContent value="api">
                        "API reference content."
                    </TabsContent>
                    <TabsContent value="examples">
                        "Examples content."
                    </TabsContent>
                </Tabs>
            </div>
            <p data-rs-showcase-preview-anchor="">
                "Tab selection fully governed by data-rs-state — zero JS on SSR."
            </p>
            <div data-rs-showcase-preview-section="">
                <span data-rs-showcase-preview-label="">"Second tab active"</span>
                <div data-rs-showcase-preview-row="">
                    <Tabs>
                        <TabsList>
                            <TabsTrigger value="a">"Tab A"</TabsTrigger>
                            <TabsTrigger value="b" active=ActivityState::Active>"Tab B"</TabsTrigger>
                            <TabsTrigger value="c">"Tab C"</TabsTrigger>
                        </TabsList>
                        <TabsContent value="a">"Content A"</TabsContent>
                        <TabsContent value="b" active=ActivityState::Active>"Content B"</TabsContent>
                        <TabsContent value="c">"Content C"</TabsContent>
                    </Tabs>
                </div>
            </div>
        </div>
    }
}
