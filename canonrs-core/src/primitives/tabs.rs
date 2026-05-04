//! @canon-level: strict
//! @canon-owner: primitives-team
//! Tabs Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{ActivityState, DisabledState};

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum TabsOrientation {
    #[default]
    Horizontal,
    Vertical,
}
impl TabsOrientation {
    pub fn as_str(&self) -> &'static str {
        match self { Self::Horizontal => "horizontal", Self::Vertical => "vertical" }
    }
}


#[component]
pub fn TabsPrimitive(
    children: Children,
    #[prop(into, default = String::new())] class: String,
    #[prop(optional)] node_ref: Option<NodeRef<leptos::html::Div>>,
) -> impl IntoView {
    let uid_tab = crate::infra::uid::generate("tab");
    view! {
        <div
            data-rs-tabs=""
            data-rs-uid=uid_tab
            data-rs-interaction="nav"
            class=class
            node_ref=node_ref.unwrap_or_default()
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TabsListPrimitive(
    children: Children,
    #[prop(default = TabsOrientation::Horizontal)] orientation: TabsOrientation,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    view! {
        <div
            data-rs-tabs-list=""
            role="tablist"
            aria-orientation=orientation.as_str()
            class=class
        >
            {children()}
        </div>
    }
}

#[component]
pub fn TabsTriggerPrimitive(
    children: Children,
    #[prop(into)] value: String,
    #[prop(default = ActivityState::Inactive)] active: ActivityState,
    #[prop(default = DisabledState::Enabled)] disabled: DisabledState,
    #[prop(into, default = String::new())] class: String,
) -> impl IntoView {
    let is_disabled = disabled == DisabledState::Disabled;
    view! {
        <button
            type="button"
            role="tab"
            data-rs-tabs-trigger=""
            data-rs-value=value
            data-rs-activity=active.as_str()
            data-rs-disabled=if is_disabled { Some("disabled") } else { None }
            aria-selected=active.aria_selected()
            aria-disabled=if is_disabled { "true" } else { "false" }
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
            data-rs-activity=active.as_str()
            role="tabpanel"
            hidden=active.hidden()
            class=class
        >
            {children()}
        </div>
    }
}
