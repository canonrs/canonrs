//! @canon-level: strict
//! @canon-owner: primitives-team
//! Tabs Primitive - CSS puro com radio inputs + ARIA completo

use leptos::prelude::*;

#[component]
pub fn TabsPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-tabs=""
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
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-tabs-list=""
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
    #[prop(into, default = String::new())] name: String,
    #[prop(into, default = String::new())] value: String,
    #[prop(default = false)] checked: bool,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
    
) -> impl IntoView {
    view! {
        <input
            type="radio"
            data-rs-tabs-input=""
            name=name
            value={value}
            checked=checked
            class=class
            id=id
            
        />
    }
}

#[component]
pub fn TabsTriggerLabelPrimitive(
    children: Children,
    #[prop(into, default = String::new())] for_id: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(default = false)] selected: bool,
    #[prop(into, default = String::new())] id: String,
    #[prop(into, default = String::new())] controls: String,
) -> impl IntoView {
    view! {
        <label
            for=for_id
            data-rs-tabs-trigger=""
            role="tab"
            id=id
            attr:aria-controls=controls
            attr:aria-selected=selected.to_string()
            class=class
        >
            {children()}
        </label>
    }
}

#[component]
pub fn TabsContentPrimitive(
    children: Children,
    #[prop(into, default = String::new())] value: String,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
    #[prop(into, default = String::new())] labelledby: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-tabs-content=""
            data-value={value}
            role="tabpanel"
            attr:aria-labelledby=labelledby
            id=id
            class=class
        >
            {children()}
        </div>
    }
}
