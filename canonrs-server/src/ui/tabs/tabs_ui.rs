
use leptos::prelude::*;
use canonrs_core::meta::ActivityState;
use canonrs_core::primitives::{
    TabsPrimitive, TabsListPrimitive,
    TabsTriggerPrimitive, TabsContentPrimitive,
};

#[component]
pub fn Tabs(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TabsPrimitive class=class>
            {children()}
        </TabsPrimitive>
    }
}

#[component]
pub fn TabsList(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TabsListPrimitive class=class>
            {children()}
        </TabsListPrimitive>
    }
}

#[component]
pub fn TabsTrigger(
    #[prop(into)] value: String,
    children: Children,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TabsTriggerPrimitive value=value active=active class=class>
            {children()}
        </TabsTriggerPrimitive>
    }
}

#[component]
pub fn TabsContent(
    #[prop(into)] value: String,
    children: Children,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TabsContentPrimitive value=value active=active class=class>
            {children()}
        </TabsContentPrimitive>
    }
}

#[component]
pub fn TabsPreview() -> impl IntoView {
    view! {
        <Tabs>
            <TabsList>
                <TabsTrigger value="tab1" active=ActivityState::Active>"Tab 1"</TabsTrigger>
                <TabsTrigger value="tab2">"Tab 2"</TabsTrigger>
            </TabsList>
            <TabsContent value="tab1" active=ActivityState::Active>"Content 1"</TabsContent>
            <TabsContent value="tab2">"Content 2"</TabsContent>
        </Tabs>
    }
}
