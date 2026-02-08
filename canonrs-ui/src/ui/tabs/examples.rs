use leptos::prelude::*;
use super::tabs_ui::*;

pub fn basic_example() -> impl IntoView {
    view! {
        <Tabs>
            <TabsList>
                <TabsTrigger value="tab-1".to_string() controls_id="content-1".to_string() selected=true>"Tab 1"</TabsTrigger>
                <TabsTrigger value="tab-2".to_string() controls_id="content-2".to_string()>"Tab 2"</TabsTrigger>
                <TabsTrigger value="tab-3".to_string() controls_id="content-3".to_string()>"Tab 3"</TabsTrigger>
            </TabsList>
            <TabsContent value="tab-1".to_string() content_id="content-1".to_string() labelledby="tab-1".to_string()>
                <p>"Content of tab 1."</p>
            </TabsContent>
            <TabsContent value="tab-2".to_string() content_id="content-2".to_string() labelledby="tab-2".to_string()>
                <p>"Content of tab 2."</p>
            </TabsContent>
            <TabsContent value="tab-3".to_string() content_id="content-3".to_string() labelledby="tab-3".to_string()>
                <p>"Content of tab 3."</p>
            </TabsContent>
        </Tabs>
    }
}
