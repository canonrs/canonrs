//! @canon-level: strict
//! @canon-owner: primitives-team
//! Tabs Primitive - CSS puro com radio inputs

use leptos::prelude::*;

#[component]
pub fn TabsPrimitive(
    children: Children,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-tabs=""
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
            data-tabs-list=""
            role="tablist"
            class=class
            id=id
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TabsTriggerPrimitive(
    #[prop(default = String::new())] name: String,
    #[prop(default = String::new())] value: String,
    #[prop(default = false)] checked: bool,
    #[prop(default = String::new())] class: String,
    #[prop(default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <input
            type="radio"
            data-tabs-input=""
            name=name
            value=value
            checked=checked
            class=class
            id=id
        />
    }
}

#[component]
pub fn TabsTriggerLabelPrimitive(
    children: Children,
    #[prop(default = String::new())] for_id: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <label
            for=for_id
            data-tabs-trigger=""
            class=class
        >
            {children()}
        </label>
    }
}

#[component]
pub fn TabsContentPrimitive(
    children: Children,
    #[prop(default = String::new())] value: String,
    #[prop(default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-tabs-content=""
            data-value=value
            class=class
        >
            {children()}
        </div>
    }
}
