//! @canon-level: strict
//! @canon-owner: primitives-team
//! Tabs Primitive - HTML puro + ARIA
//! Base: Roving tabindex + painéis controlados

use leptos::prelude::*;

#[component]
pub fn TabsPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-tabs=""
            class=class
            id=id
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TabsListPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-tabs-list=""
            role="tablist"
            attr:aria-orientation="horizontal"
            class=class
            id=id
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TabsTriggerPrimitive(
    children: Children,
    #[prop(default = -1)] tabindex: i32,
    #[prop(default = String::new())] controls_id: String,
    #[prop(default = false)] selected: bool,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <button
            attr:data-tabs-trigger=""
            attr:data-value={value}
            type="button"
            role="tab"
            tabindex={tabindex}
            attr:aria-controls={controls_id}
            attr:aria-selected={if selected { "true" } else { "false" }}
            class=class
            id=id
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabsContentPrimitive(
    children: Children,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] content_id: String,
    #[prop(default = String::new())] labelledby: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            attr:data-tabs-content=""
            attr:data-value={value}
            role="tabpanel"
            id={content_id}
            attr:aria-labelledby={labelledby}
            tabindex="-1"
            class=class
        >
            {children()}
        </div>
    }
}
