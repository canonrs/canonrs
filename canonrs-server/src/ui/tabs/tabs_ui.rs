//! @canon-level: ui
//! Tabs - estado via behavior JS (padrão accordion/dialog)

use leptos::prelude::*;

#[derive(Clone)]
struct TabsDefault(String);

#[component]
pub fn Tabs(
    children: Children,
    #[prop(into)] id: String,
    #[prop(into, default = String::new())] default: String,
    #[prop(into, default = String::new())] class_name: String,
) -> impl IntoView {
    provide_context(TabsDefault(default.clone()));
    view! {
        <div data-tabs="" id=id data-default=default class=class_name>
            {children()}
        </div>
    }
}

#[component]
pub fn TabsList(
    children: Children,
    #[prop(into, default = String::new())] class_name: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div data-tabs-list="" role="tablist" id=id class=class_name>
            {children()}
        </div>
    }
}

#[component]
pub fn TabsTrigger(
    #[prop(into)] value: String,
    children: Children,
    #[prop(into, default = String::new())] id: String,
    #[prop(into, default = String::new())] class_name: String,
) -> impl IntoView {
    let default = use_context::<TabsDefault>().map(|d| d.0).unwrap_or_default();
    let is_default = value == default;
    let panel_id = format!("panel-{}", value);
    view! {
        <button
            id=id
            role="tab"
            class=class_name
            data-tabs-trigger=""
            data-tabs-value=value.clone()
            aria-controls=panel_id
            aria-selected=if is_default { "true" } else { "false" }
            data-state=if is_default { "active" } else { "inactive" }
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabsContent(
    #[prop(into)] value: String,
    children: Children,
    #[prop(into, default = String::new())] class_name: String,
) -> impl IntoView {
    let default = use_context::<TabsDefault>().map(|d| d.0).unwrap_or_default();
    let is_default = value == default;
    let panel_id = format!("panel-{}", value);
    view! {
        <div
            data-tabs-content=""
            data-value=value.clone()
            role="tabpanel"
            id=panel_id
            class=class_name
            data-state=if is_default { "active" } else { "inactive" }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TabsPreview() -> impl IntoView {
    view! {
        <Tabs id="tabs-preview".to_string() default="tab1".to_string()>
            <TabsList>
                <TabsTrigger value="tab1".to_string()>"Tab 1"</TabsTrigger>
                <TabsTrigger value="tab2".to_string()>"Tab 2"</TabsTrigger>
            </TabsList>
            <TabsContent value="tab1".to_string()>"Content 1"</TabsContent>
        </Tabs>
    }
}
