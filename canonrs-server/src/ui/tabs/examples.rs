use leptos::prelude::*;
use super::{Tabs, TabsList, TabsTrigger, TabsContent};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Tabs>
            <TabsList>
                <TabsTrigger value="tab-1" default=true>"Tab 1"</TabsTrigger>
                <TabsTrigger value="tab-2">"Tab 2"</TabsTrigger>
                <TabsTrigger value="tab-3">"Tab 3"</TabsTrigger>
            </TabsList>
            <TabsContent value="tab-1" default=true><p>"Content for Tab 1"</p></TabsContent>
            <TabsContent value="tab-2"><p>"Content for Tab 2"</p></TabsContent>
            <TabsContent value="tab-3"><p>"Content for Tab 3"</p></TabsContent>
        </Tabs>
    }
}

#[component]
pub fn WithIconsExample() -> impl IntoView {
    view! {
        <Tabs>
            <TabsList>
                <TabsTrigger value="home" default=true>"🏠 Home"</TabsTrigger>
                <TabsTrigger value="profile">"👤 Profile"</TabsTrigger>
                <TabsTrigger value="settings">"⚙️ Settings"</TabsTrigger>
            </TabsList>
            <TabsContent value="home" default=true><p>"Welcome to the home page"</p></TabsContent>
            <TabsContent value="profile"><p>"View your profile information"</p></TabsContent>
            <TabsContent value="settings"><p>"Manage your settings"</p></TabsContent>
        </Tabs>
    }
}

#[component]
pub fn VerticalExample() -> impl IntoView {
    view! {
        <Tabs class="flex flex-row".to_string()>
            <TabsList class="flex flex-col".to_string()>
                <TabsTrigger value="v1" default=true>"Option 1"</TabsTrigger>
                <TabsTrigger value="v2">"Option 2"</TabsTrigger>
                <TabsTrigger value="v3">"Option 3"</TabsTrigger>
            </TabsList>
            <div>
                <TabsContent value="v1" default=true><p>"Vertical tab content 1"</p></TabsContent>
                <TabsContent value="v2"><p>"Vertical tab content 2"</p></TabsContent>
                <TabsContent value="v3"><p>"Vertical tab content 3"</p></TabsContent>
            </div>
        </Tabs>
    }
}
