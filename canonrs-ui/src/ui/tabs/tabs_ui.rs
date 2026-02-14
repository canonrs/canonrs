//! @canon-level: ui
//! Tabs - Declarative UI wrapper over TabsPrimitive

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
    #[prop(into, default = String::new())] class_name: String,
) -> impl IntoView {
    view! {
        <TabsPrimitive class={class_name} id=id>
            {children()}
        </TabsPrimitive>
    }
}

#[component]
pub fn TabsList(
    children: Children,
    #[prop(into, default = String::new())] class_name: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <TabsListPrimitive class={class_name} id=id>
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
    #[prop(into, default = String::new())] class_name: String,
) -> impl IntoView {
    let panel_id = format!("panel-{}", value);
    let id_clone = id.clone();
    let label_id = format!("label-{}", id_clone);
    view! {
        <TabsTriggerPrimitive
            name=name
            value=value
            checked=checked
            id=id
            class={class_name.clone()}
            
        />
        <TabsTriggerLabelPrimitive
            for_id=id_clone
            controls=panel_id
            id=label_id
            class={class_name}
            selected=checked
        >
            {children()}
        </TabsTriggerLabelPrimitive>
    }
}

#[component]
pub fn TabsContent(
    #[prop(into)] value: String,
    children: Children,
    #[prop(into, default = String::new())] class_name: String,
    #[prop(optional)] tab_id: Option<String>,
) -> impl IntoView {
    let panel_id = format!("panel-{}", value);
    let labelledby = tab_id.unwrap_or_else(|| format!("label-{}", value));
    view! {
        <TabsContentPrimitive
            value=value
            class={class_name}
            id=panel_id
            labelledby=labelledby
        >
            {children()}
        </TabsContentPrimitive>
    }
}
