//! @canon-level: strict
//! @canon-owner: primitives-team
//! Tabs Primitive - data-rs-state SSR + behavior

use leptos::prelude::*;
use crate::meta::ActivityState;

#[component]
pub fn TabsPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(into, default = String::new())] id: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-tabs=""
            data-rs-component="Tabs"
            data-rs-behavior="navigation"
            class=class
            id=if id.is_empty() { None } else { Some(id) }
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TabsListPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div data-rs-tabs-list="" role="tablist" aria-orientation="horizontal" class=class>
            {children()}
        </div>
    }
}

#[component]
pub fn TabsTriggerPrimitive(
    children: Children,
    #[prop(into)] value: String,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <button
            type="button"
            role="tab"
            data-rs-tabs-trigger=""
            data-rs-value=value
            data-rs-state=active.as_str()
            aria-selected={if active == ActivityState::Active { "true" } else { "false" }}
            class=class
        >
            {children()}
        </button>
    }
}

#[component]
pub fn TabsContentPrimitive(
    children: Children,
    #[prop(into)] value: String,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-tabs-content=""
            data-rs-value=value
            data-rs-state=active.as_str()
            role="tabpanel"
            hidden={active == ActivityState::Inactive}
            class=class
        >
            {children()}
        </div>
    }
}
