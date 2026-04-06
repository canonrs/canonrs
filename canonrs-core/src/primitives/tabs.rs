//! @canon-level: strict
//! @canon-owner: primitives-team
//! Tabs Primitive - HTML puro + ARIA

use leptos::prelude::*;
use crate::meta::{ActivityState, DisabledState};
use crate::infra::state_engine::activity_attrs;

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
    view! {
        <div
            data-rs-tabs=""
            data-rs-component="Tabs"
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
    let s = activity_attrs(active);
    let state_str = if disabled == DisabledState::Disabled {
        format!("{} disabled", s.data_rs_state)
    } else {
        s.data_rs_state.to_string()
    };
    let aria_disabled = if disabled == DisabledState::Disabled { "true" } else { "false" };
    view! {
        <button
            type="button"
            role="tab"
            data-rs-tabs-trigger=""
            data-rs-value=value
            data-rs-state=state_str
            aria-selected=s.aria_selected
            aria-disabled=aria_disabled
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
    let s = activity_attrs(active);
    view! {
        <div
            data-rs-tabs-content=""
            data-rs-value=value
            data-rs-state=s.data_rs_state
            role="tabpanel"
            hidden=s.hidden
            class=class
        >
            {children()}
        </div>
    }
}
