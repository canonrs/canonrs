use leptos::prelude::*;
use canonrs_core::meta::{ActivityState, DisabledState};
use canonrs_core::primitives::{
    TabsPrimitive, TabsListPrimitive,
    TabsTriggerPrimitive, TabsContentPrimitive,
};

#[component]
pub fn Tabs(
    children: Children,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TabsPrimitive class=class node_ref=node_ref.unwrap_or_default()>
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
    children: Children,
    #[prop(into)] value: String,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <TabsTriggerPrimitive value=value active=active disabled=disabled class=class>
            {children()}
        </TabsTriggerPrimitive>
    }
}

#[component]
pub fn TabsContent(
    children: Children,
    #[prop(into)] value: String,
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
