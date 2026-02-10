use leptos::prelude::*;
use crate::primitives::{
    TabsPrimitive,
    TabsListPrimitive,
    TabsTriggerPrimitive,
    TabsTriggerLabelPrimitive,
    TabsContentPrimitive,
};

#[component]
pub fn Tabs(
    #[prop(into)] id: String,
    children: Children,
    #[prop(default = String::new())] class_name: String,
) -> impl IntoView {
    view! {
        <TabsPrimitive
            class={class_name}
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
    #[prop(into)] id: String,
    #[prop(into)] name: String,
    #[prop(into)] value: String,
    children: Children,
    #[prop(default = false)] checked: bool,
    #[prop(default = String::new())] class_name: String,
) -> impl IntoView {
    let id_clone = id.clone();
    view! {
        <TabsTriggerPrimitive
            name=name
            value=value.clone()
            checked=checked
            id=id
            class={class_name.clone()}
        />
        <TabsTriggerLabelPrimitive
            for_id=id_clone
            class={class_name}
        >
            {children()}
        </TabsTriggerLabelPrimitive>
    }
}

#[component]
pub fn TabsContent(
    #[prop(into)] value: String,
    children: Children,
    #[prop(default = String::new())] class_name: String,
) -> impl IntoView {
    view! {
        <TabsContentPrimitive
            value=value
            class={class_name}
        >
            {children()}
        </TabsContentPrimitive>
    }
}
