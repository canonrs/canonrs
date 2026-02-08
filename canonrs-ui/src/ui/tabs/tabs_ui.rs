use leptos::prelude::*;
use crate::primitives::{
    TabsPrimitive,
    TabsListPrimitive,
    TabsTriggerPrimitive,
    TabsContentPrimitive,
};

#[component]
pub fn Tabs(
    children: Children,
    #[prop(default = String::new())] class_name: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <TabsPrimitive
            attr:class={class_name}
            id=id
        >
            {children()}
        </TabsPrimitive>
    }
}

#[component]
pub fn TabsList(
    children: Children,
    #[prop(default = String::new())] class_name: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <TabsListPrimitive
            class={class_name}
            id=id
        >
            {children()}
        </TabsListPrimitive>
    }
}

#[component]
pub fn TabsTrigger(
    children: Children,
    #[prop(default = -1)] tabindex: i32,
    #[prop(default = String::new())] controls_id: String,
    #[prop(default = false)] selected: bool,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = String::new())] class_name: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <TabsTriggerPrimitive
            tabindex=tabindex
            controls_id=controls_id
            selected=selected
            value=value
            attr:class={class_name}
            id=id
        >
            {children()}
        </TabsTriggerPrimitive>
    }
}

#[component]
pub fn TabsContent(
    children: Children,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = String::new())] content_id: String,
    #[prop(default = String::new())] labelledby: String,
    #[prop(default = String::new())] class_name: String,
) -> impl IntoView {
    view! {
        <TabsContentPrimitive
            value=value
            content_id=content_id
            labelledby=labelledby
            attr:class={class_name}
        >
            {children()}
        </TabsContentPrimitive>
    }
}
