use leptos::prelude::*;
use super::{Tabs, TabsList, TabsTrigger, TabsContent};

#[component]
pub fn BasicExample() -> impl IntoView {
    view! {
        <Tabs id="tabs-basic" default="tab-1">
            <TabsList>
                <TabsTrigger id="tab-1" value="tab-1">"Tab 1"</TabsTrigger>
                <TabsTrigger id="tab-2" value="tab-2">"Tab 2"</TabsTrigger>
                <TabsTrigger id="tab-3" value="tab-3">"Tab 3"</TabsTrigger>
            </TabsList>
            <TabsContent value="tab-1"><p>"Content for Tab 1"</p></TabsContent>
            <TabsContent value="tab-2"><p>"Content for Tab 2"</p></TabsContent>
            <TabsContent value="tab-3"><p>"Content for Tab 3"</p></TabsContent>
        </Tabs>
    }
}

#[component]
pub fn with_icons_example() -> impl IntoView {
    view! {
        <Tabs id="tabs-icons" default="home">
            <TabsList>
                <TabsTrigger id="tab-home" value="home">"🏠 Home"</TabsTrigger>
                <TabsTrigger id="tab-profile" value="profile">"👤 Profile"</TabsTrigger>
                <TabsTrigger id="tab-settings" value="settings">"⚙️ Settings"</TabsTrigger>
            </TabsList>
            <TabsContent value="home"><p>"Welcome to the home page"</p></TabsContent>
            <TabsContent value="profile"><p>"View your profile information"</p></TabsContent>
            <TabsContent value="settings"><p>"Manage your settings"</p></TabsContent>
        </Tabs>
    }
}

#[component]
pub fn vertical_example() -> impl IntoView {
    view! {
        <Tabs id="tabs-vertical" default="v1" class_name="flex flex-row".to_string()>
            <TabsList class_name="flex flex-col".to_string()>
                <TabsTrigger id="tab-v1" value="v1">"Option 1"</TabsTrigger>
                <TabsTrigger id="tab-v2" value="v2">"Option 2"</TabsTrigger>
                <TabsTrigger id="tab-v3" value="v3">"Option 3"</TabsTrigger>
            </TabsList>
            <div>
                <TabsContent value="v1"><p>"Vertical tab content 1"</p></TabsContent>
                <TabsContent value="v2"><p>"Vertical tab content 2"</p></TabsContent>
                <TabsContent value="v3"><p>"Vertical tab content 3"</p></TabsContent>
            </div>
        </Tabs>
    }
}
